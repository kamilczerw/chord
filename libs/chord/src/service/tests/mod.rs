use std::net::SocketAddr;
use crate::NodeRef;

mod find_successor;

fn node(id: u64) -> NodeRef {
    NodeRef {
        id,
        addr: SocketAddr::from(([127, 0, 0, 1], 42000 + id as u16)),
    }
}
