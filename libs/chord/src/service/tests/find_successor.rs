use std::net::SocketAddr;
use crate::client::MockClient;
use crate::{Node, NodeService};
use crate::service::tests;

#[tokio::test]
async fn test_find_successor() {
    let mut client = MockClient::new();
    client.expect_find_successor()
        .times(0);
    let addr = SocketAddr::from(([127, 0, 0, 1], 42001));
    let node = Node::new(8, addr);
    let service: NodeService<MockClient> = NodeService::new(node);
    let result = service.find_successor(10).await;
    assert!(result.is_ok());
    let successor = result.unwrap();

    assert_eq!(successor.id, 8);
}

#[tokio::test]
async fn find_successor_with_2_nodes() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 42001));
    let mut node = Node::new(8, addr);
    node.successor = tests::node(16);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|_| {
        let mut client = MockClient::new();
        client.expect_find_successor()
            .times(1)
            .returning(|_| {
                Box::pin(async { Ok(tests::node(6))})
            });
        client
    });

    let service: NodeService<MockClient> = NodeService::new(node);

    assert_eq!(service.find_successor(10).await.unwrap().id, 16);
    assert_eq!(service.find_successor(2).await.unwrap().id, 6);
}
