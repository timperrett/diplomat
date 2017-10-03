
use api::eds_grpc::{EndpointDiscoveryService};
// use consul::Client as ConsulClient;
use futures::Future;
use config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct Service {
    pub config: Config,
    // pub client: Arc<ConsulClient>,
}

use ::api::discovery::{DiscoveryRequest,DiscoveryResponse};
use grpcio::{RpcStatus,RpcStatusCode,UnarySinkResult};

impl EndpointDiscoveryService for Service {
    // let consul = client.

    fn stream_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::discovery::DiscoveryRequest>,
        sink: ::grpcio::DuplexSink<::api::discovery::DiscoveryResponse>){

    }

    fn fetch_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        req: DiscoveryRequest,
        sink: ::grpcio::UnarySink<DiscoveryResponse>)
    {
        // let resp = resolve_endpoints(&self.client, req.clone());

        // let y = match resp {
        //     Ok(x) => sink.success(x),
        //     Err(_) => sink.fail(RpcStatus::new(RpcStatusCode::Internal, None)),
        // };

        // let f = y.map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        // ctx.spawn(f)
    }

    fn stream_load_stats(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::eds::LoadStatsRequest>,
        sink: ::grpcio::DuplexSink<::api::eds::LoadStatsResponse>){

    }
}

// fn resolve_endpoints(c: &ConsulClient, req: DiscoveryRequest) -> Result<DiscoveryResponse, String> {
//     // let item = req.resource_names.pop()

//     let foo = c.catalog.get_nodes("consul".to_owned());
//     Ok(DiscoveryResponse::new())
// }