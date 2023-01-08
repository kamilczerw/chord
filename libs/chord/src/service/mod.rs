#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use crate::{Client, Node, NodeRef};

pub struct NodeService<C: Client> {
    node: Node,
    phantom: PhantomData<C>,
}


impl<C: Client> NodeService<C> {
    pub fn new(node: Node) -> Self {
        Self {
            node,
            phantom: PhantomData,
        }
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
        if Node::is_between_on_ring(id, self.node.id, self.node.successor.id) {
            Ok(self.node.successor.clone())
        } else {
            let client: C = self.closest_preceding_node(id).client();
            let successor = client.find_successor(id).await?;
            Ok(successor)
        }
    }

    fn closest_preceding_node(&self, _id: u64) -> &NodeRef {
        &self.node.successor
    }

    /// Join the chord ring.
    ///
    /// This method is used to join the chord ring. It will find the successor of its own id
    /// and set it as the successor.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to join the ring with. It's an existing node in the ring.
    pub async fn join(&mut self, node: NodeRef) -> Result<(), error::ServiceError> {
        let client: C = node.client();
        let successor = client.find_successor(self.node.id).await?;
        self.node.successor = successor;

        Ok(())
    }
    //
    // /// Notify the node about a potential new predecessor.
    // ///
    // /// If the predecessor is not set or the given node is in the range of the current node and the
    // /// predecessor, the predecessor is set to the given node.
    // ///
    // /// # Arguments
    // ///
    // /// * `node` - The node which might be the new predecessor
    // pub fn notify(&mut self, node: NodeRef) {
    //     todo!("not implemented")
    // }
    //
    // /// Stabilize the node
    // ///
    // /// This method is used to stabilize the node. It will check if a predecessor of the successor
    // /// is in the range of the current node and its successor. If so, the successor will be set to
    // /// the retrieved predecessor.
    // ///
    // /// It will also notify the successor about the current node.
    // ///
    // /// > **Note**
    // /// >
    // /// > This method should be called periodically.
    // pub async fn stabilize(&mut self) -> Result<(), error::ServiceError> {
    //     todo!("not implemented")
    // }
    //
    // /// Check predecessor
    // ///
    // /// This method is used to check if the predecessor is still alive. If not, the predecessor is
    // /// set to `None`.
    // ///
    // /// > **Note**
    // /// >
    // /// > This method should be called periodically.
    // pub async fn check_predecessor(&mut self) {
    //     todo!("not implemented")
    // }
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
