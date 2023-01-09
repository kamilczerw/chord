use std::net::SocketAddr;
use std::sync::Arc;
use async_trait::async_trait;
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{info, LevelFilter};
use tokio::time;
use chord_rs::{Client, Node, NodeRef, NodeService};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log();
    let node = Node::new(8, SocketAddr::from(([127, 0, 0, 1], 43000)));
    let service = NodeService::<EmptyClient>::new(node);

    // let t1 = tokio::spawn(async move {
    //     let s: &'a NodeService<_> = &service;
    //     let mut interval = time::interval(time::Duration::from_secs(1));
    //     loop {
    //         interval.tick().await;
    //         info!("t1");
    //         &service.stabilize().await.unwrap();
    //     }
    //
    // });

    let t2 = tokio::spawn({
        let mut interval = time::interval(time::Duration::from_secs(2));
        let service = service.clone();
        let mut port = 42000;
        async move {
            loop {
                interval.tick().await;
                port += 1;
                let addr = SocketAddr::from(([127, 0, 0, 1], port));
                service.update_successor(NodeRef::new(addr));
                let s = service.find_successor(10).await.unwrap();
                // let node = Node::new(16, SocketAddr::from(([127, 0, 0, 1], 43001)));
                // let service = NodeService::<EmptyClient>::new(node);
                // service.run().await;
                info!("Found successor: {}", s.addr());
            }
        }
    });

    let _ = tokio::join!(
        stabilize(service.clone()),
        t2
    );
    info!("Hello, world!");

    Ok(())
}

async fn stabilize(service: Arc<NodeService<EmptyClient>>) {
    let mut interval = time::interval(time::Duration::from_secs(1));

    loop {
        interval.tick().await;
        info!("stabilize");
        service.stabilize().await.unwrap();
    }
}

fn log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{t}] {h({l}):>7} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
}

struct EmptyClient {}

#[allow(unused_variables)]
#[async_trait]
impl Client for EmptyClient {
    fn init(addr: SocketAddr) -> Self {
        EmptyClient {}
    }

    async fn find_successor(&self, id: u64) -> Result<NodeRef, chord_rs::client::ClientError> {
        todo!()
    }

    async fn successor(&self) -> Result<NodeRef, chord_rs::client::ClientError> {
        todo!()
    }

    async fn predecessor(&self) -> Result<Option<NodeRef>, chord_rs::client::ClientError> {
        Ok(None)
    }

    async fn notify(&self, predecessor: NodeRef) -> Result<(), chord_rs::client::ClientError> {
        Ok(())
    }

    async fn ping(&self) -> Result<(), chord_rs::client::ClientError> {
        todo!()
    }
}
