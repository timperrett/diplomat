mod eds;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;

use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use api::eds_grpc;
use config::Config;
use consul::Client;

pub fn start(cfg: Config, consul: Client) {
    let env = Arc::new(Environment::new(1));

    // EDS
    let eds_instance = eds::Service { 
        config: cfg,
        consul: consul
    };

    // let eds_service = eds_grpc::create_endpoint_discovery_service(eds_instance);

    // CDS

    // LDS

    // RDS

    let mut server = ServerBuilder::new(env)
        // .register_service(eds_service)
        .bind("127.0.0.1", 3000)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
