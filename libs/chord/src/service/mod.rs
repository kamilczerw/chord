#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, sync_channel, SyncSender};
use log::info;
use crate::{Client, Node, NodeRef, NodeUpdateTask};
use crate::client::ClientError;

pub struct NodeService<C: Client> {
    node: Arc<RwLock<Node>>,
    phantom: PhantomData<C>,

    executor: TaskExecutor<NodeUpdateTask, NodeContext>,
}

impl Task<NodeContext> for NodeUpdateTask {
    fn execute(&self, context: &NodeContext) {
        info!("Executing task");
        match self {
            NodeUpdateTask::UpdateSuccessor(node_ref, tx) => {
                let node = context.node.try_write();
                node.unwrap().successor = node_ref.clone();
                info!("Received update successor task for: {}", node_ref.id);
                tx.send(()).unwrap();
            }
            NodeUpdateTask::UpdatePredecessor(node_ref, tx) => {
                context.node.try_write().unwrap().predecessor = Some(node_ref.clone());
                info!("Received update predecessor task for: {}", node_ref.id);
                tx.send(()).unwrap();
            }
            NodeUpdateTask::UnsetPredecessor(tx) => {
                context.node.try_write().unwrap().predecessor = None;
                info!("Received unset predecessor task");
                tx.send(()).unwrap();
            }
        }
    }
}

trait Task<Ctx>: Sized {
    fn execute(&self, ctx: &Ctx);
}

struct TaskExecutor<T: Task<Ctx>, Ctx: 'static> {
    sender: SyncSender<T>,
    phantom: PhantomData<Ctx>,
}

#[derive(Clone)]
struct NodeContext {
    node: Arc<RwLock<Node>>,
}

impl <T, Ctx> TaskExecutor<T, Ctx>
where
    T: Task<Ctx> + Send + 'static,
    Ctx: Send
{
    fn new(ctx: Ctx) -> Self {
        let (tx, rx) = sync_channel::<T>(10);
        std::thread::spawn(move || {
            while let Ok(task) = rx.recv() {
                task.execute(&ctx);
            }
        });

        Self {
            sender: tx,
            phantom: PhantomData,
        }
    }

    fn execute(&self, task: T) {
        self.sender.send(task).unwrap();
    }
}


impl<C: Client> NodeService<C> {
    pub fn new(node: Node) -> Arc<Self> {
        let node = Arc::new(RwLock::new(node));

        let ctx = NodeContext {
            node: Arc::clone(&node),
        };

        let task_executor = TaskExecutor::new(ctx);

        let service = Arc::new(Self {
            node,
            phantom: PhantomData,
            executor: task_executor,
        });

        service
    }

    /// Find the successor of the given id.
    ///
    /// If the given id is in the range of the current node and its successor, the successor is returned.
    /// Otherwise, the successor of the closest preceding node is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to find the successor for
    pub async fn find_successor(&self, id: u64) -> Result<NodeRef, error::ServiceError> {
        let (node_id, successor) = {
            let node = self.node.read().unwrap();
            (node.id, node.successor.clone())
        };
        if Node::is_between_on_ring(id, node_id, successor.id) {
            Ok(successor.clone())
        } else {
            let client: C = self.closest_preceding_node(id).client();
            let successor = client.find_successor(id).await?;
            Ok(successor)
        }
    }

    fn closest_preceding_node(&self, _id: u64) -> NodeRef {
        self.node.clone().read().unwrap().successor.clone()
    }

    /// Join the chord ring.
    ///
    /// This method is used to join the chord ring. It will find the successor of its own id
    /// and set it as the successor.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to join the ring with. It's an existing node in the ring.
    pub async fn join(&self, node: NodeRef) -> Result<(), error::ServiceError> {
        let successor = {
            let self_node = self.node.read().unwrap();
            let client: C = node.client();
            client.find_successor(self_node.id).await?
        };
        let (tx, rx) = channel();
        self.executor.execute(NodeUpdateTask::UpdateSuccessor(successor.clone(), tx));

        rx.recv().unwrap();

        Ok(())
    }

    /// Notify the node about a potential new predecessor.
    ///
    /// If the predecessor is not set or the given node is in the range of the current node and the
    /// predecessor, the predecessor is set to the given node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node which might be the new predecessor
    pub fn notify(&self, node: NodeRef) {
        let node_id = {
            let node = self.node.read().unwrap();
            node.id
        };
        let predecessor = self.predecessor();
        if predecessor.is_none() || Node::is_between_on_ring(node.id.clone(), predecessor.as_ref().unwrap().id, node_id) {
            let (tx, rx) = channel();
            self.executor.execute(NodeUpdateTask::UpdatePredecessor(node.clone(), tx));
            rx.recv().unwrap();
            // self.node.predecessor = Some(node);
        }
    }

    /// Stabilize the node
    ///
    /// This method is used to stabilize the node. It will check if a predecessor of the successor
    /// is in the range of the current node and its successor. If so, the successor will be set to
    /// the retrieved predecessor.
    ///
    /// It will also notify the successor about the current node.
    ///
    /// > **Note**
    /// >
    /// > This method should be called periodically.
    pub async fn stabilize(&self) -> Result<(), error::ServiceError> {
        let (successor, node_id) = {
            let node = self.node.read().unwrap();
            (node.successor.clone(), node.id)
        };
        let client: C = successor.client();

        let result = client.predecessor().await;
        if let Ok(Some(x)) = result {
            if Node::is_between_on_ring(x.id.clone(), node_id, successor.id) {
                println!("Set the successor to {}", x.id);
                let (tx, rx) = channel();
                self.executor.execute(NodeUpdateTask::UpdateSuccessor(x, tx));
                rx.recv().unwrap();
            }
        }

        let node = self.node.read().unwrap();
        let client: C = node.successor.client();
        client.notify(node.node_ref()).await?;

        Ok(())
    }

    /// Check predecessor
    ///
    /// This method is used to check if the predecessor is still alive. If not, the predecessor is
    /// set to `None`.
    ///
    /// > **Note**
    /// >
    /// > This method should be called periodically.
    pub async fn check_predecessor(&self) {
        let node = self.node.read().unwrap();
        if let Some(predecessor) = &node.predecessor {
            let client: C = predecessor.client();
            if let Err(ClientError::ConnectionFailed(_)) = client.ping().await {
                drop(node);
                let (tx, rx) = channel();
                self.executor.execute(NodeUpdateTask::UnsetPredecessor(tx));
                rx.recv().unwrap();
                // self.node.predecessor = None;
            };
        }
    }

    pub fn update_successor(&self, successor: NodeRef) {
        let (tx, rx) = channel();
        self.executor.execute(NodeUpdateTask::UpdateSuccessor(successor, tx));
        rx.recv().unwrap();
    }

    pub fn successor(&self) -> NodeRef {
        let node = self.node.read().unwrap();
        node.successor.clone()
    }

    pub fn predecessor(&self) -> Option<NodeRef> {
        let node = self.node.read().unwrap();
        node.predecessor.clone()
    }
}

pub mod error {
    use std::fmt::Display;
    use crate::client;

    #[derive(Debug)]
    pub enum ServiceError {
        Unexpected(String),
    }

    impl From<client::ClientError> for ServiceError {
        fn from(err: client::ClientError) -> Self {
            Self::Unexpected(format!("Client error: {}", err))
        }
    }

    impl Display for ServiceError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unexpected(message) => write!(f, "{}", message),
            }
        }
    }
}
