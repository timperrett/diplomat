
use api::eds_grpc::EndpointDiscoveryService;
use futures::Future;
use config::Config;
use std::sync::Arc;
use consul::Client as ConsulClient;

#[derive(Clone)]
pub struct Service {
    pub config: Config,
    pub consul: ConsulClient,
}

use api::discovery::{DiscoveryRequest, DiscoveryResponse};
use grpcio::{RpcStatus, RpcStatusCode, UnarySinkResult};

impl EndpointDiscoveryService for Service {

    /// The resource_names field in DiscoveryRequest specifies a list of clusters
    /// to subscribe to updates for.
    fn stream_endpoints(
        &self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::discovery::DiscoveryRequest>,
        sink: ::grpcio::DuplexSink<::api::discovery::DiscoveryResponse>,
    ) {

    }

    fn fetch_endpoints(
        &self,
        ctx: ::grpcio::RpcContext,
        req: DiscoveryRequest,
        sink: ::grpcio::UnarySink<DiscoveryResponse>,
    ) {
        // let resp = resolve_endpoints(&self.client, req.clone());

        // let y = match resp {
        //     Ok(x) => sink.success(x),
        //     Err(_) => sink.fail(RpcStatus::new(RpcStatusCode::Internal, None)),
        // };

        // let f = y.map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        // ctx.spawn(f)
    }

    /// Advanced API to allow for multi-dimensional load balancing by remote
    /// server. For receiving LB assignments, the steps are:
    /// 1, The management server is configured with per cluster/zone/load metric
    ///    capacity configuration. The capacity configuration definition is
    ///    outside of the scope of this document.
    /// 2. Envoy issues a standard {Stream,Fetch}Endpoints request for the clusters
    ///    to balance.
    ///
    /// Independently, Envoy will initiate a StreamLoadStats bidi stream with a
    /// management server:
    /// 1. Once a connection establishes, the management server publishes a
    ///    LoadStatsResponse for all clusters it is interested in learning load
    ///    stats about.
    /// 2. For each cluster, Envoy load balances incoming traffic to upstream hosts
    ///    based on per-zone weights and/or per-instance weights (if specified)
    ///    based on intra-zone LbPolicy. This information comes from the above
    ///    {Stream,Fetch}Endpoints.
    /// 3. When upstream hosts reply, they optionally add header <define header
    ///    name> with ASCII representation of EndpointLoadMetricStats.
    /// 4. Envoy aggregates load reports over the period of time given to it in
    ///    LoadStatsResponse.load_reporting_interval. This includes aggregation
    ///    stats Envoy maintains by itself (total_requests, rpc_errors etc.) as
    ///    well as load metrics from upstream hosts.
    /// 5. When the timer of load_reporting_interval expires, Envoy sends new
    ///    LoadStatsRequest filled with load reports for each cluster.
    /// 6. The management server uses the load reports from all reported Envoys
    ///    from around the world, computes global assignment and prepares traffic
    ///    assignment destined for each zone Envoys are located in. Goto 2.
    fn stream_load_stats(
        &self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<::api::eds::LoadStatsRequest>,
        sink: ::grpcio::DuplexSink<::api::eds::LoadStatsResponse>,
    ) {

    }
}

use consul::catalog::Catalog;
use protobuf::{Message,RepeatedField};
use protobuf::error::ProtobufError;
use protobuf::well_known_types::Any;

/// here we're taking any `A` that has a `::protobuf::Message` implementation, such that
/// we can encode the response (using protobuf); its turtles all the way down.
fn create_discovery_response<A: Message>(r: Vec<A>, nested_type_url: String) -> DiscoveryResponse {
    let serialized: Vec<Any> = r.iter().map(|x| pack_to_any(x.write_to_bytes(), nested_type_url.clone())).collect();
    let repeated = RepeatedField::from_vec(serialized);
    let mut d = DiscoveryResponse::new();
    d.set_canary(false);
    d.set_version_info("1".to_string());
    d.set_type_url("type.googleapis.com/envoy.api.v2.DiscoveryResponse".to_string());
    d.set_resources(repeated);
    d
}

fn pack_to_any(r: Result<Vec<u8>, ProtobufError>, turl: String) -> Any {
    match r {
        Ok(bytes) => any_from_bytes(bytes, turl),
        Err(_)    => Any::new(),
    }
}

fn any_from_bytes(bytes: Vec<u8>, turl: String) -> Any {
    let mut a = Any::new();
    a.set_value(bytes);
    a.set_type_url(turl);
    a
}

