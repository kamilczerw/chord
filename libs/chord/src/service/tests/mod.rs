use std::net::SocketAddr;
use crate::{Node, NodeRef};

mod find_successor;
mod join;

fn node_ref(id: u64) -> NodeRef {
    node(id).node_ref()
}

fn node(id: u64) -> Node {
    let addr = SocketAddr::from(([127, 0, 0, 1], 42000 + id as u16));
    Node::new(id, addr)
}
