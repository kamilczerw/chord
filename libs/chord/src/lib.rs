mod client;

use std::net::SocketAddr;
use seahash::hash;

pub use client::Client;

/// A node in the chord ring
///
/// This struct is used to represent a node in the chord ring.
pub struct Node {
    id: u64,
    socket_addr: SocketAddr,
    successor: NodeRef,
    predecessor: Option<NodeRef>,
}

impl Node {
    /// Create a new node
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node
    /// * `socket_addr` - The socket address of the node
    pub fn new(id: u64, socket_addr: SocketAddr) -> Self {
        Self {
            id,
            socket_addr,
            successor: NodeRef::with_id(id, socket_addr),
            predecessor: None,
        }
    }

    fn node_ref(&self) -> NodeRef {
        NodeRef::with_id(self.id, self.socket_addr)
    }
}

/// A reference to a node in the chord ring
pub struct NodeRef {
    id: u64,
    addr: SocketAddr
}

impl NodeRef {
    pub fn new(addr: SocketAddr) -> Self {
        Self { id: hash(&addr.to_string().as_bytes()), addr }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    fn with_id(id: u64, addr: SocketAddr) -> Self {
        Self { id, addr }
    }
}
