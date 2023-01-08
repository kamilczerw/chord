use crate::NodeService;
use crate::service::tests;
use crate::client::MockClient;


#[tokio::test]
async fn when_calling_notify_and_predecessor_is_none_then_the_predecessor_should_be_set() {
    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);

    assert!(service.node.predecessor.is_none());
    service.notify(tests::node_ref(8));

    assert_eq!(service.node.predecessor.unwrap().id, 8);
}

#[tokio::test]
async fn when_calling_notify_and_predecessor_set_and_request_node_is_in_range_then_the_predecessor_should_be_set() {
    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);
    service.node.predecessor = Some(tests::node_ref(4));

    assert!(service.node.predecessor.is_some());
    service.notify(tests::node_ref(8));

    assert_eq!(service.node.predecessor.unwrap().id, 8);
}

#[tokio::test]
async fn when_calling_notify_and_predecessor_set_and_request_node_is_not_in_range_then_the_predecessor_should_not_be_set() {
    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);
    service.node.predecessor = Some(tests::node_ref(4));

    assert!(service.node.predecessor.is_some());
    service.notify(tests::node_ref(16));

    assert_eq!(service.node.predecessor.unwrap().id, 4);
}
