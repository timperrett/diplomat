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
pub struct DiscoveryRequest {
    // message fields
    pub version_info: ::std::string::String,
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub resource_names: ::protobuf::RepeatedField<::std::string::String>,
    pub type_url: ::std::string::String,
    pub response_nonce: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiscoveryRequest {}

impl DiscoveryRequest {
    pub fn new() -> DiscoveryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiscoveryRequest {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryRequest,
        };
        unsafe {
            instance.get(DiscoveryRequest::new)
        }
    }

    // string version_info = 1;

    pub fn clear_version_info(&mut self) {
        self.version_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_version_info(&mut self, v: ::std::string::String) {
        self.version_info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_info(&mut self) -> &mut ::std::string::String {
        &mut self.version_info
    }

    // Take field
    pub fn take_version_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version_info, ::std::string::String::new())
    }

    pub fn get_version_info(&self) -> &str {
        &self.version_info
    }

    fn get_version_info_for_reflect(&self) -> &::std::string::String {
        &self.version_info
    }

    fn mut_version_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version_info
    }

    // .envoy.api.v2.Node node = 2;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: super::base::Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut super::base::Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> super::base::Node {
        self.node.take().unwrap_or_else(|| super::base::Node::new())
    }

    pub fn get_node(&self) -> &super::base::Node {
        self.node.as_ref().unwrap_or_else(|| super::base::Node::default_instance())
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Node> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Node> {
        &mut self.node
    }

    // repeated string resource_names = 3;

    pub fn clear_resource_names(&mut self) {
        self.resource_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.resource_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resource_names
    }

    // Take field
    pub fn take_resource_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.resource_names, ::protobuf::RepeatedField::new())
    }

    pub fn get_resource_names(&self) -> &[::std::string::String] {
        &self.resource_names
    }

    fn get_resource_names_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.resource_names
    }

    fn mut_resource_names_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resource_names
    }

    // string type_url = 4;

    pub fn clear_type_url(&mut self) {
        self.type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_url(&mut self, v: ::std::string::String) {
        self.type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // Take field
    pub fn take_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_url, ::std::string::String::new())
    }

    pub fn get_type_url(&self) -> &str {
        &self.type_url
    }

    fn get_type_url_for_reflect(&self) -> &::std::string::String {
        &self.type_url
    }

    fn mut_type_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // string response_nonce = 5;

    pub fn clear_response_nonce(&mut self) {
        self.response_nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_nonce(&mut self, v: ::std::string::String) {
        self.response_nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.response_nonce
    }

    // Take field
    pub fn take_response_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.response_nonce, ::std::string::String::new())
    }

    pub fn get_response_nonce(&self) -> &str {
        &self.response_nonce
    }

    fn get_response_nonce_for_reflect(&self) -> &::std::string::String {
        &self.response_nonce
    }

    fn mut_response_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.response_nonce
    }
}

