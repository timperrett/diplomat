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

pub trait RouteDiscoveryService {
    fn stream_routes(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse>;

    fn fetch_routes(&self, o: ::grpc::RequestOptions, p: super::discovery::DiscoveryRequest) -> ::grpc::SingleResponse<super::discovery::DiscoveryResponse>;
}

// client

pub struct RouteDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamRoutes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse>>,
    method_FetchRoutes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse>>,
}

impl RouteDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        RouteDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamRoutes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/envoy.api.v2.RouteDiscoveryService/StreamRoutes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchRoutes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/envoy.api.v2.RouteDiscoveryService/FetchRoutes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            RouteDiscoveryServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            RouteDiscoveryServiceClient::with_client(c)
        })
    }
}

impl RouteDiscoveryService for RouteDiscoveryServiceClient {
    fn stream_routes(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse> {
        self.grpc_client.call_bidi(o, p, self.method_StreamRoutes.clone())
    }

    fn fetch_routes(&self, o: ::grpc::RequestOptions, p: super::discovery::DiscoveryRequest) -> ::grpc::SingleResponse<super::discovery::DiscoveryResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchRoutes.clone())
    }
}

// server

pub struct RouteDiscoveryServiceServer;


impl RouteDiscoveryServiceServer {
    pub fn new_service_def<H : RouteDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/envoy.api.v2.RouteDiscoveryService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/envoy.api.v2.RouteDiscoveryService/StreamRoutes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.stream_routes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/envoy.api.v2.RouteDiscoveryService/FetchRoutes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_routes(o, p))
                    },
                ),
            ],
        )
    }
}
