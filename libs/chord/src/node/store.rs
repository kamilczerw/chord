use crate::Node;

/// A node in the chord ring
///
/// This struct is used to represent a node in the chord ring.
pub struct NodeStore {
    successor: Node,
    predecessor: Option<Node>,
}

impl NodeStore {
    /// Create a new node
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node
    /// * `socket_addr` - The socket address of the node
    pub(crate) fn new(successor: Node) -> Self {
        Self {
            successor,
            predecessor: None,
        }
    }

    /// Set the predecessor of the node
    ///
    /// # Arguments
    ///
    /// * `predecessor` - The predecessor node
    pub(crate) fn set_predecessor(&mut self, predecessor: Node) {
        self.predecessor = Some(predecessor);
    }

    /// Unset the predecessor of the node
    pub(crate) fn unset_predecessor(&mut self) {
        self.predecessor = None;
    }

    /// Get the predecessor of the node
    pub(crate) fn predecessor(&self) -> Option<&Node> {
        self.predecessor.as_ref()
    }

    /// Set the successor of the node
    ///
    /// # Arguments
    ///
    /// * `successor` - The successor node
    pub(crate) fn set_successor(&mut self, successor: Node) {
        self.successor = successor;
    }

    /// Get the successor of the node
    pub(crate) fn successor(&self) -> &Node {
        &self.successor
    }
}


#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::with_id(1, SocketAddr::from(([127, 0, 0, 1], 42001)));
        let store = NodeStore::new(node.clone());

        assert_eq!(store.successor(), &node);
        assert_eq!(store.predecessor(), None);
    }

    #[test]
    fn test_predecessor() {
        let node = Node::with_id(1, SocketAddr::from(([127, 0, 0, 1], 42001)));
        let mut store = NodeStore::new(node.clone());
        let predecessor = Node::with_id(2, SocketAddr::from(([127, 0, 0, 1], 42002)));
        assert_eq!(store.predecessor(), None);
        store.set_predecessor(predecessor.clone());

        assert_eq!(store.predecessor(), Some(&predecessor));

        store.unset_predecessor();
        assert_eq!(store.predecessor(), None);
    }

    #[test]
    fn test_successor() {
        let node = Node::with_id(1, SocketAddr::from(([127, 0, 0, 1], 42001)));
        let mut store = NodeStore::new(node.clone());
        let successor = Node::with_id(2, SocketAddr::from(([127, 0, 0, 1], 42002)));
        assert_eq!(store.successor(), &node);
        store.set_successor(successor.clone());

        assert_eq!(store.successor(), &successor);
    }
}
