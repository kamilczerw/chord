use chord_proto::chord_node_server::ChordNode;
pub use chord_proto::chord_node_server::ChordNodeServer;
use chord_proto::{PingRequest, PingResponse};
pub use tonic::transport::Server;
use tonic::{Request, Response, Status};

use self::chord_proto::{FindSuccessorRequest, FindSuccessorResponse};

pub mod chord_proto {
    include!(concat!(env!("OUT_DIR"), "/chord.rs"));
}

#[derive(Debug, Default)]
pub struct ChordService {}

#[tonic::async_trait]
impl ChordNode for ChordService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = chord_proto::PingResponse {};

        Ok(Response::new(reply))
    }

    async fn find_successor(
        &self,
        request: Request<FindSuccessorRequest>,
    ) -> Result<Response<FindSuccessorResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = FindSuccessorResponse { id: 1, node: None };

        Ok(Response::new(reply))
    }
}
