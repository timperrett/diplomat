
// use std::sync::Arc;
// use std::io::Read;
// use std::time::Instant;
// use std::{io, thread};

use api::eds_grpc::{EndpointDiscoveryService};
use consul::Client as ConsulClient;
use futures::Future;

#[derive(Clone)]
pub struct Service {
    // consul: ConsulClient,
}

impl EndpointDiscoveryService for Service {
    fn stream_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::discovery::DiscoveryRequest>,
        sink: ::grpcio::DuplexSink<::api::discovery::DiscoveryResponse>){

    }
    fn fetch_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        req: ::api::discovery::DiscoveryRequest,
        sink: ::grpcio::UnarySink<::api::discovery::DiscoveryResponse>)
    {
        let mut resp = ::api::discovery::DiscoveryResponse::new();
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn stream_load_stats(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::eds::LoadStatsRequest>,
        sink: ::grpcio::DuplexSink<::api::eds::LoadStatsResponse>){

    }
}
