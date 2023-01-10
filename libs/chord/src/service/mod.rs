#[cfg(test)]
mod tests;

use std::marker::PhantomData;
use std::net::SocketAddr;
use seahash::hash;
use crate::{Client, NodeStore, NodeRef};
use crate::client::ClientError;

pub struct NodeService<C: Client> {
    id: u64,
    addr: SocketAddr,
    store: NodeStore,
    phantom: PhantomData<C>,
}


impl<C: Client> NodeService<C> {
    pub fn new(socket_addr: SocketAddr) -> Self {
        let id = hash(socket_addr.ip().to_string().as_bytes());
        Self::with_id(id, socket_addr)
    }

    fn with_id(id: u64, addr: SocketAddr) -> Self {
        let store = NodeStore::new(NodeRef::with_id(id, addr));
        Self {
            id,
            addr,
            store,
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
    pub fn find_successor(&self, id: u64) -> Result<NodeRef, error::ServiceError> {
        if NodeStore::is_between_on_ring(id, self.id, self.store.successor.id) {
            Ok(self.store.successor.clone())
        } else {
            let client: C = self.closest_preceding_node(id).client();
            let successor = client.find_successor(id)?;
            Ok(successor)
        }
    }

    /// Join the chord ring.
    ///
    /// This method is used to join the chord ring. It will find the successor of its own id
    /// and set it as the successor.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to join the ring with. It's an existing node in the ring.
    pub fn join(&mut self, node: NodeRef) -> Result<(), error::ServiceError> {
        let client: C = node.client();
        let successor = client.find_successor(self.id)?;
        self.store.successor = successor;

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
    pub fn notify(&mut self, node: NodeRef) {
        if self.store.predecessor.is_none() || NodeStore::is_between_on_ring(node.id.clone(), self.store.predecessor.as_ref().unwrap().id, self.id) {
            self.store.predecessor = Some(node);
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
        pub fn stabilize(&mut self) -> Result<(), error::ServiceError> {
            let client: C = self.store.successor.client();
            let result = client.predecessor();
            if let Ok(Some(x)) = result {
                if NodeStore::is_between_on_ring(x.id.clone(), self.id, self.store.successor.id) {
                    self.store.successor = x;
                }
            }

            let client: C = self.store.successor.client();
            client.notify(NodeRef::with_id(self.id, self.addr))?;

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
    pub fn check_predecessor(&mut self) {
        if let Some(predecessor) = &self.store.predecessor {
            let client: C = predecessor.client();
            if let Err(ClientError::ConnectionFailed(_)) = client.ping() {
                self.store.predecessor = None;
            };
        }
    }

    fn closest_preceding_node(&self, _id: u64) -> &NodeRef {
        &self.store.successor
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
