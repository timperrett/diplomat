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
pub struct Capability {
    // message fields
    pub health_check_protocol: ::std::vec::Vec<Capability_Protocol>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Capability {}

impl Capability {
    pub fn new() -> Capability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Capability {
        static mut instance: ::protobuf::lazy::Lazy<Capability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Capability,
        };
        unsafe {
            instance.get(Capability::new)
        }
    }

    // repeated .envoy.api.v2.Capability.Protocol health_check_protocol = 1;

    pub fn clear_health_check_protocol(&mut self) {
        self.health_check_protocol.clear();
    }

    // Param is passed by value, moved
    pub fn set_health_check_protocol(&mut self, v: ::std::vec::Vec<Capability_Protocol>) {
        self.health_check_protocol = v;
    }

    // Mutable pointer to the field.
    pub fn mut_health_check_protocol(&mut self) -> &mut ::std::vec::Vec<Capability_Protocol> {
        &mut self.health_check_protocol
    }

    // Take field
    pub fn take_health_check_protocol(&mut self) -> ::std::vec::Vec<Capability_Protocol> {
        ::std::mem::replace(&mut self.health_check_protocol, ::std::vec::Vec::new())
    }

    pub fn get_health_check_protocol(&self) -> &[Capability_Protocol] {
        &self.health_check_protocol
    }

    fn get_health_check_protocol_for_reflect(&self) -> &::std::vec::Vec<Capability_Protocol> {
        &self.health_check_protocol
    }

    fn mut_health_check_protocol_for_reflect(&mut self) -> &mut ::std::vec::Vec<Capability_Protocol> {
        &mut self.health_check_protocol
    }
}

