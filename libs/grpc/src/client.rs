use std::net::SocketAddr;

use crate::server::chord_proto::chord_node_client::ChordNodeClient;
use crate::server::chord_proto::FindSuccessorRequest;
use chord_rs::client::ClientError;
use chord_rs::{Client, Node};
use tonic::transport::Channel;

pub struct ChordGrpcClient {
    channel: Channel,
    // client: ChordNodeClient<Channel>,
}

impl Client for ChordGrpcClient {
    fn init(addr: SocketAddr) -> Self {
        let endpoint = Channel::from_shared(addr.to_string());
        // let client = ChordNodeClient::connect(endpoint.unwrap());
        let channel = endpoint.unwrap().connect_lazy();

        ChordGrpcClient { channel }
    }

    fn find_successor(&self, id: u64) -> Result<Node, ClientError> {
        // let mut client = ChordNodeClient::connect(self.channel.clone());

        // let mut client = ChordNodeClient::new(self.channel.clone());

        // let request = tonic::Request::new(FindSuccessorRequest { id });

        // client.send(request);

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
