use std::net::SocketAddr;
use mockall::predicate;
use crate::client::{ClientError, MockClient};
use crate::service::tests;
use crate::{NodeRef, NodeService};
use crate::service::tests::{get_lock, MTX};

#[tokio::test]
async fn stabilize_when_predecessor_is_between_node_and_successor_then_set_set_the_it_as_new_successor() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42016 {
            client.expect_predecessor()
                .times(1)
                .returning(|| {
                    Box::pin(async { Ok(Some(tests::node_ref(12)))})
                });
        }

        if addr.port() == 42012 {
            client.expect_notify()
                .with(predicate::function(|n: &NodeRef| n.id == 8))
                .times(1)
                .returning(|_| {
                    Box::pin(async { Ok(()) })
                });
        }
        client
    });

    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);

    assert_eq!(service.node.successor.id, 16);
    let result = service.stabilize().await;
    assert!(result.is_ok());

    assert_eq!(service.node.successor.id, 12);
}

#[tokio::test]
async fn when_predecessor_is_not_between_node_and_successor_then_the_old_one_should_be_kept() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42016 {
            client.expect_predecessor()
                .returning(|| {
                    Box::pin(async { Ok(Some(tests::node_ref(1)))})
                });
            client.expect_notify()
                .with(predicate::function(|n: &NodeRef| n.id == 8))
                .returning(|_| {
                    Box::pin(async { Ok(()) })
                });
        }
        client
    });

    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);

    assert_eq!(service.node.successor.id, 16);
    let result = service.stabilize().await;
    assert!(result.is_ok());

    assert_eq!(service.node.successor.id, 16);
}

#[tokio::test]
async fn when_getting_predecessor_fails_then_nothing_should_be_updated() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|_| {
        let mut client = MockClient::new();
        client.expect_predecessor()
            .returning(|| {
                let error = ClientError::Unexpected("Test".to_string());
                Box::pin(async { Err(error)})
            });
        client.expect_notify()
            .with(predicate::function(|n: &NodeRef| n.id == 8))
            .returning(|_| {
                Box::pin(async { Ok(()) })
            });
        client
    });

    let node = tests::node(8);
    let mut service: NodeService<MockClient> = NodeService::new(node);
    service.node.successor = tests::node_ref(16);

    assert_eq!(service.node.successor.id, 16);
    let _ = service.stabilize().await;

    assert_eq!(service.node.successor.id, 16);
}
