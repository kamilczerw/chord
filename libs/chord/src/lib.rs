mod client;
mod service;
mod node;

use std::net::SocketAddr;
use seahash::hash;

pub use client::Client;
pub use service::NodeService;

/// A node in the chord ring
///
/// This struct is used to represent a node in the chord ring.
pub struct NodeStore {
    successor: NodeRef,
    predecessor: Option<NodeRef>,
}

impl NodeStore {
    /// Create a new node
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node
    /// * `socket_addr` - The socket address of the node
    pub fn new(successor: NodeRef) -> Self {
        Self {
            successor,
            predecessor: None,
        }
    }

    /// Returns true if the given id is between 2 nodes on a ring
    ///
    /// # Arguments
    ///
    /// * `id` - The id to check
    /// * `node1` - First node id
    /// * `node2` - Second node id
    ///
    /// # Examples
    ///
    /// Check if 10 is between 5 and 15
    ///
    /// ```
    /// use chord::NodeStore;
    ///
    /// let id = 10;
    /// let node1 = 5;
    /// let node2 = 15;
    ///
    /// assert_eq!(NodeStore::is_between_on_ring(id, node1, node2), true);
    /// ```
    ///
    /// Check if 20 is between 15 and 5
    /// ```
    /// use chord::NodeStore;
    ///
    /// let id = 20;
    /// let node1 = 15;
    /// let node2 = 5;
    ///
    /// assert_eq!(NodeStore::is_between_on_ring(id, node1, node2), true);
    /// ```
    pub fn is_between_on_ring(id: u64, node1: u64, node2: u64) -> bool {
        if node1 < node2 {
            node1 < id && id <= node2
        } else {
            node1 < id || id <= node2
        }
    }
}

/// A reference to a node in the chord ring
#[derive(Clone)]
pub struct NodeRef {
    id: u64,
    addr: SocketAddr
}

impl NodeRef {
    pub fn new(addr: SocketAddr) -> Self {
        Self { id: hash(&addr.to_string().as_bytes()), addr }
    }

    pub fn client<C: Client>(&self) -> C {
        C::init(self.addr)
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    fn with_id(id: u64, addr: SocketAddr) -> Self {
        Self { id, addr }
    }
}
