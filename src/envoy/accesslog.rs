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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct EnvoyAccessLog {
    // message fields
    pub request_method: super::base::RequestMethod,
    pub start_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub protocol_variant: EnvoyAccessLog_Protocol,
    pub response_flags: ::std::vec::Vec<EnvoyAccessLog_ResponseFlag>,
    pub upstream_host: ::std::string::String,
    pub upstream_cluster: ::std::string::String,
    pub destination_host: ::std::string::String,
    pub request_body_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value>,
    pub response_body_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value>,
    pub request_headers_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value>,
    pub response_headers_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value>,
    pub secure: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub health_check: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub response_code: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub user_agent: ::std::string::String,
    pub path: ::std::string::String,
    pub referer: ::std::string::String,
    pub forwarded_for: ::std::string::String,
    pub request_id: ::std::string::String,
    pub authority: ::std::string::String,
    pub response_duration: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub upstream_service_duration: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub original_path: ::std::string::String,
    pub metadata: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    pub request_headers: ::protobuf::RepeatedField<super::base::HeaderValue>,
    pub response_headers: ::protobuf::RepeatedField<super::base::HeaderValue>,
    pub tls_sni_hostname: ::std::string::String,
    pub tls_version: EnvoyAccessLog_TLSVersion,
    pub tls_cipher_suite: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnvoyAccessLog {}

impl EnvoyAccessLog {
    pub fn new() -> EnvoyAccessLog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnvoyAccessLog {
        static mut instance: ::protobuf::lazy::Lazy<EnvoyAccessLog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnvoyAccessLog,
        };
        unsafe {
            instance.get(EnvoyAccessLog::new)
        }
    }

    // .envoy.api.v2.RequestMethod request_method = 1;

    pub fn clear_request_method(&mut self) {
        self.request_method = super::base::RequestMethod::METHOD_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_request_method(&mut self, v: super::base::RequestMethod) {
        self.request_method = v;
    }

    pub fn get_request_method(&self) -> super::base::RequestMethod {
        self.request_method
    }

    fn get_request_method_for_reflect(&self) -> &super::base::RequestMethod {
        &self.request_method
    }

    fn mut_request_method_for_reflect(&mut self) -> &mut super::base::RequestMethod {
        &mut self.request_method
    }

    // .google.protobuf.Timestamp start_time = 2;

    pub fn clear_start_time(&mut self) {
        self.start_time.clear();
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.start_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.start_time.is_none() {
            self.start_time.set_default();
        }
        self.start_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.start_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    pub fn get_start_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.start_time.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }

    fn get_start_time_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp> {
        &mut self.start_time
    }

    // .envoy.api.v2.EnvoyAccessLog.Protocol protocol_variant = 3;

    pub fn clear_protocol_variant(&mut self) {
        self.protocol_variant = EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_protocol_variant(&mut self, v: EnvoyAccessLog_Protocol) {
        self.protocol_variant = v;
    }

    pub fn get_protocol_variant(&self) -> EnvoyAccessLog_Protocol {
        self.protocol_variant
    }

    fn get_protocol_variant_for_reflect(&self) -> &EnvoyAccessLog_Protocol {
        &self.protocol_variant
    }

    fn mut_protocol_variant_for_reflect(&mut self) -> &mut EnvoyAccessLog_Protocol {
        &mut self.protocol_variant
    }

    // repeated .envoy.api.v2.EnvoyAccessLog.ResponseFlag response_flags = 4;

    pub fn clear_response_flags(&mut self) {
        self.response_flags.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_flags(&mut self, v: ::std::vec::Vec<EnvoyAccessLog_ResponseFlag>) {
        self.response_flags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_flags(&mut self) -> &mut ::std::vec::Vec<EnvoyAccessLog_ResponseFlag> {
        &mut self.response_flags
    }

    // Take field
    pub fn take_response_flags(&mut self) -> ::std::vec::Vec<EnvoyAccessLog_ResponseFlag> {
        ::std::mem::replace(&mut self.response_flags, ::std::vec::Vec::new())
    }

    pub fn get_response_flags(&self) -> &[EnvoyAccessLog_ResponseFlag] {
        &self.response_flags
    }

    fn get_response_flags_for_reflect(&self) -> &::std::vec::Vec<EnvoyAccessLog_ResponseFlag> {
        &self.response_flags
    }

    fn mut_response_flags_for_reflect(&mut self) -> &mut ::std::vec::Vec<EnvoyAccessLog_ResponseFlag> {
        &mut self.response_flags
    }

    // string upstream_host = 5;

    pub fn clear_upstream_host(&mut self) {
        self.upstream_host.clear();
    }

    // Param is passed by value, moved
    pub fn set_upstream_host(&mut self, v: ::std::string::String) {
        self.upstream_host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upstream_host(&mut self) -> &mut ::std::string::String {
        &mut self.upstream_host
    }

    // Take field
    pub fn take_upstream_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.upstream_host, ::std::string::String::new())
    }

    pub fn get_upstream_host(&self) -> &str {
        &self.upstream_host
    }

    fn get_upstream_host_for_reflect(&self) -> &::std::string::String {
        &self.upstream_host
    }

    fn mut_upstream_host_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.upstream_host
    }

    // string upstream_cluster = 6;

    pub fn clear_upstream_cluster(&mut self) {
        self.upstream_cluster.clear();
    }

    // Param is passed by value, moved
    pub fn set_upstream_cluster(&mut self, v: ::std::string::String) {
        self.upstream_cluster = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upstream_cluster(&mut self) -> &mut ::std::string::String {
        &mut self.upstream_cluster
    }

    // Take field
    pub fn take_upstream_cluster(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.upstream_cluster, ::std::string::String::new())
    }

    pub fn get_upstream_cluster(&self) -> &str {
        &self.upstream_cluster
    }

    fn get_upstream_cluster_for_reflect(&self) -> &::std::string::String {
        &self.upstream_cluster
    }

    fn mut_upstream_cluster_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.upstream_cluster
    }

    // string destination_host = 7;

    pub fn clear_destination_host(&mut self) {
        self.destination_host.clear();
    }

    // Param is passed by value, moved
    pub fn set_destination_host(&mut self, v: ::std::string::String) {
        self.destination_host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination_host(&mut self) -> &mut ::std::string::String {
        &mut self.destination_host
    }

    // Take field
    pub fn take_destination_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.destination_host, ::std::string::String::new())
    }

    pub fn get_destination_host(&self) -> &str {
        &self.destination_host
    }

    fn get_destination_host_for_reflect(&self) -> &::std::string::String {
        &self.destination_host
    }

    fn mut_destination_host_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.destination_host
    }

    // .google.protobuf.UInt64Value request_body_bytes = 8;

    pub fn clear_request_body_bytes(&mut self) {
        self.request_body_bytes.clear();
    }

    pub fn has_request_body_bytes(&self) -> bool {
        self.request_body_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_body_bytes(&mut self, v: ::protobuf::well_known_types::UInt64Value) {
        self.request_body_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_body_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt64Value {
        if self.request_body_bytes.is_none() {
            self.request_body_bytes.set_default();
        }
        self.request_body_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_body_bytes(&mut self) -> ::protobuf::well_known_types::UInt64Value {
        self.request_body_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::new())
    }

    pub fn get_request_body_bytes(&self) -> &::protobuf::well_known_types::UInt64Value {
        self.request_body_bytes.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::default_instance())
    }

    fn get_request_body_bytes_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &self.request_body_bytes
    }

    fn mut_request_body_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &mut self.request_body_bytes
    }

    // .google.protobuf.UInt64Value response_body_bytes = 9;

    pub fn clear_response_body_bytes(&mut self) {
        self.response_body_bytes.clear();
    }

    pub fn has_response_body_bytes(&self) -> bool {
        self.response_body_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_body_bytes(&mut self, v: ::protobuf::well_known_types::UInt64Value) {
        self.response_body_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_body_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt64Value {
        if self.response_body_bytes.is_none() {
            self.response_body_bytes.set_default();
        }
        self.response_body_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_body_bytes(&mut self) -> ::protobuf::well_known_types::UInt64Value {
        self.response_body_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::new())
    }

    pub fn get_response_body_bytes(&self) -> &::protobuf::well_known_types::UInt64Value {
        self.response_body_bytes.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::default_instance())
    }

    fn get_response_body_bytes_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &self.response_body_bytes
    }

    fn mut_response_body_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &mut self.response_body_bytes
    }

    // .google.protobuf.UInt64Value request_headers_bytes = 10;

    pub fn clear_request_headers_bytes(&mut self) {
        self.request_headers_bytes.clear();
    }

    pub fn has_request_headers_bytes(&self) -> bool {
        self.request_headers_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_headers_bytes(&mut self, v: ::protobuf::well_known_types::UInt64Value) {
        self.request_headers_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_headers_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt64Value {
        if self.request_headers_bytes.is_none() {
            self.request_headers_bytes.set_default();
        }
        self.request_headers_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_headers_bytes(&mut self) -> ::protobuf::well_known_types::UInt64Value {
        self.request_headers_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::new())
    }

    pub fn get_request_headers_bytes(&self) -> &::protobuf::well_known_types::UInt64Value {
        self.request_headers_bytes.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::default_instance())
    }

    fn get_request_headers_bytes_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &self.request_headers_bytes
    }

    fn mut_request_headers_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &mut self.request_headers_bytes
    }

    // .google.protobuf.UInt64Value response_headers_bytes = 11;

    pub fn clear_response_headers_bytes(&mut self) {
        self.response_headers_bytes.clear();
    }

    pub fn has_response_headers_bytes(&self) -> bool {
        self.response_headers_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_headers_bytes(&mut self, v: ::protobuf::well_known_types::UInt64Value) {
        self.response_headers_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_headers_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt64Value {
        if self.response_headers_bytes.is_none() {
            self.response_headers_bytes.set_default();
        }
        self.response_headers_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_headers_bytes(&mut self) -> ::protobuf::well_known_types::UInt64Value {
        self.response_headers_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::new())
    }

    pub fn get_response_headers_bytes(&self) -> &::protobuf::well_known_types::UInt64Value {
        self.response_headers_bytes.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt64Value::default_instance())
    }

    fn get_response_headers_bytes_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &self.response_headers_bytes
    }

    fn mut_response_headers_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt64Value> {
        &mut self.response_headers_bytes
    }

    // .google.protobuf.BoolValue secure = 12;

    pub fn clear_secure(&mut self) {
        self.secure.clear();
    }

    pub fn has_secure(&self) -> bool {
        self.secure.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secure(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.secure = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secure(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.secure.is_none() {
            self.secure.set_default();
        }
        self.secure.as_mut().unwrap()
    }

    // Take field
    pub fn take_secure(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.secure.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_secure(&self) -> &::protobuf::well_known_types::BoolValue {
        self.secure.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_secure_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.secure
    }

    fn mut_secure_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.secure
    }

    // .google.protobuf.BoolValue health_check = 13;

    pub fn clear_health_check(&mut self) {
        self.health_check.clear();
    }

    pub fn has_health_check(&self) -> bool {
        self.health_check.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_check(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.health_check = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_health_check(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.health_check.is_none() {
            self.health_check.set_default();
        }
        self.health_check.as_mut().unwrap()
    }

    // Take field
    pub fn take_health_check(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.health_check.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_health_check(&self) -> &::protobuf::well_known_types::BoolValue {
        self.health_check.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_health_check_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.health_check
    }

    fn mut_health_check_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.health_check
    }

    // .google.protobuf.UInt32Value response_code = 14;

    pub fn clear_response_code(&mut self) {
        self.response_code.clear();
    }

    pub fn has_response_code(&self) -> bool {
        self.response_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_code(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.response_code = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_code(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.response_code.is_none() {
            self.response_code.set_default();
        }
        self.response_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_code(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.response_code.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_response_code(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.response_code.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_response_code_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.response_code
    }

    fn mut_response_code_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.response_code
    }

    // string user_agent = 15;

    pub fn clear_user_agent(&mut self) {
        self.user_agent.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_agent(&mut self, v: ::std::string::String) {
        self.user_agent = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_agent(&mut self) -> &mut ::std::string::String {
        &mut self.user_agent
    }

    // Take field
    pub fn take_user_agent(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_agent, ::std::string::String::new())
    }

    pub fn get_user_agent(&self) -> &str {
        &self.user_agent
    }

    fn get_user_agent_for_reflect(&self) -> &::std::string::String {
        &self.user_agent
    }

    fn mut_user_agent_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.user_agent
    }

    // string path = 17;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path, ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::string::String {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path
    }

    // string referer = 18;

    pub fn clear_referer(&mut self) {
        self.referer.clear();
    }

    // Param is passed by value, moved
    pub fn set_referer(&mut self, v: ::std::string::String) {
        self.referer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_referer(&mut self) -> &mut ::std::string::String {
        &mut self.referer
    }

    // Take field
    pub fn take_referer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.referer, ::std::string::String::new())
    }

    pub fn get_referer(&self) -> &str {
        &self.referer
    }

    fn get_referer_for_reflect(&self) -> &::std::string::String {
        &self.referer
    }

    fn mut_referer_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.referer
    }

    // string forwarded_for = 19;

    pub fn clear_forwarded_for(&mut self) {
        self.forwarded_for.clear();
    }

    // Param is passed by value, moved
    pub fn set_forwarded_for(&mut self, v: ::std::string::String) {
        self.forwarded_for = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forwarded_for(&mut self) -> &mut ::std::string::String {
        &mut self.forwarded_for
    }

    // Take field
    pub fn take_forwarded_for(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.forwarded_for, ::std::string::String::new())
    }

    pub fn get_forwarded_for(&self) -> &str {
        &self.forwarded_for
    }

    fn get_forwarded_for_for_reflect(&self) -> &::std::string::String {
        &self.forwarded_for
    }

    fn mut_forwarded_for_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.forwarded_for
    }

    // string request_id = 20;

    pub fn clear_request_id(&mut self) {
        self.request_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_id(&mut self, v: ::std::string::String) {
        self.request_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_id(&mut self) -> &mut ::std::string::String {
        &mut self.request_id
    }

    // Take field
    pub fn take_request_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.request_id, ::std::string::String::new())
    }

    pub fn get_request_id(&self) -> &str {
        &self.request_id
    }

    fn get_request_id_for_reflect(&self) -> &::std::string::String {
        &self.request_id
    }

    fn mut_request_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.request_id
    }

    // string authority = 21;

    pub fn clear_authority(&mut self) {
        self.authority.clear();
    }

    // Param is passed by value, moved
    pub fn set_authority(&mut self, v: ::std::string::String) {
        self.authority = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_authority(&mut self) -> &mut ::std::string::String {
        &mut self.authority
    }

    // Take field
    pub fn take_authority(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.authority, ::std::string::String::new())
    }

    pub fn get_authority(&self) -> &str {
        &self.authority
    }

    fn get_authority_for_reflect(&self) -> &::std::string::String {
        &self.authority
    }

    fn mut_authority_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.authority
    }

    // .google.protobuf.Duration response_duration = 22;

    pub fn clear_response_duration(&mut self) {
        self.response_duration.clear();
    }

    pub fn has_response_duration(&self) -> bool {
        self.response_duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response_duration(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.response_duration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_duration(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.response_duration.is_none() {
            self.response_duration.set_default();
        }
        self.response_duration.as_mut().unwrap()
    }

    // Take field
    pub fn take_response_duration(&mut self) -> ::protobuf::well_known_types::Duration {
        self.response_duration.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_response_duration(&self) -> &::protobuf::well_known_types::Duration {
        self.response_duration.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_response_duration_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.response_duration
    }

    fn mut_response_duration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.response_duration
    }

    // .google.protobuf.Duration upstream_service_duration = 23;

    pub fn clear_upstream_service_duration(&mut self) {
        self.upstream_service_duration.clear();
    }

    pub fn has_upstream_service_duration(&self) -> bool {
        self.upstream_service_duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upstream_service_duration(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.upstream_service_duration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upstream_service_duration(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.upstream_service_duration.is_none() {
            self.upstream_service_duration.set_default();
        }
        self.upstream_service_duration.as_mut().unwrap()
    }

    // Take field
    pub fn take_upstream_service_duration(&mut self) -> ::protobuf::well_known_types::Duration {
        self.upstream_service_duration.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_upstream_service_duration(&self) -> &::protobuf::well_known_types::Duration {
        self.upstream_service_duration.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_upstream_service_duration_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.upstream_service_duration
    }

    fn mut_upstream_service_duration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.upstream_service_duration
    }

    // string original_path = 24;

    pub fn clear_original_path(&mut self) {
        self.original_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_original_path(&mut self, v: ::std::string::String) {
        self.original_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_original_path(&mut self) -> &mut ::std::string::String {
        &mut self.original_path
    }

    // Take field
    pub fn take_original_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.original_path, ::std::string::String::new())
    }

    pub fn get_original_path(&self) -> &str {
        &self.original_path
    }

    fn get_original_path_for_reflect(&self) -> &::std::string::String {
        &self.original_path
    }

    fn mut_original_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.original_path
    }

    // .google.protobuf.Struct metadata = 25;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::well_known_types::Struct {
        self.metadata.take().unwrap_or_else(|| ::protobuf::well_known_types::Struct::new())
    }

    pub fn get_metadata(&self) -> &::protobuf::well_known_types::Struct {
        self.metadata.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Struct::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &mut self.metadata
    }

    // repeated .envoy.api.v2.HeaderValue request_headers = 26;

    pub fn clear_request_headers(&mut self) {
        self.request_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValue>) {
        self.request_headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValue> {
        &mut self.request_headers
    }

    // Take field
    pub fn take_request_headers(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValue> {
        ::std::mem::replace(&mut self.request_headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_headers(&self) -> &[super::base::HeaderValue] {
        &self.request_headers
    }

    fn get_request_headers_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValue> {
        &self.request_headers
    }

    fn mut_request_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValue> {
        &mut self.request_headers
    }

    // repeated .envoy.api.v2.HeaderValue response_headers = 27;

    pub fn clear_response_headers(&mut self) {
        self.response_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_headers(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValue>) {
        self.response_headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_headers(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValue> {
        &mut self.response_headers
    }

    // Take field
    pub fn take_response_headers(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValue> {
        ::std::mem::replace(&mut self.response_headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_response_headers(&self) -> &[super::base::HeaderValue] {
        &self.response_headers
    }

    fn get_response_headers_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValue> {
        &self.response_headers
    }

    fn mut_response_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValue> {
        &mut self.response_headers
    }

    // string tls_sni_hostname = 28;

    pub fn clear_tls_sni_hostname(&mut self) {
        self.tls_sni_hostname.clear();
    }

    // Param is passed by value, moved
    pub fn set_tls_sni_hostname(&mut self, v: ::std::string::String) {
        self.tls_sni_hostname = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_sni_hostname(&mut self) -> &mut ::std::string::String {
        &mut self.tls_sni_hostname
    }

    // Take field
    pub fn take_tls_sni_hostname(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tls_sni_hostname, ::std::string::String::new())
    }

    pub fn get_tls_sni_hostname(&self) -> &str {
        &self.tls_sni_hostname
    }

    fn get_tls_sni_hostname_for_reflect(&self) -> &::std::string::String {
        &self.tls_sni_hostname
    }

    fn mut_tls_sni_hostname_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tls_sni_hostname
    }

    // .envoy.api.v2.EnvoyAccessLog.TLSVersion tls_version = 29;

    pub fn clear_tls_version(&mut self) {
        self.tls_version = EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_tls_version(&mut self, v: EnvoyAccessLog_TLSVersion) {
        self.tls_version = v;
    }

    pub fn get_tls_version(&self) -> EnvoyAccessLog_TLSVersion {
        self.tls_version
    }

    fn get_tls_version_for_reflect(&self) -> &EnvoyAccessLog_TLSVersion {
        &self.tls_version
    }

    fn mut_tls_version_for_reflect(&mut self) -> &mut EnvoyAccessLog_TLSVersion {
        &mut self.tls_version
    }

    // .google.protobuf.UInt32Value tls_cipher_suite = 30;

    pub fn clear_tls_cipher_suite(&mut self) {
        self.tls_cipher_suite.clear();
    }

    pub fn has_tls_cipher_suite(&self) -> bool {
        self.tls_cipher_suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tls_cipher_suite(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.tls_cipher_suite = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_cipher_suite(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.tls_cipher_suite.is_none() {
            self.tls_cipher_suite.set_default();
        }
        self.tls_cipher_suite.as_mut().unwrap()
    }

    // Take field
    pub fn take_tls_cipher_suite(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.tls_cipher_suite.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_tls_cipher_suite(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.tls_cipher_suite.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_tls_cipher_suite_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.tls_cipher_suite
    }

    fn mut_tls_cipher_suite_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.tls_cipher_suite
    }
}

impl ::protobuf::Message for EnvoyAccessLog {
    fn is_initialized(&self) -> bool {
        for v in &self.start_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_body_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_body_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_headers_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.secure {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.health_check {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_code {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_duration {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upstream_service_duration {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_headers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tls_cipher_suite {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.request_method = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_time)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.protocol_variant = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.response_flags)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.upstream_host)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.upstream_cluster)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.destination_host)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request_body_bytes)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_body_bytes)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request_headers_bytes)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_headers_bytes)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.secure)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.health_check)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_code)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_agent)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.referer)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.forwarded_for)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.request_id)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.authority)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response_duration)?;
                },
                23 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upstream_service_duration)?;
                },
                24 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.original_path)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                26 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_headers)?;
                },
                27 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.response_headers)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tls_sni_hostname)?;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.tls_version = tmp;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_cipher_suite)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.request_method != super::base::RequestMethod::METHOD_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(1, self.request_method);
        }
        if let Some(ref v) = self.start_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.protocol_variant != EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(3, self.protocol_variant);
        }
        for value in &self.response_flags {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        if !self.upstream_host.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.upstream_host);
        }
        if !self.upstream_cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.upstream_cluster);
        }
        if !self.destination_host.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.destination_host);
        }
        if let Some(ref v) = self.request_body_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response_body_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.request_headers_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response_headers_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.secure.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.health_check.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response_code.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.user_agent.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.user_agent);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.path);
        }
        if !self.referer.is_empty() {
            my_size += ::protobuf::rt::string_size(18, &self.referer);
        }
        if !self.forwarded_for.is_empty() {
            my_size += ::protobuf::rt::string_size(19, &self.forwarded_for);
        }
        if !self.request_id.is_empty() {
            my_size += ::protobuf::rt::string_size(20, &self.request_id);
        }
        if !self.authority.is_empty() {
            my_size += ::protobuf::rt::string_size(21, &self.authority);
        }
        if let Some(ref v) = self.response_duration.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.upstream_service_duration.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.original_path.is_empty() {
            my_size += ::protobuf::rt::string_size(24, &self.original_path);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.request_headers {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.response_headers {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.tls_sni_hostname.is_empty() {
            my_size += ::protobuf::rt::string_size(28, &self.tls_sni_hostname);
        }
        if self.tls_version != EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(29, self.tls_version);
        }
        if let Some(ref v) = self.tls_cipher_suite.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.request_method != super::base::RequestMethod::METHOD_UNSPECIFIED {
            os.write_enum(1, self.request_method.value())?;
        }
        if let Some(ref v) = self.start_time.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.protocol_variant != EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED {
            os.write_enum(3, self.protocol_variant.value())?;
        }
        for v in &self.response_flags {
            os.write_enum(4, v.value())?;
        };
        if !self.upstream_host.is_empty() {
            os.write_string(5, &self.upstream_host)?;
        }
        if !self.upstream_cluster.is_empty() {
            os.write_string(6, &self.upstream_cluster)?;
        }
        if !self.destination_host.is_empty() {
            os.write_string(7, &self.destination_host)?;
        }
        if let Some(ref v) = self.request_body_bytes.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response_body_bytes.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.request_headers_bytes.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response_headers_bytes.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.secure.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.health_check.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response_code.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.user_agent.is_empty() {
            os.write_string(15, &self.user_agent)?;
        }
        if !self.path.is_empty() {
            os.write_string(17, &self.path)?;
        }
        if !self.referer.is_empty() {
            os.write_string(18, &self.referer)?;
        }
        if !self.forwarded_for.is_empty() {
            os.write_string(19, &self.forwarded_for)?;
        }
        if !self.request_id.is_empty() {
            os.write_string(20, &self.request_id)?;
        }
        if !self.authority.is_empty() {
            os.write_string(21, &self.authority)?;
        }
        if let Some(ref v) = self.response_duration.as_ref() {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.upstream_service_duration.as_ref() {
            os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.original_path.is_empty() {
            os.write_string(24, &self.original_path)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.request_headers {
            os.write_tag(26, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.response_headers {
            os.write_tag(27, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.tls_sni_hostname.is_empty() {
            os.write_string(28, &self.tls_sni_hostname)?;
        }
        if self.tls_version != EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED {
            os.write_enum(29, self.tls_version.value())?;
        }
        if let Some(ref v) = self.tls_cipher_suite.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnvoyAccessLog {
    fn new() -> EnvoyAccessLog {
        EnvoyAccessLog::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnvoyAccessLog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::base::RequestMethod>>(
                    "request_method",
                    EnvoyAccessLog::get_request_method_for_reflect,
                    EnvoyAccessLog::mut_request_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "start_time",
                    EnvoyAccessLog::get_start_time_for_reflect,
                    EnvoyAccessLog::mut_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EnvoyAccessLog_Protocol>>(
                    "protocol_variant",
                    EnvoyAccessLog::get_protocol_variant_for_reflect,
                    EnvoyAccessLog::mut_protocol_variant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EnvoyAccessLog_ResponseFlag>>(
                    "response_flags",
                    EnvoyAccessLog::get_response_flags_for_reflect,
                    EnvoyAccessLog::mut_response_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "upstream_host",
                    EnvoyAccessLog::get_upstream_host_for_reflect,
                    EnvoyAccessLog::mut_upstream_host_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "upstream_cluster",
                    EnvoyAccessLog::get_upstream_cluster_for_reflect,
                    EnvoyAccessLog::mut_upstream_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "destination_host",
                    EnvoyAccessLog::get_destination_host_for_reflect,
                    EnvoyAccessLog::mut_destination_host_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt64Value>>(
                    "request_body_bytes",
                    EnvoyAccessLog::get_request_body_bytes_for_reflect,
                    EnvoyAccessLog::mut_request_body_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt64Value>>(
                    "response_body_bytes",
                    EnvoyAccessLog::get_response_body_bytes_for_reflect,
                    EnvoyAccessLog::mut_response_body_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt64Value>>(
                    "request_headers_bytes",
                    EnvoyAccessLog::get_request_headers_bytes_for_reflect,
                    EnvoyAccessLog::mut_request_headers_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt64Value>>(
                    "response_headers_bytes",
                    EnvoyAccessLog::get_response_headers_bytes_for_reflect,
                    EnvoyAccessLog::mut_response_headers_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "secure",
                    EnvoyAccessLog::get_secure_for_reflect,
                    EnvoyAccessLog::mut_secure_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "health_check",
                    EnvoyAccessLog::get_health_check_for_reflect,
                    EnvoyAccessLog::mut_health_check_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "response_code",
                    EnvoyAccessLog::get_response_code_for_reflect,
                    EnvoyAccessLog::mut_response_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "user_agent",
                    EnvoyAccessLog::get_user_agent_for_reflect,
                    EnvoyAccessLog::mut_user_agent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    EnvoyAccessLog::get_path_for_reflect,
                    EnvoyAccessLog::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "referer",
                    EnvoyAccessLog::get_referer_for_reflect,
                    EnvoyAccessLog::mut_referer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "forwarded_for",
                    EnvoyAccessLog::get_forwarded_for_for_reflect,
                    EnvoyAccessLog::mut_forwarded_for_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "request_id",
                    EnvoyAccessLog::get_request_id_for_reflect,
                    EnvoyAccessLog::mut_request_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "authority",
                    EnvoyAccessLog::get_authority_for_reflect,
                    EnvoyAccessLog::mut_authority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "response_duration",
                    EnvoyAccessLog::get_response_duration_for_reflect,
                    EnvoyAccessLog::mut_response_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "upstream_service_duration",
                    EnvoyAccessLog::get_upstream_service_duration_for_reflect,
                    EnvoyAccessLog::mut_upstream_service_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "original_path",
                    EnvoyAccessLog::get_original_path_for_reflect,
                    EnvoyAccessLog::mut_original_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "metadata",
                    EnvoyAccessLog::get_metadata_for_reflect,
                    EnvoyAccessLog::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValue>>(
                    "request_headers",
                    EnvoyAccessLog::get_request_headers_for_reflect,
                    EnvoyAccessLog::mut_request_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValue>>(
                    "response_headers",
                    EnvoyAccessLog::get_response_headers_for_reflect,
                    EnvoyAccessLog::mut_response_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tls_sni_hostname",
                    EnvoyAccessLog::get_tls_sni_hostname_for_reflect,
                    EnvoyAccessLog::mut_tls_sni_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EnvoyAccessLog_TLSVersion>>(
                    "tls_version",
                    EnvoyAccessLog::get_tls_version_for_reflect,
                    EnvoyAccessLog::mut_tls_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "tls_cipher_suite",
                    EnvoyAccessLog::get_tls_cipher_suite_for_reflect,
                    EnvoyAccessLog::mut_tls_cipher_suite_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnvoyAccessLog>(
                    "EnvoyAccessLog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnvoyAccessLog {
    fn clear(&mut self) {
        self.clear_request_method();
        self.clear_start_time();
        self.clear_protocol_variant();
        self.clear_response_flags();
        self.clear_upstream_host();
        self.clear_upstream_cluster();
        self.clear_destination_host();
        self.clear_request_body_bytes();
        self.clear_response_body_bytes();
        self.clear_request_headers_bytes();
        self.clear_response_headers_bytes();
        self.clear_secure();
        self.clear_health_check();
        self.clear_response_code();
        self.clear_user_agent();
        self.clear_path();
        self.clear_referer();
        self.clear_forwarded_for();
        self.clear_request_id();
        self.clear_authority();
        self.clear_response_duration();
        self.clear_upstream_service_duration();
        self.clear_original_path();
        self.clear_metadata();
        self.clear_request_headers();
        self.clear_response_headers();
        self.clear_tls_sni_hostname();
        self.clear_tls_version();
        self.clear_tls_cipher_suite();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnvoyAccessLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnvoyAccessLog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EnvoyAccessLog_Protocol {
    PROTOCOL_UNSPECIFIED = 0,
    HTTP10 = 1,
    HTTP11 = 2,
    HTTP2 = 3,
}

impl ::protobuf::ProtobufEnum for EnvoyAccessLog_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnvoyAccessLog_Protocol> {
        match value {
            0 => ::std::option::Option::Some(EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED),
            1 => ::std::option::Option::Some(EnvoyAccessLog_Protocol::HTTP10),
            2 => ::std::option::Option::Some(EnvoyAccessLog_Protocol::HTTP11),
            3 => ::std::option::Option::Some(EnvoyAccessLog_Protocol::HTTP2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnvoyAccessLog_Protocol] = &[
            EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED,
            EnvoyAccessLog_Protocol::HTTP10,
            EnvoyAccessLog_Protocol::HTTP11,
            EnvoyAccessLog_Protocol::HTTP2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EnvoyAccessLog_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnvoyAccessLog_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnvoyAccessLog_Protocol {
}

impl ::std::default::Default for EnvoyAccessLog_Protocol {
    fn default() -> Self {
        EnvoyAccessLog_Protocol::PROTOCOL_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for EnvoyAccessLog_Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EnvoyAccessLog_ResponseFlag {
    FAILED_LOCAL_HEALTHCHECK = 0,
    NO_HEALTHY_UPSTREAM = 1,
    UPSTREAM_REQUEST_TIMEOUT = 2,
    LOCAL_RESET = 3,
    UPSTREAM_REMOTE_RESET = 4,
    UPSTREAM_CONNECTION_FAILURE = 5,
    UPSTREAM_CONNECTION_TERMINATION = 6,
    UPSTREAM_OVERFLOW = 7,
    NO_ROUTE_FOUND = 8,
    DELAY_INJECTED = 9,
    FAULT_INJECTED = 10,
    RATE_LIMITED = 11,
}

impl ::protobuf::ProtobufEnum for EnvoyAccessLog_ResponseFlag {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnvoyAccessLog_ResponseFlag> {
        match value {
            0 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::FAILED_LOCAL_HEALTHCHECK),
            1 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::NO_HEALTHY_UPSTREAM),
            2 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::UPSTREAM_REQUEST_TIMEOUT),
            3 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::LOCAL_RESET),
            4 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::UPSTREAM_REMOTE_RESET),
            5 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::UPSTREAM_CONNECTION_FAILURE),
            6 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::UPSTREAM_CONNECTION_TERMINATION),
            7 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::UPSTREAM_OVERFLOW),
            8 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::NO_ROUTE_FOUND),
            9 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::DELAY_INJECTED),
            10 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::FAULT_INJECTED),
            11 => ::std::option::Option::Some(EnvoyAccessLog_ResponseFlag::RATE_LIMITED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnvoyAccessLog_ResponseFlag] = &[
            EnvoyAccessLog_ResponseFlag::FAILED_LOCAL_HEALTHCHECK,
            EnvoyAccessLog_ResponseFlag::NO_HEALTHY_UPSTREAM,
            EnvoyAccessLog_ResponseFlag::UPSTREAM_REQUEST_TIMEOUT,
            EnvoyAccessLog_ResponseFlag::LOCAL_RESET,
            EnvoyAccessLog_ResponseFlag::UPSTREAM_REMOTE_RESET,
            EnvoyAccessLog_ResponseFlag::UPSTREAM_CONNECTION_FAILURE,
            EnvoyAccessLog_ResponseFlag::UPSTREAM_CONNECTION_TERMINATION,
            EnvoyAccessLog_ResponseFlag::UPSTREAM_OVERFLOW,
            EnvoyAccessLog_ResponseFlag::NO_ROUTE_FOUND,
            EnvoyAccessLog_ResponseFlag::DELAY_INJECTED,
            EnvoyAccessLog_ResponseFlag::FAULT_INJECTED,
            EnvoyAccessLog_ResponseFlag::RATE_LIMITED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EnvoyAccessLog_ResponseFlag>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnvoyAccessLog_ResponseFlag", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnvoyAccessLog_ResponseFlag {
}

impl ::std::default::Default for EnvoyAccessLog_ResponseFlag {
    fn default() -> Self {
        EnvoyAccessLog_ResponseFlag::FAILED_LOCAL_HEALTHCHECK
    }
}

impl ::protobuf::reflect::ProtobufValue for EnvoyAccessLog_ResponseFlag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EnvoyAccessLog_TLSVersion {
    VERSION_UNSPECIFIED = 0,
    TLSv1 = 1,
    TLSv1_1 = 2,
    TLSv1_2 = 3,
    TLSv1_3 = 4,
}

impl ::protobuf::ProtobufEnum for EnvoyAccessLog_TLSVersion {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EnvoyAccessLog_TLSVersion> {
        match value {
            0 => ::std::option::Option::Some(EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED),
            1 => ::std::option::Option::Some(EnvoyAccessLog_TLSVersion::TLSv1),
            2 => ::std::option::Option::Some(EnvoyAccessLog_TLSVersion::TLSv1_1),
            3 => ::std::option::Option::Some(EnvoyAccessLog_TLSVersion::TLSv1_2),
            4 => ::std::option::Option::Some(EnvoyAccessLog_TLSVersion::TLSv1_3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EnvoyAccessLog_TLSVersion] = &[
            EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED,
            EnvoyAccessLog_TLSVersion::TLSv1,
            EnvoyAccessLog_TLSVersion::TLSv1_1,
            EnvoyAccessLog_TLSVersion::TLSv1_2,
            EnvoyAccessLog_TLSVersion::TLSv1_3,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EnvoyAccessLog_TLSVersion>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EnvoyAccessLog_TLSVersion", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EnvoyAccessLog_TLSVersion {
}

impl ::std::default::Default for EnvoyAccessLog_TLSVersion {
    fn default() -> Self {
        EnvoyAccessLog_TLSVersion::VERSION_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for EnvoyAccessLog_TLSVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13api/accesslog.proto\x12\x0cenvoy.api.v2\x1a\x0eapi/base.proto\x1a\
    \x1egoogle/protobuf/duration.proto\x1a\x1cgoogle/protobuf/struct.proto\
    \x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x1egoogle/protobuf/wrappers.\
    proto\"\xf1\x10\n\x0eEnvoyAccessLog\x12B\n\x0erequest_method\x18\x01\x20\
    \x01(\x0e2\x1b.envoy.api.v2.RequestMethodR\rrequestMethod\x129\n\nstart_\
    time\x18\x02\x20\x01(\x0b2\x1a.google.protobuf.TimestampR\tstartTime\x12\
    P\n\x10protocol_variant\x18\x03\x20\x01(\x0e2%.envoy.api.v2.EnvoyAccessL\
    og.ProtocolR\x0fprotocolVariant\x12P\n\x0eresponse_flags\x18\x04\x20\x03\
    (\x0e2).envoy.api.v2.EnvoyAccessLog.ResponseFlagR\rresponseFlags\x12#\n\
    \rupstream_host\x18\x05\x20\x01(\tR\x0cupstreamHost\x12)\n\x10upstream_c\
    luster\x18\x06\x20\x01(\tR\x0fupstreamCluster\x12)\n\x10destination_host\
    \x18\x07\x20\x01(\tR\x0fdestinationHost\x12J\n\x12request_body_bytes\x18\
    \x08\x20\x01(\x0b2\x1c.google.protobuf.UInt64ValueR\x10requestBodyBytes\
    \x12L\n\x13response_body_bytes\x18\t\x20\x01(\x0b2\x1c.google.protobuf.U\
    Int64ValueR\x11responseBodyBytes\x12P\n\x15request_headers_bytes\x18\n\
    \x20\x01(\x0b2\x1c.google.protobuf.UInt64ValueR\x13requestHeadersBytes\
    \x12R\n\x16response_headers_bytes\x18\x0b\x20\x01(\x0b2\x1c.google.proto\
    buf.UInt64ValueR\x14responseHeadersBytes\x122\n\x06secure\x18\x0c\x20\
    \x01(\x0b2\x1a.google.protobuf.BoolValueR\x06secure\x12=\n\x0chealth_che\
    ck\x18\r\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\x0bhealthCheck\x12\
    A\n\rresponse_code\x18\x0e\x20\x01(\x0b2\x1c.google.protobuf.UInt32Value\
    R\x0cresponseCode\x12\x1d\n\nuser_agent\x18\x0f\x20\x01(\tR\tuserAgent\
    \x12\x12\n\x04path\x18\x11\x20\x01(\tR\x04path\x12\x18\n\x07referer\x18\
    \x12\x20\x01(\tR\x07referer\x12#\n\rforwarded_for\x18\x13\x20\x01(\tR\
    \x0cforwardedFor\x12\x1d\n\nrequest_id\x18\x14\x20\x01(\tR\trequestId\
    \x12\x1c\n\tauthority\x18\x15\x20\x01(\tR\tauthority\x12F\n\x11response_\
    duration\x18\x16\x20\x01(\x0b2\x19.google.protobuf.DurationR\x10response\
    Duration\x12U\n\x19upstream_service_duration\x18\x17\x20\x01(\x0b2\x19.g\
    oogle.protobuf.DurationR\x17upstreamServiceDuration\x12#\n\roriginal_pat\
    h\x18\x18\x20\x01(\tR\x0coriginalPath\x123\n\x08metadata\x18\x19\x20\x01\
    (\x0b2\x17.google.protobuf.StructR\x08metadata\x12B\n\x0frequest_headers\
    \x18\x1a\x20\x03(\x0b2\x19.envoy.api.v2.HeaderValueR\x0erequestHeaders\
    \x12D\n\x10response_headers\x18\x1b\x20\x03(\x0b2\x19.envoy.api.v2.Heade\
    rValueR\x0fresponseHeaders\x12(\n\x10tls_sni_hostname\x18\x1c\x20\x01(\t\
    R\x0etlsSniHostname\x12H\n\x0btls_version\x18\x1d\x20\x01(\x0e2'.envoy.a\
    pi.v2.EnvoyAccessLog.TLSVersionR\ntlsVersion\x12F\n\x10tls_cipher_suite\
    \x18\x1e\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x0etlsCipherSuit\
    e\"G\n\x08Protocol\x12\x18\n\x14PROTOCOL_UNSPECIFIED\x10\0\x12\n\n\x06HT\
    TP10\x10\x01\x12\n\n\x06HTTP11\x10\x02\x12\t\n\x05HTTP2\x10\x03\"\xba\
    \x02\n\x0cResponseFlag\x12\x1c\n\x18FAILED_LOCAL_HEALTHCHECK\x10\0\x12\
    \x17\n\x13NO_HEALTHY_UPSTREAM\x10\x01\x12\x1c\n\x18UPSTREAM_REQUEST_TIME\
    OUT\x10\x02\x12\x0f\n\x0bLOCAL_RESET\x10\x03\x12\x19\n\x15UPSTREAM_REMOT\
    E_RESET\x10\x04\x12\x1f\n\x1bUPSTREAM_CONNECTION_FAILURE\x10\x05\x12#\n\
    \x1fUPSTREAM_CONNECTION_TERMINATION\x10\x06\x12\x15\n\x11UPSTREAM_OVERFL\
    OW\x10\x07\x12\x12\n\x0eNO_ROUTE_FOUND\x10\x08\x12\x12\n\x0eDELAY_INJECT\
    ED\x10\t\x12\x12\n\x0eFAULT_INJECTED\x10\n\x12\x10\n\x0cRATE_LIMITED\x10\
    \x0b\"W\n\nTLSVersion\x12\x17\n\x13VERSION_UNSPECIFIED\x10\0\x12\t\n\x05\
    TLSv1\x10\x01\x12\x0b\n\x07TLSv1_1\x10\x02\x12\x0b\n\x07TLSv1_2\x10\x03\
    \x12\x0b\n\x07TLSv1_3\x10\x04J\xa63\n\x07\x12\x05\0\0\xa9\x01\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\n\t\n\x02\
    \x03\0\x12\x03\x04\x07\x17\n\t\n\x02\x03\x01\x12\x03\x06\x07'\n\t\n\x02\
    \x03\x02\x12\x03\x07\x07%\n\t\n\x02\x03\x03\x12\x03\x08\x07(\n\t\n\x02\
    \x03\x04\x12\x03\t\x07'\n\x0b\n\x02\x04\0\x12\x05\x0b\0\xa9\x01\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x0b\x08\x16\n6\n\x04\x04\0\x02\0\x12\x03\r\x02#\
    \x1a)\x20The\x20HTTP\x20request\x20method\x20(RFC\x207231/2616)\n\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\r\x02\x0b\x18\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03\r\x02\x0f\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\x10\x1e\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\r!\"\nA\n\x04\x04\0\x02\x01\x12\x03\x10\x02\
    +\x1a4\x20The\x20time\x20that\x20Envoy\x20started\x20servicing\x20this\
    \x20request\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x10\x02\r#\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\x10\x02\x1b\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x10\x1c&\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10)*\n2\n\x04\x04\0\
    \x04\0\x12\x04\x13\x02\x18\x03\x1a$\x20Incoming\x20protocol\x20variation\
    \x20spoken\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x13\x07\x0f\n\r\n\x06\
    \x04\0\x04\0\x02\0\x12\x03\x14\x04\x1d\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\
    \x12\x03\x14\x04\x18\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x14\x1b\
    \x1c\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03\x15\x04\x0f\n\x0e\n\x07\x04\0\
    \x04\0\x02\x01\x01\x12\x03\x15\x04\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\
    \x12\x03\x15\r\x0e\n\r\n\x06\x04\0\x04\0\x02\x02\x12\x03\x16\x04\x0f\n\
    \x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03\x16\x04\n\n\x0e\n\x07\x04\0\
    \x04\0\x02\x02\x02\x12\x03\x16\r\x0e\n\r\n\x06\x04\0\x04\0\x02\x03\x12\
    \x03\x17\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\x03\x01\x12\x03\x17\x04\t\n\
    \x0e\n\x07\x04\0\x04\0\x02\x03\x02\x12\x03\x17\x0c\r\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\x19\x02\x20\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x19\x02\
    \x18\x03\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x19\x02\n\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03\x19\x0b\x1b\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x19\x1e\x1f\nh\n\x04\x04\0\x04\x01\x12\x04\x1d\x026\x03\x1aZ\x20This\
    \x20enum\x20defines\x20the\x20various\x20things\x20that\x20may\x20have\
    \x20occurred\x20while\n\x20processing\x20a\x20request.\n\n\x0c\n\x05\x04\
    \0\x04\x01\x01\x12\x03\x1d\x07\x13\n1\n\x06\x04\0\x04\x01\x02\0\x12\x03\
    \x1f\x04!\x1a\"\x20Local\x20server\x20healthcheck\x20failed.\n\n\x0e\n\
    \x07\x04\0\x04\x01\x02\0\x01\x12\x03\x1f\x04\x1c\n\x0e\n\x07\x04\0\x04\
    \x01\x02\0\x02\x12\x03\x1f\x1f\x20\n%\n\x06\x04\0\x04\x01\x02\x01\x12\
    \x03!\x04\x1c\x1a\x16\x20No\x20healthy\x20upstream.\n\n\x0e\n\x07\x04\0\
    \x04\x01\x02\x01\x01\x12\x03!\x04\x17\n\x0e\n\x07\x04\0\x04\x01\x02\x01\
    \x02\x12\x03!\x1a\x1b\n-\n\x06\x04\0\x04\x01\x02\x02\x12\x03#\x04!\x1a\
    \x1e\x20Request\x20timeout\x20on\x20upstream.\n\n\x0e\n\x07\x04\0\x04\
    \x01\x02\x02\x01\x12\x03#\x04\x1c\n\x0e\n\x07\x04\0\x04\x01\x02\x02\x02\
    \x12\x03#\x1f\x20\n@\n\x06\x04\0\x04\x01\x02\x03\x12\x03%\x04\x14\x1a1\
    \x20Local\x20codec\x20level\x20reset\x20was\x20sent\x20on\x20the\x20stre\
    am.\n\n\x0e\n\x07\x04\0\x04\x01\x02\x03\x01\x12\x03%\x04\x0f\n\x0e\n\x07\
    \x04\0\x04\x01\x02\x03\x02\x12\x03%\x12\x13\nE\n\x06\x04\0\x04\x01\x02\
    \x04\x12\x03'\x04\x1e\x1a6\x20Remote\x20codec\x20level\x20reset\x20was\
    \x20received\x20on\x20the\x20stream.\n\n\x0e\n\x07\x04\0\x04\x01\x02\x04\
    \x01\x12\x03'\x04\x19\n\x0e\n\x07\x04\0\x04\x01\x02\x04\x02\x12\x03'\x1c\
    \x1d\nW\n\x06\x04\0\x04\x01\x02\x05\x12\x03)\x04$\x1aH\x20Local\x20reset\
    \x20by\x20a\x20connection\x20pool\x20due\x20to\x20an\x20initial\x20conne\
    ction\x20failure.\n\n\x0e\n\x07\x04\0\x04\x01\x02\x05\x01\x12\x03)\x04\
    \x1f\n\x0e\n\x07\x04\0\x04\x01\x02\x05\x02\x12\x03)\"#\nO\n\x06\x04\0\
    \x04\x01\x02\x06\x12\x03+\x04(\x1a@\x20If\x20the\x20stream\x20was\x20loc\
    ally\x20reset\x20due\x20to\x20connection\x20termination.\n\n\x0e\n\x07\
    \x04\0\x04\x01\x02\x06\x01\x12\x03+\x04#\n\x0e\n\x07\x04\0\x04\x01\x02\
    \x06\x02\x12\x03+&'\nE\n\x06\x04\0\x04\x01\x02\x07\x12\x03-\x04\x1a\x1a6\
    \x20The\x20stream\x20was\x20reset\x20because\x20of\x20a\x20resource\x20o\
    verflow.\n\n\x0e\n\x07\x04\0\x04\x01\x02\x07\x01\x12\x03-\x04\x15\n\x0e\
    \n\x07\x04\0\x04\x01\x02\x07\x02\x12\x03-\x18\x19\n4\n\x06\x04\0\x04\x01\
    \x02\x08\x12\x03/\x04\x17\x1a%\x20No\x20route\x20found\x20for\x20a\x20gi\
    ven\x20request.\n\n\x0e\n\x07\x04\0\x04\x01\x02\x08\x01\x12\x03/\x04\x12\
    \n\x0e\n\x07\x04\0\x04\x01\x02\x08\x02\x12\x03/\x15\x16\n5\n\x06\x04\0\
    \x04\x01\x02\t\x12\x031\x04\x17\x1a&\x20Request\x20was\x20delayed\x20bef\
    ore\x20proxying.\n\n\x0e\n\x07\x04\0\x04\x01\x02\t\x01\x12\x031\x04\x12\
    \n\x0e\n\x07\x04\0\x04\x01\x02\t\x02\x12\x031\x15\x16\n4\n\x06\x04\0\x04\
    \x01\x02\n\x12\x033\x04\x18\x1a%\x20Abort\x20with\x20error\x20code\x20wa\
    s\x20injected.\n\n\x0e\n\x07\x04\0\x04\x01\x02\n\x01\x12\x033\x04\x12\n\
    \x0e\n\x07\x04\0\x04\x01\x02\n\x02\x12\x033\x15\x17\nF\n\x06\x04\0\x04\
    \x01\x02\x0b\x12\x035\x04\x16\x1a7\x20Request\x20was\x20ratelimited\x20l\
    ocally\x20by\x20rate\x20limit\x20filter.\n\n\x0e\n\x07\x04\0\x04\x01\x02\
    \x0b\x01\x12\x035\x04\x10\n\x0e\n\x07\x04\0\x04\x01\x02\x0b\x02\x12\x035\
    \x13\x15\n/\n\x04\x04\0\x02\x03\x12\x038\x02+\x1a\"\x20Status\x20flags\
    \x20about\x20the\x20response.\n\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x038\
    \x02\n\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x038\x0b\x17\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x038\x18&\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x038)*\n\
    \xcf\x01\n\x04\x04\0\x02\x04\x12\x03@\x02\x1b\x1a\xc1\x01\x20The\x20upst\
    ream\x20host\x20URL\x20(Envoy\x20connects\x20to).\n\n\x20For\x20example,\
    \x20tcp://ip:port\x20for\x20TCP\x20connections.\n\n\x20IPv6\x20addresses\
    \x20should\x20be\x20stored\x20in\x20canonical\x20(compressed)\x20format\
    \x20using\n\x20[address]:port\x20notation.\n\n\r\n\x05\x04\0\x02\x04\x04\
    \x12\x04@\x028+\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03@\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03@\t\x16\n\x0c\n\x05\x04\0\x02\x04\x03\x12\
    \x03@\x19\x1a\nF\n\x04\x04\0\x02\x05\x12\x03C\x02\x1e\x1a9\x20The\x20Ups\
    tream\x20Cluster\x20that\x20the\x20upstream\x20host\x20belongs\x20to.\n\
    \n\r\n\x05\x04\0\x02\x05\x04\x12\x04C\x02@\x1b\n\x0c\n\x05\x04\0\x02\x05\
    \x05\x12\x03C\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03C\t\x19\n\x0c\
    \n\x05\x04\0\x02\x05\x03\x12\x03C\x1c\x1d\n\x89\x01\n\x04\x04\0\x02\x06\
    \x12\x03G\x02\x1e\x1a|\x20This\x20field\x20is\x20the\x20IP\x20and\x20por\
    t\x20on\x20which\x20the\x20request\x20from\x20the\x20user\x20was\n\x20re\
    ceived,\x20stored\x20in\x20ipv4:port\x20or\x20[ipv6]:port\x20format.\n\n\
    \r\n\x05\x04\0\x02\x06\x04\x12\x04G\x02C\x1e\n\x0c\n\x05\x04\0\x02\x06\
    \x05\x12\x03G\x02\x08\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03G\t\x19\n\x0c\
    \n\x05\x04\0\x02\x06\x03\x12\x03G\x1c\x1d\n5\n\x04\x04\0\x02\x07\x12\x03\
    J\x025\x1a(\x20Size\x20of\x20the\x20HTTP\x20request\x20body\x20in\x20byt\
    es\n\n\r\n\x05\x04\0\x02\x07\x04\x12\x04J\x02G\x1e\n\x0c\n\x05\x04\0\x02\
    \x07\x06\x12\x03J\x02\x1d\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03J\x1e0\n\
    \x0c\n\x05\x04\0\x02\x07\x03\x12\x03J34\n6\n\x04\x04\0\x02\x08\x12\x03M\
    \x026\x1a)\x20Size\x20of\x20the\x20HTTP\x20response\x20body\x20in\x20byt\
    es\n\n\r\n\x05\x04\0\x02\x08\x04\x12\x04M\x02J5\n\x0c\n\x05\x04\0\x02\
    \x08\x06\x12\x03M\x02\x1d\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03M\x1e1\n\
    \x0c\n\x05\x04\0\x02\x08\x03\x12\x03M45\n8\n\x04\x04\0\x02\t\x12\x03P\
    \x029\x1a+\x20Size\x20of\x20the\x20HTTP\x20request\x20headers\x20in\x20b\
    ytes\n\n\r\n\x05\x04\0\x02\t\x04\x12\x04P\x02M6\n\x0c\n\x05\x04\0\x02\t\
    \x06\x12\x03P\x02\x1d\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03P\x1e3\n\x0c\n\
    \x05\x04\0\x02\t\x03\x12\x03P68\n9\n\x04\x04\0\x02\n\x12\x03S\x02:\x1a,\
    \x20Size\x20of\x20the\x20HTTP\x20response\x20headers\x20in\x20bytes\n\n\
    \r\n\x05\x04\0\x02\n\x04\x12\x04S\x02P9\n\x0c\n\x05\x04\0\x02\n\x06\x12\
    \x03S\x02\x1d\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03S\x1e4\n\x0c\n\x05\x04\
    \0\x02\n\x03\x12\x03S79\nF\n\x04\x04\0\x02\x0b\x12\x03V\x02(\x1a9\x20Whe\
    ther\x20the\x20request\x20arrived\x20via\x20a\x20secure\x20(TLS)\x20prot\
    ocol\n\n\r\n\x05\x04\0\x02\x0b\x04\x12\x04V\x02S:\n\x0c\n\x05\x04\0\x02\
    \x0b\x06\x12\x03V\x02\x1b\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03V\x1c\"\n\
    \x0c\n\x05\x04\0\x02\x0b\x03\x12\x03V%'\n;\n\x04\x04\0\x02\x0c\x12\x03Y\
    \x02.\x1a.\x20Whether\x20the\x20request\x20is\x20a\x20HealthCheck\x20req\
    uest\n\n\r\n\x05\x04\0\x02\x0c\x04\x12\x04Y\x02V(\n\x0c\n\x05\x04\0\x02\
    \x0c\x06\x12\x03Y\x02\x1b\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03Y\x1c(\n\
    \x0c\n\x05\x04\0\x02\x0c\x03\x12\x03Y+-\n%\n\x04\x04\0\x02\r\x12\x03\\\
    \x021\x1a\x18\x20The\x20HTTP\x20response\x20code\n\n\r\n\x05\x04\0\x02\r\
    \x04\x12\x04\\\x02Y.\n\x0c\n\x05\x04\0\x02\r\x06\x12\x03\\\x02\x1d\n\x0c\
    \n\x05\x04\0\x02\r\x01\x12\x03\\\x1e+\n\x0c\n\x05\x04\0\x02\r\x03\x12\
    \x03\\.0\n0\n\x04\x04\0\x02\x0e\x12\x03_\x02\x19\x1a#\x20User\x20agent\
    \x20as\x20sent\x20by\x20client\x20HTTP\n\n\r\n\x05\x04\0\x02\x0e\x04\x12\
    \x04_\x02\\1\n\x0c\n\x05\x04\0\x02\x0e\x05\x12\x03_\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x0e\x01\x12\x03_\t\x13\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03_\
    \x16\x18\nL\n\x04\x04\0\x02\x0f\x12\x03d\x02\x13\x1a?\x20Path\n\n\x20Thi\
    s\x20is\x20the\x20Path\x20portion\x20from\x20the\x20incoming\x20request\
    \x20URI\n\n\r\n\x05\x04\0\x02\x0f\x04\x12\x04d\x02_\x19\n\x0c\n\x05\x04\
    \0\x02\x0f\x05\x12\x03d\x02\x08\n\x0c\n\x05\x04\0\x02\x0f\x01\x12\x03d\t\
    \r\n\x0c\n\x05\x04\0\x02\x0f\x03\x12\x03d\x10\x12\no\n\x04\x04\0\x02\x10\
    \x12\x03h\x02\x16\x1ab\x20Referer\x20header\x20as\x20sent\x20by\x20clien\
    t\x20HTTP\n\x20(Referer\x20is\x20spelled\x20to\x20match\x20the\x20HTTP\
    \x20spec,\x20not\x20English).\n\n\r\n\x05\x04\0\x02\x10\x04\x12\x04h\x02\
    d\x13\n\x0c\n\x05\x04\0\x02\x10\x05\x12\x03h\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x10\x01\x12\x03h\t\x10\n\x0c\n\x05\x04\0\x02\x10\x03\x12\x03h\x13\
    \x15\n-\n\x04\x04\0\x02\x11\x12\x03k\x02\x1c\x1a\x20\x20X-Forwarded-For\
    \x20request\x20header\n\n\r\n\x05\x04\0\x02\x11\x04\x12\x04k\x02h\x16\n\
    \x0c\n\x05\x04\0\x02\x11\x05\x12\x03k\x02\x08\n\x0c\n\x05\x04\0\x02\x11\
    \x01\x12\x03k\t\x16\n\x0c\n\x05\x04\0\x02\x11\x03\x12\x03k\x19\x1b\n\xaf\
    \x02\n\x04\x04\0\x02\x12\x12\x03s\x02\x19\x1a\xa1\x02\x20X-Request-Id\
    \x20request\x20header\n\n\x20This\x20header\x20is\x20used\x20by\x20Envoy\
    \x20to\x20uniquely\x20identify\x20a\x20request.\n\x20It\x20will\x20be\
    \x20generated\x20for\x20all\x20external\x20requests\x20and\x20internal\
    \x20requests\x20that\n\x20do\x20not\x20already\x20have\x20a\x20request\
    \x20ID.\x20\x20So\x20this\x20field\x20can\x20be\x20guaranteed\x20to\x20e\
    xist\n\x20and\x20be\x20unique\x20for\x20request\x20tracing\x20purposes.\
    \n\n\r\n\x05\x04\0\x02\x12\x04\x12\x04s\x02k\x1c\n\x0c\n\x05\x04\0\x02\
    \x12\x05\x12\x03s\x02\x08\n\x0c\n\x05\x04\0\x02\x12\x01\x12\x03s\t\x13\n\
    \x0c\n\x05\x04\0\x02\x12\x03\x12\x03s\x16\x18\nI\n\x04\x04\0\x02\x13\x12\
    \x03v\x02\x18\x1a<\x20HTTP2\x20:authority\x20header\x20value\x20or\x20HT\
    TP1.1\x20Host\x20header\x20value\n\n\r\n\x05\x04\0\x02\x13\x04\x12\x04v\
    \x02s\x19\n\x0c\n\x05\x04\0\x02\x13\x05\x12\x03v\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x13\x01\x12\x03v\t\x12\n\x0c\n\x05\x04\0\x02\x13\x03\x12\x03v\x15\
    \x17\n\x9c\x01\n\x04\x04\0\x02\x14\x12\x03|\x022\x1a\x8e\x01\x20Duration\
    \x20(milliseconds)\n\n\x20The\x20total\x20duration\x20it\x20took\x20to\
    \x20service\x20this\x20request\x20from\x20the\x20StartTime\x20until\n\
    \x20the\x20response\x20was\x20written\x20to\x20the\x20user.\n\n\r\n\x05\
    \x04\0\x02\x14\x04\x12\x04|\x02v\x18\n\x0c\n\x05\x04\0\x02\x14\x06\x12\
    \x03|\x02\x1a\n\x0c\n\x05\x04\0\x02\x14\x01\x12\x03|\x1b,\n\x0c\n\x05\
    \x04\0\x02\x14\x03\x12\x03|/1\n\xb1\x01\n\x04\x04\0\x02\x15\x12\x04\x82\
    \x01\x02:\x1a\xa2\x01\x20Upstream\x20Service\x20Time\x20Duration\n\n\x20\
    From\x20the\x20X-Envoy-Upstream-Service-Time\x20response\x20header.\x20T\
    his\x20is\x20the\x20amount\x20it\x20took\n\x20the\x20upstream\x20server\
    \x20to\x20service\x20the\x20request.\n\n\x0e\n\x05\x04\0\x02\x15\x04\x12\
    \x05\x82\x01\x02|2\n\r\n\x05\x04\0\x02\x15\x06\x12\x04\x82\x01\x02\x1a\n\
    \r\n\x05\x04\0\x02\x15\x01\x12\x04\x82\x01\x1b4\n\r\n\x05\x04\0\x02\x15\
    \x03\x12\x04\x82\x0179\nD\n\x04\x04\0\x02\x16\x12\x04\x86\x01\x02\x1c\
    \x1a6\x20Original\x20Path\x20from\x20the\x20X-Envoy-Original-Path\x20hea\
    der.\n\n\x0f\n\x05\x04\0\x02\x16\x04\x12\x06\x86\x01\x02\x82\x01:\n\r\n\
    \x05\x04\0\x02\x16\x05\x12\x04\x86\x01\x02\x08\n\r\n\x05\x04\0\x02\x16\
    \x01\x12\x04\x86\x01\t\x16\n\r\n\x05\x04\0\x02\x16\x03\x12\x04\x86\x01\
    \x19\x1b\n\xbb\x03\n\x04\x04\0\x02\x17\x12\x04\x90\x01\x02'\x1a\xac\x03\
    \x20All\x20metadata\x20encountered\x20during\x20request\x20processing,\
    \x20including\x20endpoint\n\x20selection.\n\n\x20This\x20can\x20be\x20us\
    ed\x20to\x20associate\x20IDs\x20attached\x20to\x20the\x20various\x20conf\
    igurations\n\x20used\x20to\x20process\x20this\x20request\x20with\x20the\
    \x20access\x20log\x20entry.\x20\x20For\x20example,\x20a\n\x20route\x20cr\
    eated\x20from\x20a\x20higher\x20level\x20forwarding\x20rule\x20with\x20s\
    ome\x20ID\x20can\x20place\n\x20that\x20ID\x20in\x20this\x20field\x20and\
    \x20cross\x20reference\x20later.\x20It\x20can\x20also\x20be\x20used\x20t\
    o\n\x20determine\x20if\x20a\x20canary\x20endpoint\x20was\x20used\x20or\
    \x20not.\n\n\x0f\n\x05\x04\0\x02\x17\x04\x12\x06\x90\x01\x02\x86\x01\x1c\
    \n\r\n\x05\x04\0\x02\x17\x06\x12\x04\x90\x01\x02\x18\n\r\n\x05\x04\0\x02\
    \x17\x01\x12\x04\x90\x01\x19!\n\r\n\x05\x04\0\x02\x17\x03\x12\x04\x90\
    \x01$&\nS\n\x04\x04\0\x02\x18\x12\x04\x93\x01\x02,\x1aE\x20Headers\x20co\
    nfigured\x20for\x20logging\x20but\x20not\x20covered\x20by\x20a\x20specif\
    ic\x20field.\n\n\r\n\x05\x04\0\x02\x18\x04\x12\x04\x93\x01\x02\n\n\r\n\
    \x05\x04\0\x02\x18\x06\x12\x04\x93\x01\x0b\x16\n\r\n\x05\x04\0\x02\x18\
    \x01\x12\x04\x93\x01\x17&\n\r\n\x05\x04\0\x02\x18\x03\x12\x04\x93\x01)+\
    \n\x0c\n\x04\x04\0\x02\x19\x12\x04\x94\x01\x02-\n\r\n\x05\x04\0\x02\x19\
    \x04\x12\x04\x94\x01\x02\n\n\r\n\x05\x04\0\x02\x19\x06\x12\x04\x94\x01\
    \x0b\x16\n\r\n\x05\x04\0\x02\x19\x01\x12\x04\x94\x01\x17'\n\r\n\x05\x04\
    \0\x02\x19\x03\x12\x04\x94\x01*,\n,\n\x04\x04\0\x02\x1a\x12\x04\x97\x01\
    \x02\x1f\x1a\x1e\x20SNI\x20hostname\x20from\x20handshake.\n\n\x0f\n\x05\
    \x04\0\x02\x1a\x04\x12\x06\x97\x01\x02\x94\x01-\n\r\n\x05\x04\0\x02\x1a\
    \x05\x12\x04\x97\x01\x02\x08\n\r\n\x05\x04\0\x02\x1a\x01\x12\x04\x97\x01\
    \t\x19\n\r\n\x05\x04\0\x02\x1a\x03\x12\x04\x97\x01\x1c\x1e\nH\n\x04\x04\
    \0\x04\x02\x12\x06\x9a\x01\x02\xa0\x01\x03\x1a8\x20TLS\x20Version\x20or\
    \x20VERSION_UNSPECIFIED\x20if\x20TLS\x20was\x20not\x20used\n\n\r\n\x05\
    \x04\0\x04\x02\x01\x12\x04\x9a\x01\x07\x11\n\x0e\n\x06\x04\0\x04\x02\x02\
    \0\x12\x04\x9b\x01\x04\x1c\n\x0f\n\x07\x04\0\x04\x02\x02\0\x01\x12\x04\
    \x9b\x01\x04\x17\n\x0f\n\x07\x04\0\x04\x02\x02\0\x02\x12\x04\x9b\x01\x1a\
    \x1b\n\x0e\n\x06\x04\0\x04\x02\x02\x01\x12\x04\x9c\x01\x04\x0e\n\x0f\n\
    \x07\x04\0\x04\x02\x02\x01\x01\x12\x04\x9c\x01\x04\t\n\x0f\n\x07\x04\0\
    \x04\x02\x02\x01\x02\x12\x04\x9c\x01\x0c\r\n\x0e\n\x06\x04\0\x04\x02\x02\
    \x02\x12\x04\x9d\x01\x04\x10\n\x0f\n\x07\x04\0\x04\x02\x02\x02\x01\x12\
    \x04\x9d\x01\x04\x0b\n\x0f\n\x07\x04\0\x04\x02\x02\x02\x02\x12\x04\x9d\
    \x01\x0e\x0f\n\x0e\n\x06\x04\0\x04\x02\x02\x03\x12\x04\x9e\x01\x04\x10\n\
    \x0f\n\x07\x04\0\x04\x02\x02\x03\x01\x12\x04\x9e\x01\x04\x0b\n\x0f\n\x07\
    \x04\0\x04\x02\x02\x03\x02\x12\x04\x9e\x01\x0e\x0f\n\x0e\n\x06\x04\0\x04\
    \x02\x02\x04\x12\x04\x9f\x01\x04\x10\n\x0f\n\x07\x04\0\x04\x02\x02\x04\
    \x01\x12\x04\x9f\x01\x04\x0b\n\x0f\n\x07\x04\0\x04\x02\x02\x04\x02\x12\
    \x04\x9f\x01\x0e\x0f\n\x0c\n\x04\x04\0\x02\x1b\x12\x04\xa1\x01\x02\x1e\n\
    \x0f\n\x05\x04\0\x02\x1b\x04\x12\x06\xa1\x01\x02\xa0\x01\x03\n\r\n\x05\
    \x04\0\x02\x1b\x06\x12\x04\xa1\x01\x02\x0c\n\r\n\x05\x04\0\x02\x1b\x01\
    \x12\x04\xa1\x01\r\x18\n\r\n\x05\x04\0\x02\x1b\x03\x12\x04\xa1\x01\x1b\
    \x1d\n\xe6\x01\n\x04\x04\0\x02\x1c\x12\x04\xa8\x01\x024\x1a\xd7\x01\x20T\
    LS\x20Cipher\x20suite\x20negotiated\x20during\x20TLS\x20handshake.\n\x20\
    The\x20value\x20is\x20four\x20hex\x20digits\x20defined\x20by\x20the\x20I\
    ANA\x20TLS\x20Cipher\x20Suite\x20Registry,\n\x20eg,\x20\"009C\"\x20for\
    \x20TLS_RSA_WITH_AES_128_GCM_SHA256.\n\n\x20Here\x20is\x20is\x20expresse\
    d\x20as\x20an\x20integer.\n\n\x0f\n\x05\x04\0\x02\x1c\x04\x12\x06\xa8\
    \x01\x02\xa1\x01\x1e\n\r\n\x05\x04\0\x02\x1c\x06\x12\x04\xa8\x01\x02\x1d\
    \n\r\n\x05\x04\0\x02\x1c\x01\x12\x04\xa8\x01\x1e.\n\r\n\x05\x04\0\x02\
    \x1c\x03\x12\x04\xa8\x0113b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
