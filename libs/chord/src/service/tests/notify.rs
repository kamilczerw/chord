use std::net::SocketAddr;
use crate::NodeService;
use crate::service::tests;
use crate::client::MockClient;


#[test]
fn when_calling_notify_and_predecessor_is_none_then_the_predecessor_should_be_set() {
    let mut service: NodeService<MockClient> = NodeService::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
    service.store.successor = tests::node(16);

    assert!(service.store.predecessor.is_none());
    service.notify(tests::node(8));

    assert_eq!(service.store.predecessor.unwrap().id, 8);
}

#[test]
fn when_calling_notify_and_predecessor_set_and_request_node_is_in_range_then_the_predecessor_should_be_set() {
    let mut service: NodeService<MockClient> = NodeService::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
    service.store.successor = tests::node(16);
    service.store.predecessor = Some(tests::node(4));

    assert!(service.store.predecessor.is_some());
    service.notify(tests::node(8));

    assert_eq!(service.store.predecessor.unwrap().id, 8);
}

#[test]
fn when_calling_notify_and_predecessor_set_and_request_node_is_not_in_range_then_the_predecessor_should_not_be_set() {
    let mut service: NodeService<MockClient> = NodeService::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
    service.store.successor = tests::node(16);
    service.store.predecessor = Some(tests::node(4));

    assert!(service.store.predecessor.is_some());
    service.notify(tests::node(16));

    assert_eq!(service.store.predecessor.unwrap().id, 4);
}
