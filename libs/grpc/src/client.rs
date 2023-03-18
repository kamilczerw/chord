use std::net::SocketAddr;

use crate::server::chord_proto::chord_node_client::ChordNodeClient;
use crate::server::chord_proto::FindSuccessorRequest;
use chord_rs::client::ClientError;
use chord_rs::{Client, Node};
use tonic::async_trait;
use tonic::transport::{Channel, Endpoint};

#[derive(Debug)]
pub struct ChordGrpcClient {
    endpoint: Endpoint,
    // client: ChordNodeClient<Channel>,
}

#[async_trait]
impl Client for ChordGrpcClient {
    fn init(addr: SocketAddr) -> Self {
        let endpoint = Channel::from_shared(addr.to_string()).unwrap();
        // let client = ChordNodeClient::connect(endpoint.unwrap());
        // let channel = endpoint.unwrap().connect_lazy();

        ChordGrpcClient { endpoint }
    }

    async fn find_successor(&self, id: u64) -> Result<Node, ClientError> {
        let mut client = ChordNodeClient::connect(self.endpoint.clone())
            .await
            .unwrap();

        // let mut client = ChordNodeClient::new(self.channel.clone());

        let request = tonic::Request::new(FindSuccessorRequest { id });
        let response = client.find_successor(request).await.unwrap();

        println!("response: {:?}", response);
        unimplemented!()
    }

    fn successor(&self) -> Result<Node, ClientError> {
        unimplemented!()
    }

    fn predecessor(&self) -> Result<Option<Node>, ClientError> {
        unimplemented!()
    }

    fn notify(&self, predecessor: Node) -> Result<(), ClientError> {
        unimplemented!()
    }

    fn ping(&self) -> Result<(), ClientError> {
        unimplemented!()
    }
}
