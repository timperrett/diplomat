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

pub trait HealthDiscoveryService {
    fn stream_health_check(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::hds::HealthCheckRequestOrEndpointHealthResponse>) -> ::grpc::StreamingResponse<super::hds::HealthCheckSpecifier>;

    fn fetch_health_check(&self, o: ::grpc::RequestOptions, p: super::hds::HealthCheckRequestOrEndpointHealthResponse) -> ::grpc::SingleResponse<super::hds::HealthCheckSpecifier>;
}

// client

pub struct HealthDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamHealthCheck: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::hds::HealthCheckRequestOrEndpointHealthResponse, super::hds::HealthCheckSpecifier>>,
    method_FetchHealthCheck: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::hds::HealthCheckRequestOrEndpointHealthResponse, super::hds::HealthCheckSpecifier>>,
}

impl HealthDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        HealthDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamHealthCheck: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.HealthDiscoveryService/StreamHealthCheck".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchHealthCheck: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.HealthDiscoveryService/FetchHealthCheck".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            HealthDiscoveryServiceClient::with_client(c)
        })
    }
}

impl HealthDiscoveryService for HealthDiscoveryServiceClient {
    fn stream_health_check(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::hds::HealthCheckRequestOrEndpointHealthResponse>) -> ::grpc::StreamingResponse<super::hds::HealthCheckSpecifier> {
        self.grpc_client.call_bidi(o, p, self.method_StreamHealthCheck.clone())
    }

    fn fetch_health_check(&self, o: ::grpc::RequestOptions, p: super::hds::HealthCheckRequestOrEndpointHealthResponse) -> ::grpc::SingleResponse<super::hds::HealthCheckSpecifier> {
        self.grpc_client.call_unary(o, p, self.method_FetchHealthCheck.clone())
    }
}

// server

pub struct HealthDiscoveryServiceServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for HealthDiscoveryServiceServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl HealthDiscoveryServiceServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : HealthDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = HealthDiscoveryServiceServer::new_service_def(h);
        HealthDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : HealthDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = HealthDiscoveryServiceServer::new_service_def(h);
        HealthDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : HealthDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.HealthDiscoveryService/StreamHealthCheck".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.stream_health_check(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.HealthDiscoveryService/FetchHealthCheck".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_health_check(o, p))
                    },
                ),
            ],
        )
    }
}
