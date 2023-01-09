use tokio::time::sleep;
use crate::NodeService;
use crate::service::tests;
use crate::client::MockClient;


#[tokio::test]
async fn when_calling_notify_and_predecessor_is_none_then_the_predecessor_should_be_set() {
    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);

    let service = NodeService::<MockClient>::new(node);

    assert!(service.predecessor().is_none());
    service.notify(tests::node_ref(8));

    sleep(tokio::time::Duration::from_millis(100)).await;

    assert_eq!(service.predecessor().unwrap().id, 8);
}

#[tokio::test]
async fn when_calling_notify_and_predecessor_set_and_request_node_is_in_range_then_the_predecessor_should_be_set() {
    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);
    node.predecessor = Some(tests::node_ref(4));

    let service = NodeService::<MockClient>::new(node);

    assert!(service.predecessor().is_some());
    service.notify(tests::node_ref(8));

    sleep(tokio::time::Duration::from_millis(100)).await;
    assert_eq!(service.predecessor().unwrap().id, 8);
}

#[tokio::test]
async fn when_calling_notify_and_predecessor_set_and_request_node_is_not_in_range_then_the_predecessor_should_not_be_set() {
    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);
    node.predecessor = Some(tests::node_ref(4));
    let service = NodeService::<MockClient>::new(node);

    assert!(service.predecessor().is_some());
    service.notify(tests::node_ref(16));

    sleep(tokio::time::Duration::from_millis(100)).await;
    assert_eq!(service.predecessor().unwrap().id, 4);
}
