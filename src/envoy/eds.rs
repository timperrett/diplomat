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
pub struct LbEndpoint {
    // message fields
    pub endpoint: ::protobuf::SingularPtrField<super::base::Endpoint>,
    pub health_status: super::health_check::HealthStatus,
    pub metadata: ::protobuf::SingularPtrField<super::base::Metadata>,
    pub load_balancing_weight: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LbEndpoint {}

impl LbEndpoint {
    pub fn new() -> LbEndpoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LbEndpoint {
        static mut instance: ::protobuf::lazy::Lazy<LbEndpoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LbEndpoint,
        };
        unsafe {
            instance.get(LbEndpoint::new)
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

    // .envoy.api.v2.Metadata metadata = 3;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: super::base::Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut super::base::Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> super::base::Metadata {
        self.metadata.take().unwrap_or_else(|| super::base::Metadata::new())
    }

    pub fn get_metadata(&self) -> &super::base::Metadata {
        self.metadata.as_ref().unwrap_or_else(|| super::base::Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Metadata> {
        &mut self.metadata
    }

    // .google.protobuf.UInt32Value load_balancing_weight = 4;

    pub fn clear_load_balancing_weight(&mut self) {
        self.load_balancing_weight.clear();
    }

    pub fn has_load_balancing_weight(&self) -> bool {
        self.load_balancing_weight.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_balancing_weight(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.load_balancing_weight = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_balancing_weight(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.load_balancing_weight.is_none() {
            self.load_balancing_weight.set_default();
        }
        self.load_balancing_weight.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_balancing_weight(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.load_balancing_weight.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_load_balancing_weight(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.load_balancing_weight.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_load_balancing_weight_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.load_balancing_weight
    }

    fn mut_load_balancing_weight_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.load_balancing_weight
    }
}

impl ::protobuf::Message for LbEndpoint {
    fn is_initialized(&self) -> bool {
        for v in &self.endpoint {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.load_balancing_weight {
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
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_balancing_weight)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.load_balancing_weight.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.load_balancing_weight.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LbEndpoint {
    fn new() -> LbEndpoint {
        LbEndpoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<LbEndpoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Endpoint>>(
                    "endpoint",
                    LbEndpoint::get_endpoint_for_reflect,
                    LbEndpoint::mut_endpoint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::health_check::HealthStatus>>(
                    "health_status",
                    LbEndpoint::get_health_status_for_reflect,
                    LbEndpoint::mut_health_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata",
                    LbEndpoint::get_metadata_for_reflect,
                    LbEndpoint::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "load_balancing_weight",
                    LbEndpoint::get_load_balancing_weight_for_reflect,
                    LbEndpoint::mut_load_balancing_weight_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LbEndpoint>(
                    "LbEndpoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LbEndpoint {
    fn clear(&mut self) {
        self.clear_endpoint();
        self.clear_health_status();
        self.clear_metadata();
        self.clear_load_balancing_weight();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LbEndpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LbEndpoint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocalityLbEndpoints {
    // message fields
    pub locality: ::protobuf::SingularPtrField<super::base::Locality>,
    pub lb_endpoints: ::protobuf::RepeatedField<LbEndpoint>,
    pub load_balancing_weight: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocalityLbEndpoints {}

impl LocalityLbEndpoints {
    pub fn new() -> LocalityLbEndpoints {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocalityLbEndpoints {
        static mut instance: ::protobuf::lazy::Lazy<LocalityLbEndpoints> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocalityLbEndpoints,
        };
        unsafe {
            instance.get(LocalityLbEndpoints::new)
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

    // repeated .envoy.api.v2.LbEndpoint lb_endpoints = 2;

    pub fn clear_lb_endpoints(&mut self) {
        self.lb_endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_lb_endpoints(&mut self, v: ::protobuf::RepeatedField<LbEndpoint>) {
        self.lb_endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lb_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<LbEndpoint> {
        &mut self.lb_endpoints
    }

    // Take field
    pub fn take_lb_endpoints(&mut self) -> ::protobuf::RepeatedField<LbEndpoint> {
        ::std::mem::replace(&mut self.lb_endpoints, ::protobuf::RepeatedField::new())
    }

    pub fn get_lb_endpoints(&self) -> &[LbEndpoint] {
        &self.lb_endpoints
    }

    fn get_lb_endpoints_for_reflect(&self) -> &::protobuf::RepeatedField<LbEndpoint> {
        &self.lb_endpoints
    }

    fn mut_lb_endpoints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LbEndpoint> {
        &mut self.lb_endpoints
    }

    // .google.protobuf.UInt32Value load_balancing_weight = 3;

    pub fn clear_load_balancing_weight(&mut self) {
        self.load_balancing_weight.clear();
    }

    pub fn has_load_balancing_weight(&self) -> bool {
        self.load_balancing_weight.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_balancing_weight(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.load_balancing_weight = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_balancing_weight(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.load_balancing_weight.is_none() {
            self.load_balancing_weight.set_default();
        }
        self.load_balancing_weight.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_balancing_weight(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.load_balancing_weight.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_load_balancing_weight(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.load_balancing_weight.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_load_balancing_weight_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.load_balancing_weight
    }

    fn mut_load_balancing_weight_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.load_balancing_weight
    }
}

impl ::protobuf::Message for LocalityLbEndpoints {
    fn is_initialized(&self) -> bool {
        for v in &self.locality {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lb_endpoints {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.load_balancing_weight {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lb_endpoints)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_balancing_weight)?;
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
        for value in &self.lb_endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.load_balancing_weight.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
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
        for v in &self.lb_endpoints {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.load_balancing_weight.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LocalityLbEndpoints {
    fn new() -> LocalityLbEndpoints {
        LocalityLbEndpoints::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocalityLbEndpoints>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Locality>>(
                    "locality",
                    LocalityLbEndpoints::get_locality_for_reflect,
                    LocalityLbEndpoints::mut_locality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LbEndpoint>>(
                    "lb_endpoints",
                    LocalityLbEndpoints::get_lb_endpoints_for_reflect,
                    LocalityLbEndpoints::mut_lb_endpoints_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "load_balancing_weight",
                    LocalityLbEndpoints::get_load_balancing_weight_for_reflect,
                    LocalityLbEndpoints::mut_load_balancing_weight_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocalityLbEndpoints>(
                    "LocalityLbEndpoints",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocalityLbEndpoints {
    fn clear(&mut self) {
        self.clear_locality();
        self.clear_lb_endpoints();
        self.clear_load_balancing_weight();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocalityLbEndpoints {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocalityLbEndpoints {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EndpointLoadMetricStats {
    // message fields
    pub metric_name: ::std::string::String,
    pub num_requests_finished_with_metric: u64,
    pub total_metric_value: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EndpointLoadMetricStats {}

impl EndpointLoadMetricStats {
    pub fn new() -> EndpointLoadMetricStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndpointLoadMetricStats {
        static mut instance: ::protobuf::lazy::Lazy<EndpointLoadMetricStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndpointLoadMetricStats,
        };
        unsafe {
            instance.get(EndpointLoadMetricStats::new)
        }
    }

    // string metric_name = 1;

    pub fn clear_metric_name(&mut self) {
        self.metric_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_metric_name(&mut self, v: ::std::string::String) {
        self.metric_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metric_name(&mut self) -> &mut ::std::string::String {
        &mut self.metric_name
    }

    // Take field
    pub fn take_metric_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.metric_name, ::std::string::String::new())
    }

    pub fn get_metric_name(&self) -> &str {
        &self.metric_name
    }

    fn get_metric_name_for_reflect(&self) -> &::std::string::String {
        &self.metric_name
    }

    fn mut_metric_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.metric_name
    }

    // uint64 num_requests_finished_with_metric = 2;

    pub fn clear_num_requests_finished_with_metric(&mut self) {
        self.num_requests_finished_with_metric = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_requests_finished_with_metric(&mut self, v: u64) {
        self.num_requests_finished_with_metric = v;
    }

    pub fn get_num_requests_finished_with_metric(&self) -> u64 {
        self.num_requests_finished_with_metric
    }

    fn get_num_requests_finished_with_metric_for_reflect(&self) -> &u64 {
        &self.num_requests_finished_with_metric
    }

    fn mut_num_requests_finished_with_metric_for_reflect(&mut self) -> &mut u64 {
        &mut self.num_requests_finished_with_metric
    }

    // double total_metric_value = 3;

    pub fn clear_total_metric_value(&mut self) {
        self.total_metric_value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_total_metric_value(&mut self, v: f64) {
        self.total_metric_value = v;
    }

    pub fn get_total_metric_value(&self) -> f64 {
        self.total_metric_value
    }

    fn get_total_metric_value_for_reflect(&self) -> &f64 {
        &self.total_metric_value
    }

    fn mut_total_metric_value_for_reflect(&mut self) -> &mut f64 {
        &mut self.total_metric_value
    }
}

impl ::protobuf::Message for EndpointLoadMetricStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.metric_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.num_requests_finished_with_metric = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.total_metric_value = tmp;
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
        if !self.metric_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.metric_name);
        }
        if self.num_requests_finished_with_metric != 0 {
            my_size += ::protobuf::rt::value_size(2, self.num_requests_finished_with_metric, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_metric_value != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.metric_name.is_empty() {
            os.write_string(1, &self.metric_name)?;
        }
        if self.num_requests_finished_with_metric != 0 {
            os.write_uint64(2, self.num_requests_finished_with_metric)?;
        }
        if self.total_metric_value != 0. {
            os.write_double(3, self.total_metric_value)?;
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

impl ::protobuf::MessageStatic for EndpointLoadMetricStats {
    fn new() -> EndpointLoadMetricStats {
        EndpointLoadMetricStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndpointLoadMetricStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metric_name",
                    EndpointLoadMetricStats::get_metric_name_for_reflect,
                    EndpointLoadMetricStats::mut_metric_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "num_requests_finished_with_metric",
                    EndpointLoadMetricStats::get_num_requests_finished_with_metric_for_reflect,
                    EndpointLoadMetricStats::mut_num_requests_finished_with_metric_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "total_metric_value",
                    EndpointLoadMetricStats::get_total_metric_value_for_reflect,
                    EndpointLoadMetricStats::mut_total_metric_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndpointLoadMetricStats>(
                    "EndpointLoadMetricStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndpointLoadMetricStats {
    fn clear(&mut self) {
        self.clear_metric_name();
        self.clear_num_requests_finished_with_metric();
        self.clear_total_metric_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndpointLoadMetricStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndpointLoadMetricStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpstreamLocalityStats {
    // message fields
    pub locality: ::protobuf::SingularPtrField<super::base::Locality>,
    pub total_successful_requests: u64,
    pub total_requests_in_progress: u64,
    pub total_error_requests: u64,
    pub total_dropped_requests: u64,
    pub load_metric_stats: ::protobuf::RepeatedField<EndpointLoadMetricStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpstreamLocalityStats {}

impl UpstreamLocalityStats {
    pub fn new() -> UpstreamLocalityStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpstreamLocalityStats {
        static mut instance: ::protobuf::lazy::Lazy<UpstreamLocalityStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpstreamLocalityStats,
        };
        unsafe {
            instance.get(UpstreamLocalityStats::new)
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

    // uint64 total_successful_requests = 2;

    pub fn clear_total_successful_requests(&mut self) {
        self.total_successful_requests = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_successful_requests(&mut self, v: u64) {
        self.total_successful_requests = v;
    }

    pub fn get_total_successful_requests(&self) -> u64 {
        self.total_successful_requests
    }

    fn get_total_successful_requests_for_reflect(&self) -> &u64 {
        &self.total_successful_requests
    }

    fn mut_total_successful_requests_for_reflect(&mut self) -> &mut u64 {
        &mut self.total_successful_requests
    }

    // uint64 total_requests_in_progress = 3;

    pub fn clear_total_requests_in_progress(&mut self) {
        self.total_requests_in_progress = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_requests_in_progress(&mut self, v: u64) {
        self.total_requests_in_progress = v;
    }

    pub fn get_total_requests_in_progress(&self) -> u64 {
        self.total_requests_in_progress
    }

    fn get_total_requests_in_progress_for_reflect(&self) -> &u64 {
        &self.total_requests_in_progress
    }

    fn mut_total_requests_in_progress_for_reflect(&mut self) -> &mut u64 {
        &mut self.total_requests_in_progress
    }

    // uint64 total_error_requests = 4;

    pub fn clear_total_error_requests(&mut self) {
        self.total_error_requests = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_error_requests(&mut self, v: u64) {
        self.total_error_requests = v;
    }

    pub fn get_total_error_requests(&self) -> u64 {
        self.total_error_requests
    }

    fn get_total_error_requests_for_reflect(&self) -> &u64 {
        &self.total_error_requests
    }

    fn mut_total_error_requests_for_reflect(&mut self) -> &mut u64 {
        &mut self.total_error_requests
    }

    // uint64 total_dropped_requests = 5;

    pub fn clear_total_dropped_requests(&mut self) {
        self.total_dropped_requests = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_dropped_requests(&mut self, v: u64) {
        self.total_dropped_requests = v;
    }

    pub fn get_total_dropped_requests(&self) -> u64 {
        self.total_dropped_requests
    }

    fn get_total_dropped_requests_for_reflect(&self) -> &u64 {
        &self.total_dropped_requests
    }

    fn mut_total_dropped_requests_for_reflect(&mut self) -> &mut u64 {
        &mut self.total_dropped_requests
    }

    // repeated .envoy.api.v2.EndpointLoadMetricStats load_metric_stats = 6;

    pub fn clear_load_metric_stats(&mut self) {
        self.load_metric_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_load_metric_stats(&mut self, v: ::protobuf::RepeatedField<EndpointLoadMetricStats>) {
        self.load_metric_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_load_metric_stats(&mut self) -> &mut ::protobuf::RepeatedField<EndpointLoadMetricStats> {
        &mut self.load_metric_stats
    }

    // Take field
    pub fn take_load_metric_stats(&mut self) -> ::protobuf::RepeatedField<EndpointLoadMetricStats> {
        ::std::mem::replace(&mut self.load_metric_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_load_metric_stats(&self) -> &[EndpointLoadMetricStats] {
        &self.load_metric_stats
    }

    fn get_load_metric_stats_for_reflect(&self) -> &::protobuf::RepeatedField<EndpointLoadMetricStats> {
        &self.load_metric_stats
    }

    fn mut_load_metric_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EndpointLoadMetricStats> {
        &mut self.load_metric_stats
    }
}

impl ::protobuf::Message for UpstreamLocalityStats {
    fn is_initialized(&self) -> bool {
        for v in &self.locality {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.load_metric_stats {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_successful_requests = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_requests_in_progress = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_error_requests = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_dropped_requests = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.load_metric_stats)?;
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
        if self.total_successful_requests != 0 {
            my_size += ::protobuf::rt::value_size(2, self.total_successful_requests, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_requests_in_progress != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total_requests_in_progress, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_error_requests != 0 {
            my_size += ::protobuf::rt::value_size(4, self.total_error_requests, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_dropped_requests != 0 {
            my_size += ::protobuf::rt::value_size(5, self.total_dropped_requests, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.load_metric_stats {
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
        if self.total_successful_requests != 0 {
            os.write_uint64(2, self.total_successful_requests)?;
        }
        if self.total_requests_in_progress != 0 {
            os.write_uint64(3, self.total_requests_in_progress)?;
        }
        if self.total_error_requests != 0 {
            os.write_uint64(4, self.total_error_requests)?;
        }
        if self.total_dropped_requests != 0 {
            os.write_uint64(5, self.total_dropped_requests)?;
        }
        for v in &self.load_metric_stats {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for UpstreamLocalityStats {
    fn new() -> UpstreamLocalityStats {
        UpstreamLocalityStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpstreamLocalityStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Locality>>(
                    "locality",
                    UpstreamLocalityStats::get_locality_for_reflect,
                    UpstreamLocalityStats::mut_locality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_successful_requests",
                    UpstreamLocalityStats::get_total_successful_requests_for_reflect,
                    UpstreamLocalityStats::mut_total_successful_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_requests_in_progress",
                    UpstreamLocalityStats::get_total_requests_in_progress_for_reflect,
                    UpstreamLocalityStats::mut_total_requests_in_progress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_error_requests",
                    UpstreamLocalityStats::get_total_error_requests_for_reflect,
                    UpstreamLocalityStats::mut_total_error_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_dropped_requests",
                    UpstreamLocalityStats::get_total_dropped_requests_for_reflect,
                    UpstreamLocalityStats::mut_total_dropped_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EndpointLoadMetricStats>>(
                    "load_metric_stats",
                    UpstreamLocalityStats::get_load_metric_stats_for_reflect,
                    UpstreamLocalityStats::mut_load_metric_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpstreamLocalityStats>(
                    "UpstreamLocalityStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpstreamLocalityStats {
    fn clear(&mut self) {
        self.clear_locality();
        self.clear_total_successful_requests();
        self.clear_total_requests_in_progress();
        self.clear_total_error_requests();
        self.clear_total_dropped_requests();
        self.clear_load_metric_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpstreamLocalityStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpstreamLocalityStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterStats {
    // message fields
    pub cluster_name: ::std::string::String,
    pub upstream_locality_stats: ::protobuf::RepeatedField<UpstreamLocalityStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterStats {}

impl ClusterStats {
    pub fn new() -> ClusterStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterStats {
        static mut instance: ::protobuf::lazy::Lazy<ClusterStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterStats,
        };
        unsafe {
            instance.get(ClusterStats::new)
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

    // repeated .envoy.api.v2.UpstreamLocalityStats upstream_locality_stats = 2;

    pub fn clear_upstream_locality_stats(&mut self) {
        self.upstream_locality_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_upstream_locality_stats(&mut self, v: ::protobuf::RepeatedField<UpstreamLocalityStats>) {
        self.upstream_locality_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_upstream_locality_stats(&mut self) -> &mut ::protobuf::RepeatedField<UpstreamLocalityStats> {
        &mut self.upstream_locality_stats
    }

    // Take field
    pub fn take_upstream_locality_stats(&mut self) -> ::protobuf::RepeatedField<UpstreamLocalityStats> {
        ::std::mem::replace(&mut self.upstream_locality_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_upstream_locality_stats(&self) -> &[UpstreamLocalityStats] {
        &self.upstream_locality_stats
    }

    fn get_upstream_locality_stats_for_reflect(&self) -> &::protobuf::RepeatedField<UpstreamLocalityStats> {
        &self.upstream_locality_stats
    }

    fn mut_upstream_locality_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UpstreamLocalityStats> {
        &mut self.upstream_locality_stats
    }
}

impl ::protobuf::Message for ClusterStats {
    fn is_initialized(&self) -> bool {
        for v in &self.upstream_locality_stats {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.upstream_locality_stats)?;
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
        for value in &self.upstream_locality_stats {
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
        for v in &self.upstream_locality_stats {
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

impl ::protobuf::MessageStatic for ClusterStats {
    fn new() -> ClusterStats {
        ClusterStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster_name",
                    ClusterStats::get_cluster_name_for_reflect,
                    ClusterStats::mut_cluster_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpstreamLocalityStats>>(
                    "upstream_locality_stats",
                    ClusterStats::get_upstream_locality_stats_for_reflect,
                    ClusterStats::mut_upstream_locality_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterStats>(
                    "ClusterStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterStats {
    fn clear(&mut self) {
        self.clear_cluster_name();
        self.clear_upstream_locality_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadStatsRequest {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub cluster_stats: ::protobuf::RepeatedField<ClusterStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoadStatsRequest {}

impl LoadStatsRequest {
    pub fn new() -> LoadStatsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoadStatsRequest {
        static mut instance: ::protobuf::lazy::Lazy<LoadStatsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoadStatsRequest,
        };
        unsafe {
            instance.get(LoadStatsRequest::new)
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

    // repeated .envoy.api.v2.ClusterStats cluster_stats = 2;

    pub fn clear_cluster_stats(&mut self) {
        self.cluster_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster_stats(&mut self, v: ::protobuf::RepeatedField<ClusterStats>) {
        self.cluster_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cluster_stats(&mut self) -> &mut ::protobuf::RepeatedField<ClusterStats> {
        &mut self.cluster_stats
    }

    // Take field
    pub fn take_cluster_stats(&mut self) -> ::protobuf::RepeatedField<ClusterStats> {
        ::std::mem::replace(&mut self.cluster_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_cluster_stats(&self) -> &[ClusterStats] {
        &self.cluster_stats
    }

    fn get_cluster_stats_for_reflect(&self) -> &::protobuf::RepeatedField<ClusterStats> {
        &self.cluster_stats
    }

    fn mut_cluster_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClusterStats> {
        &mut self.cluster_stats
    }
}

impl ::protobuf::Message for LoadStatsRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cluster_stats {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cluster_stats)?;
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
        for value in &self.cluster_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.cluster_stats {
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

impl ::protobuf::MessageStatic for LoadStatsRequest {
    fn new() -> LoadStatsRequest {
        LoadStatsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoadStatsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                    "node",
                    LoadStatsRequest::get_node_for_reflect,
                    LoadStatsRequest::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterStats>>(
                    "cluster_stats",
                    LoadStatsRequest::get_cluster_stats_for_reflect,
                    LoadStatsRequest::mut_cluster_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoadStatsRequest>(
                    "LoadStatsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoadStatsRequest {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_cluster_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadStatsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadStatsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterLoadAssignment {
    // message fields
    pub cluster_name: ::std::string::String,
    pub endpoints: ::protobuf::RepeatedField<LocalityLbEndpoints>,
    pub failover_endpoints: ::protobuf::RepeatedField<LocalityLbEndpoints>,
    pub policy: ::protobuf::SingularPtrField<ClusterLoadAssignment_Policy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterLoadAssignment {}

impl ClusterLoadAssignment {
    pub fn new() -> ClusterLoadAssignment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterLoadAssignment {
        static mut instance: ::protobuf::lazy::Lazy<ClusterLoadAssignment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterLoadAssignment,
        };
        unsafe {
            instance.get(ClusterLoadAssignment::new)
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

    // repeated .envoy.api.v2.LocalityLbEndpoints endpoints = 2;

    pub fn clear_endpoints(&mut self) {
        self.endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_endpoints(&mut self, v: ::protobuf::RepeatedField<LocalityLbEndpoints>) {
        self.endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<LocalityLbEndpoints> {
        &mut self.endpoints
    }

    // Take field
    pub fn take_endpoints(&mut self) -> ::protobuf::RepeatedField<LocalityLbEndpoints> {
        ::std::mem::replace(&mut self.endpoints, ::protobuf::RepeatedField::new())
    }

    pub fn get_endpoints(&self) -> &[LocalityLbEndpoints] {
        &self.endpoints
    }

    fn get_endpoints_for_reflect(&self) -> &::protobuf::RepeatedField<LocalityLbEndpoints> {
        &self.endpoints
    }

    fn mut_endpoints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LocalityLbEndpoints> {
        &mut self.endpoints
    }

    // repeated .envoy.api.v2.LocalityLbEndpoints failover_endpoints = 3;

    pub fn clear_failover_endpoints(&mut self) {
        self.failover_endpoints.clear();
    }

    // Param is passed by value, moved
    pub fn set_failover_endpoints(&mut self, v: ::protobuf::RepeatedField<LocalityLbEndpoints>) {
        self.failover_endpoints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_failover_endpoints(&mut self) -> &mut ::protobuf::RepeatedField<LocalityLbEndpoints> {
        &mut self.failover_endpoints
    }

    // Take field
    pub fn take_failover_endpoints(&mut self) -> ::protobuf::RepeatedField<LocalityLbEndpoints> {
        ::std::mem::replace(&mut self.failover_endpoints, ::protobuf::RepeatedField::new())
    }

    pub fn get_failover_endpoints(&self) -> &[LocalityLbEndpoints] {
        &self.failover_endpoints
    }

    fn get_failover_endpoints_for_reflect(&self) -> &::protobuf::RepeatedField<LocalityLbEndpoints> {
        &self.failover_endpoints
    }

    fn mut_failover_endpoints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LocalityLbEndpoints> {
        &mut self.failover_endpoints
    }

    // .envoy.api.v2.ClusterLoadAssignment.Policy policy = 4;

    pub fn clear_policy(&mut self) {
        self.policy.clear();
    }

    pub fn has_policy(&self) -> bool {
        self.policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policy(&mut self, v: ClusterLoadAssignment_Policy) {
        self.policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_policy(&mut self) -> &mut ClusterLoadAssignment_Policy {
        if self.policy.is_none() {
            self.policy.set_default();
        }
        self.policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_policy(&mut self) -> ClusterLoadAssignment_Policy {
        self.policy.take().unwrap_or_else(|| ClusterLoadAssignment_Policy::new())
    }

    pub fn get_policy(&self) -> &ClusterLoadAssignment_Policy {
        self.policy.as_ref().unwrap_or_else(|| ClusterLoadAssignment_Policy::default_instance())
    }

    fn get_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<ClusterLoadAssignment_Policy> {
        &self.policy
    }

    fn mut_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClusterLoadAssignment_Policy> {
        &mut self.policy
    }
}

impl ::protobuf::Message for ClusterLoadAssignment {
    fn is_initialized(&self) -> bool {
        for v in &self.endpoints {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.failover_endpoints {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.policy {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.endpoints)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.failover_endpoints)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.policy)?;
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
        for value in &self.endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.failover_endpoints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cluster_name.is_empty() {
            os.write_string(1, &self.cluster_name)?;
        }
        for v in &self.endpoints {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.failover_endpoints {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.policy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClusterLoadAssignment {
    fn new() -> ClusterLoadAssignment {
        ClusterLoadAssignment::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterLoadAssignment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster_name",
                    ClusterLoadAssignment::get_cluster_name_for_reflect,
                    ClusterLoadAssignment::mut_cluster_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocalityLbEndpoints>>(
                    "endpoints",
                    ClusterLoadAssignment::get_endpoints_for_reflect,
                    ClusterLoadAssignment::mut_endpoints_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocalityLbEndpoints>>(
                    "failover_endpoints",
                    ClusterLoadAssignment::get_failover_endpoints_for_reflect,
                    ClusterLoadAssignment::mut_failover_endpoints_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterLoadAssignment_Policy>>(
                    "policy",
                    ClusterLoadAssignment::get_policy_for_reflect,
                    ClusterLoadAssignment::mut_policy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterLoadAssignment>(
                    "ClusterLoadAssignment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterLoadAssignment {
    fn clear(&mut self) {
        self.clear_cluster_name();
        self.clear_endpoints();
        self.clear_failover_endpoints();
        self.clear_policy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterLoadAssignment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterLoadAssignment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterLoadAssignment_Policy {
    // message fields
    pub drop_overload: f64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterLoadAssignment_Policy {}

impl ClusterLoadAssignment_Policy {
    pub fn new() -> ClusterLoadAssignment_Policy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterLoadAssignment_Policy {
        static mut instance: ::protobuf::lazy::Lazy<ClusterLoadAssignment_Policy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterLoadAssignment_Policy,
        };
        unsafe {
            instance.get(ClusterLoadAssignment_Policy::new)
        }
    }

    // double drop_overload = 1;

    pub fn clear_drop_overload(&mut self) {
        self.drop_overload = 0.;
    }

    // Param is passed by value, moved
    pub fn set_drop_overload(&mut self, v: f64) {
        self.drop_overload = v;
    }

    pub fn get_drop_overload(&self) -> f64 {
        self.drop_overload
    }

    fn get_drop_overload_for_reflect(&self) -> &f64 {
        &self.drop_overload
    }

    fn mut_drop_overload_for_reflect(&mut self) -> &mut f64 {
        &mut self.drop_overload
    }
}

impl ::protobuf::Message for ClusterLoadAssignment_Policy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.drop_overload = tmp;
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
        if self.drop_overload != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.drop_overload != 0. {
            os.write_double(1, self.drop_overload)?;
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

impl ::protobuf::MessageStatic for ClusterLoadAssignment_Policy {
    fn new() -> ClusterLoadAssignment_Policy {
        ClusterLoadAssignment_Policy::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterLoadAssignment_Policy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "drop_overload",
                    ClusterLoadAssignment_Policy::get_drop_overload_for_reflect,
                    ClusterLoadAssignment_Policy::mut_drop_overload_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterLoadAssignment_Policy>(
                    "ClusterLoadAssignment_Policy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterLoadAssignment_Policy {
    fn clear(&mut self) {
        self.clear_drop_overload();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterLoadAssignment_Policy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterLoadAssignment_Policy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadStatsResponse {
    // message fields
    pub clusters: ::protobuf::RepeatedField<::std::string::String>,
    pub load_reporting_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoadStatsResponse {}

impl LoadStatsResponse {
    pub fn new() -> LoadStatsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoadStatsResponse {
        static mut instance: ::protobuf::lazy::Lazy<LoadStatsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoadStatsResponse,
        };
        unsafe {
            instance.get(LoadStatsResponse::new)
        }
    }

    // repeated string clusters = 1;

    pub fn clear_clusters(&mut self) {
        self.clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_clusters(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clusters(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.clusters
    }

    // Take field
    pub fn take_clusters(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_clusters(&self) -> &[::std::string::String] {
        &self.clusters
    }

    fn get_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.clusters
    }

    fn mut_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.clusters
    }

    // .google.protobuf.Duration load_reporting_interval = 2;

    pub fn clear_load_reporting_interval(&mut self) {
        self.load_reporting_interval.clear();
    }

    pub fn has_load_reporting_interval(&self) -> bool {
        self.load_reporting_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_reporting_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.load_reporting_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_reporting_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.load_reporting_interval.is_none() {
            self.load_reporting_interval.set_default();
        }
        self.load_reporting_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_reporting_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.load_reporting_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_load_reporting_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.load_reporting_interval.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_load_reporting_interval_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.load_reporting_interval
    }

    fn mut_load_reporting_interval_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.load_reporting_interval
    }
}

impl ::protobuf::Message for LoadStatsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.load_reporting_interval {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.clusters)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_reporting_interval)?;
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
        for value in &self.clusters {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.load_reporting_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.clusters {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.load_reporting_interval.as_ref() {
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

impl ::protobuf::MessageStatic for LoadStatsResponse {
    fn new() -> LoadStatsResponse {
        LoadStatsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoadStatsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clusters",
                    LoadStatsResponse::get_clusters_for_reflect,
                    LoadStatsResponse::mut_clusters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "load_reporting_interval",
                    LoadStatsResponse::get_load_reporting_interval_for_reflect,
                    LoadStatsResponse::mut_load_reporting_interval_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoadStatsResponse>(
                    "LoadStatsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoadStatsResponse {
    fn clear(&mut self) {
        self.clear_clusters();
        self.clear_load_reporting_interval();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/eds.proto\x12\x0cenvoy.api.v2\x1a\x0eapi/base.proto\x1a\x13api/d\
    iscovery.proto\x1a\x16api/health_check.proto\x1a\x1cgoogle/api/annotatio\
    ns.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\x1egoogle/protobuf/wr\
    appers.proto\"\x87\x02\n\nLbEndpoint\x122\n\x08endpoint\x18\x01\x20\x01(\
    \x0b2\x16.envoy.api.v2.EndpointR\x08endpoint\x12?\n\rhealth_status\x18\
    \x02\x20\x01(\x0e2\x1a.envoy.api.v2.HealthStatusR\x0chealthStatus\x122\n\
    \x08metadata\x18\x03\x20\x01(\x0b2\x16.envoy.api.v2.MetadataR\x08metadat\
    a\x12P\n\x15load_balancing_weight\x18\x04\x20\x01(\x0b2\x1c.google.proto\
    buf.UInt32ValueR\x13loadBalancingWeight\"\xd8\x01\n\x13LocalityLbEndpoin\
    ts\x122\n\x08locality\x18\x01\x20\x01(\x0b2\x16.envoy.api.v2.LocalityR\
    \x08locality\x12;\n\x0clb_endpoints\x18\x02\x20\x03(\x0b2\x18.envoy.api.\
    v2.LbEndpointR\x0blbEndpoints\x12P\n\x15load_balancing_weight\x18\x03\
    \x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x13loadBalancingWeight\"\
    \xb2\x01\n\x17EndpointLoadMetricStats\x12\x1f\n\x0bmetric_name\x18\x01\
    \x20\x01(\tR\nmetricName\x12H\n!num_requests_finished_with_metric\x18\
    \x02\x20\x01(\x04R\x1dnumRequestsFinishedWithMetric\x12,\n\x12total_metr\
    ic_value\x18\x03\x20\x01(\x01R\x10totalMetricValue\"\xff\x02\n\x15Upstre\
    amLocalityStats\x122\n\x08locality\x18\x01\x20\x01(\x0b2\x16.envoy.api.v\
    2.LocalityR\x08locality\x12:\n\x19total_successful_requests\x18\x02\x20\
    \x01(\x04R\x17totalSuccessfulRequests\x12;\n\x1atotal_requests_in_progre\
    ss\x18\x03\x20\x01(\x04R\x17totalRequestsInProgress\x120\n\x14total_erro\
    r_requests\x18\x04\x20\x01(\x04R\x12totalErrorRequests\x124\n\x16total_d\
    ropped_requests\x18\x05\x20\x01(\x04R\x14totalDroppedRequests\x12Q\n\x11\
    load_metric_stats\x18\x06\x20\x03(\x0b2%.envoy.api.v2.EndpointLoadMetric\
    StatsR\x0floadMetricStats\"\x8e\x01\n\x0cClusterStats\x12!\n\x0ccluster_\
    name\x18\x01\x20\x01(\tR\x0bclusterName\x12[\n\x17upstream_locality_stat\
    s\x18\x02\x20\x03(\x0b2#.envoy.api.v2.UpstreamLocalityStatsR\x15upstream\
    LocalityStats\"{\n\x10LoadStatsRequest\x12&\n\x04node\x18\x01\x20\x01(\
    \x0b2\x12.envoy.api.v2.NodeR\x04node\x12?\n\rcluster_stats\x18\x02\x20\
    \x03(\x0b2\x1a.envoy.api.v2.ClusterStatsR\x0cclusterStats\"\xc0\x02\n\
    \x15ClusterLoadAssignment\x12!\n\x0ccluster_name\x18\x01\x20\x01(\tR\x0b\
    clusterName\x12?\n\tendpoints\x18\x02\x20\x03(\x0b2!.envoy.api.v2.Locali\
    tyLbEndpointsR\tendpoints\x12P\n\x12failover_endpoints\x18\x03\x20\x03(\
    \x0b2!.envoy.api.v2.LocalityLbEndpointsR\x11failoverEndpoints\x12B\n\x06\
    policy\x18\x04\x20\x01(\x0b2*.envoy.api.v2.ClusterLoadAssignment.PolicyR\
    \x06policy\x1a-\n\x06Policy\x12#\n\rdrop_overload\x18\x01\x20\x01(\x01R\
    \x0cdropOverload\"\x82\x01\n\x11LoadStatsResponse\x12\x1a\n\x08clusters\
    \x18\x01\x20\x03(\tR\x08clusters\x12Q\n\x17load_reporting_interval\x18\
    \x02\x20\x01(\x0b2\x19.google.protobuf.DurationR\x15loadReportingInterva\
    l2\xc5\x02\n\x18EndpointDiscoveryService\x12X\n\x0fStreamEndpoints\x12\
    \x1e.envoy.api.v2.DiscoveryRequest\x1a\x1f.envoy.api.v2.DiscoveryRespons\
    e\"\0(\x010\x01\x12u\n\x0eFetchEndpoints\x12\x1e.envoy.api.v2.DiscoveryR\
    equest\x1a\x1f.envoy.api.v2.DiscoveryResponse\"\"\x82\xd3\xe4\x93\x02\
    \x1c\"\x17/v2/discovery:endpoints:\x01*\x12X\n\x0fStreamLoadStats\x12\
    \x1e.envoy.api.v2.LoadStatsRequest\x1a\x1f.envoy.api.v2.LoadStatsRespons\
    e\"\0(\x010\x01J\xe1G\n\x07\x12\x05\0\0\xc8\x01\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\n\t\n\x02\x03\0\x12\x03\
    \x04\x07\x17\n\t\n\x02\x03\x01\x12\x03\x05\x07\x1c\n\t\n\x02\x03\x02\x12\
    \x03\x06\x07\x1f\n\t\n\x02\x03\x03\x12\x03\x08\x07%\n\t\n\x02\x03\x04\
    \x12\x03\t\x07'\n\t\n\x02\x03\x05\x12\x03\n\x07'\n\n\n\x02\x06\0\x12\x04\
    \x0c\0:\x01\n\n\n\x03\x06\0\x01\x12\x03\x0c\x08\x20\nw\n\x04\x06\0\x02\0\
    \x12\x04\x0f\x02\x11\x03\x1ai\x20The\x20resource_names\x20field\x20in\
    \x20DiscoveryRequest\x20specifies\x20a\x20list\x20of\x20clusters\n\x20to\
    \x20subscribe\x20to\x20updates\x20for.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\
    \x03\x0f\x06\x15\n\x0c\n\x05\x06\0\x02\0\x05\x12\x03\x0f\x16\x1c\n\x0c\n\
    \x05\x06\0\x02\0\x02\x12\x03\x0f\x1d-\n\x0c\n\x05\x06\0\x02\0\x06\x12\
    \x03\x10\x0f\x15\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x10\x16'\n\x0c\n\
    \x04\x06\0\x02\x01\x12\x04\x13\x02\x19\x03\n\x0c\n\x05\x06\0\x02\x01\x01\
    \x12\x03\x13\x06\x14\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x13\x15%\n\
    \x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x14\x0f\x20\n\r\n\x05\x06\0\x02\x01\
    \x04\x12\x04\x15\x04\x18\x06\n\x10\n\x08\x06\0\x02\x01\x04\xe7\x07\0\x12\
    \x04\x15\x04\x18\x06\n\x10\n\t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03\
    \x15\x0b\x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x12\x03\x15\x0b\
    \x1c\n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x03\x15\x0c\
    \x1b\n\x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\x12\x04\x15\x1f\x18\x05\n\
    \x8c\r\n\x04\x06\0\x02\x02\x12\x047\x029\x03\x1a\xfd\x0c\x20Advanced\x20\
    API\x20to\x20allow\x20for\x20multi-dimensional\x20load\x20balancing\x20b\
    y\x20remote\n\x20server.\x20For\x20receiving\x20LB\x20assignments,\x20th\
    e\x20steps\x20are:\n\x201,\x20The\x20management\x20server\x20is\x20confi\
    gured\x20with\x20per\x20cluster/zone/load\x20metric\n\x20\x20\x20\x20cap\
    acity\x20configuration.\x20The\x20capacity\x20configuration\x20definitio\
    n\x20is\n\x20\x20\x20\x20outside\x20of\x20the\x20scope\x20of\x20this\x20\
    document.\n\x202.\x20Envoy\x20issues\x20a\x20standard\x20{Stream,Fetch}E\
    ndpoints\x20request\x20for\x20the\x20clusters\n\x20\x20\x20\x20to\x20bal\
    ance.\n\n\x20Independently,\x20Envoy\x20will\x20initiate\x20a\x20StreamL\
    oadStats\x20bidi\x20stream\x20with\x20a\n\x20management\x20server:\n\x20\
    1.\x20Once\x20a\x20connection\x20establishes,\x20the\x20management\x20se\
    rver\x20publishes\x20a\n\x20\x20\x20\x20LoadStatsResponse\x20for\x20all\
    \x20clusters\x20it\x20is\x20interested\x20in\x20learning\x20load\n\x20\
    \x20\x20\x20stats\x20about.\n\x202.\x20For\x20each\x20cluster,\x20Envoy\
    \x20load\x20balances\x20incoming\x20traffic\x20to\x20upstream\x20hosts\n\
    \x20\x20\x20\x20based\x20on\x20per-zone\x20weights\x20and/or\x20per-inst\
    ance\x20weights\x20(if\x20specified)\n\x20\x20\x20\x20based\x20on\x20int\
    ra-zone\x20LbPolicy.\x20This\x20information\x20comes\x20from\x20the\x20a\
    bove\n\x20\x20\x20\x20{Stream,Fetch}Endpoints.\n\x203.\x20When\x20upstre\
    am\x20hosts\x20reply,\x20they\x20optionally\x20add\x20header\x20<define\
    \x20header\n\x20\x20\x20\x20name>\x20with\x20ASCII\x20representation\x20\
    of\x20EndpointLoadMetricStats.\n\x204.\x20Envoy\x20aggregates\x20load\
    \x20reports\x20over\x20the\x20period\x20of\x20time\x20given\x20to\x20it\
    \x20in\n\x20\x20\x20\x20LoadStatsResponse.load_reporting_interval.\x20Th\
    is\x20includes\x20aggregation\n\x20\x20\x20\x20stats\x20Envoy\x20maintai\
    ns\x20by\x20itself\x20(total_requests,\x20rpc_errors\x20etc.)\x20as\n\
    \x20\x20\x20\x20well\x20as\x20load\x20metrics\x20from\x20upstream\x20hos\
    ts.\n\x205.\x20When\x20the\x20timer\x20of\x20load_reporting_interval\x20\
    expires,\x20Envoy\x20sends\x20new\n\x20\x20\x20\x20LoadStatsRequest\x20f\
    illed\x20with\x20load\x20reports\x20for\x20each\x20cluster.\n\x206.\x20T\
    he\x20management\x20server\x20uses\x20the\x20load\x20reports\x20from\x20\
    all\x20reported\x20Envoys\n\x20\x20\x20\x20from\x20around\x20the\x20worl\
    d,\x20computes\x20global\x20assignment\x20and\x20prepares\x20traffic\n\
    \x20\x20\x20\x20assignment\x20destined\x20for\x20each\x20zone\x20Envoys\
    \x20are\x20located\x20in.\x20Goto\x202.\n\n\x0c\n\x05\x06\0\x02\x02\x01\
    \x12\x037\x06\x15\n\x0c\n\x05\x06\0\x02\x02\x05\x12\x037\x16\x1c\n\x0c\n\
    \x05\x06\0\x02\x02\x02\x12\x037\x1d-\n\x0c\n\x05\x06\0\x02\x02\x06\x12\
    \x038\x0f\x15\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x038\x16'\n\n\n\x02\x04\
    \0\x12\x04<\0U\x01\n\n\n\x03\x04\0\x01\x12\x03<\x08\x12\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03=\x02\x18\n\r\n\x05\x04\0\x02\0\x04\x12\x04=\x02<\x14\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03=\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03=\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03=\x16\x17\nL\n\x04\
    \x04\0\x02\x01\x12\x03@\x02!\x1a?\x20Optional\x20health\x20status\x20whe\
    n\x20known\x20and\x20supplied\x20by\x20EDS\x20server.\n\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04@\x02=\x18\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03@\
    \x02\x0e\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03@\x0f\x1c\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03@\x1f\x20\n\xc6\x04\n\x04\x04\0\x02\x02\x12\x03J\
    \x02\x18\x1a\xb8\x04\x20The\x20endpoint\x20metadata\x20specifies\x20valu\
    es\x20that\x20may\x20be\x20used\x20by\x20the\x20load\n\x20balancer\x20to\
    \x20select\x20endpoints\x20in\x20a\x20cluster\x20for\x20a\x20given\x20re\
    quest.\x20The\x20filter\n\x20name\x20should\x20be\x20specified\x20as\x20\
    \"envoy.lb\".\x20An\x20example\x20boolean\x20key-value\x20pair\n\x20is\
    \x20\"canary\",\x20providing\x20the\x20optional\x20canary\x20status\x20o\
    f\x20the\x20upstream\x20host.\n\x20This\x20may\x20be\x20matched\x20again\
    st\x20in\x20a\x20route's\x20ForwardAction\x20metadata_match\x20field\n\
    \x20to\x20subset\x20the\x20endpoints\x20considered\x20in\x20cluster\x20l\
    oad\x20balancing.\n\x20TODO(htuch:\x20[V2-API-DIFF]\x20Need\x20to\x20plu\
    mb\x20this\x20through\x20Envoy\x20and\x20have\n\x20everywhere\x20that\
    \x20canary\x20is\x20used\x20be\x20capable\x20of\x20working\x20on\x20meta\
    data.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04J\x02@!\n\x0c\n\x05\x04\0\x02\
    \x02\x06\x12\x03J\x02\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03J\x0b\x13\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03J\x16\x17\n\xa2\x04\n\x04\x04\0\x02\
    \x03\x12\x03T\x028\x1a\x94\x04\x20The\x20optional\x20load\x20balancing\
    \x20weight\x20of\x20the\x20upstream\x20host,\x20in\x20the\x20range\x201\
    \x20-\n\x20100.\x20Envoy\x20uses\x20the\x20load\x20balancing\x20weight\
    \x20in\x20some\x20of\x20the\x20built\x20in\x20load\n\x20balancers.\x20Th\
    e\x20load\x20balancing\x20weight\x20for\x20an\x20endpoint\x20is\x20divid\
    ed\x20by\x20the\x20sum\n\x20of\x20the\x20weights\x20of\x20all\x20endpoin\
    ts\x20in\x20the\x20endpoint's\x20locality\x20to\x20produce\x20a\n\x20per\
    centage\x20of\x20traffic\x20for\x20the\x20endpoint.\x20This\x20percentag\
    e\x20is\x20then\x20further\n\x20weighted\x20by\x20the\x20endpoint's\x20l\
    ocality's\x20load\x20balancing\x20weight\x20from\n\x20LocalityLbEndpoint\
    s.\x20If\x20unspecified,\x20each\x20host\x20is\x20presumed\x20to\x20have\
    \x20equal\n\x20weight\x20in\x20a\x20locality.\n\n\r\n\x05\x04\0\x02\x03\
    \x04\x12\x04T\x02J\x18\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03T\x02\x1d\n\
    \x0c\n\x05\x04\0\x02\x03\x01\x12\x03T\x1e3\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03T67\n4\n\x02\x04\x01\x12\x04X\0b\x01\x1a(\x20All\x20endpoints\
    \x20belonging\x20to\x20a\x20Locality.\n\n\n\n\x03\x04\x01\x01\x12\x03X\
    \x08\x1b\n\x0b\n\x04\x04\x01\x02\0\x12\x03Y\x02\x18\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04Y\x02X\x1d\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03Y\x02\
    \n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03Y\x0b\x13\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03Y\x16\x17\n\x0b\n\x04\x04\x01\x02\x01\x12\x03Z\x02'\n\
    \x0c\n\x05\x04\x01\x02\x01\x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x06\x12\x03Z\x0b\x15\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03Z\x16\"\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03Z%&\n\xba\x02\n\x04\x04\x01\x02\
    \x02\x12\x03a\x028\x1a\xac\x02\x20Optional:\x20Per\x20region/zone/sub_zo\
    ne\x20weight\x20-\x20range\x201-100.\x20The\x20load\x20balancing\n\x20we\
    ight\x20for\x20a\x20locality\x20is\x20divided\x20by\x20the\x20sum\x20of\
    \x20the\x20weights\x20of\x20all\n\x20localities\x20to\x20produce\x20the\
    \x20effective\x20percentage\x20of\x20traffic\x20for\x20the\x20locality.\
    \n\x20If\x20unspecified,\x20each\x20locality\x20is\x20presumed\x20to\x20\
    have\x20equal\x20weight\x20in\x20a\n\x20cluster.\n\n\r\n\x05\x04\x01\x02\
    \x02\x04\x12\x04a\x02Z'\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03a\x02\x1d\
    \n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03a\x1e3\n\x0c\n\x05\x04\x01\x02\
    \x02\x03\x12\x03a67\n\x8b\x03\n\x02\x04\x02\x12\x04q\0y\x01\x1a\xfe\x02\
    \x20Example\x20load\x20report\x20from\x20a\x20single\x20request:\n\n\x20\
    [metric\x20name,\x20metric\x20value]\n\x20*\x20cpu_seconds,\x200.7\n\x20\
    *\x20flash_utilization,\x2075\n\n\x20When\x20aggregating\x20Envoy\x20nee\
    ds\x20to\x20count\x20how\x20many\x20request's\x20load\x20reports\n\x20in\
    cluded\x20each\x20metric\x20type,\x20so\x20Envoy\x20can\x20account\x20fo\
    r\x20requests\x20that\x20don't\n\x20include\x20that\x20metric\x20type.\
    \x20e.g.:\n\n\x20[name,\x20count,\x20sum\x20of\x20values]\n\x20*\x20cpu_\
    seconds,\x2010,\x2017.5\n\x20*\x20flash_utilization,\x205,\x20375\n\n\n\
    \n\x03\x04\x02\x01\x12\x03q\x08\x1f\n0\n\x04\x04\x02\x02\0\x12\x03s\x02\
    \x19\x1a#\x20Name\x20of\x20the\x20metric;\x20may\x20be\x20empty.\n\n\r\n\
    \x05\x04\x02\x02\0\x04\x12\x04s\x02q!\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03s\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03s\t\x14\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03s\x17\x18\nF\n\x04\x04\x02\x02\x01\x12\x03u\
    \x02/\x1a9\x20Number\x20of\x20calls\x20that\x20finished\x20and\x20includ\
    ed\x20this\x20metric.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04u\x02s\x19\
    \n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03u\x02\x08\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03u\t*\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03u-.\nq\n\
    \x04\x04\x02\x02\x02\x12\x03x\x02\x20\x1ad\x20Sum\x20of\x20metric\x20val\
    ues\x20across\x20all\x20calls\x20that\x20finished\x20with\x20this\x20met\
    ric\x20for\n\x20load_reporting_interval.\n\n\r\n\x05\x04\x02\x02\x02\x04\
    \x12\x04x\x02u/\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03x\x02\x08\n\x0c\n\
    \x05\x04\x02\x02\x02\x01\x12\x03x\t\x1b\n\x0c\n\x05\x04\x02\x02\x02\x03\
    \x12\x03x\x1e\x1f\n\xbf\x01\n\x02\x04\x03\x12\x05~\0\x9b\x01\x01\x1a\xb1\
    \x01\x20These\x20are\x20stats\x20Envoy\x20reports\x20to\x20GLB\x20every\
    \x20so\x20often.\x20Report\x20frequency\x20is\n\x20defined\x20by\x20Load\
    AssignmentResponse.interval\n\x20Stats\x20per\x20upstream\x20region/zone\
    \x20and\x20optionally\x20per\x20subzone\n\n\n\n\x03\x04\x03\x01\x12\x03~\
    \x08\x1d\n\x96\x01\n\x04\x04\x03\x02\0\x12\x04\x81\x01\x02\x18\x1a\x87\
    \x01\x20Name\x20of\x20zone,\x20region\x20and\x20optionally\x20endpoint\
    \x20group\x20this\x20metrics\x20was\n\x20collected\x20from.\x20Zone\x20a\
    nd\x20region\x20names\x20could\x20be\x20empty\x20if\x20unknown.\n\n\x0e\
    \n\x05\x04\x03\x02\0\x04\x12\x05\x81\x01\x02~\x1f\n\r\n\x05\x04\x03\x02\
    \0\x06\x12\x04\x81\x01\x02\n\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\x81\x01\
    \x0b\x13\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\x81\x01\x16\x17\n\xc5\x05\n\
    \x04\x04\x03\x02\x01\x12\x04\x8d\x01\x02'\x1a\xb6\x05\x20The\x20total\
    \x20number\x20of\x20requests\x20sent\x20by\x20this\x20Envoy\x20since\x20\
    the\x20last\x20report.\x20A\n\x20single\x20HTTP\x20or\x20gRPC\x20request\
    \x20or\x20stream\x20is\x20counted\x20as\x20one\x20request.\x20A\x20TCP\n\
    \x20connection\x20is\x20also\x20treated\x20as\x20one\x20request.\x20Ther\
    e\x20is\x20no\x20explicit\n\x20total_requests\x20field\x20below,\x20but\
    \x20it\x20may\x20be\x20inferred\x20from:\n\x20\x20\x20total_requests\x20\
    =\x20total_successful_requests\x20+\x20total_requests_in_progress\x20+\n\
    \x20\x20\x20\x20\x20\x20\x20total_error_requests\x20+\x20total_dropped_r\
    equests\n\x20The\x20total\x20number\x20of\x20requests\x20successfully\
    \x20completed\x20by\x20the\x20endpoints\x20in\x20the\n\x20locality.\x20T\
    hese\x20include\x20non-5xx\x20responses\x20for\x20HTTP,\x20where\x20erro\
    rs\n\x20originate\x20at\x20the\x20client\x20and\x20the\x20endpoint\x20re\
    sponded\x20successfuly.\x20For\x20gRPC,\n\x20the\x20grpc-status\x20value\
    s\x20are\x20those\x20not\x20covered\x20by\x20total_error_requests\x20bel\
    ow.\n\n\x0f\n\x05\x04\x03\x02\x01\x04\x12\x06\x8d\x01\x02\x81\x01\x18\n\
    \r\n\x05\x04\x03\x02\x01\x05\x12\x04\x8d\x01\x02\x08\n\r\n\x05\x04\x03\
    \x02\x01\x01\x12\x04\x8d\x01\t\"\n\r\n\x05\x04\x03\x02\x01\x03\x12\x04\
    \x8d\x01%&\n7\n\x04\x04\x03\x02\x02\x12\x04\x8f\x01\x02(\x1a)\x20The\x20\
    total\x20number\x20of\x20unfinished\x20requests\n\n\x0f\n\x05\x04\x03\
    \x02\x02\x04\x12\x06\x8f\x01\x02\x8d\x01'\n\r\n\x05\x04\x03\x02\x02\x05\
    \x12\x04\x8f\x01\x02\x08\n\r\n\x05\x04\x03\x02\x02\x01\x12\x04\x8f\x01\t\
    #\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\x8f\x01&'\n\xff\x01\n\x04\x04\
    \x03\x02\x03\x12\x04\x94\x01\x02\"\x1a\xf0\x01\x20The\x20total\x20number\
    \x20of\x20requests\x20that\x20failed\x20due\x20to\x20errors\x20at\x20the\
    \x20endpoint.\n\x20For\x20HTTP\x20these\x20are\x20responses\x20with\x205\
    xx\x20status\x20codes\x20and\x20for\x20gRPC\x20the\n\x20grpc-status\x20v\
    alues\x20{DeadlineExceeded,\x20Unimplemented,\x20Internal,\n\x20Unavaila\
    ble,\x20Unknown,\x20DataLoss}.\n\n\x0f\n\x05\x04\x03\x02\x03\x04\x12\x06\
    \x94\x01\x02\x8f\x01(\n\r\n\x05\x04\x03\x02\x03\x05\x12\x04\x94\x01\x02\
    \x08\n\r\n\x05\x04\x03\x02\x03\x01\x12\x04\x94\x01\t\x1d\n\r\n\x05\x04\
    \x03\x02\x03\x03\x12\x04\x94\x01\x20!\n\x93\x01\n\x04\x04\x03\x02\x04\
    \x12\x04\x97\x01\x02$\x1a\x84\x01\x20The\x20total\x20number\x20of\x20dro\
    pped\x20requests.\x20This\x20covers\x20requests\n\x20deliberately\x20dro\
    pped\x20by\x20the\x20drop_overload\x20policy\x20and\x20circuit\x20breaki\
    ng.\n\n\x0f\n\x05\x04\x03\x02\x04\x04\x12\x06\x97\x01\x02\x94\x01\"\n\r\
    \n\x05\x04\x03\x02\x04\x05\x12\x04\x97\x01\x02\x08\n\r\n\x05\x04\x03\x02\
    \x04\x01\x12\x04\x97\x01\t\x1f\n\r\n\x05\x04\x03\x02\x04\x03\x12\x04\x97\
    \x01\"#\n;\n\x04\x04\x03\x02\x05\x12\x04\x9a\x01\x029\x1a-\x20Stats\x20f\
    or\x20multi-dimensional\x20load\x20balancing.\n\n\r\n\x05\x04\x03\x02\
    \x05\x04\x12\x04\x9a\x01\x02\n\n\r\n\x05\x04\x03\x02\x05\x06\x12\x04\x9a\
    \x01\x0b\"\n\r\n\x05\x04\x03\x02\x05\x01\x12\x04\x9a\x01#4\n\r\n\x05\x04\
    \x03\x02\x05\x03\x12\x04\x9a\x0178\n!\n\x02\x04\x04\x12\x06\x9e\x01\0\
    \xa2\x01\x01\x1a\x13\x20Per\x20cluster\x20stats\n\n\x0b\n\x03\x04\x04\
    \x01\x12\x04\x9e\x01\x08\x14\n\x0c\n\x04\x04\x04\x02\0\x12\x04\x9f\x01\
    \x02\x1a\n\x0f\n\x05\x04\x04\x02\0\x04\x12\x06\x9f\x01\x02\x9e\x01\x16\n\
    \r\n\x05\x04\x04\x02\0\x05\x12\x04\x9f\x01\x02\x08\n\r\n\x05\x04\x04\x02\
    \0\x01\x12\x04\x9f\x01\t\x15\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\x9f\x01\
    \x18\x19\n\"\n\x04\x04\x04\x02\x01\x12\x04\xa1\x01\x02=\x1a\x14\x20Need\
    \x20at\x20least\x20one.\n\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\xa1\x01\
    \x02\n\n\r\n\x05\x04\x04\x02\x01\x06\x12\x04\xa1\x01\x0b\x20\n\r\n\x05\
    \x04\x04\x02\x01\x01\x12\x04\xa1\x01!8\n\r\n\x05\x04\x04\x02\x01\x03\x12\
    \x04\xa1\x01;<\n\x0c\n\x02\x04\x05\x12\x06\xa4\x01\0\xa7\x01\x01\n\x0b\n\
    \x03\x04\x05\x01\x12\x04\xa4\x01\x08\x18\n1\n\x04\x04\x05\x02\0\x12\x04\
    \xa5\x01\x02\x10\"#\x20zone/region\x20where\x20this\x20Envoy\x20runs\n\n\
    \x0f\n\x05\x04\x05\x02\0\x04\x12\x06\xa5\x01\x02\xa4\x01\x1a\n\r\n\x05\
    \x04\x05\x02\0\x06\x12\x04\xa5\x01\x02\x06\n\r\n\x05\x04\x05\x02\0\x01\
    \x12\x04\xa5\x01\x07\x0b\n\r\n\x05\x04\x05\x02\0\x03\x12\x04\xa5\x01\x0e\
    \x0f\n\x0c\n\x04\x04\x05\x02\x01\x12\x04\xa6\x01\x02*\n\r\n\x05\x04\x05\
    \x02\x01\x04\x12\x04\xa6\x01\x02\n\n\r\n\x05\x04\x05\x02\x01\x06\x12\x04\
    \xa6\x01\x0b\x17\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xa6\x01\x18%\n\r\
    \n\x05\x04\x05\x02\x01\x03\x12\x04\xa6\x01()\n\xeb\x03\n\x02\x04\x06\x12\
    \x06\xb1\x01\0\xc1\x01\x01\x1a\xdc\x03\x20Each\x20route\x20from\x20RDS\
    \x20will\x20map\x20to\x20a\x20single\x20cluster\x20or\x20traffic\x20spli\
    t\x20across\n\x20clusters\x20using\x20weights\x20expressed\x20in\x20the\
    \x20RDS\x20WeightedCluster.\n\n\x20With\x20EDS,\x20each\x20cluster\x20is\
    \x20treated\x20independently\x20from\x20a\x20LB\x20perspective,\x20with\
    \n\x20LB\x20taking\x20place\x20between\x20the\x20Localities\x20within\
    \x20a\x20cluster\x20and\x20at\x20a\x20finer\n\x20granularity\x20between\
    \x20the\x20hosts\x20within\x20a\x20locality.\x20For\x20a\x20given\x20clu\
    ster,\x20the\n\x20effective\x20weight\x20of\x20a\x20host\x20is\x20its\
    \x20load_balancing_weight\x20multiplied\x20by\x20the\n\x20load_balancing\
    _weight\x20of\x20its\x20Locality.\n\n\x0b\n\x03\x04\x06\x01\x12\x04\xb1\
    \x01\x08\x1d\n\x0c\n\x04\x04\x06\x02\0\x12\x04\xb2\x01\x02\x1a\n\x0f\n\
    \x05\x04\x06\x02\0\x04\x12\x06\xb2\x01\x02\xb1\x01\x1f\n\r\n\x05\x04\x06\
    \x02\0\x05\x12\x04\xb2\x01\x02\x08\n\r\n\x05\x04\x06\x02\0\x01\x12\x04\
    \xb2\x01\t\x15\n\r\n\x05\x04\x06\x02\0\x03\x12\x04\xb2\x01\x18\x19\n\x0c\
    \n\x04\x04\x06\x02\x01\x12\x04\xb3\x01\x02-\n\r\n\x05\x04\x06\x02\x01\
    \x04\x12\x04\xb3\x01\x02\n\n\r\n\x05\x04\x06\x02\x01\x06\x12\x04\xb3\x01\
    \x0b\x1e\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\xb3\x01\x1f(\n\r\n\x05\
    \x04\x06\x02\x01\x03\x12\x04\xb3\x01+,\n\xcf\x01\n\x04\x04\x06\x02\x02\
    \x12\x04\xb7\x01\x026\x1a\xc0\x01\x20In\x20the\x20case\x20where\x20all\
    \x20endpoints\x20for\x20a\x20particular\x20zone/subzone\x20are\n\x20unav\
    ailable/unhealthy,\x20additional\x20endpoints\x20are\x20given\x20out\x20\
    for\x20use\x20in\x20case\n\x20of\x20catastrophic\x20failure.\x20They\x20\
    also\x20have\x20weights.\n\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04\xb7\x01\
    \x02\n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\xb7\x01\x0b\x1e\n\r\n\x05\
    \x04\x06\x02\x02\x01\x12\x04\xb7\x01\x1f1\n\r\n\x05\x04\x06\x02\x02\x03\
    \x12\x04\xb7\x0145\n\x0e\n\x04\x04\x06\x03\0\x12\x06\xb8\x01\x02\xbf\x01\
    \x03\n\r\n\x05\x04\x06\x03\0\x01\x12\x04\xb8\x01\n\x10\n\xd4\x02\n\x06\
    \x04\x06\x03\0\x02\0\x12\x04\xbe\x01\x04\x1d\x1a\xc3\x02\x20Percentage\
    \x20of\x20traffic\x20(0-100)\x20that\x20should\x20be\x20dropped.\x20This\
    \n\x20action\x20allows\x20protection\x20of\x20upstream\x20hosts\x20shoul\
    d\x20they\x20unable\x20to\n\x20recover\x20from\x20an\x20outage\x20or\x20\
    should\x20they\x20be\x20unable\x20to\x20autoscale\x20and\x20hence\n\x20o\
    verall\x20incoming\x20traffic\x20volume\x20need\x20to\x20be\x20trimmed\
    \x20to\x20protect\x20them.\n\x20[V2-API-DIFF]\x20This\x20is\x20known\x20\
    as\x20maintenance\x20mode\x20in\x20v1.\n\n\x11\n\x07\x04\x06\x03\0\x02\0\
    \x04\x12\x06\xbe\x01\x04\xb8\x01\x12\n\x0f\n\x07\x04\x06\x03\0\x02\0\x05\
    \x12\x04\xbe\x01\x04\n\n\x0f\n\x07\x04\x06\x03\0\x02\0\x01\x12\x04\xbe\
    \x01\x0b\x18\n\x0f\n\x07\x04\x06\x03\0\x02\0\x03\x12\x04\xbe\x01\x1b\x1c\
    \n\x0c\n\x04\x04\x06\x02\x03\x12\x04\xc0\x01\x02\x14\n\x0f\n\x05\x04\x06\
    \x02\x03\x04\x12\x06\xc0\x01\x02\xbf\x01\x03\n\r\n\x05\x04\x06\x02\x03\
    \x06\x12\x04\xc0\x01\x02\x08\n\r\n\x05\x04\x06\x02\x03\x01\x12\x04\xc0\
    \x01\t\x0f\n\r\n\x05\x04\x06\x02\x03\x03\x12\x04\xc0\x01\x12\x13\n\x0c\n\
    \x02\x04\x07\x12\x06\xc3\x01\0\xc8\x01\x01\n\x0b\n\x03\x04\x07\x01\x12\
    \x04\xc3\x01\x08\x19\n-\n\x04\x04\x07\x02\0\x12\x04\xc5\x01\x02\x1f\x1a\
    \x1f\x20Clusters\x20to\x20report\x20stats\x20for.\n\n\r\n\x05\x04\x07\
    \x02\0\x04\x12\x04\xc5\x01\x02\n\n\r\n\x05\x04\x07\x02\0\x05\x12\x04\xc5\
    \x01\x0b\x11\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\xc5\x01\x12\x1a\n\r\n\
    \x05\x04\x07\x02\0\x03\x12\x04\xc5\x01\x1d\x1e\n*\n\x04\x04\x07\x02\x01\
    \x12\x04\xc7\x01\x027\x1a\x1c\x20The\x20default\x20is\x2010\x20seconds.\
    \n\n\x0f\n\x05\x04\x07\x02\x01\x04\x12\x06\xc7\x01\x02\xc5\x01\x1f\n\r\n\
    \x05\x04\x07\x02\x01\x06\x12\x04\xc7\x01\x02\x1a\n\r\n\x05\x04\x07\x02\
    \x01\x01\x12\x04\xc7\x01\x1b2\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\xc7\
    \x0156b\x06proto3\
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
