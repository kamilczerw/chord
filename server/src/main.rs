use tamto_grpc::server::chord::chord_node_server::ChordNodeServer;
use tamto_grpc::server::{MyChord, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chord = MyChord::default();

    Server::builder()
        .add_service(ChordNodeServer::new(chord))
        .serve(addr)
        .await?;

    Ok(())
}
