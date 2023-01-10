use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use crate::NodeRef;
use mockall::automock;

#[automock]
pub trait Client {

    /// Init the client
    ///
    /// # Arguments
    ///
    /// * `addr` - The node address to connect to
    fn init(addr: SocketAddr) -> Self;

    /// Find a successor of a given id.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to find the successor for
    fn find_successor(&self, id: u64) -> Result<NodeRef, ClientError>;

    /// Get the successor of the node
    fn successor(&self) -> Result<NodeRef, ClientError>;

    /// Get the predecessor of the node
    fn predecessor(&self) -> Result<Option<NodeRef>, ClientError>;

    /// Notify the node about a new predecessor
    ///
    /// # Arguments
    ///
    /// * `predecessor` - The new predecessor
    fn notify(&self, predecessor: NodeRef) -> Result<(), ClientError>;

    /// Ping the node
    fn ping(&self) -> Result<(), ClientError>;
}

pub enum ClientError {
    ConnectionFailed(NodeRef),
    Unexpected(String),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ConnectionFailed(node) => write!(f, "Connection to node {} failed", node.addr()),
            ClientError::Unexpected(message) => write!(f, "{}", message),
        }
    }
}