impl ::protobuf::Message for Capability {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.health_check_protocol)?;
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
        for value in &self.health_check_protocol {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.health_check_protocol {
            os.write_enum(1, v.value())?;
        };
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

impl ::protobuf::MessageStatic for Capability {
    fn new() -> Capability {
        Capability::new()
    }

    fn descriptor_static(_: ::std::option::Option<Capability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Capability_Protocol>>(
                    "health_check_protocol",
                    Capability::get_health_check_protocol_for_reflect,
                    Capability::mut_health_check_protocol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Capability>(
                    "Capability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Capability {
    fn clear(&mut self) {
        self.clear_health_check_protocol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Capability {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Capability_Protocol {
    HTTP = 0,
    TCP = 1,
    REDIS = 2,
}

impl ::protobuf::ProtobufEnum for Capability_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Capability_Protocol> {
        match value {
            0 => ::std::option::Option::Some(Capability_Protocol::HTTP),
            1 => ::std::option::Option::Some(Capability_Protocol::TCP),
            2 => ::std::option::Option::Some(Capability_Protocol::REDIS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Capability_Protocol] = &[
            Capability_Protocol::HTTP,
            Capability_Protocol::TCP,
            Capability_Protocol::REDIS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Capability_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Capability_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Capability_Protocol {
}

impl ::std::default::Default for Capability_Protocol {
    fn default() -> Self {
        Capability_Protocol::HTTP
    }
}

impl ::protobuf::reflect::ProtobufValue for Capability_Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckRequest {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub capability: ::protobuf::SingularPtrField<Capability>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckRequest {}

impl HealthCheckRequest {
    pub fn new() -> HealthCheckRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckRequest {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckRequest,
        };
        unsafe {
            instance.get(HealthCheckRequest::new)
        }
    }

    // .envoy.api.v2.Node node = 1;

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

    // .envoy.api.v2.Capability capability = 2;

    pub fn clear_capability(&mut self) {
        self.capability.clear();
    }

    pub fn has_capability(&self) -> bool {
        self.capability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capability(&mut self, v: Capability) {
        self.capability = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_capability(&mut self) -> &mut Capability {
        if self.capability.is_none() {
            self.capability.set_default();
        }
        self.capability.as_mut().unwrap()
    }

    // Take field
    pub fn take_capability(&mut self) -> Capability {
        self.capability.take().unwrap_or_else(|| Capability::new())
    }

    pub fn get_capability(&self) -> &Capability {
        self.capability.as_ref().unwrap_or_else(|| Capability::default_instance())
    }

    fn get_capability_for_reflect(&self) -> &::protobuf::SingularPtrField<Capability> {
        &self.capability
    }

    fn mut_capability_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Capability> {
        &mut self.capability
    }
}

impl ::protobuf::Message for HealthCheckRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.capability {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.capability)?;
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
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.capability.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.capability.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for HealthCheckRequest {
    fn new() -> HealthCheckRequest {
        HealthCheckRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                    "node",
                    HealthCheckRequest::get_node_for_reflect,
                    HealthCheckRequest::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Capability>>(
                    "capability",
                    HealthCheckRequest::get_capability_for_reflect,
                    HealthCheckRequest::mut_capability_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckRequest>(
                    "HealthCheckRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckRequest {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_capability();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EndpointHealth {
    // message fields
    pub endpoint: ::protobuf::SingularPtrField<super::base::Endpoint>,
    pub health_status: super::health_check::HealthStatus,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EndpointHealth {}

impl EndpointHealth {
    pub fn new() -> EndpointHealth {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndpointHealth {
        static mut instance: ::protobuf::lazy::Lazy<EndpointHealth> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndpointHealth,
        };
        unsafe {
            instance.get(EndpointHealth::new)
        }
    }

    // .envoy.api.v2.Endpoint endpoint = 1;

    pub fn clear_endpoint(&mut self) {
        self.endpoint.clear();
    }

    pub fn has_endpoint(&self) -> bool {
        self.endpoint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endpoint(&mut self, v: super::base::Endpoint) {
        self.endpoint = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_endpoint(&mut self) -> &mut super::base::Endpoint {
        if self.endpoint.is_none() {
            self.endpoint.set_default();
        }
        self.endpoint.as_mut().unwrap()
    }

    // Take field
    pub fn take_endpoint(&mut self) -> super::base::Endpoint {
        self.endpoint.take().unwrap_or_else(|| super::base::Endpoint::new())
    }

    pub fn get_endpoint(&self) -> &super::base::Endpoint {
        self.endpoint.as_ref().unwrap_or_else(|| super::base::Endpoint::default_instance())
    }

    fn get_endpoint_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Endpoint> {
        &self.endpoint
    }

    fn mut_endpoint_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Endpoint> {
        &mut self.endpoint
    }

    // .envoy.api.v2.HealthStatus health_status = 2;

    pub fn clear_health_status(&mut self) {
        self.health_status = super::health_check::HealthStatus::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_health_status(&mut self, v: super::health_check::HealthStatus) {
        self.health_status = v;
    }

    pub fn get_health_status(&self) -> super::health_check::HealthStatus {
        self.health_status
    }

    fn get_health_status_for_reflect(&self) -> &super::health_check::HealthStatus {
        &self.health_status
    }

    fn mut_health_status_for_reflect(&mut self) -> &mut super::health_check::HealthStatus {
        &mut self.health_status
    }
}

impl ::protobuf::Message for EndpointHealth {
    fn is_initialized(&self) -> bool {
        for v in &self.endpoint {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.endpoint)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.health_status = tmp;
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
        if let Some(ref v) = self.endpoint.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.health_status != super::health_check::HealthStatus::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(2, self.health_status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.endpoint.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.health_status != super::health_check::HealthStatus::UNKNOWN {
            os.write_enum(2, self.health_status.value())?;
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

impl ::protobuf::MessageStatic for EndpointHealth {
    fn new() -> EndpointHealth {
        EndpointHealth::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndpointHealth>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Endpoint>>(
                    "endpoint",
                    EndpointHealth::get_endpoint_for_reflect,
                    EndpointHealth::mut_endpoint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::health_check::HealthStatus>>(
                    "health_status",
                    EndpointHealth::get_health_status_for_reflect,
                    EndpointHealth::mut_health_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndpointHealth>(
                    "EndpointHealth",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndpointHealth {
    fn clear(&mut self) {
        self.clear_endpoint();
        self.clear_health_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndpointHealth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndpointHealth {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EndpointHealthResponse {
    // message fields
    pub endpoints_health: ::protobuf::RepeatedField<EndpointHealth>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EndpointHealthResponse {}

impl EndpointHealthResponse {
    pub fn new() -> EndpointHealthResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndpointHealthResponse {
        static mut instance: ::protobuf::lazy::Lazy<EndpointHealthResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndpointHealthResponse,
        };
        unsafe {
            instance.get(EndpointHealthResponse::new)
        }
    }

    // repeated .envoy.api.v2.EndpointHealth endpoints_health = 1;

    pub fn clear_endpoints_health(&mut self) {
        self.endpoints_health.clear();
    }

    // Param is passed by value, moved
    pub fn set_endpoints_health(&mut self, v: ::protobuf::RepeatedField<EndpointHealth>) {
        self.endpoints_health = v;
    }

    // Mutable pointer to the field.
    pub fn mut_endpoints_health(&mut self) -> &mut ::protobuf::RepeatedField<EndpointHealth> {
        &mut self.endpoints_health
    }

    // Take field
    pub fn take_endpoints_health(&mut self) -> ::protobuf::RepeatedField<EndpointHealth> {
        ::std::mem::replace(&mut self.endpoints_health, ::protobuf::RepeatedField::new())
    }

    pub fn get_endpoints_health(&self) -> &[EndpointHealth] {
        &self.endpoints_health
    }

    fn get_endpoints_health_for_reflect(&self) -> &::protobuf::RepeatedField<EndpointHealth> {
        &self.endpoints_health
    }

    fn mut_endpoints_health_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EndpointHealth> {
        &mut self.endpoints_health
    }
}

impl ::protobuf::Message for EndpointHealthResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.endpoints_health {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.endpoints_health)?;
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
        for value in &self.endpoints_health {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.endpoints_health {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for EndpointHealthResponse {
    fn new() -> EndpointHealthResponse {
        EndpointHealthResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndpointHealthResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EndpointHealth>>(
                    "endpoints_health",
                    EndpointHealthResponse::get_endpoints_health_for_reflect,
                    EndpointHealthResponse::mut_endpoints_health_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndpointHealthResponse>(
                    "EndpointHealthResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndpointHealthResponse {
    fn clear(&mut self) {
        self.clear_endpoints_health();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndpointHealthResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndpointHealthResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckRequestOrEndpointHealthResponse {
    // message oneof groups
    request_type: ::std::option::Option<HealthCheckRequestOrEndpointHealthResponse_oneof_request_type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckRequestOrEndpointHealthResponse {}

#[derive(Clone,PartialEq)]
pub enum HealthCheckRequestOrEndpointHealthResponse_oneof_request_type {
    health_check_request(HealthCheckRequest),
    endpoint_health_response(EndpointHealthResponse),
}

impl HealthCheckRequestOrEndpointHealthResponse {
    pub fn new() -> HealthCheckRequestOrEndpointHealthResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckRequestOrEndpointHealthResponse {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckRequestOrEndpointHealthResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckRequestOrEndpointHealthResponse,
        };
        unsafe {
            instance.get(HealthCheckRequestOrEndpointHealthResponse::new)
        }
    }

    // .envoy.api.v2.HealthCheckRequest health_check_request = 1;

    pub fn clear_health_check_request(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_health_check_request(&self) -> bool {
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_health_check_request(&mut self, v: HealthCheckRequest) {
        self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(v))
    }

    // Mutable pointer to the field.
    pub fn mut_health_check_request(&mut self) -> &mut HealthCheckRequest {
        if let ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(_)) = self.request_type {
        } else {
            self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(HealthCheckRequest::new()));
        }
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_health_check_request(&mut self) -> HealthCheckRequest {
        if self.has_health_check_request() {
            match self.request_type.take() {
                ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(v)) => v,
                _ => panic!(),
            }
        } else {
            HealthCheckRequest::new()
        }
    }

    pub fn get_health_check_request(&self) -> &HealthCheckRequest {
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(ref v)) => v,
            _ => HealthCheckRequest::default_instance(),
        }
    }

    // .envoy.api.v2.EndpointHealthResponse endpoint_health_response = 2;

    pub fn clear_endpoint_health_response(&mut self) {
        self.request_type = ::std::option::Option::None;
    }

    pub fn has_endpoint_health_response(&self) -> bool {
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_endpoint_health_response(&mut self, v: EndpointHealthResponse) {
        self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_endpoint_health_response(&mut self) -> &mut EndpointHealthResponse {
        if let ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(_)) = self.request_type {
        } else {
            self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(EndpointHealthResponse::new()));
        }
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_endpoint_health_response(&mut self) -> EndpointHealthResponse {
        if self.has_endpoint_health_response() {
            match self.request_type.take() {
                ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(v)) => v,
                _ => panic!(),
            }
        } else {
            EndpointHealthResponse::new()
        }
    }

    pub fn get_endpoint_health_response(&self) -> &EndpointHealthResponse {
        match self.request_type {
            ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(ref v)) => v,
            _ => EndpointHealthResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for HealthCheckRequestOrEndpointHealthResponse {
    fn is_initialized(&self) -> bool {
        if let Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(ref v)) = self.request_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(ref v)) = self.request_type {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request_type = ::std::option::Option::Some(HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.request_type {
            match v {
                &HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.request_type {
            match v {
                &HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::health_check_request(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &HealthCheckRequestOrEndpointHealthResponse_oneof_request_type::endpoint_health_response(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for HealthCheckRequestOrEndpointHealthResponse {
    fn new() -> HealthCheckRequestOrEndpointHealthResponse {
        HealthCheckRequestOrEndpointHealthResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckRequestOrEndpointHealthResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, HealthCheckRequest>(
                    "health_check_request",
                    HealthCheckRequestOrEndpointHealthResponse::has_health_check_request,
                    HealthCheckRequestOrEndpointHealthResponse::get_health_check_request,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, EndpointHealthResponse>(
                    "endpoint_health_response",
                    HealthCheckRequestOrEndpointHealthResponse::has_endpoint_health_response,
                    HealthCheckRequestOrEndpointHealthResponse::get_endpoint_health_response,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckRequestOrEndpointHealthResponse>(
                    "HealthCheckRequestOrEndpointHealthResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckRequestOrEndpointHealthResponse {
    fn clear(&mut self) {
        self.clear_health_check_request();
        self.clear_endpoint_health_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckRequestOrEndpointHealthResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckRequestOrEndpointHealthResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocalityEndpoints {
    // message fields
    pub locality: ::protobuf::SingularPtrField<super::base::Locality>,
    pub endpoints: ::protobuf::RepeatedField<super::base::Endpoint>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocalityEndpoints {}

impl LocalityEndpoints {
    pub fn new() -> LocalityEndpoints {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocalityEndpoints {
        static mut instance: ::protobuf::lazy::Lazy<LocalityEndpoints> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocalityEndpoints,
        };
        unsafe {
            instance.get(LocalityEndpoints::new)
        }
    }

    // .envoy.api.v2.Locality locality = 1;

    pub fn clear_locality(&mut self) {
        self.locality.clear();
    }

    pub fn has_locality(&self) -> bool {
        self.locality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locality(&mut self, v: super::base::Locality) {
        self.locality = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locality(&mut self) -> &mut super::base::Locality {
        if self.locality.is_none() {
            self.locality.set_default();
        }
        self.locality.as_mut().unwrap()
    }

    // Take field
    pub fn take_locality(&mut self) -> super::base::Locality {
        self.locality.take().unwrap_or_else(|| super::base::Locality::new())
    }

    pub fn get_locality(&self) -> &super::base::Locality {
        self.locality.as_ref().unwrap_or_else(|| super::base::Locality::default_instance())
    }

    fn get_locality_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Locality> {
        &self.locality
    }

    fn mut_locality_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Locality> {
        &mut self.locality
    }

    // repeated .envoy.api.v2.Endpoint endpoints = 2;

    pub fn clear_endpoints(&mut self) {
        self.endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_endpoints(&mut self, v: ::protobuf::RepeatedField<super::base::Endpoint>) {
        self.endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<super::base::Endpoint> {
        &mut self.endpoints
    }

    // Take field
    pub fn take_endpoints(&mut self) -> ::protobuf::RepeatedField<super::base::Endpoint> {
        ::std::mem::replace(&mut self.endpoints, ::protobuf::RepeatedField::new())
    }

    pub fn get_endpoints(&self) -> &[super::base::Endpoint] {
        &self.endpoints
    }

    fn get_endpoints_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::Endpoint> {
        &self.endpoints
    }

    fn mut_endpoints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::Endpoint> {
        &mut self.endpoints
    }
}

impl ::protobuf::Message for LocalityEndpoints {
    fn is_initialized(&self) -> bool {
        for v in &self.locality {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.endpoints {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locality)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.endpoints)?;
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
        if let Some(ref v) = self.locality.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.locality.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.endpoints {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for LocalityEndpoints {
    fn new() -> LocalityEndpoints {
        LocalityEndpoints::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocalityEndpoints>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Locality>>(
                    "locality",
                    LocalityEndpoints::get_locality_for_reflect,
                    LocalityEndpoints::mut_locality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Endpoint>>(
                    "endpoints",
                    LocalityEndpoints::get_endpoints_for_reflect,
                    LocalityEndpoints::mut_endpoints_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocalityEndpoints>(
                    "LocalityEndpoints",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocalityEndpoints {
    fn clear(&mut self) {
        self.clear_locality();
        self.clear_endpoints();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocalityEndpoints {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocalityEndpoints {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterHealthCheck {
    // message fields
    pub cluster_name: ::std::string::String,
    pub health_checks: ::protobuf::RepeatedField<super::health_check::HealthCheck>,
    pub endpoints: ::protobuf::RepeatedField<LocalityEndpoints>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterHealthCheck {}

impl ClusterHealthCheck {
    pub fn new() -> ClusterHealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterHealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<ClusterHealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterHealthCheck,
        };
        unsafe {
            instance.get(ClusterHealthCheck::new)
        }
    }

    // string cluster_name = 1;

    pub fn clear_cluster_name(&mut self) {
        self.cluster_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_name(&mut self) -> &mut ::std::string::String {
        &mut self.cluster_name
    }

    // Take field
    pub fn take_cluster_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cluster_name, ::std::string::String::new())
    }

    pub fn get_cluster_name(&self) -> &str {
        &self.cluster_name
    }

    fn get_cluster_name_for_reflect(&self) -> &::std::string::String {
        &self.cluster_name
    }

    fn mut_cluster_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cluster_name
    }

    // repeated .envoy.api.v2.HealthCheck health_checks = 2;

    pub fn clear_health_checks(&mut self) {
        self.health_checks.clear();
    }

    // Param is passed by value, moved
    pub fn set_health_checks(&mut self, v: ::protobuf::RepeatedField<super::health_check::HealthCheck>) {
        self.health_checks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_health_checks(&mut self) -> &mut ::protobuf::RepeatedField<super::health_check::HealthCheck> {
        &mut self.health_checks
    }

    // Take field
    pub fn take_health_checks(&mut self) -> ::protobuf::RepeatedField<super::health_check::HealthCheck> {
        ::std::mem::replace(&mut self.health_checks, ::protobuf::RepeatedField::new())
    }

    pub fn get_health_checks(&self) -> &[super::health_check::HealthCheck] {
        &self.health_checks
    }

    fn get_health_checks_for_reflect(&self) -> &::protobuf::RepeatedField<super::health_check::HealthCheck> {
        &self.health_checks
    }

    fn mut_health_checks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::health_check::HealthCheck> {
        &mut self.health_checks
    }

    // repeated .envoy.api.v2.LocalityEndpoints endpoints = 3;

    pub fn clear_endpoints(&mut self) {
        self.endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_endpoints(&mut self, v: ::protobuf::RepeatedField<LocalityEndpoints>) {
        self.endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<LocalityEndpoints> {
        &mut self.endpoints
    }

    // Take field
    pub fn take_endpoints(&mut self) -> ::protobuf::RepeatedField<LocalityEndpoints> {
        ::std::mem::replace(&mut self.endpoints, ::protobuf::RepeatedField::new())
    }

    pub fn get_endpoints(&self) -> &[LocalityEndpoints] {
        &self.endpoints
    }

    fn get_endpoints_for_reflect(&self) -> &::protobuf::RepeatedField<LocalityEndpoints> {
        &self.endpoints
    }

    fn mut_endpoints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LocalityEndpoints> {
        &mut self.endpoints
    }
}

impl ::protobuf::Message for ClusterHealthCheck {
    fn is_initialized(&self) -> bool {
        for v in &self.health_checks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.endpoints {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cluster_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.health_checks)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.endpoints)?;
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
        if !self.cluster_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cluster_name);
        }
        for value in &self.health_checks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cluster_name.is_empty() {
            os.write_string(1, &self.cluster_name)?;
        }
        for v in &self.health_checks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.endpoints {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ClusterHealthCheck {
    fn new() -> ClusterHealthCheck {
        ClusterHealthCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterHealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster_name",
                    ClusterHealthCheck::get_cluster_name_for_reflect,
                    ClusterHealthCheck::mut_cluster_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::health_check::HealthCheck>>(
                    "health_checks",
                    ClusterHealthCheck::get_health_checks_for_reflect,
                    ClusterHealthCheck::mut_health_checks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocalityEndpoints>>(
                    "endpoints",
                    ClusterHealthCheck::get_endpoints_for_reflect,
                    ClusterHealthCheck::mut_endpoints_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterHealthCheck>(
                    "ClusterHealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterHealthCheck {
    fn clear(&mut self) {
        self.clear_cluster_name();
        self.clear_health_checks();
        self.clear_endpoints();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterHealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterHealthCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheckSpecifier {
    // message fields
    pub health_check: ::protobuf::RepeatedField<ClusterHealthCheck>,
    pub interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheckSpecifier {}

impl HealthCheckSpecifier {
    pub fn new() -> HealthCheckSpecifier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheckSpecifier {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheckSpecifier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheckSpecifier,
        };
        unsafe {
            instance.get(HealthCheckSpecifier::new)
        }
    }

    // repeated .envoy.api.v2.ClusterHealthCheck health_check = 1;

    pub fn clear_health_check(&mut self) {
        self.health_check.clear();
    }

    // Param is passed by value, moved
    pub fn set_health_check(&mut self, v: ::protobuf::RepeatedField<ClusterHealthCheck>) {
        self.health_check = v;
    }

    // Mutable pointer to the field.
    pub fn mut_health_check(&mut self) -> &mut ::protobuf::RepeatedField<ClusterHealthCheck> {
        &mut self.health_check
    }

    // Take field
    pub fn take_health_check(&mut self) -> ::protobuf::RepeatedField<ClusterHealthCheck> {
        ::std::mem::replace(&mut self.health_check, ::protobuf::RepeatedField::new())
    }

    pub fn get_health_check(&self) -> &[ClusterHealthCheck] {
        &self.health_check
    }

    fn get_health_check_for_reflect(&self) -> &::protobuf::RepeatedField<ClusterHealthCheck> {
        &self.health_check
    }

    fn mut_health_check_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClusterHealthCheck> {
        &mut self.health_check
    }

    // .google.protobuf.Duration interval = 2;

    pub fn clear_interval(&mut self) {
        self.interval.clear();
    }

    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.interval.is_none() {
            self.interval.set_default();
        }
        self.interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.interval.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_interval_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.interval
    }

    fn mut_interval_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.interval
    }
}

impl ::protobuf::Message for HealthCheckSpecifier {
    fn is_initialized(&self) -> bool {
        for v in &self.health_check {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.health_check)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval)?;
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
        for value in &self.health_check {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.health_check {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.interval.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for HealthCheckSpecifier {
    fn new() -> HealthCheckSpecifier {
        HealthCheckSpecifier::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheckSpecifier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterHealthCheck>>(
                    "health_check",
                    HealthCheckSpecifier::get_health_check_for_reflect,
                    HealthCheckSpecifier::mut_health_check_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "interval",
                    HealthCheckSpecifier::get_interval_for_reflect,
                    HealthCheckSpecifier::mut_interval_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheckSpecifier>(
                    "HealthCheckSpecifier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheckSpecifier {
    fn clear(&mut self) {
        self.clear_health_check();
        self.clear_interval();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheckSpecifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheckSpecifier {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/hds.proto\x12\x0cenvoy.api.v2\x1a\x0eapi/base.proto\x1a\x16api/h\
    ealth_check.proto\x1a\x1cgoogle/api/annotations.proto\x1a\x1egoogle/prot\
    obuf/duration.proto\"\x8d\x01\n\nCapability\x12U\n\x15health_check_proto\
    col\x18\x01\x20\x03(\x0e2!.envoy.api.v2.Capability.ProtocolR\x13healthCh\
    eckProtocol\"(\n\x08Protocol\x12\x08\n\x04HTTP\x10\0\x12\x07\n\x03TCP\
    \x10\x01\x12\t\n\x05REDIS\x10\x02\"v\n\x12HealthCheckRequest\x12&\n\x04n\
    ode\x18\x01\x20\x01(\x0b2\x12.envoy.api.v2.NodeR\x04node\x128\n\ncapabil\
    ity\x18\x02\x20\x01(\x0b2\x18.envoy.api.v2.CapabilityR\ncapability\"\x85\
    \x01\n\x0eEndpointHealth\x122\n\x08endpoint\x18\x01\x20\x01(\x0b2\x16.en\
    voy.api.v2.EndpointR\x08endpoint\x12?\n\rhealth_status\x18\x02\x20\x01(\
    \x0e2\x1a.envoy.api.v2.HealthStatusR\x0chealthStatus\"a\n\x16EndpointHea\
    lthResponse\x12G\n\x10endpoints_health\x18\x01\x20\x03(\x0b2\x1c.envoy.a\
    pi.v2.EndpointHealthR\x0fendpointsHealth\"\xf4\x01\n*HealthCheckRequestO\
    rEndpointHealthResponse\x12T\n\x14health_check_request\x18\x01\x20\x01(\
    \x0b2\x20.envoy.api.v2.HealthCheckRequestH\0R\x12healthCheckRequest\x12`\
    \n\x18endpoint_health_response\x18\x02\x20\x01(\x0b2$.envoy.api.v2.Endpo\
    intHealthResponseH\0R\x16endpointHealthResponseB\x0e\n\x0crequest_type\"\
    }\n\x11LocalityEndpoints\x122\n\x08locality\x18\x01\x20\x01(\x0b2\x16.en\
    voy.api.v2.LocalityR\x08locality\x124\n\tendpoints\x18\x02\x20\x03(\x0b2\
    \x16.envoy.api.v2.EndpointR\tendpoints\"\xb6\x01\n\x12ClusterHealthCheck\
    \x12!\n\x0ccluster_name\x18\x01\x20\x01(\tR\x0bclusterName\x12>\n\rhealt\
    h_checks\x18\x02\x20\x03(\x0b2\x19.envoy.api.v2.HealthCheckR\x0chealthCh\
    ecks\x12=\n\tendpoints\x18\x03\x20\x03(\x0b2\x1f.envoy.api.v2.LocalityEn\
    dpointsR\tendpoints\"\x92\x01\n\x14HealthCheckSpecifier\x12C\n\x0chealth\
    _check\x18\x01\x20\x03(\x0b2\x20.envoy.api.v2.ClusterHealthCheckR\x0bhea\
    lthCheck\x125\n\x08interval\x18\x02\x20\x01(\x0b2\x19.google.protobuf.Du\
    rationR\x08interval2\xab\x02\n\x16HealthDiscoveryService\x12w\n\x11Strea\
    mHealthCheck\x128.envoy.api.v2.HealthCheckRequestOrEndpointHealthRespons\
    e\x1a\".envoy.api.v2.HealthCheckSpecifier\"\0(\x010\x01\x12\x97\x01\n\
    \x10FetchHealthCheck\x128.envoy.api.v2.HealthCheckRequestOrEndpointHealt\
    hResponse\x1a\".envoy.api.v2.HealthCheckSpecifier\"%\x82\xd3\xe4\x93\x02\
    \x1f\"\x1a/v2/discovery:health_check:\x01*J\xdd&\n\x06\x12\x04\0\0w\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\n\t\
    \n\x02\x03\0\x12\x03\x04\x07\x17\n\t\n\x02\x03\x01\x12\x03\x05\x07\x1f\n\
    \t\n\x02\x03\x02\x12\x03\x07\x07%\n\t\n\x02\x03\x03\x12\x03\x08\x07'\n\
    \xd3\x02\n\x02\x06\0\x12\x04\x0f\0@\x01\x1a\xc6\x02\x20HDS\x20is\x20Heal\
    th\x20Discovery\x20Service.\x20It\x20compliments\x20Envoy\xe2\x80\x99s\
    \x20health\x20checking\n\x20service\x20by\x20designating\x20this\x20Envo\
    y\x20to\x20be\x20a\x20healthchecker\x20for\x20a\x20subset\x20of\x20hosts\
    \n\x20in\x20the\x20cluster.\x20The\x20status\x20of\x20these\x20health\
    \x20checks\x20will\x20be\x20reported\x20to\x20the\n\x20management\x20ser\
    ver,\x20where\x20it\x20can\x20be\x20aggregated\x20etc\x20and\x20redistri\
    buted\x20back\x20to\n\x20Envoy\x20through\x20EDS.\n\n\n\n\x03\x06\0\x01\
    \x12\x03\x0f\x08\x1e\n\xf1\x10\n\x04\x06\0\x02\0\x12\x042\x024\x03\x1a\
    \xe2\x10\x201.\x20Envoy\x20starts\x20up\x20and\x20if\x20its\x20can_healt\
    hcheck\x20option\x20in\x20the\x20static\n\x20\x20\x20\x20bootstrap\x20co\
    nfig\x20is\x20enabled,\x20sends\x20HealthCheckRequest\x20to\x20the\x20ma\
    nagement\n\x20\x20\x20\x20server.\x20It\x20supplies\x20its\x20capabiliti\
    es\x20(which\x20protocol\x20it\x20can\x20health\x20check\n\x20\x20\x20\
    \x20with,\x20what\x20zone\x20it\x20resides\x20in,\x20etc.).\n\x202.\x20I\
    n\x20response\x20to\x20(1),\x20the\x20management\x20server\x20designates\
    \x20this\x20Envoy\x20as\x20a\n\x20\x20\x20\x20healthchecker\x20to\x20hea\
    lth\x20check\x20a\x20subset\x20of\x20all\x20upstream\x20hosts\x20for\x20\
    a\x20given\n\x20\x20\x20\x20cluster\x20(for\x20example\x20upstream\x20Ho\
    st\x201\x20and\x20Host\x202).\x20It\x20streams\n\x20\x20\x20\x20HealthCh\
    eckSpecifier\x20messages\x20with\x20cluster\x20related\x20configuration\
    \x20for\x20all\n\x20\x20\x20\x20clusters\x20this\x20Envoy\x20is\x20desig\
    nated\x20to\x20health\x20check.\x20Subsequent\n\x20\x20\x20\x20HealthChe\
    ckSpecifier\x20message\x20will\x20be\x20sent\x20on\x20changes\x20to:\n\
    \x20\x20\x20\x20a.\x20Endpoints\x20to\x20health\x20checks\n\x20\x20\x20\
    \x20b.\x20Per\x20cluster\x20configuration\x20change\n\x203.\x20Envoy\x20\
    creates\x20a\x20health\x20probe\x20based\x20on\x20the\x20HealthCheck\x20\
    config\x20and\x20sends\n\x20\x20\x20\x20it\x20to\x20endpoint(ip:port)\
    \x20of\x20Host\x201\x20and\x202.\x20Based\x20on\x20the\x20HealthCheck\n\
    \x20\x20\x20\x20configuration\x20Envoy\x20waits\x20upon\x20the\x20arriva\
    l\x20of\x20the\x20probe\x20response\x20and\n\x20\x20\x20\x20looks\x20at\
    \x20the\x20content\x20of\x20the\x20response\x20to\x20decide\x20whether\
    \x20the\x20endpoint\x20is\n\x20\x20\x20\x20healthy\x20or\x20not.\x20If\
    \x20a\x20response\x20hasn\xe2\x80\x99t\x20been\x20received\x20within\x20\
    the\x20timeout\n\x20\x20\x20\x20interval,\x20the\x20endpoint\x20health\
    \x20status\x20is\x20considered\x20TIMEOUT.\n\x204.\x20Envoy\x20reports\
    \x20results\x20back\x20in\x20an\x20EndpointHealthResponse\x20message.\n\
    \x20\x20\x20\x20Envoy\x20streams\x20responses\x20as\x20often\x20as\x20th\
    e\x20interval\x20configured\x20by\x20the\n\x20\x20\x20\x20management\x20\
    server\x20in\x20HealthCheckSpecifier.\n\x205.\x20The\x20management\x20Se\
    rver\x20collects\x20health\x20statuses\x20for\x20all\x20endpoints\x20in\
    \x20the\n\x20\x20\x20\x20cluster\x20(for\x20all\x20clusters)\x20and\x20u\
    ses\x20this\x20information\x20to\x20construct\n\x20\x20\x20\x20EndpointD\
    iscoveryResponse\x20messages.\n\x206.\x20Once\x20Envoy\x20has\x20a\x20li\
    st\x20of\x20upstream\x20endpoints\x20to\x20send\x20traffic\x20to,\x20it\
    \x20load\n\x20\x20\x20\x20balances\x20traffic\x20to\x20them\x20without\
    \x20additional\x20health\x20checking.\x20It\x20may\n\x20\x20\x20\x20use\
    \x20inline\x20healthcheck\x20(i.e.\x20consider\x20endpoint\x20UNHEALTHY\
    \x20if\x20connection\n\x20\x20\x20\x20failed\x20to\x20a\x20particular\
    \x20endpoint\x20to\x20account\x20for\x20health\x20status\x20propagation\
    \n\x20\x20\x20\x20delay\x20between\x20HDS\x20and\x20EDS).\n\x20By\x20def\
    ault,\x20can_healthcheck\x20is\x20true.\x20If\x20can_healthcheck\x20is\
    \x20false,\x20Cluster\n\x20configuration\x20may\x20not\x20contain\x20Hea\
    lthCheck\x20message.\n\x20TODO(htuch):\x20How\x20is\x20can_healthcheck\
    \x20communicated\x20to\x20CDS\x20to\x20ensure\x20the\x20above\n\x20invar\
    iant?\n\x20TODO(htuch):\x20Add\x20@amb67's\x20diagram.\n\n\x0c\n\x05\x06\
    \0\x02\0\x01\x12\x032\x06\x17\n\x0c\n\x05\x06\0\x02\0\x05\x12\x032\x18\
    \x1e\n\x0c\n\x05\x06\0\x02\0\x02\x12\x032\x1fI\n\x0c\n\x05\x06\0\x02\0\
    \x06\x12\x033\x0f\x15\n\x0c\n\x05\x06\0\x02\0\x03\x12\x033\x16*\n\xc1\
    \x01\n\x04\x06\0\x02\x01\x12\x049\x02?\x03\x1a\xb2\x01\x20TODO(htuch):\
    \x20Unlike\x20the\x20gRPC\x20version,\x20there\x20is\x20no\x20stream-bas\
    ed\x20binding\x20of\n\x20request/response.\x20Should\x20we\x20add\x20an\
    \x20identifier\x20to\x20the\x20HealthCheckSpecifier\n\x20to\x20bind\x20w\
    ith\x20the\x20response?\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x039\x06\x16\
    \n\x0c\n\x05\x06\0\x02\x01\x02\x12\x039\x17A\n\x0c\n\x05\x06\0\x02\x01\
    \x03\x12\x03:\x0f#\n\r\n\x05\x06\0\x02\x01\x04\x12\x04;\x04>\x06\n\x10\n\
    \x08\x06\0\x02\x01\x04\xe7\x07\0\x12\x04;\x04>\x06\n\x10\n\t\x06\0\x02\
    \x01\x04\xe7\x07\0\x02\x12\x03;\x0b\x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\
    \x07\0\x02\0\x12\x03;\x0b\x1c\n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\
    \x02\0\x01\x12\x03;\x0c\x1b\n\x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\
    \x12\x04;\x1f>\x05\nt\n\x02\x04\0\x12\x04D\0M\x01\x1ah\x20Defines\x20sup\
    ported\x20protocols\x20etc,\x20so\x20the\x20management\x20server\x20can\
    \x20assign\x20proper\n\x20endpoints\x20to\x20healthcheck.\n\n\n\n\x03\
    \x04\0\x01\x12\x03D\x08\x12\n\x8a\x01\n\x04\x04\0\x04\0\x12\x04G\x02K\
    \x03\x1a|\x20Different\x20Envoy\x20instances\x20may\x20have\x20different\
    \x20capabilities\x20(e.g.\x20Redis)\n\x20and/or\x20have\x20ports\x20enab\
    led\x20for\x20different\x20protocols.\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\
    \x03G\x07\x0f\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03H\x04\r\n\x0e\n\x07\x04\
    \0\x04\0\x02\0\x01\x12\x03H\x04\x08\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\
    \x12\x03H\x0b\x0c\n\r\n\x06\x04\0\x04\0\x02\x01\x12\x03I\x04\x0c\n\x0e\n\
    \x07\x04\0\x04\0\x02\x01\x01\x12\x03I\x04\x07\n\x0e\n\x07\x04\0\x04\0\
    \x02\x01\x02\x12\x03I\n\x0b\n\r\n\x06\x04\0\x04\0\x02\x02\x12\x03J\x04\
    \x0e\n\x0e\n\x07\x04\0\x04\0\x02\x02\x01\x12\x03J\x04\t\n\x0e\n\x07\x04\
    \0\x04\0\x02\x02\x02\x12\x03J\x0c\r\n\x0b\n\x04\x04\0\x02\0\x12\x03L\x02\
    .\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03L\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03L\x14)\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03L,-\n\n\n\x02\x04\x01\x12\x04O\0R\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03O\x08\x1a\n\x0b\n\x04\x04\x01\x02\0\x12\x03P\x02\x10\
    \n\r\n\x05\x04\x01\x02\0\x04\x12\x04P\x02O\x1c\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03P\x02\x06\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03P\x07\x0b\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03P\x0e\x0f\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03Q\x02\x1c\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04Q\x02P\x10\n\
    \x0c\n\x05\x04\x01\x02\x01\x06\x12\x03Q\x02\x0c\n\x0c\n\x05\x04\x01\x02\
    \x01\x01\x12\x03Q\r\x17\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03Q\x1a\x1b\
    \n\n\n\x02\x04\x02\x12\x04T\0W\x01\n\n\n\x03\x04\x02\x01\x12\x03T\x08\
    \x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03U\x02\x18\n\r\n\x05\x04\x02\x02\0\
    \x04\x12\x04U\x02T\x18\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03U\x02\n\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03U\x0b\x13\n\x0c\n\x05\x04\x02\x02\0\
    \x03\x12\x03U\x16\x17\n\x0b\n\x04\x04\x02\x02\x01\x12\x03V\x02!\n\r\n\
    \x05\x04\x02\x02\x01\x04\x12\x04V\x02U\x18\n\x0c\n\x05\x04\x02\x02\x01\
    \x06\x12\x03V\x02\x0e\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03V\x0f\x1c\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03V\x1f\x20\n\n\n\x02\x04\x03\x12\
    \x04Y\0[\x01\n\n\n\x03\x04\x03\x01\x12\x03Y\x08\x1e\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03Z\x02/\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03Z\x02\n\n\x0c\
    \n\x05\x04\x03\x02\0\x06\x12\x03Z\x0b\x19\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03Z\x1a*\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03Z-.\n\n\n\x02\x04\
    \x04\x12\x04]\0b\x01\n\n\n\x03\x04\x04\x01\x12\x03]\x082\n\x0c\n\x04\x04\
    \x04\x08\0\x12\x04^\x02a\x03\n\x0c\n\x05\x04\x04\x08\0\x01\x12\x03^\x08\
    \x14\n\x0b\n\x04\x04\x04\x02\0\x12\x03_\x040\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x03_\x04\x16\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03_\x17+\n\x0c\
    \n\x05\x04\x04\x02\0\x03\x12\x03_./\n\x0b\n\x04\x04\x04\x02\x01\x12\x03`\
    \x048\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03`\x04\x1a\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03`\x1b3\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03`6\
    7\n\n\n\x02\x04\x05\x12\x04d\0g\x01\n\n\n\x03\x04\x05\x01\x12\x03d\x08\
    \x19\n\x0b\n\x04\x04\x05\x02\0\x12\x03e\x02\x18\n\r\n\x05\x04\x05\x02\0\
    \x04\x12\x04e\x02d\x1b\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03e\x02\n\n\
    \x0c\n\x05\x04\x05\x02\0\x01\x12\x03e\x0b\x13\n\x0c\n\x05\x04\x05\x02\0\
    \x03\x12\x03e\x16\x17\n\x0b\n\x04\x04\x05\x02\x01\x12\x03f\x02\"\n\x0c\n\
    \x05\x04\x05\x02\x01\x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x06\
    \x12\x03f\x0b\x13\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03f\x14\x1d\n\x0c\
    \n\x05\x04\x05\x02\x01\x03\x12\x03f\x20!\n\xa7\x02\n\x02\x04\x06\x12\x04\
    m\0q\x01\x1a\x9a\x02\x20The\x20cluster\x20name\x20and\x20locality\x20is\
    \x20provided\x20to\x20Envoy\x20for\x20the\x20endpoints\x20that\x20it\n\
    \x20health\x20checks\x20to\x20support\x20statistics\x20reporting,\x20log\
    ging\x20and\x20debugging\x20by\x20the\n\x20Envoy\x20instance\x20(outside\
    \x20of\x20HDS).\x20For\x20maximum\x20usefulness,\x20it\x20should\x20matc\
    h\x20the\n\x20same\x20cluster\x20structure\x20as\x20that\x20provided\x20\
    by\x20EDS.\n\n\n\n\x03\x04\x06\x01\x12\x03m\x08\x1a\n\x0b\n\x04\x04\x06\
    \x02\0\x12\x03n\x02\x1a\n\r\n\x05\x04\x06\x02\0\x04\x12\x04n\x02m\x1c\n\
    \x0c\n\x05\x04\x06\x02\0\x05\x12\x03n\x02\x08\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03n\t\x15\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03n\x18\x19\n\x0b\
    \n\x04\x04\x06\x02\x01\x12\x03o\x02)\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\
    \x03o\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03o\x0b\x16\n\x0c\n\x05\
    \x04\x06\x02\x01\x01\x12\x03o\x17$\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\
    \x03o'(\n\x0b\n\x04\x04\x06\x02\x02\x12\x03p\x02+\n\x0c\n\x05\x04\x06\
    \x02\x02\x04\x12\x03p\x02\n\n\x0c\n\x05\x04\x06\x02\x02\x06\x12\x03p\x0b\
    \x1c\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03p\x1d&\n\x0c\n\x05\x04\x06\
    \x02\x02\x03\x12\x03p)*\n\n\n\x02\x04\x07\x12\x04s\0w\x01\n\n\n\x03\x04\
    \x07\x01\x12\x03s\x08\x1c\n\x0b\n\x04\x04\x07\x02\0\x12\x03t\x02/\n\x0c\
    \n\x05\x04\x07\x02\0\x04\x12\x03t\x02\n\n\x0c\n\x05\x04\x07\x02\0\x06\
    \x12\x03t\x0b\x1d\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03t\x1e*\n\x0c\n\
    \x05\x04\x07\x02\0\x03\x12\x03t-.\n'\n\x04\x04\x07\x02\x01\x12\x03v\x02(\
    \x1a\x1a\x20The\x20default\x20is\x201\x20second.\n\n\r\n\x05\x04\x07\x02\
    \x01\x04\x12\x04v\x02t/\n\x0c\n\x05\x04\x07\x02\x01\x06\x12\x03v\x02\x1a\
    \n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03v\x1b#\n\x0c\n\x05\x04\x07\x02\
    \x01\x03\x12\x03v&'b\x06proto3\
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
