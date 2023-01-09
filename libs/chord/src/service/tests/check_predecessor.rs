use std::net::SocketAddr;
use crate::client::{ClientError, MockClient};
use crate::service::tests;
use crate::NodeService;
use crate::service::tests::{get_lock, MTX};

#[tokio::test]
async fn when_predecessor_is_up_it_should_not_be_removed() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42012 {
            client.expect_ping()
                .times(1)
                .returning(|| {
                    Box::pin(async { Ok(()) })
                });
        }
        client
    });

    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);
    node.predecessor = Some(tests::node_ref(12));
    let service = NodeService::<MockClient>::new(node);

    service.check_predecessor().await;

    assert!(service.predecessor().is_some());
    assert_eq!(service.predecessor().unwrap().id, 12);
}

#[tokio::test]
async fn when_predecessor_is_down_it_should_be_removed() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42016 {
            client.expect_ping()
                .times(1)
                .returning(|| {
                    Box::pin(async { Err(ClientError::ConnectionFailed(tests::node_ref(16))) })
                });
        }

        client
    });

    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);
    node.predecessor = Some(tests::node_ref(16));
    let service = NodeService::<MockClient>::new(node);

    service.check_predecessor().await;

    assert!(service.predecessor().is_none());
}

#[tokio::test]
async fn when_ping_fails_with_unexpected_error_predecessor_should_not_be_removed() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42008 {
            client.expect_ping()
                .times(1)
                .returning(|| {
                    Box::pin(async { Err(ClientError::Unexpected("Error".to_string())) })
                });
        }
        client
    });

    let mut node = tests::node(8);
    node.successor = tests::node_ref(16);
    node.predecessor = Some(tests::node_ref(8));
    let service = NodeService::<MockClient>::new(node);

    service.check_predecessor().await;

    assert!(service.predecessor().is_some());
    assert_eq!(service.predecessor().unwrap().id, 8);
}
