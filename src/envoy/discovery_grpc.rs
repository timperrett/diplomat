// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait AggregatedDiscoveryService {
    fn stream_aggregated_resources(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse>;
}

// client

pub struct AggregatedDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamAggregatedResources: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse>>,
}

impl AggregatedDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        AggregatedDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamAggregatedResources: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/envoy.api.v2.AggregatedDiscoveryService/StreamAggregatedResources".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            AggregatedDiscoveryServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            AggregatedDiscoveryServiceClient::with_client(c)
        })
    }
}

impl AggregatedDiscoveryService for AggregatedDiscoveryServiceClient {
    fn stream_aggregated_resources(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse> {
        self.grpc_client.call_bidi(o, p, self.method_StreamAggregatedResources.clone())
    }
}

// server

pub struct AggregatedDiscoveryServiceServer;


impl AggregatedDiscoveryServiceServer {
    pub fn new_service_def<H : AggregatedDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/envoy.api.v2.AggregatedDiscoveryService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/envoy.api.v2.AggregatedDiscoveryService/StreamAggregatedResources".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.stream_aggregated_resources(o, p))
                    },
                ),
            ],
        )
    }
}
