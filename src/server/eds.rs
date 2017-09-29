
// use std::sync::Arc;
// use std::io::Read;
// use std::time::Instant;
// use std::{io, thread};

use api::eds_grpc::{EndpointDiscoveryService};
use consul::{Client};

#[derive(Clone)]
struct EDS;

impl EndpointDiscoveryService for EDS {
    fn stream_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::discovery::DiscoveryRequest>,
        sink: ::grpcio::DuplexSink<::api::discovery::DiscoveryResponse>){

    }
    fn fetch_endpoints(&self,
        ctx: ::grpcio::RpcContext,
        req: ::api::discovery::DiscoveryRequest,
        sink: ::grpcio::UnarySink<::api::discovery::DiscoveryResponse>){
    }

    fn stream_load_stats(&self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::eds::LoadStatsRequest>,
        sink: ::grpcio::DuplexSink<::api::eds::LoadStatsResponse>){

    }
}