impl ::protobuf::Message for DiscoveryRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version_info)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resource_names)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_url)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.response_nonce)?;
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
        if !self.version_info.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version_info);
        }
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.resource_names {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.type_url);
        }
        if !self.response_nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.response_nonce);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version_info.is_empty() {
            os.write_string(1, &self.version_info)?;
        }
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.resource_names {
            os.write_string(3, &v)?;
        };
        if !self.type_url.is_empty() {
            os.write_string(4, &self.type_url)?;
        }
        if !self.response_nonce.is_empty() {
            os.write_string(5, &self.response_nonce)?;
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

impl ::protobuf::MessageStatic for DiscoveryRequest {
    fn new() -> DiscoveryRequest {
        DiscoveryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiscoveryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version_info",
                    DiscoveryRequest::get_version_info_for_reflect,
                    DiscoveryRequest::mut_version_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                    "node",
                    DiscoveryRequest::get_node_for_reflect,
                    DiscoveryRequest::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource_names",
                    DiscoveryRequest::get_resource_names_for_reflect,
                    DiscoveryRequest::mut_resource_names_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_url",
                    DiscoveryRequest::get_type_url_for_reflect,
                    DiscoveryRequest::mut_type_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "response_nonce",
                    DiscoveryRequest::get_response_nonce_for_reflect,
                    DiscoveryRequest::mut_response_nonce_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryRequest>(
                    "DiscoveryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiscoveryRequest {
    fn clear(&mut self) {
        self.clear_version_info();
        self.clear_node();
        self.clear_resource_names();
        self.clear_type_url();
        self.clear_response_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscoveryResponse {
    // message fields
    pub version_info: ::std::string::String,
    pub resources: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    pub canary: bool,
    pub type_url: ::std::string::String,
    pub nonce: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiscoveryResponse {}

impl DiscoveryResponse {
    pub fn new() -> DiscoveryResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiscoveryResponse {
        static mut instance: ::protobuf::lazy::Lazy<DiscoveryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscoveryResponse,
        };
        unsafe {
            instance.get(DiscoveryResponse::new)
        }
    }

    // string version_info = 1;

    pub fn clear_version_info(&mut self) {
        self.version_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_version_info(&mut self, v: ::std::string::String) {
        self.version_info = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_info(&mut self) -> &mut ::std::string::String {
        &mut self.version_info
    }

    // Take field
    pub fn take_version_info(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version_info, ::std::string::String::new())
    }

    pub fn get_version_info(&self) -> &str {
        &self.version_info
    }

    fn get_version_info_for_reflect(&self) -> &::std::string::String {
        &self.version_info
    }

    fn mut_version_info_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version_info
    }

    // repeated .google.protobuf.Any resources = 2;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.resources
    }

    // Take field
    pub fn take_resources(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_resources(&self) -> &[::protobuf::well_known_types::Any] {
        &self.resources
    }

    fn get_resources_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &self.resources
    }

    fn mut_resources_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.resources
    }

    // bool canary = 3;

    pub fn clear_canary(&mut self) {
        self.canary = false;
    }

    // Param is passed by value, moved
    pub fn set_canary(&mut self, v: bool) {
        self.canary = v;
    }

    pub fn get_canary(&self) -> bool {
        self.canary
    }

    fn get_canary_for_reflect(&self) -> &bool {
        &self.canary
    }

    fn mut_canary_for_reflect(&mut self) -> &mut bool {
        &mut self.canary
    }

    // string type_url = 4;

    pub fn clear_type_url(&mut self) {
        self.type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_url(&mut self, v: ::std::string::String) {
        self.type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // Take field
    pub fn take_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_url, ::std::string::String::new())
    }

    pub fn get_type_url(&self) -> &str {
        &self.type_url
    }

    fn get_type_url_for_reflect(&self) -> &::std::string::String {
        &self.type_url
    }

    fn mut_type_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // string nonce = 5;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }
}

impl ::protobuf::Message for DiscoveryResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.resources {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version_info)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resources)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.canary = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_url)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
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
        if !self.version_info.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.version_info);
        }
        for value in &self.resources {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.canary != false {
            my_size += 2;
        }
        if !self.type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.type_url);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.nonce);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.version_info.is_empty() {
            os.write_string(1, &self.version_info)?;
        }
        for v in &self.resources {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.canary != false {
            os.write_bool(3, self.canary)?;
        }
        if !self.type_url.is_empty() {
            os.write_string(4, &self.type_url)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(5, &self.nonce)?;
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

impl ::protobuf::MessageStatic for DiscoveryResponse {
    fn new() -> DiscoveryResponse {
        DiscoveryResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiscoveryResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version_info",
                    DiscoveryResponse::get_version_info_for_reflect,
                    DiscoveryResponse::mut_version_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                    "resources",
                    DiscoveryResponse::get_resources_for_reflect,
                    DiscoveryResponse::mut_resources_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canary",
                    DiscoveryResponse::get_canary_for_reflect,
                    DiscoveryResponse::mut_canary_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_url",
                    DiscoveryResponse::get_type_url_for_reflect,
                    DiscoveryResponse::mut_type_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    DiscoveryResponse::get_nonce_for_reflect,
                    DiscoveryResponse::mut_nonce_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscoveryResponse>(
                    "DiscoveryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiscoveryResponse {
    fn clear(&mut self) {
        self.clear_version_info();
        self.clear_resources();
        self.clear_canary();
        self.clear_type_url();
        self.clear_nonce();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscoveryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscoveryResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13api/discovery.proto\x12\x0cenvoy.api.v2\x1a\x0eapi/base.proto\x1a\
    \x19google/protobuf/any.proto\"\xc6\x01\n\x10DiscoveryRequest\x12!\n\x0c\
    version_info\x18\x01\x20\x01(\tR\x0bversionInfo\x12&\n\x04node\x18\x02\
    \x20\x01(\x0b2\x12.envoy.api.v2.NodeR\x04node\x12%\n\x0eresource_names\
    \x18\x03\x20\x03(\tR\rresourceNames\x12\x19\n\x08type_url\x18\x04\x20\
    \x01(\tR\x07typeUrl\x12%\n\x0eresponse_nonce\x18\x05\x20\x01(\tR\rrespon\
    seNonce\"\xb3\x01\n\x11DiscoveryResponse\x12!\n\x0cversion_info\x18\x01\
    \x20\x01(\tR\x0bversionInfo\x122\n\tresources\x18\x02\x20\x03(\x0b2\x14.\
    google.protobuf.AnyR\tresources\x12\x16\n\x06canary\x18\x03\x20\x01(\x08\
    R\x06canary\x12\x19\n\x08type_url\x18\x04\x20\x01(\tR\x07typeUrl\x12\x14\
    \n\x05nonce\x18\x05\x20\x01(\tR\x05nonce2\x80\x01\n\x1aAggregatedDiscove\
    ryService\x12b\n\x19StreamAggregatedResources\x12\x1e.envoy.api.v2.Disco\
    veryRequest\x1a\x1f.envoy.api.v2.DiscoveryResponse\"\0(\x010\x01J\xe8!\n\
    \x06\x12\x04\0\0Q\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x14\n\t\n\x02\x03\0\x12\x03\x04\x07\x17\n\t\n\x02\x03\
    \x01\x12\x03\x06\x07\"\n\xcf\x03\n\x02\x06\0\x12\x04\x0e\0\x13\x01\x1a\
    \xc2\x03\x20See\x20https://github.com/lyft/envoy-api#apis\x20for\x20a\
    \x20description\x20of\x20the\x20role\x20of\n\x20ADS\x20and\x20how\x20it\
    \x20is\x20intended\x20to\x20be\x20used\x20by\x20a\x20management\x20serve\
    r.\x20ADS\x20requests\n\x20have\x20the\x20same\x20structure\x20as\x20the\
    ir\x20singleton\x20xDS\x20counterparts,\x20but\x20can\n\x20multiplex\x20\
    many\x20resource\x20types\x20on\x20a\x20single\x20stream.\x20The\x20type\
    _url\x20in\x20the\n\x20DiscoveryRequest/DiscoveryResponse\x20provides\
    \x20sufficient\x20information\x20to\x20recover\n\x20the\x20multiplexed\
    \x20singleton\x20APIs\x20at\x20the\x20Envoy\x20instance\x20and\x20manage\
    ment\x20server.\n\n\n\n\x03\x06\0\x01\x12\x03\x0e\x08\"\n(\n\x04\x06\0\
    \x02\0\x12\x04\x10\x02\x12\x03\x1a\x1a\x20This\x20is\x20a\x20gRPC-only\
    \x20API.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\x06\x1f\n\x0c\n\x05\
    \x06\0\x02\0\x05\x12\x03\x10\x20&\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\
    \x10'7\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\x11\x0f\x15\n\x0c\n\x05\x06\0\
    \x02\0\x03\x12\x03\x11\x16'\n|\n\x02\x04\0\x12\x04\x17\02\x01\x1ap\x20A\
    \x20DiscoveryRequest\x20requests\x20a\x20set\x20of\x20versioned\x20resou\
    rces\x20of\x20the\x20same\x20type\x20for\n\x20a\x20given\x20Envoy\x20nod\
    e\x20on\x20some\x20API.\n\n\n\n\x03\x04\0\x01\x12\x03\x17\x08\x18\n\x8c\
    \x04\n\x04\x04\0\x02\0\x12\x03\x1f\x02\x1a\x1a\xfe\x03\x20The\x20version\
    _info\x20provided\x20in\x20the\x20request\x20messages\x20will\x20be\x20t\
    he\x20version_info\n\x20received\x20with\x20the\x20most\x20recent\x20suc\
    cessfully\x20processed\x20response\x20or\x20empty\x20on\n\x20the\x20firs\
    t\x20request.\x20It\x20is\x20expected\x20that\x20no\x20new\x20request\
    \x20is\x20sent\x20after\x20a\n\x20response\x20is\x20received\x20until\
    \x20the\x20Envoy\x20instance\x20is\x20ready\x20to\x20ACK/NACK\x20the\x20\
    new\n\x20configuration.\x20ACK/NACK\x20takes\x20place\x20by\x20returning\
    \x20the\x20new\x20API\x20config\x20version\n\x20as\x20applied\x20or\x20t\
    he\x20previous\x20API\x20config\x20version\x20respectively.\x20Each\x20t\
    ype_url\n\x20(see\x20below)\x20has\x20an\x20independent\x20version\x20as\
    sociated\x20with\x20it.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1f\x02\x17\
    \x1a\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x1f\t\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1f\x18\
    \x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x20\x02\x10\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04\x20\x02\x1f\x1a\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\
    \x20\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x20\x07\x0b\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x20\x0e\x0f\n\xa3\x03\n\x04\x04\0\x02\x02\
    \x12\x03'\x02%\x1a\x95\x03\x20List\x20of\x20resources\x20to\x20subscribe\
    \x20to,\x20e.g.\x20list\x20of\x20cluster\x20names\x20or\x20a\x20route\n\
    \x20configuration\x20name.\x20If\x20this\x20is\x20empty,\x20all\x20resou\
    rces\x20for\x20the\x20API\x20are\n\x20returned.\x20LDS/CDS\x20expect\x20\
    empty\x20resource_names,\x20since\x20this\x20is\x20global\n\x20discovery\
    \x20for\x20the\x20Envoy\x20instance.\x20The\x20LDS\x20and\x20CDS\x20resp\
    onses\x20will\x20then\x20imply\n\x20a\x20number\x20of\x20resources\x20th\
    at\x20need\x20to\x20be\x20fetched\x20via\x20EDS/RDS,\x20which\x20will\
    \x20be\n\x20explicitly\x20enumerated\x20in\x20resource_names.\n\n\x0c\n\
    \x05\x04\0\x02\x02\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03'\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03'\x12\x20\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03'#$\n\xe8\x01\n\x04\x04\0\x02\x03\x12\x03,\x02\
    \x16\x1a\xda\x01\x20Type\x20of\x20the\x20resource\x20that\x20is\x20being\
    \x20requested,\x20e.g.\n\x20\"type.googleapis.com/envoy.api.v2.ClusterLo\
    adAssignment\".\x20This\x20is\x20implicit\n\x20in\x20requests\x20made\
    \x20via\x20singleton\x20xDS\x20APIs\x20such\x20as\x20CDS,\x20LDS,\x20etc\
    .\x20but\x20is\n\x20required\x20for\x20ADS.\n\n\r\n\x05\x04\0\x02\x03\
    \x04\x12\x04,\x02'%\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03,\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x03\x01\x12\x03,\t\x11\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03,\x14\x15\n\xfe\x01\n\x04\x04\0\x02\x04\x12\x031\x02\x1c\x1a\xf0\
    \x01\x20nonce\x20corresponding\x20to\x20DiscoveryResponse\x20being\x20AC\
    K/NACKed.\x20See\x20above\n\x20discussion\x20on\x20version_info\x20and\
    \x20the\x20DiscoveryResponse\x20nonce\x20comment.\x20This\n\x20may\x20be\
    \x20empty\x20if\x20no\x20nonce\x20is\x20available,\x20e.g.\x20at\x20star\
    tup\x20or\x20for\x20non-stream\n\x20xDS\x20implementations.\n\n\r\n\x05\
    \x04\0\x02\x04\x04\x12\x041\x02,\x16\n\x0c\n\x05\x04\0\x02\x04\x05\x12\
    \x031\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x031\t\x17\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x031\x1a\x1b\n\n\n\x02\x04\x01\x12\x044\0Q\x01\n\
    \n\n\x03\x04\x01\x01\x12\x034\x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\x035\
    \x02\x1a\n\r\n\x05\x04\x01\x02\0\x04\x12\x045\x024\x1b\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x035\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x035\t\
    \x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x035\x18\x19\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x036\x02-\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x036\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x036\x0b\x1e\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x036\x1f(\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x036+,\n\xac\
    \x05\n\x04\x04\x01\x02\x02\x12\x03C\x02\x12\x1a\x9e\x05\x20Canary\x20is\
    \x20used\x20to\x20support\x20two\x20Envoy\x20command\x20line\x20flags:\n\
    \x20*\x20--terminate-on-canary-transition-failure.\x20When\x20set,\x20En\
    voy\x20is\x20able\x20to\n\x20\x20\x20terminate\x20if\x20it\x20detects\
    \x20that\x20configuration\x20is\x20stuck\x20at\x20canary.\x20Consider\n\
    \x20\x20\x20this\x20example\x20sequence\x20of\x20updates:\n\x20\x20\x20-\
    \x20Management\x20server\x20applies\x20a\x20canary\x20config\x20successf\
    ully.\n\x20\x20\x20-\x20Management\x20server\x20rolls\x20back\x20to\x20a\
    \x20production\x20config.\n\x20\x20\x20-\x20Envoy\x20rejects\x20the\x20n\
    ew\x20production\x20config.\n\x20\x20\x20Since\x20there\x20is\x20no\x20s\
    ensible\x20way\x20to\x20continue\x20receiving\x20configuration\n\x20\x20\
    \x20updates,\x20Envoy\x20will\x20then\x20terminate\x20and\x20apply\x20pr\
    oduction\x20config\x20from\x20a\n\x20\x20\x20clean\x20slate.\n\x20*\x20-\
    -dry-run-canary.\x20When\x20set,\x20a\x20canary\x20response\x20will\x20n\
    ever\x20be\x20applied,\x20only\n\x20\x20\x20validated\x20via\x20a\x20dry\
    \x20run.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04C\x026-\n\x0c\n\x05\x04\
    \x01\x02\x02\x05\x12\x03C\x02\x06\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03C\x07\r\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03C\x10\x11\n\xce\x01\n\
    \x04\x04\x01\x02\x03\x12\x03G\x02\x16\x1a\xc0\x01\x20Type\x20URL\x20for\
    \x20resources.\x20This\x20must\x20be\x20consistent\x20with\x20the\x20typ\
    e_url\x20in\x20the\n\x20Any\x20messages\x20for\x20resources\x20if\x20res\
    ources\x20is\x20non-empty.\x20This\x20effectively\n\x20identifies\x20the\
    \x20xDS\x20API\x20when\x20muxing\x20over\x20ADS.\n\n\r\n\x05\x04\x01\x02\
    \x03\x04\x12\x04G\x02C\x12\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03G\x02\
    \x08\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03G\t\x11\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03G\x14\x15\n\xc3\x04\n\x04\x04\x01\x02\x04\x12\x03P\
    \x02\x13\x1a\xb5\x04\x20For\x20gRPC\x20based\x20subscriptions,\x20the\
    \x20nonce\x20provides\x20a\x20way\x20to\x20explicitly\x20ack\x20a\n\x20s\
    pecific\x20DiscoveryResponse\x20in\x20a\x20following\x20DiscoveryRequest\
    .\x20Additional\n\x20messages\x20may\x20have\x20been\x20sent\x20by\x20En\
    voy\x20to\x20the\x20management\x20server\x20for\x20the\n\x20previous\x20\
    version\x20on\x20the\x20stream\x20prior\x20to\x20this\x20DiscoveryRespon\
    se,\x20that\x20were\n\x20unprocessed\x20at\x20response\x20send\x20time.\
    \x20The\x20nonce\x20allows\x20the\x20management\x20server\n\x20to\x20ign\
    ore\x20any\x20further\x20DiscoveryRequests\x20for\x20the\x20previous\x20\
    version\x20until\x20a\n\x20DiscoveryRequest\x20bearing\x20the\x20nonce.\
    \x20The\x20nonce\x20is\x20optional\x20and\x20is\x20not\n\x20required\x20\
    for\x20non-stream\x20based\x20xDS\x20implementations.\n\n\r\n\x05\x04\
    \x01\x02\x04\x04\x12\x04P\x02G\x16\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\
    \x03P\x02\x08\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03P\t\x0e\n\x0c\n\x05\
    \x04\x01\x02\x04\x03\x12\x03P\x11\x12b\x06proto3\
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
