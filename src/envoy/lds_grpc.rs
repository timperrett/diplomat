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

pub trait ListenerDiscoveryService {
    fn stream_listeners(&self, o: ::grpc::RequestOptions, p: super::lds::ListenerDiscoveryRequest) -> ::grpc::StreamingResponse<super::lds::ListenerDiscoverResponse>;

    fn fetch_listeners(&self, o: ::grpc::RequestOptions, p: super::lds::ListenerDiscoveryRequest) -> ::grpc::SingleResponse<super::lds::ListenerDiscoverResponse>;
}

// client

pub struct ListenerDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamListeners: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::lds::ListenerDiscoveryRequest, super::lds::ListenerDiscoverResponse>>,
    method_FetchListeners: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::lds::ListenerDiscoveryRequest, super::lds::ListenerDiscoverResponse>>,
}

impl ListenerDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ListenerDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamListeners: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.ListenerDiscoveryService/StreamListeners".to_string(),
                streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchListeners: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/envoy.api.v2.ListenerDiscoveryService/FetchListeners".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new(host: &str, port: u16, tls: bool, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new(host, port, tls, conf).map(|c| {
            ListenerDiscoveryServiceClient::with_client(c)
        })
    }
}

impl ListenerDiscoveryService for ListenerDiscoveryServiceClient {
    fn stream_listeners(&self, o: ::grpc::RequestOptions, p: super::lds::ListenerDiscoveryRequest) -> ::grpc::StreamingResponse<super::lds::ListenerDiscoverResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_StreamListeners.clone())
    }

    fn fetch_listeners(&self, o: ::grpc::RequestOptions, p: super::lds::ListenerDiscoveryRequest) -> ::grpc::SingleResponse<super::lds::ListenerDiscoverResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchListeners.clone())
    }
}

// server

pub struct ListenerDiscoveryServiceServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for ListenerDiscoveryServiceServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl ListenerDiscoveryServiceServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : ListenerDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> Self {
        let service_definition = ListenerDiscoveryServiceServer::new_service_def(h);
        ListenerDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition),
        }
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : ListenerDiscoveryService + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> Self {
        let service_definition = ListenerDiscoveryServiceServer::new_service_def(h);
        ListenerDiscoveryServiceServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool),
        }
    }

    pub fn new_service_def<H : ListenerDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.ListenerDiscoveryService/StreamListeners".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerServerStreaming::new(move |o, p| handler_copy.stream_listeners(o, p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/envoy.api.v2.ListenerDiscoveryService/FetchListeners".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_listeners(o, p))
                    },
                ),
            ],
        )
    }
}
