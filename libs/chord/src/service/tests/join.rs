use std::net::SocketAddr;
use mockall::predicate;
use crate::client::{ClientError, MockClient};
use crate::NodeService;
use crate::service::tests;
use crate::service::tests::{get_lock, MTX};

#[test]
fn join_test() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42115 {
            client.expect_find_successor()
                .with(predicate::eq(1))
                .times(1)
                .returning(|_| {
                    Ok(tests::node(115))
                });
        }

        client
    });
    let mut service: NodeService<MockClient> = NodeService::with_id(1, SocketAddr::from(([127, 0, 0, 1], 42001)));

    service.join(tests::node(115)).unwrap();

    assert_eq!(service.store.successor().id, 115);
}

#[test]
fn join_error_test() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|addr: SocketAddr| {
        let mut client = MockClient::new();
        if addr.port() == 42116 {
            client.expect_find_successor()
                .with(predicate::eq(2))
                .times(1)
                .returning(|_| {
                    Err(ClientError::Unexpected("Test".to_string()))
                });
        }
        client
    });
    let mut service: NodeService<MockClient> = NodeService::with_id(2, SocketAddr::from(([127, 0, 0, 1], 42001)));

    let result = service.join(tests::node(116));

    assert!(result.is_err());
    let message = result.unwrap_err().to_string();
    assert_eq!(message, "Client error: Test");
}
