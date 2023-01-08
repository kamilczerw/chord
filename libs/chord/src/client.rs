use std::fmt::{Display, Formatter};
use crate::NodeRef;
use mockall::automock;

#[async_trait::async_trait]
#[automock]
pub trait Client {

    /// Init the client
    ///
    /// # Arguments
    ///
    /// * `node` - The node to connect to
    fn init(node: NodeRef) -> Self;

    /// Find a successor of a given id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to find the successor for
    async fn find_successor(&self, id: u64) -> Result<NodeRef, ClientError>;

    /// Get the successor of the node
    async fn successor(&self) -> Result<NodeRef, ClientError>;

    /// Get the predecessor of the node
    async fn predecessor(&self) -> Result<Option<NodeRef>, ClientError>;

    /// Notify the node about a new predecessor
    ///
    /// # Arguments
    ///
    /// * `predecessor` - The new predecessor
    async fn notify(&self, predecessor: NodeRef) -> Result<(), ClientError>;

    /// Ping the node
    async fn ping(&self) -> Result<(), ClientError>;
}

pub enum ClientError {
    ConnectionFailed(NodeRef),
    Unexpected(String),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ConnectionFailed(node) => write!(f, "Connection to node {} failed", node.addr()),
            ClientError::Unexpected(message) => write!(f, "Unexpected error: {}", message),
        }
    }
}