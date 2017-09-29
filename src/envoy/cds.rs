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
pub struct UpstreamBindConfig {
    // message fields
    pub source_address: ::protobuf::SingularPtrField<super::address::Address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpstreamBindConfig {}

impl UpstreamBindConfig {
    pub fn new() -> UpstreamBindConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpstreamBindConfig {
        static mut instance: ::protobuf::lazy::Lazy<UpstreamBindConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpstreamBindConfig,
        };
        unsafe {
            instance.get(UpstreamBindConfig::new)
        }
    }

    // .envoy.api.v2.Address source_address = 1;

    pub fn clear_source_address(&mut self) {
        self.source_address.clear();
    }

    pub fn has_source_address(&self) -> bool {
        self.source_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_address(&mut self, v: super::address::Address) {
        self.source_address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_address(&mut self) -> &mut super::address::Address {
        if self.source_address.is_none() {
            self.source_address.set_default();
        }
        self.source_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_address(&mut self) -> super::address::Address {
        self.source_address.take().unwrap_or_else(|| super::address::Address::new())
    }

    pub fn get_source_address(&self) -> &super::address::Address {
        self.source_address.as_ref().unwrap_or_else(|| super::address::Address::default_instance())
    }

    fn get_source_address_for_reflect(&self) -> &::protobuf::SingularPtrField<super::address::Address> {
        &self.source_address
    }

    fn mut_source_address_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::address::Address> {
        &mut self.source_address
    }
}

impl ::protobuf::Message for UpstreamBindConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.source_address {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source_address)?;
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
        if let Some(ref v) = self.source_address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.source_address.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for UpstreamBindConfig {
    fn new() -> UpstreamBindConfig {
        UpstreamBindConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpstreamBindConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "source_address",
                    UpstreamBindConfig::get_source_address_for_reflect,
                    UpstreamBindConfig::mut_source_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpstreamBindConfig>(
                    "UpstreamBindConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpstreamBindConfig {
    fn clear(&mut self) {
        self.clear_source_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpstreamBindConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpstreamBindConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CircuitBreakers {
    // message fields
    pub thresholds: ::protobuf::RepeatedField<CircuitBreakers_Thresholds>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CircuitBreakers {}

impl CircuitBreakers {
    pub fn new() -> CircuitBreakers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CircuitBreakers {
        static mut instance: ::protobuf::lazy::Lazy<CircuitBreakers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CircuitBreakers,
        };
        unsafe {
            instance.get(CircuitBreakers::new)
        }
    }

    // repeated .envoy.api.v2.CircuitBreakers.Thresholds thresholds = 1;

    pub fn clear_thresholds(&mut self) {
        self.thresholds.clear();
    }

    // Param is passed by value, moved
    pub fn set_thresholds(&mut self, v: ::protobuf::RepeatedField<CircuitBreakers_Thresholds>) {
        self.thresholds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_thresholds(&mut self) -> &mut ::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        &mut self.thresholds
    }

    // Take field
    pub fn take_thresholds(&mut self) -> ::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        ::std::mem::replace(&mut self.thresholds, ::protobuf::RepeatedField::new())
    }

    pub fn get_thresholds(&self) -> &[CircuitBreakers_Thresholds] {
        &self.thresholds
    }

    fn get_thresholds_for_reflect(&self) -> &::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        &self.thresholds
    }

    fn mut_thresholds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        &mut self.thresholds
    }
}

impl ::protobuf::Message for CircuitBreakers {
    fn is_initialized(&self) -> bool {
        for v in &self.thresholds {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.thresholds)?;
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
        for value in &self.thresholds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.thresholds {
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

impl ::protobuf::MessageStatic for CircuitBreakers {
    fn new() -> CircuitBreakers {
        CircuitBreakers::new()
    }

    fn descriptor_static(_: ::std::option::Option<CircuitBreakers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CircuitBreakers_Thresholds>>(
                    "thresholds",
                    CircuitBreakers::get_thresholds_for_reflect,
                    CircuitBreakers::mut_thresholds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CircuitBreakers>(
                    "CircuitBreakers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CircuitBreakers {
    fn clear(&mut self) {
        self.clear_thresholds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CircuitBreakers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CircuitBreakers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CircuitBreakers_Thresholds {
    // message fields
    pub priority: super::base::RoutingPriority,
    pub max_connections: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_pending_requests: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_requests: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_retries: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CircuitBreakers_Thresholds {}

impl CircuitBreakers_Thresholds {
    pub fn new() -> CircuitBreakers_Thresholds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CircuitBreakers_Thresholds {
        static mut instance: ::protobuf::lazy::Lazy<CircuitBreakers_Thresholds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CircuitBreakers_Thresholds,
        };
        unsafe {
            instance.get(CircuitBreakers_Thresholds::new)
        }
    }

    // .envoy.api.v2.RoutingPriority priority = 1;

    pub fn clear_priority(&mut self) {
        self.priority = super::base::RoutingPriority::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: super::base::RoutingPriority) {
        self.priority = v;
    }

    pub fn get_priority(&self) -> super::base::RoutingPriority {
        self.priority
    }

    fn get_priority_for_reflect(&self) -> &super::base::RoutingPriority {
        &self.priority
    }

    fn mut_priority_for_reflect(&mut self) -> &mut super::base::RoutingPriority {
        &mut self.priority
    }

    // .google.protobuf.UInt32Value max_connections = 2;

    pub fn clear_max_connections(&mut self) {
        self.max_connections.clear();
    }

    pub fn has_max_connections(&self) -> bool {
        self.max_connections.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_connections(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_connections = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_connections(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_connections.is_none() {
            self.max_connections.set_default();
        }
        self.max_connections.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_connections(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_connections.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_connections(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_connections.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_connections_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_connections
    }

    fn mut_max_connections_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_connections
    }

    // .google.protobuf.UInt32Value max_pending_requests = 3;

    pub fn clear_max_pending_requests(&mut self) {
        self.max_pending_requests.clear();
    }

    pub fn has_max_pending_requests(&self) -> bool {
        self.max_pending_requests.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_pending_requests(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_pending_requests = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_pending_requests(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_pending_requests.is_none() {
            self.max_pending_requests.set_default();
        }
        self.max_pending_requests.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_pending_requests(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_pending_requests.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_pending_requests(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_pending_requests.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_pending_requests_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_pending_requests
    }

    fn mut_max_pending_requests_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_pending_requests
    }

    // .google.protobuf.UInt32Value max_requests = 4;

    pub fn clear_max_requests(&mut self) {
        self.max_requests.clear();
    }

    pub fn has_max_requests(&self) -> bool {
        self.max_requests.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_requests(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_requests = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_requests(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_requests.is_none() {
            self.max_requests.set_default();
        }
        self.max_requests.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_requests(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_requests.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_requests(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_requests.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_requests_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_requests
    }

    fn mut_max_requests_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_requests
    }

    // .google.protobuf.UInt32Value max_retries = 5;

    pub fn clear_max_retries(&mut self) {
        self.max_retries.clear();
    }

    pub fn has_max_retries(&self) -> bool {
        self.max_retries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_retries(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_retries = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_retries(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_retries.is_none() {
            self.max_retries.set_default();
        }
        self.max_retries.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_retries(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_retries.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_retries(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_retries.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_retries_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_retries
    }

    fn mut_max_retries_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_retries
    }
}

impl ::protobuf::Message for CircuitBreakers_Thresholds {
    fn is_initialized(&self) -> bool {
        for v in &self.max_connections {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_pending_requests {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_requests {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_retries {
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
                    self.priority = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_connections)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_pending_requests)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_requests)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_retries)?;
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
        if self.priority != super::base::RoutingPriority::DEFAULT {
            my_size += ::protobuf::rt::enum_size(1, self.priority);
        }
        if let Some(ref v) = self.max_connections.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_pending_requests.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_requests.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_retries.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.priority != super::base::RoutingPriority::DEFAULT {
            os.write_enum(1, self.priority.value())?;
        }
        if let Some(ref v) = self.max_connections.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_pending_requests.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_requests.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_retries.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CircuitBreakers_Thresholds {
    fn new() -> CircuitBreakers_Thresholds {
        CircuitBreakers_Thresholds::new()
    }

    fn descriptor_static(_: ::std::option::Option<CircuitBreakers_Thresholds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::base::RoutingPriority>>(
                    "priority",
                    CircuitBreakers_Thresholds::get_priority_for_reflect,
                    CircuitBreakers_Thresholds::mut_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_connections",
                    CircuitBreakers_Thresholds::get_max_connections_for_reflect,
                    CircuitBreakers_Thresholds::mut_max_connections_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_pending_requests",
                    CircuitBreakers_Thresholds::get_max_pending_requests_for_reflect,
                    CircuitBreakers_Thresholds::mut_max_pending_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_requests",
                    CircuitBreakers_Thresholds::get_max_requests_for_reflect,
                    CircuitBreakers_Thresholds::mut_max_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_retries",
                    CircuitBreakers_Thresholds::get_max_retries_for_reflect,
                    CircuitBreakers_Thresholds::mut_max_retries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CircuitBreakers_Thresholds>(
                    "CircuitBreakers_Thresholds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CircuitBreakers_Thresholds {
    fn clear(&mut self) {
        self.clear_priority();
        self.clear_max_connections();
        self.clear_max_pending_requests();
        self.clear_max_requests();
        self.clear_max_retries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CircuitBreakers_Thresholds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CircuitBreakers_Thresholds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster {
    // message fields
    pub name: ::std::string::String,
    pub field_type: Cluster_DiscoveryType,
    pub eds_cluster_config: ::protobuf::SingularPtrField<Cluster_EdsClusterConfig>,
    pub connect_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub per_connection_buffer_limit_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub lb_policy: Cluster_LbPolicy,
    pub hosts: ::protobuf::RepeatedField<super::address::Address>,
    pub health_checks: ::protobuf::RepeatedField<super::health_check::HealthCheck>,
    pub max_requests_per_connection: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub circuit_breakers: ::protobuf::SingularPtrField<CircuitBreakers>,
    pub tls_context: ::protobuf::SingularPtrField<super::tls_context::UpstreamTlsContext>,
    pub dns_refresh_rate: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub dns_lookup_family: Cluster_DnsLookupFamily,
    pub dns_resolvers: ::protobuf::RepeatedField<super::address::Address>,
    pub outlier_detection: ::protobuf::SingularPtrField<Cluster_OutlierDetection>,
    pub cleanup_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub upstream_bind_config: ::protobuf::SingularPtrField<super::address::BindConfig>,
    pub lb_subset_config: ::protobuf::SingularPtrField<Cluster_LbSubsetConfig>,
    // message oneof groups
    protocol_options: ::std::option::Option<Cluster_oneof_protocol_options>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster {}

#[derive(Clone,PartialEq)]
pub enum Cluster_oneof_protocol_options {
    tcp_protocol_options(super::protocol::TcpProtocolOptions),
    http_protocol_options(super::protocol::Http1ProtocolOptions),
    http2_protocol_options(super::protocol::Http2ProtocolOptions),
    grpc_protocol_options(super::protocol::GrpcProtocolOptions),
}

impl Cluster {
    pub fn new() -> Cluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster {
        static mut instance: ::protobuf::lazy::Lazy<Cluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster,
        };
        unsafe {
            instance.get(Cluster::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .envoy.api.v2.Cluster.DiscoveryType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = Cluster_DiscoveryType::STATIC;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Cluster_DiscoveryType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Cluster_DiscoveryType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Cluster_DiscoveryType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Cluster_DiscoveryType {
        &mut self.field_type
    }

    // .envoy.api.v2.Cluster.EdsClusterConfig eds_cluster_config = 3;

    pub fn clear_eds_cluster_config(&mut self) {
        self.eds_cluster_config.clear();
    }

    pub fn has_eds_cluster_config(&self) -> bool {
        self.eds_cluster_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eds_cluster_config(&mut self, v: Cluster_EdsClusterConfig) {
        self.eds_cluster_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_eds_cluster_config(&mut self) -> &mut Cluster_EdsClusterConfig {
        if self.eds_cluster_config.is_none() {
            self.eds_cluster_config.set_default();
        }
        self.eds_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_eds_cluster_config(&mut self) -> Cluster_EdsClusterConfig {
        self.eds_cluster_config.take().unwrap_or_else(|| Cluster_EdsClusterConfig::new())
    }

    pub fn get_eds_cluster_config(&self) -> &Cluster_EdsClusterConfig {
        self.eds_cluster_config.as_ref().unwrap_or_else(|| Cluster_EdsClusterConfig::default_instance())
    }

    fn get_eds_cluster_config_for_reflect(&self) -> &::protobuf::SingularPtrField<Cluster_EdsClusterConfig> {
        &self.eds_cluster_config
    }

    fn mut_eds_cluster_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Cluster_EdsClusterConfig> {
        &mut self.eds_cluster_config
    }

    // .google.protobuf.Duration connect_timeout = 4;

    pub fn clear_connect_timeout(&mut self) {
        self.connect_timeout.clear();
    }

    pub fn has_connect_timeout(&self) -> bool {
        self.connect_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.connect_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.connect_timeout.is_none() {
            self.connect_timeout.set_default();
        }
        self.connect_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_connect_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.connect_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_connect_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.connect_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_connect_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.connect_timeout
    }

    fn mut_connect_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.connect_timeout
    }

    // .google.protobuf.UInt32Value per_connection_buffer_limit_bytes = 5;

    pub fn clear_per_connection_buffer_limit_bytes(&mut self) {
        self.per_connection_buffer_limit_bytes.clear();
    }

    pub fn has_per_connection_buffer_limit_bytes(&self) -> bool {
        self.per_connection_buffer_limit_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_per_connection_buffer_limit_bytes(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.per_connection_buffer_limit_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_per_connection_buffer_limit_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.per_connection_buffer_limit_bytes.is_none() {
            self.per_connection_buffer_limit_bytes.set_default();
        }
        self.per_connection_buffer_limit_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_per_connection_buffer_limit_bytes(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.per_connection_buffer_limit_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_per_connection_buffer_limit_bytes(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.per_connection_buffer_limit_bytes.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_per_connection_buffer_limit_bytes_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.per_connection_buffer_limit_bytes
    }

    fn mut_per_connection_buffer_limit_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.per_connection_buffer_limit_bytes
    }

    // .envoy.api.v2.Cluster.LbPolicy lb_policy = 6;

    pub fn clear_lb_policy(&mut self) {
        self.lb_policy = Cluster_LbPolicy::ROUND_ROBIN;
    }

    // Param is passed by value, moved
    pub fn set_lb_policy(&mut self, v: Cluster_LbPolicy) {
        self.lb_policy = v;
    }

    pub fn get_lb_policy(&self) -> Cluster_LbPolicy {
        self.lb_policy
    }

    fn get_lb_policy_for_reflect(&self) -> &Cluster_LbPolicy {
        &self.lb_policy
    }

    fn mut_lb_policy_for_reflect(&mut self) -> &mut Cluster_LbPolicy {
        &mut self.lb_policy
    }

    // repeated .envoy.api.v2.Address hosts = 7;

    pub fn clear_hosts(&mut self) {
        self.hosts.clear();
    }

    // Param is passed by value, moved
    pub fn set_hosts(&mut self, v: ::protobuf::RepeatedField<super::address::Address>) {
        self.hosts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hosts(&mut self) -> &mut ::protobuf::RepeatedField<super::address::Address> {
        &mut self.hosts
    }

    // Take field
    pub fn take_hosts(&mut self) -> ::protobuf::RepeatedField<super::address::Address> {
        ::std::mem::replace(&mut self.hosts, ::protobuf::RepeatedField::new())
    }

    pub fn get_hosts(&self) -> &[super::address::Address] {
        &self.hosts
    }

    fn get_hosts_for_reflect(&self) -> &::protobuf::RepeatedField<super::address::Address> {
        &self.hosts
    }

    fn mut_hosts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::address::Address> {
        &mut self.hosts
    }

    // repeated .envoy.api.v2.HealthCheck health_checks = 8;

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

    // .google.protobuf.UInt32Value max_requests_per_connection = 9;

    pub fn clear_max_requests_per_connection(&mut self) {
        self.max_requests_per_connection.clear();
    }

    pub fn has_max_requests_per_connection(&self) -> bool {
        self.max_requests_per_connection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_requests_per_connection(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_requests_per_connection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_requests_per_connection(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_requests_per_connection.is_none() {
            self.max_requests_per_connection.set_default();
        }
        self.max_requests_per_connection.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_requests_per_connection(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_requests_per_connection.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_requests_per_connection(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_requests_per_connection.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_requests_per_connection_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_requests_per_connection
    }

    fn mut_max_requests_per_connection_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_requests_per_connection
    }

    // .envoy.api.v2.CircuitBreakers circuit_breakers = 10;

    pub fn clear_circuit_breakers(&mut self) {
        self.circuit_breakers.clear();
    }

    pub fn has_circuit_breakers(&self) -> bool {
        self.circuit_breakers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_circuit_breakers(&mut self, v: CircuitBreakers) {
        self.circuit_breakers = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_circuit_breakers(&mut self) -> &mut CircuitBreakers {
        if self.circuit_breakers.is_none() {
            self.circuit_breakers.set_default();
        }
        self.circuit_breakers.as_mut().unwrap()
    }

    // Take field
    pub fn take_circuit_breakers(&mut self) -> CircuitBreakers {
        self.circuit_breakers.take().unwrap_or_else(|| CircuitBreakers::new())
    }

    pub fn get_circuit_breakers(&self) -> &CircuitBreakers {
        self.circuit_breakers.as_ref().unwrap_or_else(|| CircuitBreakers::default_instance())
    }

    fn get_circuit_breakers_for_reflect(&self) -> &::protobuf::SingularPtrField<CircuitBreakers> {
        &self.circuit_breakers
    }

    fn mut_circuit_breakers_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CircuitBreakers> {
        &mut self.circuit_breakers
    }

    // .envoy.api.v2.UpstreamTlsContext tls_context = 11;

    pub fn clear_tls_context(&mut self) {
        self.tls_context.clear();
    }

    pub fn has_tls_context(&self) -> bool {
        self.tls_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tls_context(&mut self, v: super::tls_context::UpstreamTlsContext) {
        self.tls_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_context(&mut self) -> &mut super::tls_context::UpstreamTlsContext {
        if self.tls_context.is_none() {
            self.tls_context.set_default();
        }
        self.tls_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_tls_context(&mut self) -> super::tls_context::UpstreamTlsContext {
        self.tls_context.take().unwrap_or_else(|| super::tls_context::UpstreamTlsContext::new())
    }

    pub fn get_tls_context(&self) -> &super::tls_context::UpstreamTlsContext {
        self.tls_context.as_ref().unwrap_or_else(|| super::tls_context::UpstreamTlsContext::default_instance())
    }

    fn get_tls_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tls_context::UpstreamTlsContext> {
        &self.tls_context
    }

    fn mut_tls_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tls_context::UpstreamTlsContext> {
        &mut self.tls_context
    }

    // .envoy.api.v2.TcpProtocolOptions tcp_protocol_options = 12;

    pub fn clear_tcp_protocol_options(&mut self) {
        self.protocol_options = ::std::option::Option::None;
    }

    pub fn has_tcp_protocol_options(&self) -> bool {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tcp_protocol_options(&mut self, v: super::protocol::TcpProtocolOptions) {
        self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tcp_protocol_options(&mut self) -> &mut super::protocol::TcpProtocolOptions {
        if let ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(_)) = self.protocol_options {
        } else {
            self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(super::protocol::TcpProtocolOptions::new()));
        }
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tcp_protocol_options(&mut self) -> super::protocol::TcpProtocolOptions {
        if self.has_tcp_protocol_options() {
            match self.protocol_options.take() {
                ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(v)) => v,
                _ => panic!(),
            }
        } else {
            super::protocol::TcpProtocolOptions::new()
        }
    }

    pub fn get_tcp_protocol_options(&self) -> &super::protocol::TcpProtocolOptions {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(ref v)) => v,
            _ => super::protocol::TcpProtocolOptions::default_instance(),
        }
    }

    // .envoy.api.v2.Http1ProtocolOptions http_protocol_options = 13;

    pub fn clear_http_protocol_options(&mut self) {
        self.protocol_options = ::std::option::Option::None;
    }

    pub fn has_http_protocol_options(&self) -> bool {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_http_protocol_options(&mut self, v: super::protocol::Http1ProtocolOptions) {
        self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(v))
    }

    // Mutable pointer to the field.
    pub fn mut_http_protocol_options(&mut self) -> &mut super::protocol::Http1ProtocolOptions {
        if let ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(_)) = self.protocol_options {
        } else {
            self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(super::protocol::Http1ProtocolOptions::new()));
        }
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_http_protocol_options(&mut self) -> super::protocol::Http1ProtocolOptions {
        if self.has_http_protocol_options() {
            match self.protocol_options.take() {
                ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(v)) => v,
                _ => panic!(),
            }
        } else {
            super::protocol::Http1ProtocolOptions::new()
        }
    }

    pub fn get_http_protocol_options(&self) -> &super::protocol::Http1ProtocolOptions {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(ref v)) => v,
            _ => super::protocol::Http1ProtocolOptions::default_instance(),
        }
    }

    // .envoy.api.v2.Http2ProtocolOptions http2_protocol_options = 14;

    pub fn clear_http2_protocol_options(&mut self) {
        self.protocol_options = ::std::option::Option::None;
    }

    pub fn has_http2_protocol_options(&self) -> bool {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_http2_protocol_options(&mut self, v: super::protocol::Http2ProtocolOptions) {
        self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(v))
    }

    // Mutable pointer to the field.
    pub fn mut_http2_protocol_options(&mut self) -> &mut super::protocol::Http2ProtocolOptions {
        if let ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(_)) = self.protocol_options {
        } else {
            self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(super::protocol::Http2ProtocolOptions::new()));
        }
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_http2_protocol_options(&mut self) -> super::protocol::Http2ProtocolOptions {
        if self.has_http2_protocol_options() {
            match self.protocol_options.take() {
                ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(v)) => v,
                _ => panic!(),
            }
        } else {
            super::protocol::Http2ProtocolOptions::new()
        }
    }

    pub fn get_http2_protocol_options(&self) -> &super::protocol::Http2ProtocolOptions {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(ref v)) => v,
            _ => super::protocol::Http2ProtocolOptions::default_instance(),
        }
    }

    // .envoy.api.v2.GrpcProtocolOptions grpc_protocol_options = 15;

    pub fn clear_grpc_protocol_options(&mut self) {
        self.protocol_options = ::std::option::Option::None;
    }

    pub fn has_grpc_protocol_options(&self) -> bool {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_grpc_protocol_options(&mut self, v: super::protocol::GrpcProtocolOptions) {
        self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(v))
    }

    // Mutable pointer to the field.
    pub fn mut_grpc_protocol_options(&mut self) -> &mut super::protocol::GrpcProtocolOptions {
        if let ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(_)) = self.protocol_options {
        } else {
            self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(super::protocol::GrpcProtocolOptions::new()));
        }
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_grpc_protocol_options(&mut self) -> super::protocol::GrpcProtocolOptions {
        if self.has_grpc_protocol_options() {
            match self.protocol_options.take() {
                ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(v)) => v,
                _ => panic!(),
            }
        } else {
            super::protocol::GrpcProtocolOptions::new()
        }
    }

    pub fn get_grpc_protocol_options(&self) -> &super::protocol::GrpcProtocolOptions {
        match self.protocol_options {
            ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(ref v)) => v,
            _ => super::protocol::GrpcProtocolOptions::default_instance(),
        }
    }

    // .google.protobuf.Duration dns_refresh_rate = 16;

    pub fn clear_dns_refresh_rate(&mut self) {
        self.dns_refresh_rate.clear();
    }

    pub fn has_dns_refresh_rate(&self) -> bool {
        self.dns_refresh_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dns_refresh_rate(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.dns_refresh_rate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dns_refresh_rate(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.dns_refresh_rate.is_none() {
            self.dns_refresh_rate.set_default();
        }
        self.dns_refresh_rate.as_mut().unwrap()
    }

    // Take field
    pub fn take_dns_refresh_rate(&mut self) -> ::protobuf::well_known_types::Duration {
        self.dns_refresh_rate.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_dns_refresh_rate(&self) -> &::protobuf::well_known_types::Duration {
        self.dns_refresh_rate.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_dns_refresh_rate_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.dns_refresh_rate
    }

    fn mut_dns_refresh_rate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.dns_refresh_rate
    }

    // .envoy.api.v2.Cluster.DnsLookupFamily dns_lookup_family = 17;

    pub fn clear_dns_lookup_family(&mut self) {
        self.dns_lookup_family = Cluster_DnsLookupFamily::AUTO;
    }

    // Param is passed by value, moved
    pub fn set_dns_lookup_family(&mut self, v: Cluster_DnsLookupFamily) {
        self.dns_lookup_family = v;
    }

    pub fn get_dns_lookup_family(&self) -> Cluster_DnsLookupFamily {
        self.dns_lookup_family
    }

    fn get_dns_lookup_family_for_reflect(&self) -> &Cluster_DnsLookupFamily {
        &self.dns_lookup_family
    }

    fn mut_dns_lookup_family_for_reflect(&mut self) -> &mut Cluster_DnsLookupFamily {
        &mut self.dns_lookup_family
    }

    // repeated .envoy.api.v2.Address dns_resolvers = 18;

    pub fn clear_dns_resolvers(&mut self) {
        self.dns_resolvers.clear();
    }

    // Param is passed by value, moved
    pub fn set_dns_resolvers(&mut self, v: ::protobuf::RepeatedField<super::address::Address>) {
        self.dns_resolvers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dns_resolvers(&mut self) -> &mut ::protobuf::RepeatedField<super::address::Address> {
        &mut self.dns_resolvers
    }

    // Take field
    pub fn take_dns_resolvers(&mut self) -> ::protobuf::RepeatedField<super::address::Address> {
        ::std::mem::replace(&mut self.dns_resolvers, ::protobuf::RepeatedField::new())
    }

    pub fn get_dns_resolvers(&self) -> &[super::address::Address] {
        &self.dns_resolvers
    }

    fn get_dns_resolvers_for_reflect(&self) -> &::protobuf::RepeatedField<super::address::Address> {
        &self.dns_resolvers
    }

    fn mut_dns_resolvers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::address::Address> {
        &mut self.dns_resolvers
    }

    // .envoy.api.v2.Cluster.OutlierDetection outlier_detection = 19;

    pub fn clear_outlier_detection(&mut self) {
        self.outlier_detection.clear();
    }

    pub fn has_outlier_detection(&self) -> bool {
        self.outlier_detection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outlier_detection(&mut self, v: Cluster_OutlierDetection) {
        self.outlier_detection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outlier_detection(&mut self) -> &mut Cluster_OutlierDetection {
        if self.outlier_detection.is_none() {
            self.outlier_detection.set_default();
        }
        self.outlier_detection.as_mut().unwrap()
    }

    // Take field
    pub fn take_outlier_detection(&mut self) -> Cluster_OutlierDetection {
        self.outlier_detection.take().unwrap_or_else(|| Cluster_OutlierDetection::new())
    }

    pub fn get_outlier_detection(&self) -> &Cluster_OutlierDetection {
        self.outlier_detection.as_ref().unwrap_or_else(|| Cluster_OutlierDetection::default_instance())
    }

    fn get_outlier_detection_for_reflect(&self) -> &::protobuf::SingularPtrField<Cluster_OutlierDetection> {
        &self.outlier_detection
    }

    fn mut_outlier_detection_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Cluster_OutlierDetection> {
        &mut self.outlier_detection
    }

    // .google.protobuf.Duration cleanup_interval = 20;

    pub fn clear_cleanup_interval(&mut self) {
        self.cleanup_interval.clear();
    }

    pub fn has_cleanup_interval(&self) -> bool {
        self.cleanup_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cleanup_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.cleanup_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cleanup_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.cleanup_interval.is_none() {
            self.cleanup_interval.set_default();
        }
        self.cleanup_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_cleanup_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.cleanup_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_cleanup_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.cleanup_interval.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_cleanup_interval_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.cleanup_interval
    }

    fn mut_cleanup_interval_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.cleanup_interval
    }

    // .envoy.api.v2.BindConfig upstream_bind_config = 21;

    pub fn clear_upstream_bind_config(&mut self) {
        self.upstream_bind_config.clear();
    }

    pub fn has_upstream_bind_config(&self) -> bool {
        self.upstream_bind_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upstream_bind_config(&mut self, v: super::address::BindConfig) {
        self.upstream_bind_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upstream_bind_config(&mut self) -> &mut super::address::BindConfig {
        if self.upstream_bind_config.is_none() {
            self.upstream_bind_config.set_default();
        }
        self.upstream_bind_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_upstream_bind_config(&mut self) -> super::address::BindConfig {
        self.upstream_bind_config.take().unwrap_or_else(|| super::address::BindConfig::new())
    }

    pub fn get_upstream_bind_config(&self) -> &super::address::BindConfig {
        self.upstream_bind_config.as_ref().unwrap_or_else(|| super::address::BindConfig::default_instance())
    }

    fn get_upstream_bind_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::address::BindConfig> {
        &self.upstream_bind_config
    }

    fn mut_upstream_bind_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::address::BindConfig> {
        &mut self.upstream_bind_config
    }

    // .envoy.api.v2.Cluster.LbSubsetConfig lb_subset_config = 22;

    pub fn clear_lb_subset_config(&mut self) {
        self.lb_subset_config.clear();
    }

    pub fn has_lb_subset_config(&self) -> bool {
        self.lb_subset_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lb_subset_config(&mut self, v: Cluster_LbSubsetConfig) {
        self.lb_subset_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lb_subset_config(&mut self) -> &mut Cluster_LbSubsetConfig {
        if self.lb_subset_config.is_none() {
            self.lb_subset_config.set_default();
        }
        self.lb_subset_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_lb_subset_config(&mut self) -> Cluster_LbSubsetConfig {
        self.lb_subset_config.take().unwrap_or_else(|| Cluster_LbSubsetConfig::new())
    }

    pub fn get_lb_subset_config(&self) -> &Cluster_LbSubsetConfig {
        self.lb_subset_config.as_ref().unwrap_or_else(|| Cluster_LbSubsetConfig::default_instance())
    }

    fn get_lb_subset_config_for_reflect(&self) -> &::protobuf::SingularPtrField<Cluster_LbSubsetConfig> {
        &self.lb_subset_config
    }

    fn mut_lb_subset_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Cluster_LbSubsetConfig> {
        &mut self.lb_subset_config
    }
}

impl ::protobuf::Message for Cluster {
    fn is_initialized(&self) -> bool {
        for v in &self.eds_cluster_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.connect_timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.per_connection_buffer_limit_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.hosts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.health_checks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_requests_per_connection {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.circuit_breakers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tls_context {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(Cluster_oneof_protocol_options::tcp_protocol_options(ref v)) = self.protocol_options {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Cluster_oneof_protocol_options::http_protocol_options(ref v)) = self.protocol_options {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Cluster_oneof_protocol_options::http2_protocol_options(ref v)) = self.protocol_options {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Cluster_oneof_protocol_options::grpc_protocol_options(ref v)) = self.protocol_options {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.dns_refresh_rate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dns_resolvers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.outlier_detection {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cleanup_interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upstream_bind_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lb_subset_config {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.eds_cluster_config)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.connect_timeout)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.per_connection_buffer_limit_bytes)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.lb_policy = tmp;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hosts)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.health_checks)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_requests_per_connection)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.circuit_breakers)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_context)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::tcp_protocol_options(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http_protocol_options(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::http2_protocol_options(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.protocol_options = ::std::option::Option::Some(Cluster_oneof_protocol_options::grpc_protocol_options(is.read_message()?));
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dns_refresh_rate)?;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dns_lookup_family = tmp;
                },
                18 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dns_resolvers)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.outlier_detection)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cleanup_interval)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upstream_bind_config)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lb_subset_config)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.field_type != Cluster_DiscoveryType::STATIC {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if let Some(ref v) = self.eds_cluster_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.connect_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.per_connection_buffer_limit_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.lb_policy != Cluster_LbPolicy::ROUND_ROBIN {
            my_size += ::protobuf::rt::enum_size(6, self.lb_policy);
        }
        for value in &self.hosts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.health_checks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.max_requests_per_connection.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.circuit_breakers.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.tls_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.dns_refresh_rate.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.dns_lookup_family != Cluster_DnsLookupFamily::AUTO {
            my_size += ::protobuf::rt::enum_size(17, self.dns_lookup_family);
        }
        for value in &self.dns_resolvers {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.outlier_detection.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cleanup_interval.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.upstream_bind_config.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lb_subset_config.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.protocol_options {
            match v {
                &Cluster_oneof_protocol_options::tcp_protocol_options(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Cluster_oneof_protocol_options::http_protocol_options(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Cluster_oneof_protocol_options::http2_protocol_options(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Cluster_oneof_protocol_options::grpc_protocol_options(ref v) => {
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
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.field_type != Cluster_DiscoveryType::STATIC {
            os.write_enum(2, self.field_type.value())?;
        }
        if let Some(ref v) = self.eds_cluster_config.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.connect_timeout.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.per_connection_buffer_limit_bytes.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.lb_policy != Cluster_LbPolicy::ROUND_ROBIN {
            os.write_enum(6, self.lb_policy.value())?;
        }
        for v in &self.hosts {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.health_checks {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.max_requests_per_connection.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.circuit_breakers.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.tls_context.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.dns_refresh_rate.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.dns_lookup_family != Cluster_DnsLookupFamily::AUTO {
            os.write_enum(17, self.dns_lookup_family.value())?;
        }
        for v in &self.dns_resolvers {
            os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.outlier_detection.as_ref() {
            os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cleanup_interval.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.upstream_bind_config.as_ref() {
            os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lb_subset_config.as_ref() {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.protocol_options {
            match v {
                &Cluster_oneof_protocol_options::tcp_protocol_options(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Cluster_oneof_protocol_options::http_protocol_options(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Cluster_oneof_protocol_options::http2_protocol_options(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Cluster_oneof_protocol_options::grpc_protocol_options(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Cluster {
    fn new() -> Cluster {
        Cluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Cluster::get_name_for_reflect,
                    Cluster::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cluster_DiscoveryType>>(
                    "type",
                    Cluster::get_field_type_for_reflect,
                    Cluster::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cluster_EdsClusterConfig>>(
                    "eds_cluster_config",
                    Cluster::get_eds_cluster_config_for_reflect,
                    Cluster::mut_eds_cluster_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "connect_timeout",
                    Cluster::get_connect_timeout_for_reflect,
                    Cluster::mut_connect_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "per_connection_buffer_limit_bytes",
                    Cluster::get_per_connection_buffer_limit_bytes_for_reflect,
                    Cluster::mut_per_connection_buffer_limit_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cluster_LbPolicy>>(
                    "lb_policy",
                    Cluster::get_lb_policy_for_reflect,
                    Cluster::mut_lb_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "hosts",
                    Cluster::get_hosts_for_reflect,
                    Cluster::mut_hosts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::health_check::HealthCheck>>(
                    "health_checks",
                    Cluster::get_health_checks_for_reflect,
                    Cluster::mut_health_checks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_requests_per_connection",
                    Cluster::get_max_requests_per_connection_for_reflect,
                    Cluster::mut_max_requests_per_connection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CircuitBreakers>>(
                    "circuit_breakers",
                    Cluster::get_circuit_breakers_for_reflect,
                    Cluster::mut_circuit_breakers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tls_context::UpstreamTlsContext>>(
                    "tls_context",
                    Cluster::get_tls_context_for_reflect,
                    Cluster::mut_tls_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::protocol::TcpProtocolOptions>(
                    "tcp_protocol_options",
                    Cluster::has_tcp_protocol_options,
                    Cluster::get_tcp_protocol_options,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::protocol::Http1ProtocolOptions>(
                    "http_protocol_options",
                    Cluster::has_http_protocol_options,
                    Cluster::get_http_protocol_options,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::protocol::Http2ProtocolOptions>(
                    "http2_protocol_options",
                    Cluster::has_http2_protocol_options,
                    Cluster::get_http2_protocol_options,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::protocol::GrpcProtocolOptions>(
                    "grpc_protocol_options",
                    Cluster::has_grpc_protocol_options,
                    Cluster::get_grpc_protocol_options,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "dns_refresh_rate",
                    Cluster::get_dns_refresh_rate_for_reflect,
                    Cluster::mut_dns_refresh_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cluster_DnsLookupFamily>>(
                    "dns_lookup_family",
                    Cluster::get_dns_lookup_family_for_reflect,
                    Cluster::mut_dns_lookup_family_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "dns_resolvers",
                    Cluster::get_dns_resolvers_for_reflect,
                    Cluster::mut_dns_resolvers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cluster_OutlierDetection>>(
                    "outlier_detection",
                    Cluster::get_outlier_detection_for_reflect,
                    Cluster::mut_outlier_detection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "cleanup_interval",
                    Cluster::get_cleanup_interval_for_reflect,
                    Cluster::mut_cleanup_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::BindConfig>>(
                    "upstream_bind_config",
                    Cluster::get_upstream_bind_config_for_reflect,
                    Cluster::mut_upstream_bind_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cluster_LbSubsetConfig>>(
                    "lb_subset_config",
                    Cluster::get_lb_subset_config_for_reflect,
                    Cluster::mut_lb_subset_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster>(
                    "Cluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_eds_cluster_config();
        self.clear_connect_timeout();
        self.clear_per_connection_buffer_limit_bytes();
        self.clear_lb_policy();
        self.clear_hosts();
        self.clear_health_checks();
        self.clear_max_requests_per_connection();
        self.clear_circuit_breakers();
        self.clear_tls_context();
        self.clear_tcp_protocol_options();
        self.clear_http_protocol_options();
        self.clear_http2_protocol_options();
        self.clear_grpc_protocol_options();
        self.clear_dns_refresh_rate();
        self.clear_dns_lookup_family();
        self.clear_dns_resolvers();
        self.clear_outlier_detection();
        self.clear_cleanup_interval();
        self.clear_upstream_bind_config();
        self.clear_lb_subset_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster_EdsClusterConfig {
    // message fields
    pub eds_config: ::protobuf::SingularPtrField<super::base::ConfigSource>,
    pub service_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster_EdsClusterConfig {}

impl Cluster_EdsClusterConfig {
    pub fn new() -> Cluster_EdsClusterConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster_EdsClusterConfig {
        static mut instance: ::protobuf::lazy::Lazy<Cluster_EdsClusterConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster_EdsClusterConfig,
        };
        unsafe {
            instance.get(Cluster_EdsClusterConfig::new)
        }
    }

    // .envoy.api.v2.ConfigSource eds_config = 1;

    pub fn clear_eds_config(&mut self) {
        self.eds_config.clear();
    }

    pub fn has_eds_config(&self) -> bool {
        self.eds_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eds_config(&mut self, v: super::base::ConfigSource) {
        self.eds_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_eds_config(&mut self) -> &mut super::base::ConfigSource {
        if self.eds_config.is_none() {
            self.eds_config.set_default();
        }
        self.eds_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_eds_config(&mut self) -> super::base::ConfigSource {
        self.eds_config.take().unwrap_or_else(|| super::base::ConfigSource::new())
    }

    pub fn get_eds_config(&self) -> &super::base::ConfigSource {
        self.eds_config.as_ref().unwrap_or_else(|| super::base::ConfigSource::default_instance())
    }

    fn get_eds_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ConfigSource> {
        &self.eds_config
    }

    fn mut_eds_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ConfigSource> {
        &mut self.eds_config
    }

    // string service_name = 2;

    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_name, ::std::string::String::new())
    }

    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }

    fn get_service_name_for_reflect(&self) -> &::std::string::String {
        &self.service_name
    }

    fn mut_service_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }
}

impl ::protobuf::Message for Cluster_EdsClusterConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.eds_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.eds_config)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_name)?;
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
        if let Some(ref v) = self.eds_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.service_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.service_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.eds_config.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.service_name.is_empty() {
            os.write_string(2, &self.service_name)?;
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

impl ::protobuf::MessageStatic for Cluster_EdsClusterConfig {
    fn new() -> Cluster_EdsClusterConfig {
        Cluster_EdsClusterConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster_EdsClusterConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ConfigSource>>(
                    "eds_config",
                    Cluster_EdsClusterConfig::get_eds_config_for_reflect,
                    Cluster_EdsClusterConfig::mut_eds_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_name",
                    Cluster_EdsClusterConfig::get_service_name_for_reflect,
                    Cluster_EdsClusterConfig::mut_service_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster_EdsClusterConfig>(
                    "Cluster_EdsClusterConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster_EdsClusterConfig {
    fn clear(&mut self) {
        self.clear_eds_config();
        self.clear_service_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster_EdsClusterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_EdsClusterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster_OutlierDetection {
    // message fields
    pub consecutive_5xx: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub base_ejection_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub max_ejection_percent: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub enforcing_consecutive_5xx: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub enforcing_success_rate: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub success_rate_minimum_hosts: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub success_rate_request_volume: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub success_rate_stdev_factor: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster_OutlierDetection {}

impl Cluster_OutlierDetection {
    pub fn new() -> Cluster_OutlierDetection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster_OutlierDetection {
        static mut instance: ::protobuf::lazy::Lazy<Cluster_OutlierDetection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster_OutlierDetection,
        };
        unsafe {
            instance.get(Cluster_OutlierDetection::new)
        }
    }

    // .google.protobuf.UInt32Value consecutive_5xx = 1;

    pub fn clear_consecutive_5xx(&mut self) {
        self.consecutive_5xx.clear();
    }

    pub fn has_consecutive_5xx(&self) -> bool {
        self.consecutive_5xx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consecutive_5xx(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.consecutive_5xx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_consecutive_5xx(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.consecutive_5xx.is_none() {
            self.consecutive_5xx.set_default();
        }
        self.consecutive_5xx.as_mut().unwrap()
    }

    // Take field
    pub fn take_consecutive_5xx(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.consecutive_5xx.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_consecutive_5xx(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.consecutive_5xx.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_consecutive_5xx_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.consecutive_5xx
    }

    fn mut_consecutive_5xx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.consecutive_5xx
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

    // .google.protobuf.Duration base_ejection_time = 3;

    pub fn clear_base_ejection_time(&mut self) {
        self.base_ejection_time.clear();
    }

    pub fn has_base_ejection_time(&self) -> bool {
        self.base_ejection_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_ejection_time(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.base_ejection_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base_ejection_time(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.base_ejection_time.is_none() {
            self.base_ejection_time.set_default();
        }
        self.base_ejection_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_base_ejection_time(&mut self) -> ::protobuf::well_known_types::Duration {
        self.base_ejection_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_base_ejection_time(&self) -> &::protobuf::well_known_types::Duration {
        self.base_ejection_time.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_base_ejection_time_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.base_ejection_time
    }

    fn mut_base_ejection_time_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.base_ejection_time
    }

    // .google.protobuf.UInt32Value max_ejection_percent = 4;

    pub fn clear_max_ejection_percent(&mut self) {
        self.max_ejection_percent.clear();
    }

    pub fn has_max_ejection_percent(&self) -> bool {
        self.max_ejection_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_ejection_percent(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_ejection_percent = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_ejection_percent(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_ejection_percent.is_none() {
            self.max_ejection_percent.set_default();
        }
        self.max_ejection_percent.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_ejection_percent(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_ejection_percent.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_ejection_percent(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_ejection_percent.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_ejection_percent_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_ejection_percent
    }

    fn mut_max_ejection_percent_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_ejection_percent
    }

    // .google.protobuf.UInt32Value enforcing_consecutive_5xx = 5;

    pub fn clear_enforcing_consecutive_5xx(&mut self) {
        self.enforcing_consecutive_5xx.clear();
    }

    pub fn has_enforcing_consecutive_5xx(&self) -> bool {
        self.enforcing_consecutive_5xx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enforcing_consecutive_5xx(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.enforcing_consecutive_5xx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_enforcing_consecutive_5xx(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.enforcing_consecutive_5xx.is_none() {
            self.enforcing_consecutive_5xx.set_default();
        }
        self.enforcing_consecutive_5xx.as_mut().unwrap()
    }

    // Take field
    pub fn take_enforcing_consecutive_5xx(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.enforcing_consecutive_5xx.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_enforcing_consecutive_5xx(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.enforcing_consecutive_5xx.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_enforcing_consecutive_5xx_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.enforcing_consecutive_5xx
    }

    fn mut_enforcing_consecutive_5xx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.enforcing_consecutive_5xx
    }

    // .google.protobuf.UInt32Value enforcing_success_rate = 6;

    pub fn clear_enforcing_success_rate(&mut self) {
        self.enforcing_success_rate.clear();
    }

    pub fn has_enforcing_success_rate(&self) -> bool {
        self.enforcing_success_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enforcing_success_rate(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.enforcing_success_rate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_enforcing_success_rate(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.enforcing_success_rate.is_none() {
            self.enforcing_success_rate.set_default();
        }
        self.enforcing_success_rate.as_mut().unwrap()
    }

    // Take field
    pub fn take_enforcing_success_rate(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.enforcing_success_rate.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_enforcing_success_rate(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.enforcing_success_rate.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_enforcing_success_rate_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.enforcing_success_rate
    }

    fn mut_enforcing_success_rate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.enforcing_success_rate
    }

    // .google.protobuf.UInt32Value success_rate_minimum_hosts = 7;

    pub fn clear_success_rate_minimum_hosts(&mut self) {
        self.success_rate_minimum_hosts.clear();
    }

    pub fn has_success_rate_minimum_hosts(&self) -> bool {
        self.success_rate_minimum_hosts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success_rate_minimum_hosts(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.success_rate_minimum_hosts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_success_rate_minimum_hosts(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.success_rate_minimum_hosts.is_none() {
            self.success_rate_minimum_hosts.set_default();
        }
        self.success_rate_minimum_hosts.as_mut().unwrap()
    }

    // Take field
    pub fn take_success_rate_minimum_hosts(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.success_rate_minimum_hosts.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_success_rate_minimum_hosts(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.success_rate_minimum_hosts.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_success_rate_minimum_hosts_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.success_rate_minimum_hosts
    }

    fn mut_success_rate_minimum_hosts_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.success_rate_minimum_hosts
    }

    // .google.protobuf.UInt32Value success_rate_request_volume = 8;

    pub fn clear_success_rate_request_volume(&mut self) {
        self.success_rate_request_volume.clear();
    }

    pub fn has_success_rate_request_volume(&self) -> bool {
        self.success_rate_request_volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success_rate_request_volume(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.success_rate_request_volume = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_success_rate_request_volume(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.success_rate_request_volume.is_none() {
            self.success_rate_request_volume.set_default();
        }
        self.success_rate_request_volume.as_mut().unwrap()
    }

    // Take field
    pub fn take_success_rate_request_volume(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.success_rate_request_volume.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_success_rate_request_volume(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.success_rate_request_volume.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_success_rate_request_volume_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.success_rate_request_volume
    }

    fn mut_success_rate_request_volume_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.success_rate_request_volume
    }

    // .google.protobuf.UInt32Value success_rate_stdev_factor = 9;

    pub fn clear_success_rate_stdev_factor(&mut self) {
        self.success_rate_stdev_factor.clear();
    }

    pub fn has_success_rate_stdev_factor(&self) -> bool {
        self.success_rate_stdev_factor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success_rate_stdev_factor(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.success_rate_stdev_factor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_success_rate_stdev_factor(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.success_rate_stdev_factor.is_none() {
            self.success_rate_stdev_factor.set_default();
        }
        self.success_rate_stdev_factor.as_mut().unwrap()
    }

    // Take field
    pub fn take_success_rate_stdev_factor(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.success_rate_stdev_factor.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_success_rate_stdev_factor(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.success_rate_stdev_factor.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_success_rate_stdev_factor_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.success_rate_stdev_factor
    }

    fn mut_success_rate_stdev_factor_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.success_rate_stdev_factor
    }
}

impl ::protobuf::Message for Cluster_OutlierDetection {
    fn is_initialized(&self) -> bool {
        for v in &self.consecutive_5xx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.base_ejection_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_ejection_percent {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enforcing_consecutive_5xx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enforcing_success_rate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.success_rate_minimum_hosts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.success_rate_request_volume {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.success_rate_stdev_factor {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.consecutive_5xx)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base_ejection_time)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_ejection_percent)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.enforcing_consecutive_5xx)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.enforcing_success_rate)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.success_rate_minimum_hosts)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.success_rate_request_volume)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.success_rate_stdev_factor)?;
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
        if let Some(ref v) = self.consecutive_5xx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.base_ejection_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_ejection_percent.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.enforcing_consecutive_5xx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.enforcing_success_rate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.success_rate_minimum_hosts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.success_rate_request_volume.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.success_rate_stdev_factor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.consecutive_5xx.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.interval.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.base_ejection_time.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_ejection_percent.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.enforcing_consecutive_5xx.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.enforcing_success_rate.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.success_rate_minimum_hosts.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.success_rate_request_volume.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.success_rate_stdev_factor.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Cluster_OutlierDetection {
    fn new() -> Cluster_OutlierDetection {
        Cluster_OutlierDetection::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster_OutlierDetection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "consecutive_5xx",
                    Cluster_OutlierDetection::get_consecutive_5xx_for_reflect,
                    Cluster_OutlierDetection::mut_consecutive_5xx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "interval",
                    Cluster_OutlierDetection::get_interval_for_reflect,
                    Cluster_OutlierDetection::mut_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "base_ejection_time",
                    Cluster_OutlierDetection::get_base_ejection_time_for_reflect,
                    Cluster_OutlierDetection::mut_base_ejection_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_ejection_percent",
                    Cluster_OutlierDetection::get_max_ejection_percent_for_reflect,
                    Cluster_OutlierDetection::mut_max_ejection_percent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "enforcing_consecutive_5xx",
                    Cluster_OutlierDetection::get_enforcing_consecutive_5xx_for_reflect,
                    Cluster_OutlierDetection::mut_enforcing_consecutive_5xx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "enforcing_success_rate",
                    Cluster_OutlierDetection::get_enforcing_success_rate_for_reflect,
                    Cluster_OutlierDetection::mut_enforcing_success_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "success_rate_minimum_hosts",
                    Cluster_OutlierDetection::get_success_rate_minimum_hosts_for_reflect,
                    Cluster_OutlierDetection::mut_success_rate_minimum_hosts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "success_rate_request_volume",
                    Cluster_OutlierDetection::get_success_rate_request_volume_for_reflect,
                    Cluster_OutlierDetection::mut_success_rate_request_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "success_rate_stdev_factor",
                    Cluster_OutlierDetection::get_success_rate_stdev_factor_for_reflect,
                    Cluster_OutlierDetection::mut_success_rate_stdev_factor_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster_OutlierDetection>(
                    "Cluster_OutlierDetection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster_OutlierDetection {
    fn clear(&mut self) {
        self.clear_consecutive_5xx();
        self.clear_interval();
        self.clear_base_ejection_time();
        self.clear_max_ejection_percent();
        self.clear_enforcing_consecutive_5xx();
        self.clear_enforcing_success_rate();
        self.clear_success_rate_minimum_hosts();
        self.clear_success_rate_request_volume();
        self.clear_success_rate_stdev_factor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster_OutlierDetection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_OutlierDetection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster_LbSubsetConfig {
    // message fields
    pub fallback_policy: Cluster_LbSubsetConfig_LbSubsetFallbackPolicy,
    pub default_subset: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    pub subset_keys: ::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster_LbSubsetConfig {}

impl Cluster_LbSubsetConfig {
    pub fn new() -> Cluster_LbSubsetConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster_LbSubsetConfig {
        static mut instance: ::protobuf::lazy::Lazy<Cluster_LbSubsetConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster_LbSubsetConfig,
        };
        unsafe {
            instance.get(Cluster_LbSubsetConfig::new)
        }
    }

    // .envoy.api.v2.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy fallback_policy = 1;

    pub fn clear_fallback_policy(&mut self) {
        self.fallback_policy = Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK;
    }

    // Param is passed by value, moved
    pub fn set_fallback_policy(&mut self, v: Cluster_LbSubsetConfig_LbSubsetFallbackPolicy) {
        self.fallback_policy = v;
    }

    pub fn get_fallback_policy(&self) -> Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
        self.fallback_policy
    }

    fn get_fallback_policy_for_reflect(&self) -> &Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
        &self.fallback_policy
    }

    fn mut_fallback_policy_for_reflect(&mut self) -> &mut Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
        &mut self.fallback_policy
    }

    // .google.protobuf.Struct default_subset = 2;

    pub fn clear_default_subset(&mut self) {
        self.default_subset.clear();
    }

    pub fn has_default_subset(&self) -> bool {
        self.default_subset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_subset(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.default_subset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_subset(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if self.default_subset.is_none() {
            self.default_subset.set_default();
        }
        self.default_subset.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_subset(&mut self) -> ::protobuf::well_known_types::Struct {
        self.default_subset.take().unwrap_or_else(|| ::protobuf::well_known_types::Struct::new())
    }

    pub fn get_default_subset(&self) -> &::protobuf::well_known_types::Struct {
        self.default_subset.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Struct::default_instance())
    }

    fn get_default_subset_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &self.default_subset
    }

    fn mut_default_subset_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &mut self.default_subset
    }

    // repeated .envoy.api.v2.Cluster.LbSubsetConfig.LbSubsetKeys subset_keys = 3;

    pub fn clear_subset_keys(&mut self) {
        self.subset_keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_subset_keys(&mut self, v: ::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys>) {
        self.subset_keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subset_keys(&mut self) -> &mut ::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys> {
        &mut self.subset_keys
    }

    // Take field
    pub fn take_subset_keys(&mut self) -> ::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys> {
        ::std::mem::replace(&mut self.subset_keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_subset_keys(&self) -> &[Cluster_LbSubsetConfig_LbSubsetKeys] {
        &self.subset_keys
    }

    fn get_subset_keys_for_reflect(&self) -> &::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys> {
        &self.subset_keys
    }

    fn mut_subset_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Cluster_LbSubsetConfig_LbSubsetKeys> {
        &mut self.subset_keys
    }
}

impl ::protobuf::Message for Cluster_LbSubsetConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.default_subset {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.subset_keys {
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
                    self.fallback_policy = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default_subset)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subset_keys)?;
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
        if self.fallback_policy != Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK {
            my_size += ::protobuf::rt::enum_size(1, self.fallback_policy);
        }
        if let Some(ref v) = self.default_subset.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.subset_keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.fallback_policy != Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK {
            os.write_enum(1, self.fallback_policy.value())?;
        }
        if let Some(ref v) = self.default_subset.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.subset_keys {
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

impl ::protobuf::MessageStatic for Cluster_LbSubsetConfig {
    fn new() -> Cluster_LbSubsetConfig {
        Cluster_LbSubsetConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster_LbSubsetConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cluster_LbSubsetConfig_LbSubsetFallbackPolicy>>(
                    "fallback_policy",
                    Cluster_LbSubsetConfig::get_fallback_policy_for_reflect,
                    Cluster_LbSubsetConfig::mut_fallback_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "default_subset",
                    Cluster_LbSubsetConfig::get_default_subset_for_reflect,
                    Cluster_LbSubsetConfig::mut_default_subset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cluster_LbSubsetConfig_LbSubsetKeys>>(
                    "subset_keys",
                    Cluster_LbSubsetConfig::get_subset_keys_for_reflect,
                    Cluster_LbSubsetConfig::mut_subset_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster_LbSubsetConfig>(
                    "Cluster_LbSubsetConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster_LbSubsetConfig {
    fn clear(&mut self) {
        self.clear_fallback_policy();
        self.clear_default_subset();
        self.clear_subset_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster_LbSubsetConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_LbSubsetConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster_LbSubsetConfig_LbSubsetKeys {
    // message fields
    pub keys: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster_LbSubsetConfig_LbSubsetKeys {}

impl Cluster_LbSubsetConfig_LbSubsetKeys {
    pub fn new() -> Cluster_LbSubsetConfig_LbSubsetKeys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster_LbSubsetConfig_LbSubsetKeys {
        static mut instance: ::protobuf::lazy::Lazy<Cluster_LbSubsetConfig_LbSubsetKeys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster_LbSubsetConfig_LbSubsetKeys,
        };
        unsafe {
            instance.get(Cluster_LbSubsetConfig_LbSubsetKeys::new)
        }
    }

    // repeated string keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::string::String] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }
}

impl ::protobuf::Message for Cluster_LbSubsetConfig_LbSubsetKeys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keys)?;
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
        for value in &self.keys {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for Cluster_LbSubsetConfig_LbSubsetKeys {
    fn new() -> Cluster_LbSubsetConfig_LbSubsetKeys {
        Cluster_LbSubsetConfig_LbSubsetKeys::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster_LbSubsetConfig_LbSubsetKeys>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keys",
                    Cluster_LbSubsetConfig_LbSubsetKeys::get_keys_for_reflect,
                    Cluster_LbSubsetConfig_LbSubsetKeys::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster_LbSubsetConfig_LbSubsetKeys>(
                    "Cluster_LbSubsetConfig_LbSubsetKeys",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster_LbSubsetConfig_LbSubsetKeys {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster_LbSubsetConfig_LbSubsetKeys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_LbSubsetConfig_LbSubsetKeys {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
    NO_FALLBACK = 0,
    ANY_ENDPOINT = 1,
    DEFAULT_SUBSET = 2,
}

impl ::protobuf::ProtobufEnum for Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cluster_LbSubsetConfig_LbSubsetFallbackPolicy> {
        match value {
            0 => ::std::option::Option::Some(Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK),
            1 => ::std::option::Option::Some(Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::ANY_ENDPOINT),
            2 => ::std::option::Option::Some(Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::DEFAULT_SUBSET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cluster_LbSubsetConfig_LbSubsetFallbackPolicy] = &[
            Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK,
            Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::ANY_ENDPOINT,
            Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::DEFAULT_SUBSET,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cluster_LbSubsetConfig_LbSubsetFallbackPolicy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cluster_LbSubsetConfig_LbSubsetFallbackPolicy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
}

impl ::std::default::Default for Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
    fn default() -> Self {
        Cluster_LbSubsetConfig_LbSubsetFallbackPolicy::NO_FALLBACK
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_LbSubsetConfig_LbSubsetFallbackPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cluster_DiscoveryType {
    STATIC = 0,
    STRICT_DNS = 1,
    LOGICAL_DNS = 2,
    EDS = 3,
    ORIGINAL_DST = 4,
}

impl ::protobuf::ProtobufEnum for Cluster_DiscoveryType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cluster_DiscoveryType> {
        match value {
            0 => ::std::option::Option::Some(Cluster_DiscoveryType::STATIC),
            1 => ::std::option::Option::Some(Cluster_DiscoveryType::STRICT_DNS),
            2 => ::std::option::Option::Some(Cluster_DiscoveryType::LOGICAL_DNS),
            3 => ::std::option::Option::Some(Cluster_DiscoveryType::EDS),
            4 => ::std::option::Option::Some(Cluster_DiscoveryType::ORIGINAL_DST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cluster_DiscoveryType] = &[
            Cluster_DiscoveryType::STATIC,
            Cluster_DiscoveryType::STRICT_DNS,
            Cluster_DiscoveryType::LOGICAL_DNS,
            Cluster_DiscoveryType::EDS,
            Cluster_DiscoveryType::ORIGINAL_DST,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cluster_DiscoveryType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cluster_DiscoveryType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cluster_DiscoveryType {
}

impl ::std::default::Default for Cluster_DiscoveryType {
    fn default() -> Self {
        Cluster_DiscoveryType::STATIC
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_DiscoveryType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cluster_LbPolicy {
    ROUND_ROBIN = 0,
    LEAST_REQUEST = 1,
    RING_HASH = 2,
    RANDOM = 3,
    ORIGINAL_DST_LB = 4,
}

impl ::protobuf::ProtobufEnum for Cluster_LbPolicy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cluster_LbPolicy> {
        match value {
            0 => ::std::option::Option::Some(Cluster_LbPolicy::ROUND_ROBIN),
            1 => ::std::option::Option::Some(Cluster_LbPolicy::LEAST_REQUEST),
            2 => ::std::option::Option::Some(Cluster_LbPolicy::RING_HASH),
            3 => ::std::option::Option::Some(Cluster_LbPolicy::RANDOM),
            4 => ::std::option::Option::Some(Cluster_LbPolicy::ORIGINAL_DST_LB),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cluster_LbPolicy] = &[
            Cluster_LbPolicy::ROUND_ROBIN,
            Cluster_LbPolicy::LEAST_REQUEST,
            Cluster_LbPolicy::RING_HASH,
            Cluster_LbPolicy::RANDOM,
            Cluster_LbPolicy::ORIGINAL_DST_LB,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cluster_LbPolicy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cluster_LbPolicy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cluster_LbPolicy {
}

impl ::std::default::Default for Cluster_LbPolicy {
    fn default() -> Self {
        Cluster_LbPolicy::ROUND_ROBIN
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_LbPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cluster_DnsLookupFamily {
    AUTO = 0,
    V4_ONLY = 1,
    V6_ONLY = 2,
}

impl ::protobuf::ProtobufEnum for Cluster_DnsLookupFamily {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cluster_DnsLookupFamily> {
        match value {
            0 => ::std::option::Option::Some(Cluster_DnsLookupFamily::AUTO),
            1 => ::std::option::Option::Some(Cluster_DnsLookupFamily::V4_ONLY),
            2 => ::std::option::Option::Some(Cluster_DnsLookupFamily::V6_ONLY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cluster_DnsLookupFamily] = &[
            Cluster_DnsLookupFamily::AUTO,
            Cluster_DnsLookupFamily::V4_ONLY,
            Cluster_DnsLookupFamily::V6_ONLY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cluster_DnsLookupFamily>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cluster_DnsLookupFamily", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cluster_DnsLookupFamily {
}

impl ::std::default::Default for Cluster_DnsLookupFamily {
    fn default() -> Self {
        Cluster_DnsLookupFamily::AUTO
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster_DnsLookupFamily {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/cds.proto\x12\x0cenvoy.api.v2\x1a\x11api/address.proto\x1a\x0eap\
    i/base.proto\x1a\x13api/discovery.proto\x1a\x16api/health_check.proto\
    \x1a\x12api/protocol.proto\x1a\x15api/tls_context.proto\x1a\x1cgoogle/ap\
    i/annotations.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\x1cgoogle/\
    protobuf/struct.proto\x1a\x1egoogle/protobuf/wrappers.proto\"R\n\x12Upst\
    reamBindConfig\x12<\n\x0esource_address\x18\x01\x20\x01(\x0b2\x15.envoy.\
    api.v2.AddressR\rsourceAddress\"\xbc\x03\n\x0fCircuitBreakers\x12H\n\nth\
    resholds\x18\x01\x20\x03(\x0b2(.envoy.api.v2.CircuitBreakers.ThresholdsR\
    \nthresholds\x1a\xde\x02\n\nThresholds\x129\n\x08priority\x18\x01\x20\
    \x01(\x0e2\x1d.envoy.api.v2.RoutingPriorityR\x08priority\x12E\n\x0fmax_c\
    onnections\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x0emax\
    Connections\x12N\n\x14max_pending_requests\x18\x03\x20\x01(\x0b2\x1c.goo\
    gle.protobuf.UInt32ValueR\x12maxPendingRequests\x12?\n\x0cmax_requests\
    \x18\x04\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x0bmaxRequests\
    \x12=\n\x0bmax_retries\x18\x05\x20\x01(\x0b2\x1c.google.protobuf.UInt32V\
    alueR\nmaxRetries\"\xbf\x18\n\x07Cluster\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x127\n\x04type\x18\x02\x20\x01(\x0e2#.envoy.api.v2.Clus\
    ter.DiscoveryTypeR\x04type\x12T\n\x12eds_cluster_config\x18\x03\x20\x01(\
    \x0b2&.envoy.api.v2.Cluster.EdsClusterConfigR\x10edsClusterConfig\x12B\n\
    \x0fconnect_timeout\x18\x04\x20\x01(\x0b2\x19.google.protobuf.DurationR\
    \x0econnectTimeout\x12f\n!per_connection_buffer_limit_bytes\x18\x05\x20\
    \x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x1dperConnectionBufferLimitB\
    ytes\x12;\n\tlb_policy\x18\x06\x20\x01(\x0e2\x1e.envoy.api.v2.Cluster.Lb\
    PolicyR\x08lbPolicy\x12+\n\x05hosts\x18\x07\x20\x03(\x0b2\x15.envoy.api.\
    v2.AddressR\x05hosts\x12>\n\rhealth_checks\x18\x08\x20\x03(\x0b2\x19.env\
    oy.api.v2.HealthCheckR\x0chealthChecks\x12[\n\x1bmax_requests_per_connec\
    tion\x18\t\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x18maxRequests\
    PerConnection\x12H\n\x10circuit_breakers\x18\n\x20\x01(\x0b2\x1d.envoy.a\
    pi.v2.CircuitBreakersR\x0fcircuitBreakers\x12A\n\x0btls_context\x18\x0b\
    \x20\x01(\x0b2\x20.envoy.api.v2.UpstreamTlsContextR\ntlsContext\x12T\n\
    \x14tcp_protocol_options\x18\x0c\x20\x01(\x0b2\x20.envoy.api.v2.TcpProto\
    colOptionsH\0R\x12tcpProtocolOptions\x12X\n\x15http_protocol_options\x18\
    \r\x20\x01(\x0b2\".envoy.api.v2.Http1ProtocolOptionsH\0R\x13httpProtocol\
    Options\x12Z\n\x16http2_protocol_options\x18\x0e\x20\x01(\x0b2\".envoy.a\
    pi.v2.Http2ProtocolOptionsH\0R\x14http2ProtocolOptions\x12W\n\x15grpc_pr\
    otocol_options\x18\x0f\x20\x01(\x0b2!.envoy.api.v2.GrpcProtocolOptionsH\
    \0R\x13grpcProtocolOptions\x12C\n\x10dns_refresh_rate\x18\x10\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x0ednsRefreshRate\x12Q\n\x11dns_look\
    up_family\x18\x11\x20\x01(\x0e2%.envoy.api.v2.Cluster.DnsLookupFamilyR\
    \x0fdnsLookupFamily\x12:\n\rdns_resolvers\x18\x12\x20\x03(\x0b2\x15.envo\
    y.api.v2.AddressR\x0cdnsResolvers\x12S\n\x11outlier_detection\x18\x13\
    \x20\x01(\x0b2&.envoy.api.v2.Cluster.OutlierDetectionR\x10outlierDetecti\
    on\x12D\n\x10cleanup_interval\x18\x14\x20\x01(\x0b2\x19.google.protobuf.\
    DurationR\x0fcleanupInterval\x12J\n\x14upstream_bind_config\x18\x15\x20\
    \x01(\x0b2\x18.envoy.api.v2.BindConfigR\x12upstreamBindConfig\x12N\n\x10\
    lb_subset_config\x18\x16\x20\x01(\x0b2$.envoy.api.v2.Cluster.LbSubsetCon\
    figR\x0elbSubsetConfig\x1ap\n\x10EdsClusterConfig\x129\n\neds_config\x18\
    \x01\x20\x01(\x0b2\x1a.envoy.api.v2.ConfigSourceR\tedsConfig\x12!\n\x0cs\
    ervice_name\x18\x02\x20\x01(\tR\x0bserviceName\x1a\xe8\x05\n\x10OutlierD\
    etection\x12E\n\x0fconsecutive_5xx\x18\x01\x20\x01(\x0b2\x1c.google.prot\
    obuf.UInt32ValueR\x0econsecutive5xx\x125\n\x08interval\x18\x02\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x08interval\x12G\n\x12base_ejection_\
    time\x18\x03\x20\x01(\x0b2\x19.google.protobuf.DurationR\x10baseEjection\
    Time\x12N\n\x14max_ejection_percent\x18\x04\x20\x01(\x0b2\x1c.google.pro\
    tobuf.UInt32ValueR\x12maxEjectionPercent\x12X\n\x19enforcing_consecutive\
    _5xx\x18\x05\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x17enforcing\
    Consecutive5xx\x12R\n\x16enforcing_success_rate\x18\x06\x20\x01(\x0b2\
    \x1c.google.protobuf.UInt32ValueR\x14enforcingSuccessRate\x12Y\n\x1asucc\
    ess_rate_minimum_hosts\x18\x07\x20\x01(\x0b2\x1c.google.protobuf.UInt32V\
    alueR\x17successRateMinimumHosts\x12[\n\x1bsuccess_rate_request_volume\
    \x18\x08\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x18successRateRe\
    questVolume\x12W\n\x19success_rate_stdev_factor\x18\t\x20\x01(\x0b2\x1c.\
    google.protobuf.UInt32ValueR\x16successRateStdevFactor\x1a\xff\x02\n\x0e\
    LbSubsetConfig\x12d\n\x0ffallback_policy\x18\x01\x20\x01(\x0e2;.envoy.ap\
    i.v2.Cluster.LbSubsetConfig.LbSubsetFallbackPolicyR\x0efallbackPolicy\
    \x12>\n\x0edefault_subset\x18\x02\x20\x01(\x0b2\x17.google.protobuf.Stru\
    ctR\rdefaultSubset\x12R\n\x0bsubset_keys\x18\x03\x20\x03(\x0b21.envoy.ap\
    i.v2.Cluster.LbSubsetConfig.LbSubsetKeysR\nsubsetKeys\x1a\"\n\x0cLbSubse\
    tKeys\x12\x12\n\x04keys\x18\x01\x20\x03(\tR\x04keys\"O\n\x16LbSubsetFall\
    backPolicy\x12\x0f\n\x0bNO_FALLBACK\x10\0\x12\x10\n\x0cANY_ENDPOINT\x10\
    \x01\x12\x12\n\x0eDEFAULT_SUBSET\x10\x02\"W\n\rDiscoveryType\x12\n\n\x06\
    STATIC\x10\0\x12\x0e\n\nSTRICT_DNS\x10\x01\x12\x0f\n\x0bLOGICAL_DNS\x10\
    \x02\x12\x07\n\x03EDS\x10\x03\x12\x10\n\x0cORIGINAL_DST\x10\x04\"^\n\x08\
    LbPolicy\x12\x0f\n\x0bROUND_ROBIN\x10\0\x12\x11\n\rLEAST_REQUEST\x10\x01\
    \x12\r\n\tRING_HASH\x10\x02\x12\n\n\x06RANDOM\x10\x03\x12\x13\n\x0fORIGI\
    NAL_DST_LB\x10\x04\"5\n\x0fDnsLookupFamily\x12\x08\n\x04AUTO\x10\0\x12\
    \x0b\n\x07V4_ONLY\x10\x01\x12\x0b\n\x07V6_ONLY\x10\x02B\x12\n\x10protoco\
    l_options2\xe7\x01\n\x17ClusterDiscoveryService\x12W\n\x0eStreamClusters\
    \x12\x1e.envoy.api.v2.DiscoveryRequest\x1a\x1f.envoy.api.v2.DiscoveryRes\
    ponse\"\0(\x010\x01\x12s\n\rFetchClusters\x12\x1e.envoy.api.v2.Discovery\
    Request\x1a\x1f.envoy.api.v2.DiscoveryResponse\"!\x82\xd3\xe4\x93\x02\
    \x1b\"\x16/v2/discovery:clusters:\x01*J\x82l\n\x07\x12\x05\0\0\x94\x02\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\
    \n\t\n\x02\x03\0\x12\x03\x04\x07\x1a\n\t\n\x02\x03\x01\x12\x03\x05\x07\
    \x17\n\t\n\x02\x03\x02\x12\x03\x06\x07\x1c\n\t\n\x02\x03\x03\x12\x03\x07\
    \x07\x1f\n\t\n\x02\x03\x04\x12\x03\x08\x07\x1b\n\t\n\x02\x03\x05\x12\x03\
    \t\x07\x1e\n\t\n\x02\x03\x06\x12\x03\x0b\x07%\n\t\n\x02\x03\x07\x12\x03\
    \x0c\x07'\n\t\n\x02\x03\x08\x12\x03\r\x07%\n\t\n\x02\x03\t\x12\x03\x0e\
    \x07'\nJ\n\x02\x06\0\x12\x04\x11\0\x1d\x01\x1a>\x20Return\x20list\x20of\
    \x20all\x20clusters\x20this\x20proxy\x20will\x20load\x20balance\x20to.\n\
    \n\n\n\x03\x06\0\x01\x12\x03\x11\x08\x1f\n\x0c\n\x04\x06\0\x02\0\x12\x04\
    \x12\x02\x14\x03\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x12\x06\x14\n\x0c\n\
    \x05\x06\0\x02\0\x05\x12\x03\x12\x15\x1b\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03\x12\x1c,\n\x0c\n\x05\x06\0\x02\0\x06\x12\x03\x13\x0f\x15\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03\x13\x16'\n\x0c\n\x04\x06\0\x02\x01\x12\x04\
    \x16\x02\x1c\x03\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x16\x06\x13\n\x0c\
    \n\x05\x06\0\x02\x01\x02\x12\x03\x16\x14$\n\x0c\n\x05\x06\0\x02\x01\x03\
    \x12\x03\x17\x0f\x20\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\x18\x04\x1b\x06\
    \n\x10\n\x08\x06\0\x02\x01\x04\xe7\x07\0\x12\x04\x18\x04\x1b\x06\n\x10\n\
    \t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03\x18\x0b\x1c\n\x11\n\n\x06\0\
    \x02\x01\x04\xe7\x07\0\x02\0\x12\x03\x18\x0b\x1c\n\x12\n\x0b\x06\0\x02\
    \x01\x04\xe7\x07\0\x02\0\x01\x12\x03\x18\x0c\x1b\n\x11\n\t\x06\0\x02\x01\
    \x04\xe7\x07\0\x08\x12\x04\x18\x1f\x1b\x05\nz\n\x02\x04\0\x12\x04!\0$\
    \x01\x1an\x20An\x20extensible\x20structure\x20containing\x20the\x20addre\
    ss\x20Envoy\x20should\x20bind\x20to\x20when\x20establishing\x20upstream\
    \n\x20connections.\n\n\n\n\x03\x04\0\x01\x12\x03!\x08\x1a\nW\n\x04\x04\0\
    \x02\0\x12\x03#\x02\x1d\x1aJ\x20The\x20address\x20Envoy\x20should\x20bin\
    d\x20to\x20when\x20establishing\x20upstream\x20connections.\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04#\x02!\x1c\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03#\
    \x02\t\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03#\n\x18\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03#\x1b\x1c\na\n\x02\x04\x01\x12\x04(\0=\x01\x1aU\x20Circuit\
    \x20breaking\x20settings\x20can\x20be\x20specified\x20individually\x20fo\
    r\x20each\x20defined\n\x20priority.\n\n\n\n\x03\x04\x01\x01\x12\x03(\x08\
    \x17\n\x0c\n\x04\x04\x01\x03\0\x12\x04)\x02;\x03\n\x0c\n\x05\x04\x01\x03\
    \0\x01\x12\x03)\n\x14\n\r\n\x06\x04\x01\x03\0\x02\0\x12\x03*\x04!\n\x0f\
    \n\x07\x04\x01\x03\0\x02\0\x04\x12\x04*\x04)\x16\n\x0e\n\x07\x04\x01\x03\
    \0\x02\0\x06\x12\x03*\x04\x13\n\x0e\n\x07\x04\x01\x03\0\x02\0\x01\x12\
    \x03*\x14\x1c\n\x0e\n\x07\x04\x01\x03\0\x02\0\x03\x12\x03*\x1f\x20\n\xc2\
    \x01\n\x06\x04\x01\x03\0\x02\x01\x12\x03.\x044\x1a\xb2\x01\x20The\x20max\
    imum\x20number\x20of\x20connections\x20that\x20Envoy\x20will\x20make\x20\
    to\x20the\x20upstream\n\x20cluster.\x20If\x20not\x20specified,\x20the\
    \x20default\x20is\x201024.\x20See\x20the\x20circuit\n\x20breaking\x20ove\
    rview\x20for\x20more\x20information.\n\n\x0f\n\x07\x04\x01\x03\0\x02\x01\
    \x04\x12\x04.\x04*!\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x06\x12\x03.\x04\
    \x1f\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x01\x12\x03.\x20/\n\x0e\n\x07\x04\
    \x01\x03\0\x02\x01\x03\x12\x03.23\n\xc8\x01\n\x06\x04\x01\x03\0\x02\x02\
    \x12\x032\x049\x1a\xb8\x01\x20The\x20maximum\x20number\x20of\x20pending\
    \x20requests\x20that\x20Envoy\x20will\x20allow\x20to\x20the\n\x20upstrea\
    m\x20cluster.\x20If\x20not\x20specified,\x20the\x20default\x20is\x201024\
    .\x20See\x20the\x20circuit\n\x20breaking\x20overview\x20for\x20more\x20i\
    nformation.\n\n\x0f\n\x07\x04\x01\x03\0\x02\x02\x04\x12\x042\x04.4\n\x0e\
    \n\x07\x04\x01\x03\0\x02\x02\x06\x12\x032\x04\x1f\n\x0e\n\x07\x04\x01\
    \x03\0\x02\x02\x01\x12\x032\x204\n\x0e\n\x07\x04\x01\x03\0\x02\x02\x03\
    \x12\x03278\n\xc8\x01\n\x06\x04\x01\x03\0\x02\x03\x12\x036\x041\x1a\xb8\
    \x01\x20The\x20maximum\x20number\x20of\x20parallel\x20requests\x20that\
    \x20Envoy\x20will\x20make\x20to\x20the\n\x20upstream\x20cluster.\x20If\
    \x20not\x20specified,\x20the\x20default\x20is\x201024.\x20See\x20the\x20\
    circuit\n\x20breaking\x20overview\x20for\x20more\x20information.\n\n\x0f\
    \n\x07\x04\x01\x03\0\x02\x03\x04\x12\x046\x0429\n\x0e\n\x07\x04\x01\x03\
    \0\x02\x03\x06\x12\x036\x04\x1f\n\x0e\n\x07\x04\x01\x03\0\x02\x03\x01\
    \x12\x036\x20,\n\x0e\n\x07\x04\x01\x03\0\x02\x03\x03\x12\x036/0\n\xc5\
    \x01\n\x06\x04\x01\x03\0\x02\x04\x12\x03:\x040\x1a\xb5\x01\x20The\x20max\
    imum\x20number\x20of\x20parallel\x20retries\x20that\x20Envoy\x20will\x20\
    allow\x20to\x20the\n\x20upstream\x20cluster.\x20If\x20not\x20specified,\
    \x20the\x20default\x20is\x203.\x20See\x20the\x20circuit\n\x20breaking\
    \x20overview\x20for\x20more\x20information.\n\n\x0f\n\x07\x04\x01\x03\0\
    \x02\x04\x04\x12\x04:\x0461\n\x0e\n\x07\x04\x01\x03\0\x02\x04\x06\x12\
    \x03:\x04\x1f\n\x0e\n\x07\x04\x01\x03\0\x02\x04\x01\x12\x03:\x20+\n\x0e\
    \n\x07\x04\x01\x03\0\x02\x04\x03\x12\x03:./\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03<\x02%\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03<\x02\n\n\x0c\n\x05\
    \x04\x01\x02\0\x06\x12\x03<\x0b\x15\n\x0c\n\x05\x04\x01\x02\0\x01\x12\
    \x03<\x16\x20\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03<#$\n\x0b\n\x02\x04\
    \x02\x12\x05?\0\x94\x02\x01\n\n\n\x03\x04\x02\x01\x12\x03?\x08\x0f\n\xdb\
    \x01\n\x04\x04\x02\x02\0\x12\x03C\x02\x12\x1a\xcd\x01\x20Supplies\x20the\
    \x20name\x20of\x20the\x20cluster\x20which\x20must\x20be\x20unique\x20acr\
    oss\x20all\x20clusters.\n\x20The\x20cluster\x20name\x20is\x20used\x20whe\
    n\x20emitting\x20statistics.\x20The\x20cluster\x20name\x20can\x20be\n\
    \x20at\x20most\x2060\x20characters\x20long,\x20and\x20must\x20not\x20con\
    tain\x20:.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04C\x02?\x11\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03C\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03C\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03C\x10\x11\nL\n\x04\x04\
    \x02\x04\0\x12\x04F\x02L\x03\x1a>\x20The\x20service\x20discovery\x20type\
    \x20to\x20use\x20for\x20resolving\x20the\x20cluster.\n\n\x0c\n\x05\x04\
    \x02\x04\0\x01\x12\x03F\x07\x14\n\r\n\x06\x04\x02\x04\0\x02\0\x12\x03G\
    \x04\x0f\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03G\x04\n\n\x0e\n\x07\
    \x04\x02\x04\0\x02\0\x02\x12\x03G\r\x0e\n\r\n\x06\x04\x02\x04\0\x02\x01\
    \x12\x03H\x04\x13\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x01\x12\x03H\x04\x0e\
    \n\x0e\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03H\x11\x12\n\r\n\x06\x04\
    \x02\x04\0\x02\x02\x12\x03I\x04\x14\n\x0e\n\x07\x04\x02\x04\0\x02\x02\
    \x01\x12\x03I\x04\x0f\n\x0e\n\x07\x04\x02\x04\0\x02\x02\x02\x12\x03I\x12\
    \x13\n\r\n\x06\x04\x02\x04\0\x02\x03\x12\x03J\x04\x0c\n\x0e\n\x07\x04\
    \x02\x04\0\x02\x03\x01\x12\x03J\x04\x07\n\x0e\n\x07\x04\x02\x04\0\x02\
    \x03\x02\x12\x03J\n\x0b\n\r\n\x06\x04\x02\x04\0\x02\x04\x12\x03K\x04\x15\
    \n\x0e\n\x07\x04\x02\x04\0\x02\x04\x01\x12\x03K\x04\x10\n\x0e\n\x07\x04\
    \x02\x04\0\x02\x04\x02\x12\x03K\x13\x14\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03M\x02\x19\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04M\x02L\x03\n\x0c\n\
    \x05\x04\x02\x02\x01\x06\x12\x03M\x02\x0f\n\x0c\n\x05\x04\x02\x02\x01\
    \x01\x12\x03M\x10\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03M\x17\x18\n\
    6\n\x04\x04\x02\x03\0\x12\x04P\x02V\x03\x1a(\x20Only\x20valid\x20when\
    \x20discovery\x20type\x20is\x20EDS.\n\n\x0c\n\x05\x04\x02\x03\0\x01\x12\
    \x03P\n\x1a\n\r\n\x06\x04\x02\x03\0\x02\0\x12\x03Q\x04\x20\n\x0f\n\x07\
    \x04\x02\x03\0\x02\0\x04\x12\x04Q\x04P\x1c\n\x0e\n\x07\x04\x02\x03\0\x02\
    \0\x06\x12\x03Q\x04\x10\n\x0e\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03Q\x11\
    \x1b\n\x0e\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03Q\x1e\x1f\n\xb6\x01\n\
    \x06\x04\x02\x03\0\x02\x01\x12\x03U\x04\x1c\x1a\xa6\x01\x20Optional\x20a\
    lternative\x20to\x20cluster\x20name\x20to\x20present\x20to\x20EDS.\x20Th\
    is\x20does\x20not\n\x20have\x20the\x20same\x20restrictions\x20as\x20clus\
    ter\x20name,\x20i.e.\x20it\x20may\x20be\x20arbritary\n\x20length\x20and\
    \x20contain\x20':'.\n\n\x0f\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x04U\x04\
    Q\x20\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x03U\x04\n\n\x0e\n\x07\
    \x04\x02\x03\0\x02\x01\x01\x12\x03U\x0b\x17\n\x0e\n\x07\x04\x02\x03\0\
    \x02\x01\x03\x12\x03U\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x02\x12\x03W\x02*\
    \n\r\n\x05\x04\x02\x02\x02\x04\x12\x04W\x02V\x03\n\x0c\n\x05\x04\x02\x02\
    \x02\x06\x12\x03W\x02\x12\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03W\x13%\
    \n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03W()\nO\n\x04\x04\x02\x02\x03\x12\
    \x03Z\x02/\x1aB\x20The\x20timeout\x20for\x20new\x20network\x20connection\
    s\x20to\x20hosts\x20in\x20the\x20cluster.\n\n\r\n\x05\x04\x02\x02\x03\
    \x04\x12\x04Z\x02W*\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03Z\x02\x1a\n\
    \x0c\n\x05\x04\x02\x02\x03\x01\x12\x03Z\x1b*\n\x0c\n\x05\x04\x02\x02\x03\
    \x03\x12\x03Z-.\n\x9f\x01\n\x04\x04\x02\x02\x04\x12\x03]\x02D\x1a\x91\
    \x01\x20Soft\x20limit\x20on\x20size\x20of\x20the\x20cluster\xe2\x80\x99s\
    \x20connections\x20read\x20and\x20write\x20buffers.\x20If\n\x20unspecifi\
    ed,\x20an\x20implementation\x20defined\x20default\x20is\x20applied\x20(1\
    MiB).\n\n\r\n\x05\x04\x02\x02\x04\x04\x12\x04]\x02Z/\n\x0c\n\x05\x04\x02\
    \x02\x04\x06\x12\x03]\x02\x1d\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03]\
    \x1e?\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03]BC\nQ\n\x04\x04\x02\x04\
    \x01\x12\x04`\x02f\x03\x1aC\x20The\x20load\x20balancer\x20type\x20to\x20\
    use\x20when\x20picking\x20a\x20host\x20in\x20the\x20cluster.\n\n\x0c\n\
    \x05\x04\x02\x04\x01\x01\x12\x03`\x07\x0f\n\r\n\x06\x04\x02\x04\x01\x02\
    \0\x12\x03a\x04\x14\n\x0e\n\x07\x04\x02\x04\x01\x02\0\x01\x12\x03a\x04\
    \x0f\n\x0e\n\x07\x04\x02\x04\x01\x02\0\x02\x12\x03a\x12\x13\n\r\n\x06\
    \x04\x02\x04\x01\x02\x01\x12\x03b\x04\x16\n\x0e\n\x07\x04\x02\x04\x01\
    \x02\x01\x01\x12\x03b\x04\x11\n\x0e\n\x07\x04\x02\x04\x01\x02\x01\x02\
    \x12\x03b\x14\x15\n\r\n\x06\x04\x02\x04\x01\x02\x02\x12\x03c\x04\x12\n\
    \x0e\n\x07\x04\x02\x04\x01\x02\x02\x01\x12\x03c\x04\r\n\x0e\n\x07\x04\
    \x02\x04\x01\x02\x02\x02\x12\x03c\x10\x11\n\r\n\x06\x04\x02\x04\x01\x02\
    \x03\x12\x03d\x04\x0f\n\x0e\n\x07\x04\x02\x04\x01\x02\x03\x01\x12\x03d\
    \x04\n\n\x0e\n\x07\x04\x02\x04\x01\x02\x03\x02\x12\x03d\r\x0e\n\r\n\x06\
    \x04\x02\x04\x01\x02\x04\x12\x03e\x04\x18\n\x0e\n\x07\x04\x02\x04\x01\
    \x02\x04\x01\x12\x03e\x04\x13\n\x0e\n\x07\x04\x02\x04\x01\x02\x04\x02\
    \x12\x03e\x16\x17\n\x0b\n\x04\x04\x02\x02\x05\x12\x03g\x02\x19\n\r\n\x05\
    \x04\x02\x02\x05\x04\x12\x04g\x02f\x03\n\x0c\n\x05\x04\x02\x02\x05\x06\
    \x12\x03g\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03g\x0b\x14\n\x0c\n\
    \x05\x04\x02\x02\x05\x03\x12\x03g\x17\x18\nk\n\x04\x04\x02\x02\x06\x12\
    \x03k\x02\x1d\x1a^\x20If\x20the\x20service\x20discovery\x20type\x20is\
    \x20STATIC,\x20STRICT_DNS\x20or\x20LOGICAL_DNS,\x20then\n\x20hosts\x20is\
    \x20required.\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x03k\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x06\x06\x12\x03k\x0b\x12\n\x0c\n\x05\x04\x02\x02\x06\
    \x01\x12\x03k\x13\x18\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x03k\x1b\x1c\n\
    \xd2\x01\n\x04\x04\x02\x02\x07\x12\x03p\x02)\x1a\xc4\x01\x20Optional\x20\
    active\x20health\x20checking\x20configuration\x20for\x20the\x20cluster.\
    \x20If\x20no\n\x20configuration\x20is\x20specified\x20no\x20health\x20ch\
    ecking\x20will\x20be\x20done\x20and\x20all\x20cluster\n\x20members\x20wi\
    ll\x20be\x20considered\x20healthy\x20at\x20all\x20times.\n\n\x0c\n\x05\
    \x04\x02\x02\x07\x04\x12\x03p\x02\n\n\x0c\n\x05\x04\x02\x02\x07\x06\x12\
    \x03p\x0b\x16\n\x0c\n\x05\x04\x02\x02\x07\x01\x12\x03p\x17$\n\x0c\n\x05\
    \x04\x02\x02\x07\x03\x12\x03p'(\n\x91\x02\n\x04\x04\x02\x02\x08\x12\x03v\
    \x02>\x1a\x83\x02\x20Optional\x20maximum\x20requests\x20for\x20a\x20sing\
    le\x20upstream\x20connection.\x20This\x20parameter\n\x20is\x20respected\
    \x20by\x20both\x20the\x20HTTP/1.1\x20and\x20HTTP/2\x20connection\x20pool\
    \n\x20implementations.\x20If\x20not\x20specified,\x20there\x20is\x20no\
    \x20limit.\x20Setting\x20this\n\x20parameter\x20to\x201\x20will\x20effec\
    tively\x20disable\x20keep\x20alive.\n\n\r\n\x05\x04\x02\x02\x08\x04\x12\
    \x04v\x02p)\n\x0c\n\x05\x04\x02\x02\x08\x06\x12\x03v\x02\x1d\n\x0c\n\x05\
    \x04\x02\x02\x08\x01\x12\x03v\x1e9\n\x0c\n\x05\x04\x02\x02\x08\x03\x12\
    \x03v<=\nB\n\x04\x04\x02\x02\t\x12\x03y\x02(\x1a5\x20Optional\x20circuit\
    \x20breaking\x20settings\x20for\x20the\x20cluster.\n\n\r\n\x05\x04\x02\
    \x02\t\x04\x12\x04y\x02v>\n\x0c\n\x05\x04\x02\x02\t\x06\x12\x03y\x02\x11\
    \n\x0c\n\x05\x04\x02\x02\t\x01\x12\x03y\x12\"\n\x0c\n\x05\x04\x02\x02\t\
    \x03\x12\x03y%'\n\x9f\x01\n\x04\x04\x02\x02\n\x12\x03}\x02&\x1a\x91\x01\
    \x20The\x20TLS\x20configuration\x20for\x20connections\x20to\x20the\x20up\
    stream\x20cluster.\x20If\x20no\x20TLS\n\x20configuration\x20is\x20specif\
    ied,\x20TLS\x20will\x20not\x20be\x20used\x20for\x20new\x20connections.\n\
    \n\r\n\x05\x04\x02\x02\n\x04\x12\x04}\x02y(\n\x0c\n\x05\x04\x02\x02\n\
    \x06\x12\x03}\x02\x14\n\x0c\n\x05\x04\x02\x02\n\x01\x12\x03}\x15\x20\n\
    \x0c\n\x05\x04\x02\x02\n\x03\x12\x03}#%\n\r\n\x04\x04\x02\x08\0\x12\x05\
    \x7f\x02\x8a\x01\x03\n\x0c\n\x05\x04\x02\x08\0\x01\x12\x03\x7f\x08\x18\n\
    \x0c\n\x04\x04\x02\x02\x0b\x12\x04\x80\x01\x041\n\r\n\x05\x04\x02\x02\
    \x0b\x06\x12\x04\x80\x01\x04\x16\n\r\n\x05\x04\x02\x02\x0b\x01\x12\x04\
    \x80\x01\x17+\n\r\n\x05\x04\x02\x02\x0b\x03\x12\x04\x80\x01.0\n\x0c\n\
    \x04\x04\x02\x02\x0c\x12\x04\x81\x01\x045\n\r\n\x05\x04\x02\x02\x0c\x06\
    \x12\x04\x81\x01\x04\x18\n\r\n\x05\x04\x02\x02\x0c\x01\x12\x04\x81\x01\
    \x19.\n\r\n\x05\x04\x02\x02\x0c\x03\x12\x04\x81\x0124\n\x95\x03\n\x04\
    \x04\x02\x02\r\x12\x04\x88\x01\x046\x1a\x86\x03\x20Even\x20if\x20default\
    \x20HTTP2\x20protocol\x20options\x20are\x20desired,\x20this\x20field\x20\
    must\x20be\n\x20set\x20so\x20that\x20Envoy\x20will\x20assume\x20that\x20\
    the\x20upstream\x20supports\x20HTTP/2\x20when\n\x20making\x20new\x20HTTP\
    \x20connection\x20pool\x20connections.\x20Currently,\x20Envoy\x20only\n\
    \x20supports\x20prior\x20knowledge\x20for\x20upstream\x20connections.\
    \x20Even\x20if\x20TLS\x20is\x20used\n\x20with\x20ALPN,\x20http2\x20must\
    \x20be\x20specified.\x20As\x20an\x20aside\x20this\x20allows\x20HTTP/2\n\
    \x20connections\x20to\x20happen\x20over\x20plain\x20text.\n\n\r\n\x05\
    \x04\x02\x02\r\x06\x12\x04\x88\x01\x04\x18\n\r\n\x05\x04\x02\x02\r\x01\
    \x12\x04\x88\x01\x19/\n\r\n\x05\x04\x02\x02\r\x03\x12\x04\x88\x0135\n\
    \x0c\n\x04\x04\x02\x02\x0e\x12\x04\x89\x01\x044\n\r\n\x05\x04\x02\x02\
    \x0e\x06\x12\x04\x89\x01\x04\x17\n\r\n\x05\x04\x02\x02\x0e\x01\x12\x04\
    \x89\x01\x18-\n\r\n\x05\x04\x02\x02\x0e\x03\x12\x04\x89\x0113\n\xbb\x02\
    \n\x04\x04\x02\x02\x0f\x12\x04\x91\x01\x021\x1a\xac\x02\x20If\x20the\x20\
    dns\x20refresh\x20rate\x20is\x20specified\x20and\x20the\x20cluster\x20ty\
    pe\x20is\x20either\n\x20STRICT_DNS,\x20or\x20LOGICAL_DNS,\x20this\x20val\
    ue\x20is\x20used\x20as\x20the\x20cluster\xe2\x80\x99s\x20dns\x20refresh\
    \n\x20rate.\x20If\x20this\x20setting\x20is\x20not\x20specified,\x20the\
    \x20value\x20defaults\x20to\x205000.\x20For\n\x20cluster\x20types\x20oth\
    er\x20than\x20STRICT_DNS\x20and\x20LOGICAL_DNS\x20this\x20setting\x20is\
    \n\x20ignored.\n\n\x0f\n\x05\x04\x02\x02\x0f\x04\x12\x06\x91\x01\x02\x8a\
    \x01\x03\n\r\n\x05\x04\x02\x02\x0f\x06\x12\x04\x91\x01\x02\x1a\n\r\n\x05\
    \x04\x02\x02\x0f\x01\x12\x04\x91\x01\x1b+\n\r\n\x05\x04\x02\x02\x0f\x03\
    \x12\x04\x91\x01.0\n\xec\x04\n\x04\x04\x02\x04\x02\x12\x06\x9c\x01\x02\
    \xa0\x01\x03\x1a\xdb\x04\x20The\x20DNS\x20IP\x20address\x20resolution\
    \x20policy.\x20The\x20options\x20are\x20V4_ONLY,\x20V6_ONLY,\x20and\n\
    \x20AUTO.\x20If\x20this\x20setting\x20is\x20not\x20specified,\x20the\x20\
    value\x20defaults\x20to\x20V4_ONLY.\x20When\n\x20V4_ONLY\x20is\x20select\
    ed,\x20the\x20DNS\x20resolver\x20will\x20only\x20perform\x20a\x20lookup\
    \x20for\n\x20addresses\x20in\x20the\x20IPv4\x20family.\x20If\x20V6_ONLY\
    \x20is\x20selected,\x20the\x20DNS\x20resolver\x20will\n\x20only\x20perfo\
    rm\x20a\x20lookup\x20for\x20addresses\x20in\x20the\x20IPv6\x20family.\
    \x20If\x20AUTO\x20is\n\x20specified,\x20the\x20DNS\x20resolver\x20will\
    \x20first\x20perform\x20a\x20lookup\x20for\x20addresses\x20in\n\x20the\
    \x20IPv6\x20family\x20and\x20fallback\x20to\x20a\x20lookup\x20for\x20add\
    resses\x20in\x20the\x20IPv4\x20family.\n\x20For\x20cluster\x20types\x20o\
    ther\x20than\x20STRICT_DNS\x20and\x20LOGICAL_DNS,\x20this\x20setting\x20\
    is\n\x20ignored.\n\n\r\n\x05\x04\x02\x04\x02\x01\x12\x04\x9c\x01\x07\x16\
    \n\x0e\n\x06\x04\x02\x04\x02\x02\0\x12\x04\x9d\x01\x04\r\n\x0f\n\x07\x04\
    \x02\x04\x02\x02\0\x01\x12\x04\x9d\x01\x04\x08\n\x0f\n\x07\x04\x02\x04\
    \x02\x02\0\x02\x12\x04\x9d\x01\x0b\x0c\n\x0e\n\x06\x04\x02\x04\x02\x02\
    \x01\x12\x04\x9e\x01\x04\x10\n\x0f\n\x07\x04\x02\x04\x02\x02\x01\x01\x12\
    \x04\x9e\x01\x04\x0b\n\x0f\n\x07\x04\x02\x04\x02\x02\x01\x02\x12\x04\x9e\
    \x01\x0e\x0f\n\x0e\n\x06\x04\x02\x04\x02\x02\x02\x12\x04\x9f\x01\x04\x10\
    \n\x0f\n\x07\x04\x02\x04\x02\x02\x02\x01\x12\x04\x9f\x01\x04\x0b\n\x0f\n\
    \x07\x04\x02\x04\x02\x02\x02\x02\x12\x04\x9f\x01\x0e\x0f\n\x0c\n\x04\x04\
    \x02\x02\x10\x12\x04\xa1\x01\x02)\n\x0f\n\x05\x04\x02\x02\x10\x04\x12\
    \x06\xa1\x01\x02\xa0\x01\x03\n\r\n\x05\x04\x02\x02\x10\x06\x12\x04\xa1\
    \x01\x02\x11\n\r\n\x05\x04\x02\x02\x10\x01\x12\x04\xa1\x01\x12#\n\r\n\
    \x05\x04\x02\x02\x10\x03\x12\x04\xa1\x01&(\n\xf9\x02\n\x04\x04\x02\x02\
    \x11\x12\x04\xa8\x01\x02&\x1a\xea\x02\x20If\x20DNS\x20resolvers\x20are\
    \x20specified\x20and\x20the\x20cluster\x20type\x20is\x20either\x20STRICT\
    _DNS,\n\x20or\x20LOGICAL_DNS,\x20this\x20value\x20is\x20used\x20to\x20sp\
    ecify\x20the\x20cluster\xe2\x80\x99s\x20dns\x20resolvers.\n\x20If\x20thi\
    s\x20setting\x20is\x20not\x20specified,\x20the\x20value\x20defaults\x20t\
    o\x20the\x20default\n\x20resolver,\x20which\x20uses\x20/etc/resolv.conf\
    \x20for\x20configuration.\x20For\x20cluster\x20types\n\x20other\x20than\
    \x20STRICT_DNS\x20and\x20LOGICAL_DNS\x20this\x20setting\x20is\x20ignored\
    .\n\n\r\n\x05\x04\x02\x02\x11\x04\x12\x04\xa8\x01\x02\n\n\r\n\x05\x04\
    \x02\x02\x11\x06\x12\x04\xa8\x01\x0b\x12\n\r\n\x05\x04\x02\x02\x11\x01\
    \x12\x04\xa8\x01\x13\x20\n\r\n\x05\x04\x02\x02\x11\x03\x12\x04\xa8\x01#%\
    \n\\\n\x04\x04\x02\x03\x01\x12\x06\xab\x01\x02\xd5\x01\x03\x1aL\x20If\
    \x20specified,\x20outlier\x20detection\x20will\x20be\x20enabled\x20for\
    \x20this\x20upstream\x20cluster.\n\n\r\n\x05\x04\x02\x03\x01\x01\x12\x04\
    \xab\x01\n\x1a\ns\n\x06\x04\x02\x03\x01\x02\0\x12\x04\xae\x01\x044\x1ac\
    \x20The\x20number\x20of\x20consecutive\x205xx\x20responses\x20before\x20\
    a\x20consecutive\x205xx\x20ejection\n\x20occurs.\x20Defaults\x20to\x205.\
    \n\n\x11\n\x07\x04\x02\x03\x01\x02\0\x04\x12\x06\xae\x01\x04\xab\x01\x1c\
    \n\x0f\n\x07\x04\x02\x03\x01\x02\0\x06\x12\x04\xae\x01\x04\x1f\n\x0f\n\
    \x07\x04\x02\x03\x01\x02\0\x01\x12\x04\xae\x01\x20/\n\x0f\n\x07\x04\x02\
    \x03\x01\x02\0\x03\x12\x04\xae\x0123\n\xb6\x01\n\x06\x04\x02\x03\x01\x02\
    \x01\x12\x04\xb2\x01\x04*\x1a\xa5\x01\x20The\x20time\x20interval\x20betw\
    een\x20ejection\x20analysis\x20sweeps.\x20This\x20can\x20result\x20in\n\
    \x20both\x20new\x20ejections\x20as\x20well\x20as\x20hosts\x20being\x20re\
    turned\x20to\x20service.\x20Defaults\n\x20to\x2010000ms\x20or\x2010s.\n\
    \n\x11\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x06\xb2\x01\x04\xae\x014\n\
    \x0f\n\x07\x04\x02\x03\x01\x02\x01\x06\x12\x04\xb2\x01\x04\x1c\n\x0f\n\
    \x07\x04\x02\x03\x01\x02\x01\x01\x12\x04\xb2\x01\x1d%\n\x0f\n\x07\x04\
    \x02\x03\x01\x02\x01\x03\x12\x04\xb2\x01()\n\xbf\x01\n\x06\x04\x02\x03\
    \x01\x02\x02\x12\x04\xb6\x01\x044\x1a\xae\x01\x20The\x20base\x20time\x20\
    that\x20a\x20host\x20is\x20ejected\x20for.\x20The\x20real\x20time\x20is\
    \x20equal\x20to\x20the\n\x20base\x20time\x20multiplied\x20by\x20the\x20n\
    umber\x20of\x20times\x20the\x20host\x20has\x20been\x20ejected.\n\x20Defa\
    ults\x20to\x2030000ms\x20or\x2030s.\n\n\x11\n\x07\x04\x02\x03\x01\x02\
    \x02\x04\x12\x06\xb6\x01\x04\xb2\x01*\n\x0f\n\x07\x04\x02\x03\x01\x02\
    \x02\x06\x12\x04\xb6\x01\x04\x1c\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x01\
    \x12\x04\xb6\x01\x1d/\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x03\x12\x04\
    \xb6\x0123\nv\n\x06\x04\x02\x03\x01\x02\x03\x12\x04\xb9\x01\x049\x1af\
    \x20The\x20maximum\x20%\x20of\x20an\x20upstream\x20cluster\x20that\x20ca\
    n\x20be\x20ejected\x20due\x20to\x20outlier\n\x20detection.\x20Defaults\
    \x20to\x2010%.\n\n\x11\n\x07\x04\x02\x03\x01\x02\x03\x04\x12\x06\xb9\x01\
    \x04\xb6\x014\n\x0f\n\x07\x04\x02\x03\x01\x02\x03\x06\x12\x04\xb9\x01\
    \x04\x1f\n\x0f\n\x07\x04\x02\x03\x01\x02\x03\x01\x12\x04\xb9\x01\x204\n\
    \x0f\n\x07\x04\x02\x03\x01\x02\x03\x03\x12\x04\xb9\x0178\n\xd9\x01\n\x06\
    \x04\x02\x03\x01\x02\x04\x12\x04\xbd\x01\x04>\x1a\xc8\x01\x20The\x20%\
    \x20chance\x20that\x20a\x20host\x20will\x20be\x20actually\x20ejected\x20\
    when\x20an\x20outlier\x20status\n\x20is\x20detected\x20through\x20consec\
    utive\x205xx.\x20This\x20setting\x20can\x20be\x20used\x20to\x20disable\n\
    \x20ejection\x20or\x20to\x20ramp\x20it\x20up\x20slowly.\x20Defaults\x20t\
    o\x20100.\n\n\x11\n\x07\x04\x02\x03\x01\x02\x04\x04\x12\x06\xbd\x01\x04\
    \xb9\x019\n\x0f\n\x07\x04\x02\x03\x01\x02\x04\x06\x12\x04\xbd\x01\x04\
    \x1f\n\x0f\n\x07\x04\x02\x03\x01\x02\x04\x01\x12\x04\xbd\x01\x209\n\x0f\
    \n\x07\x04\x02\x03\x01\x02\x04\x03\x12\x04\xbd\x01<=\n\xe1\x01\n\x06\x04\
    \x02\x03\x01\x02\x05\x12\x04\xc1\x01\x04;\x1a\xd0\x01\x20The\x20%\x20cha\
    nce\x20that\x20a\x20host\x20will\x20be\x20actually\x20ejected\x20when\
    \x20an\x20outlier\x20status\n\x20is\x20detected\x20through\x20success\
    \x20rate\x20statistics.\x20This\x20setting\x20can\x20be\x20used\x20to\n\
    \x20disable\x20ejection\x20or\x20to\x20ramp\x20it\x20up\x20slowly.\x20De\
    faults\x20to\x20100.\n\n\x11\n\x07\x04\x02\x03\x01\x02\x05\x04\x12\x06\
    \xc1\x01\x04\xbd\x01>\n\x0f\n\x07\x04\x02\x03\x01\x02\x05\x06\x12\x04\
    \xc1\x01\x04\x1f\n\x0f\n\x07\x04\x02\x03\x01\x02\x05\x01\x12\x04\xc1\x01\
    \x206\n\x0f\n\x07\x04\x02\x03\x01\x02\x05\x03\x12\x04\xc1\x019:\n\x99\
    \x02\n\x06\x04\x02\x03\x01\x02\x06\x12\x04\xc6\x01\x04?\x1a\x88\x02\x20T\
    he\x20number\x20of\x20hosts\x20in\x20a\x20cluster\x20that\x20must\x20hav\
    e\x20enough\x20request\x20volume\x20to\n\x20detect\x20success\x20rate\
    \x20outliers.\x20If\x20the\x20number\x20of\x20hosts\x20is\x20less\x20tha\
    n\x20this\n\x20setting,\x20outlier\x20detection\x20via\x20success\x20rat\
    e\x20statistics\x20is\x20not\x20performed\n\x20for\x20any\x20host\x20in\
    \x20the\x20cluster.\x20Defaults\x20to\x205.\n\n\x11\n\x07\x04\x02\x03\
    \x01\x02\x06\x04\x12\x06\xc6\x01\x04\xc1\x01;\n\x0f\n\x07\x04\x02\x03\
    \x01\x02\x06\x06\x12\x04\xc6\x01\x04\x1f\n\x0f\n\x07\x04\x02\x03\x01\x02\
    \x06\x01\x12\x04\xc6\x01\x20:\n\x0f\n\x07\x04\x02\x03\x01\x02\x06\x03\
    \x12\x04\xc6\x01=>\n\xd5\x02\n\x06\x04\x02\x03\x01\x02\x07\x12\x04\xcc\
    \x01\x04@\x1a\xc4\x02\x20The\x20minimum\x20number\x20of\x20total\x20requ\
    ests\x20that\x20must\x20be\x20collected\x20in\x20one\n\x20interval\x20(a\
    s\x20defined\x20by\x20the\x20interval\x20duration\x20above)\x20to\x20inc\
    lude\x20this\x20host\n\x20in\x20success\x20rate\x20based\x20outlier\x20d\
    etection.\x20If\x20the\x20volume\x20is\x20lower\x20than\x20this\n\x20set\
    ting,\x20outlier\x20detection\x20via\x20success\x20rate\x20statistics\
    \x20is\x20not\x20performed\n\x20for\x20that\x20host.\x20Defaults\x20to\
    \x20100.\n\n\x11\n\x07\x04\x02\x03\x01\x02\x07\x04\x12\x06\xcc\x01\x04\
    \xc6\x01?\n\x0f\n\x07\x04\x02\x03\x01\x02\x07\x06\x12\x04\xcc\x01\x04\
    \x1f\n\x0f\n\x07\x04\x02\x03\x01\x02\x07\x01\x12\x04\xcc\x01\x20;\n\x0f\
    \n\x07\x04\x02\x03\x01\x02\x07\x03\x12\x04\xcc\x01>?\n\xcc\x03\n\x06\x04\
    \x02\x03\x01\x02\x08\x12\x04\xd4\x01\x04>\x1a\xbb\x03\x20This\x20factor\
    \x20is\x20used\x20to\x20determine\x20the\x20ejection\x20threshold\x20for\
    \x20success\x20rate\n\x20outlier\x20ejection.\x20The\x20ejection\x20thre\
    shold\x20is\x20the\x20difference\x20between\x20the\n\x20mean\x20success\
    \x20rate,\x20and\x20the\x20product\x20of\x20this\x20factor\x20and\x20the\
    \x20standard\n\x20deviation\x20of\x20the\x20mean\x20success\x20rate:\x20\
    mean\x20-\x20(stdev\x20*\n\x20success_rate_stdev_factor).\x20This\x20fac\
    tor\x20is\x20divided\x20by\x20a\x20thousand\x20to\x20get\x20a\n\x20doubl\
    e.\x20That\x20is,\x20if\x20the\x20desired\x20factor\x20is\x201.9,\x20the\
    \x20runtime\x20value\x20should\n\x20be\x201900.\x20Defaults\x20to\x20190\
    0.\n\n\x11\n\x07\x04\x02\x03\x01\x02\x08\x04\x12\x06\xd4\x01\x04\xcc\x01\
    @\n\x0f\n\x07\x04\x02\x03\x01\x02\x08\x06\x12\x04\xd4\x01\x04\x1f\n\x0f\
    \n\x07\x04\x02\x03\x01\x02\x08\x01\x12\x04\xd4\x01\x209\n\x0f\n\x07\x04\
    \x02\x03\x01\x02\x08\x03\x12\x04\xd4\x01<=\n\x0c\n\x04\x04\x02\x02\x12\
    \x12\x04\xd6\x01\x02*\n\x0f\n\x05\x04\x02\x02\x12\x04\x12\x06\xd6\x01\
    \x02\xd5\x01\x03\n\r\n\x05\x04\x02\x02\x12\x06\x12\x04\xd6\x01\x02\x12\n\
    \r\n\x05\x04\x02\x02\x12\x01\x12\x04\xd6\x01\x13$\n\r\n\x05\x04\x02\x02\
    \x12\x03\x12\x04\xd6\x01')\n\xcd\x05\n\x04\x04\x02\x02\x13\x12\x04\xe3\
    \x01\x021\x1a\xbe\x05\x20The\x20interval\x20for\x20removing\x20stale\x20\
    hosts\x20from\x20a\x20cluster\x20type\n\x20ORIGINAL_DST.\x20\x20Hosts\
    \x20are\x20considered\x20stale\x20if\x20they\x20have\x20not\x20been\x20u\
    sed\n\x20as\x20upstream\x20destinations\x20during\x20this\x20interval.\
    \x20\x20New\x20hosts\x20are\x20added\n\x20to\x20original\x20destination\
    \x20clusters\x20on\x20demand\x20as\x20new\x20connections\x20are\n\x20red\
    irected\x20to\x20Envoy,\x20causing\x20the\x20number\x20of\x20hosts\x20in\
    \x20the\x20cluster\x20to\n\x20grow\x20over\x20time.\x20\x20Hosts\x20that\
    \x20are\x20not\x20stale\x20(they\x20are\x20actively\x20used\x20as\n\x20d\
    estinations)\x20are\x20kept\x20in\x20the\x20cluster,\x20which\x20allows\
    \x20connections\x20to\n\x20them\x20remain\x20open,\x20saving\x20the\x20l\
    atency\x20that\x20would\x20otherwise\x20be\x20spent\n\x20on\x20opening\
    \x20new\x20connections.\x20\x20If\x20this\x20setting\x20is\x20not\x20spe\
    cified,\x20the\n\x20value\x20defaults\x20to\x205000ms.\x20For\x20cluster\
    \x20types\x20other\x20than\x20ORIGINAL_DST\n\x20this\x20setting\x20is\
    \x20ignored.\n\n\x0f\n\x05\x04\x02\x02\x13\x04\x12\x06\xe3\x01\x02\xd6\
    \x01*\n\r\n\x05\x04\x02\x02\x13\x06\x12\x04\xe3\x01\x02\x1a\n\r\n\x05\
    \x04\x02\x02\x13\x01\x12\x04\xe3\x01\x1b+\n\r\n\x05\x04\x02\x02\x13\x03\
    \x12\x04\xe3\x01.0\n\xdc\x01\n\x04\x04\x02\x02\x14\x12\x04\xe8\x01\x02'\
    \x1a\xcd\x01\x20Optional\x20configuration\x20used\x20to\x20bind\x20newly\
    \x20established\x20upstream\x20connections.\n\x20This\x20overrides\x20an\
    y\x20bind_config\x20specified\x20in\x20the\x20bootstrap\x20proto.\n\x20I\
    f\x20the\x20addres\x20and\x20port\x20are\x20empty,\x20no\x20bind\x20will\
    \x20be\x20performed.\n\n\x0f\n\x05\x04\x02\x02\x14\x04\x12\x06\xe8\x01\
    \x02\xe3\x011\n\r\n\x05\x04\x02\x02\x14\x06\x12\x04\xe8\x01\x02\x0c\n\r\
    \n\x05\x04\x02\x02\x14\x01\x12\x04\xe8\x01\r!\n\r\n\x05\x04\x02\x02\x14\
    \x03\x12\x04\xe8\x01$&\n\xa2\x01\n\x04\x04\x02\x03\x02\x12\x06\xec\x01\
    \x02\x92\x02\x03\x1a\x91\x01\x20Optionally\x20divide\x20the\x20endpoints\
    \x20in\x20this\x20cluster\x20into\x20subsets\x20defined\x20by\n\x20endpo\
    int\x20metadata\x20and\x20selected\x20by\x20route\x20and\x20weighted\x20\
    cluster\x20metadata.\n\n\r\n\x05\x04\x02\x03\x02\x01\x12\x04\xec\x01\n\
    \x18\n\x89\x04\n\x06\x04\x02\x03\x02\x04\0\x12\x06\xf4\x01\x04\xf8\x01\
    \x05\x1a\xf6\x03\x20The\x20behavior\x20used\x20when\x20no\x20endpoint\
    \x20subset\x20matches\x20the\x20selected\x20route's\n\x20metadata.\x20Th\
    e\x20options\x20are\x20NO_FALLBACK,\x20ANY_ENDPOINT,\x20or\x20DEFAULT_SU\
    BSET.\n\x20The\x20value\x20defaults\x20to\x20NO_FALLBACK.\x20If\x20NO_FA\
    LLBACK\x20is\x20selected,\x20a\x20result\n\x20equivalent\x20to\x20no\x20\
    healthy\x20hosts\x20is\x20reported.\x20If\x20ANY_ENDPOINT\x20is\x20selec\
    ted,\n\x20any\x20cluster\x20endpoint\x20may\x20be\x20returned\x20(subjec\
    t\x20to\x20policy,\x20health\x20checks,\n\x20etc).\x20If\x20DEFAULT_SUBS\
    ET\x20is\x20selected,\x20load\x20balancing\x20is\x20performed\x20over\
    \x20the\n\x20endpoints\x20matching\x20the\x20values\x20from\x20the\x20de\
    fault_subset\x20field.\n\n\x0f\n\x07\x04\x02\x03\x02\x04\0\x01\x12\x04\
    \xf4\x01\t\x1f\n\x10\n\x08\x04\x02\x03\x02\x04\0\x02\0\x12\x04\xf5\x01\
    \x06\x16\n\x11\n\t\x04\x02\x03\x02\x04\0\x02\0\x01\x12\x04\xf5\x01\x06\
    \x11\n\x11\n\t\x04\x02\x03\x02\x04\0\x02\0\x02\x12\x04\xf5\x01\x14\x15\n\
    \x10\n\x08\x04\x02\x03\x02\x04\0\x02\x01\x12\x04\xf6\x01\x06\x17\n\x11\n\
    \t\x04\x02\x03\x02\x04\0\x02\x01\x01\x12\x04\xf6\x01\x06\x12\n\x11\n\t\
    \x04\x02\x03\x02\x04\0\x02\x01\x02\x12\x04\xf6\x01\x15\x16\n\x10\n\x08\
    \x04\x02\x03\x02\x04\0\x02\x02\x12\x04\xf7\x01\x06\x19\n\x11\n\t\x04\x02\
    \x03\x02\x04\0\x02\x02\x01\x12\x04\xf7\x01\x06\x14\n\x11\n\t\x04\x02\x03\
    \x02\x04\0\x02\x02\x02\x12\x04\xf7\x01\x17\x18\n\x0e\n\x06\x04\x02\x03\
    \x02\x02\0\x12\x04\xf9\x01\x04/\n\x11\n\x07\x04\x02\x03\x02\x02\0\x04\
    \x12\x06\xf9\x01\x04\xf8\x01\x05\n\x0f\n\x07\x04\x02\x03\x02\x02\0\x06\
    \x12\x04\xf9\x01\x04\x1a\n\x0f\n\x07\x04\x02\x03\x02\x02\0\x01\x12\x04\
    \xf9\x01\x1b*\n\x0f\n\x07\x04\x02\x03\x02\x02\0\x03\x12\x04\xf9\x01-.\n\
    \xd7\x02\n\x06\x04\x02\x03\x02\x02\x01\x12\x04\x80\x02\x04.\x1a\xc6\x02\
    \x20Specifies\x20the\x20default\x20subset\x20of\x20endpoints\x20used\x20\
    during\x20fallback\x20if\n\x20fallback_policy\x20is\x20DEFAULT_SUBSET.\
    \x20Each\x20field\x20in\x20default_subset\x20is\n\x20compared\x20to\x20t\
    he\x20matching\x20LbEndpoint.Metadata\x20under\x20the\x20\"envoy.lb\"\n\
    \x20namespace.\x20It\x20is\x20valid\x20for\x20no\x20hosts\x20to\x20match\
    ,\x20in\x20which\x20case\x20the\x20behavior\n\x20is\x20the\x20same\x20as\
    \x20a\x20fallback_policy\x20of\x20NO_FALLBACK.\n\n\x11\n\x07\x04\x02\x03\
    \x02\x02\x01\x04\x12\x06\x80\x02\x04\xf9\x01/\n\x0f\n\x07\x04\x02\x03\
    \x02\x02\x01\x06\x12\x04\x80\x02\x04\x1a\n\x0f\n\x07\x04\x02\x03\x02\x02\
    \x01\x01\x12\x04\x80\x02\x1b)\n\x0f\n\x07\x04\x02\x03\x02\x02\x01\x03\
    \x12\x04\x80\x02,-\n\xf7\x04\n\x06\x04\x02\x03\x02\x03\0\x12\x06\x8e\x02\
    \x04\x90\x02\x05\x1a\xe4\x04\x20Specifications\x20for\x20subsets.\x20For\
    \x20each\x20entry,\x20LbEndpoint.Metadata's\n\x20\"envoy.lb\"\x20namespa\
    ce\x20is\x20traversed\x20and\x20a\x20subset\x20is\x20created\x20for\x20e\
    ach\x20unique\n\x20combination\x20of\x20key\x20and\x20value.\x20For\x20e\
    xample:\n\x20{\x20\"subset_keys\":\x20[\n\x20\x20\x20\x20\x20{\x20\"keys\
    \":\x20[\x20\"version\"\x20]\x20},\n\x20\x20\x20\x20\x20{\x20\"keys\":\
    \x20[\x20\"stage\",\x20\"hardware_type\"\x20]\x20}\n\x20]}\n\x20A\x20sub\
    set\x20is\x20matched\x20when\x20the\x20metadata\x20from\x20the\x20select\
    ed\x20route\x20and\n\x20weighted\x20cluster\x20contains\x20the\x20keys\
    \x20and\x20values\x20from\x20the\x20subset's\n\x20metadata.\x20Extra\x20\
    keys\x20in\x20the\x20route\x20metadata\x20are\x20ignored\x20when\x20comp\
    aring\x20the\n\x20subset's\x20metadata.\x20Subsets\x20may\x20overlap.\
    \x20In\x20the\x20case\x20of\x20overlapping\n\x20subsets,\x20the\x20first\
    \x20matching\x20subset\x20is\x20selected.\n\n\x0f\n\x07\x04\x02\x03\x02\
    \x03\0\x01\x12\x04\x8e\x02\x0c\x18\n\x10\n\x08\x04\x02\x03\x02\x03\0\x02\
    \0\x12\x04\x8f\x02\x06\x1f\n\x11\n\t\x04\x02\x03\x02\x03\0\x02\0\x04\x12\
    \x04\x8f\x02\x06\x0e\n\x11\n\t\x04\x02\x03\x02\x03\0\x02\0\x05\x12\x04\
    \x8f\x02\x0f\x15\n\x11\n\t\x04\x02\x03\x02\x03\0\x02\0\x01\x12\x04\x8f\
    \x02\x16\x1a\n\x11\n\t\x04\x02\x03\x02\x03\0\x02\0\x03\x12\x04\x8f\x02\
    \x1d\x1e\n\x0e\n\x06\x04\x02\x03\x02\x02\x02\x12\x04\x91\x02\x04*\n\x0f\
    \n\x07\x04\x02\x03\x02\x02\x02\x04\x12\x04\x91\x02\x04\x0c\n\x0f\n\x07\
    \x04\x02\x03\x02\x02\x02\x06\x12\x04\x91\x02\r\x19\n\x0f\n\x07\x04\x02\
    \x03\x02\x02\x02\x01\x12\x04\x91\x02\x1a%\n\x0f\n\x07\x04\x02\x03\x02\
    \x02\x02\x03\x12\x04\x91\x02()\n\x0c\n\x04\x04\x02\x02\x15\x12\x04\x93\
    \x02\x02'\n\x0f\n\x05\x04\x02\x02\x15\x04\x12\x06\x93\x02\x02\x92\x02\
    \x03\n\r\n\x05\x04\x02\x02\x15\x06\x12\x04\x93\x02\x02\x10\n\r\n\x05\x04\
    \x02\x02\x15\x01\x12\x04\x93\x02\x11!\n\r\n\x05\x04\x02\x02\x15\x03\x12\
    \x04\x93\x02$&b\x06proto3\
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
