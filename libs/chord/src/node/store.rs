use crate::Node;

/// A node in the chord ring
///
/// This struct is used to represent a node in the chord ring.
pub struct NodeStore {
    pub(crate) successor: Node,
    pub(crate) predecessor: Option<Node>,
}

impl NodeStore {
    /// Create a new node
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node
    /// * `socket_addr` - The socket address of the node
    pub fn new(successor: Node) -> Self {
        Self {
            successor,
            predecessor: None,
        }
    }
}
