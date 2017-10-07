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

use std::string;

#[derive(Copy, Clone)]
pub enum MessageType {
    DiscoveryResponse,
    ClusterLoadAssignment,
    LocalityLbEndpoints,
    LbEndpoint,
    ClusterLoadAssignmentPolicy,
    Locality,
    Cluster,
    DiscoveryRequest
}
impl string::ToString for MessageType {
    fn to_string(&self) -> String {
        match *self {
            MessageType::DiscoveryRequest =>
                "type.googleapis.com/envoy.api.v2.DiscoveryRequest".to_string(),
            MessageType::DiscoveryResponse => {
                "type.googleapis.com/envoy.api.v2.DiscoveryResponse".to_string()
            }
            MessageType::ClusterLoadAssignment => {
                "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment".to_string()
            }
            MessageType::LocalityLbEndpoints => {
                "type.googleapis.com/envoy.api.v2.LocalityLbEndpoints".to_string()
            }
            MessageType::LbEndpoint => {
                "type.googleapis.com/envoy.api.v2.LocalityLbEndpoints".to_string()
            }
            MessageType::ClusterLoadAssignmentPolicy => {
                "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment_Policy".to_string()
            }
            MessageType::Locality => "type.googleapis.com/envoy.api.v2.Locality".to_string(),
            MessageType::Cluster => "type.googleapis.com/envoy.api.v2.Cluster".to_string(),
        }
    }
}


pub fn start(cfg: Config, consul: Client) {
    let env = Arc::new(Environment::new(1));

    // EDS
    let eds_instance = eds::Service {
        config: cfg.clone(),
        consul: consul,
    };
    let eds_service = eds_grpc::create_endpoint_discovery_service(eds_instance);

    // CDS

    // LDS

    // RDS

    let mut server = ServerBuilder::new(env)
        .register_service(eds_service)
        .bind(cfg.server.host, cfg.server.port)
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
