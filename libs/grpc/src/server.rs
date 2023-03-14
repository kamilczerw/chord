use tonic::{Request, Response, Status};
pub use tonic::transport::Server;
use chord::chord_node_server::ChordNode;
use chord::{PingRequest, PingResponse};

pub mod chord {
    include!(concat!(env!("OUT_DIR"), "/chord.rs"));
}

#[derive(Debug, Default)]
pub struct MyChord {}

#[tonic::async_trait]
impl ChordNode for MyChord {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = chord::PingResponse {};

        Ok(Response::new(reply))
    }
}
