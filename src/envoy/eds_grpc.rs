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

pub trait EndpointDiscoveryService {
    fn stream_endpoints(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::eds::EndpointDiscoveryRequest>) -> ::grpc::StreamingResponse<super::eds::EndpointDiscoveryResponse>;

    fn fetch_endpoints(&self, o: ::grpc::RequestOptions, p: super::eds::EndpointDiscoveryRequest) -> ::grpc::SingleResponse<super::eds::EndpointDiscoveryResponse>;

    fn stream_load_balance(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::eds::LoadBalanceRequest>) -> ::grpc::StreamingResponse<super::eds::LoadBalanceResponse>;

    fn fetch_load_balance(&self, o: ::grpc::RequestOptions, p: super::eds::LoadBalanceRequest) -> ::grpc::SingleResponse<super::eds::LoadBalanceResponse>;
}

// client

pub struct EndpointDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamEndpoints: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::eds::EndpointDiscoveryRequest, super::eds::EndpointDiscoveryResponse>>,
    method_FetchEndpoints: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::eds::EndpointDiscoveryRequest, super::eds::EndpointDiscoveryResponse>>,
    method_StreamLoadBalance: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::eds::LoadBalanceRequest, super::eds::LoadBalanceResponse>>,
    method_FetchLoadBalance: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::eds::LoadBalanceRequest, super::eds::LoadBalanceResponse>>,
}

impl EndpointDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        EndpointDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamEndpoints: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.EndpointDiscoveryService/StreamEndpoints".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchEndpoints: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.EndpointDiscoveryService/FetchEndpoints".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_StreamLoadBalance: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.EndpointDiscoveryService/StreamLoadBalance".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchLoadBalance: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.EndpointDiscoveryService/FetchLoadBalance".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            EndpointDiscoveryServiceClient::with_client(c)
        })
    }
}

impl EndpointDiscoveryService for EndpointDiscoveryServiceClient {
    fn stream_endpoints(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::eds::EndpointDiscoveryRequest>) -> ::grpc::StreamingResponse<super::eds::EndpointDiscoveryResponse> {
        self.grpc_client.call_bidi(o, p, self.method_StreamEndpoints.clone())
    }

    fn fetch_endpoints(&self, o: ::grpc::RequestOptions, p: super::eds::EndpointDiscoveryRequest) -> ::grpc::SingleResponse<super::eds::EndpointDiscoveryResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchEndpoints.clone())
    }

    fn stream_load_balance(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::eds::LoadBalanceRequest>) -> ::grpc::StreamingResponse<super::eds::LoadBalanceResponse> {
        self.grpc_client.call_bidi(o, p, self.method_StreamLoadBalance.clone())
    }

    fn fetch_load_balance(&self, o: ::grpc::RequestOptions, p: super::eds::LoadBalanceRequest) -> ::grpc::SingleResponse<super::eds::LoadBalanceResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchLoadBalance.clone())
    }
}

// server

pub struct EndpointDiscoveryServiceServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for EndpointDiscoveryServiceServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl EndpointDiscoveryServiceServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : EndpointDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = EndpointDiscoveryServiceServer::new_service_def(h);
        EndpointDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : EndpointDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = EndpointDiscoveryServiceServer::new_service_def(h);
        EndpointDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : EndpointDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.EndpointDiscoveryService/StreamEndpoints".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.stream_endpoints(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.EndpointDiscoveryService/FetchEndpoints".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_endpoints(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.EndpointDiscoveryService/StreamLoadBalance".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerBidi::new(move |o, p| handler_copy.stream_load_balance(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.EndpointDiscoveryService/FetchLoadBalance".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_load_balance(o, p))
                    },
                ),
            ],
        )
    }
}
