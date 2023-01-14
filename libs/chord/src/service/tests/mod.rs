use std::marker::PhantomData;
use std::net::SocketAddr;
use crate::{Node, NodeService};
use crate::client::MockClient;

mod find_successor;
mod join;
mod notify;
mod stabilize;
mod check_predecessor;

use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};
use crate::node::Finger;
use crate::node::store::NodeStore;

lazy_static! {
    static ref MTX: Mutex<()> = Mutex::new(());
}

// When a test panics, it will poison the Mutex. Since we don't actually
// care about the state of the data we ignore that it is poisoned and grab
// the lock regardless.  If you just do `let _m = &MTX.lock().unwrap()`, one
// test panicking will cause all other tests that try and acquire a lock on
// that Mutex to also panic.
fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    match m.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn node(id: u64) -> Node {
    let addr = SocketAddr::from(([127, 0, 0, 1], 42000 + id as u16));
    Node::with_id(id, addr)
}

impl Default for NodeService<MockClient> {
    fn default() -> Self {
        let node = Node::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
        let store = NodeStore::new(node.clone());
        Self {
            id: node.id,
            addr: node.addr,
            store,
            phantom: PhantomData
        }
    }
}

impl NodeService<MockClient> {
    fn find_closest_successor(id: u64, nodes: &Vec<Node>) -> Node {
        let mut nodes = nodes.clone();
        nodes.sort_by(|b, a| a.id.cmp(&b.id));

        let smallest = nodes.last().unwrap().clone();
        let mut closest = nodes[0].clone();
        for node in nodes {
            if node.id == id {
                return node;
            }
            if node.id < closest.id && node.id > id {
                closest = node;
            } else if node.id < id && Node::is_between_on_ring(id, closest.id, node.id) {
                closest = node;
            }
        }

        if closest.id > id {
            closest
        } else {
            smallest
        }
    }

    pub(crate) fn with_fingers(&mut self, nodes_ids: Vec<u64>) {
        let mut nodes: Vec<Node> = nodes_ids.into_iter().map(|id| node(id)).collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));

        let mut fingers = Vec::with_capacity(64);

        for i in 1..65 {
            let finger_id = Finger::finger_id(self.id, (i) as u8);

            let closest = Self::find_closest_successor(finger_id, &nodes);
            fingers.push(Finger { start: finger_id, node: closest });
        }

        self.store.finger_table = fingers;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_finger_table() {
        let mut service = NodeService::default();
        let nodes = vec![1, 16, 32, 64];
        service.with_fingers(nodes.clone());

        assert_eq!(9, service.store.finger_table[0].start);
        assert_eq!(16, service.store.finger_table[0].node.id);
        assert_eq!(10, service.store.finger_table[1].start);
        assert_eq!(16, service.store.finger_table[1].node.id);
        assert_eq!(12, service.store.finger_table[2].start);
        assert_eq!(16, service.store.finger_table[2].node.id);
        assert_eq!(16, service.store.finger_table[3].start);
        assert_eq!(16, service.store.finger_table[3].node.id);

        assert_eq!(264, service.store.finger_table[8].start);
        assert_eq!(1, service.store.finger_table[8].node.id);

        service.id = 2;
        service.with_fingers(nodes.clone());

        assert_eq!(16, service.store.finger_table[0].node.id);
        assert_eq!(16, service.store.finger_table[3].node.id);
        assert_eq!(32, service.store.finger_table[4].node.id);
        assert_eq!(64, service.store.finger_table[5].node.id);
        assert_eq!(1, service.store.finger_table[6].node.id);
        assert_eq!(1, service.store.finger_table[63].node.id);

        service.id = 154;
        service.with_fingers(nodes.clone());

        assert_eq!(1, service.store.finger_table[0].node.id);
        assert_eq!(1, service.store.finger_table[63].node.id);

        service.id = u64::MAX - 1;
        service.with_fingers(nodes.clone());

        assert_eq!(1, service.store.finger_table[0].node.id);
        assert_eq!(1, service.store.finger_table[1].node.id);
        assert_eq!(2, service.store.finger_table[2].start);
        assert_eq!(16, service.store.finger_table[2].node.id);
        assert_eq!(14, service.store.finger_table[4].start);
        assert_eq!(16, service.store.finger_table[4].node.id);
    }

    #[test]
    fn test_closest_successor() {
        let nodes = vec![node(1), node(16), node(32), node(64)];

        let closest = NodeService::find_closest_successor(1, &nodes);
        assert_eq!(1, closest.id);

        let closest = NodeService::find_closest_successor(2, &nodes);
        assert_eq!(16, closest.id);

        let closest = NodeService::find_closest_successor(25, &nodes);
        assert_eq!(32, closest.id);

        let closest = NodeService::find_closest_successor(33, &nodes);
        assert_eq!(64, closest.id);

        let closest = NodeService::find_closest_successor(64, &nodes);
        assert_eq!(64, closest.id);

        let closest = NodeService::find_closest_successor(65, &nodes);
        assert_eq!(1, closest.id);
    }

}
