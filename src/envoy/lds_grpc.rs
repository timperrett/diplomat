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
    fn stream_listeners(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse>;

    fn fetch_listeners(&self, o: ::grpc::RequestOptions, p: super::discovery::DiscoveryRequest) -> ::grpc::SingleResponse<super::discovery::DiscoveryResponse>;
}

// client

pub struct ListenerDiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_StreamListeners: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse>>,
    method_FetchListeners: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse>>,
}

impl ListenerDiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ListenerDiscoveryServiceClient {
            grpc_client: grpc_client,
            method_StreamListeners: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/envoy.api.v2.ListenerDiscoveryService/StreamListeners".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FetchListeners: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/envoy.api.v2.ListenerDiscoveryService/FetchListeners".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ListenerDiscoveryServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ListenerDiscoveryServiceClient::with_client(c)
        })
    }
}

impl ListenerDiscoveryService for ListenerDiscoveryServiceClient {
    fn stream_listeners(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::discovery::DiscoveryRequest>) -> ::grpc::StreamingResponse<super::discovery::DiscoveryResponse> {
        self.grpc_client.call_bidi(o, p, self.method_StreamListeners.clone())
    }

    fn fetch_listeners(&self, o: ::grpc::RequestOptions, p: super::discovery::DiscoveryRequest) -> ::grpc::SingleResponse<super::discovery::DiscoveryResponse> {
        self.grpc_client.call_unary(o, p, self.method_FetchListeners.clone())
    }
}

// server

pub struct ListenerDiscoveryServiceServer;


impl ListenerDiscoveryServiceServer {
    pub fn new_service_def<H : ListenerDiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/envoy.api.v2.ListenerDiscoveryService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/envoy.api.v2.ListenerDiscoveryService/StreamListeners".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.stream_listeners(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/envoy.api.v2.ListenerDiscoveryService/FetchListeners".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.fetch_listeners(o, p))
                    },
                ),
            ],
        )
    }
}
