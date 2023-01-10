use std::net::SocketAddr;
use crate::client::MockClient;
use crate::{NodeService};
use crate::service::tests;
use crate::service::tests::{get_lock, MTX};

#[test]
fn test_find_successor() {
    let _m = get_lock(&MTX);
    let service: NodeService<MockClient> = NodeService::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
    let result = service.find_successor(10);
    assert!(result.is_ok());
    let successor = result.unwrap();

    assert_eq!(successor.id, 8);
}

#[test]
fn find_successor_with_2_nodes() {
    let _m = get_lock(&MTX);
    let ctx = MockClient::init_context();

    ctx.expect().returning(|_| {
        let mut client = MockClient::new();
        client.expect_find_successor()
            .times(1)
            .returning(|_| {
                Ok(tests::node(6))
            });
        client
    });

    let mut service: NodeService<MockClient> = NodeService::with_id(8, SocketAddr::from(([127, 0, 0, 1], 42001)));
    service.store.set_successor(tests::node(16));

    assert_eq!(service.find_successor(10).unwrap().id, 16);
    assert_eq!(service.find_successor(2).unwrap().id, 6);
}
