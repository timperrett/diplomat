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
pub struct Locality {
    // message fields
    pub region: ::std::string::String,
    pub zone: ::std::string::String,
    pub sub_zone: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Locality {}

impl Locality {
    pub fn new() -> Locality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Locality {
        static mut instance: ::protobuf::lazy::Lazy<Locality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Locality,
        };
        unsafe {
            instance.get(Locality::new)
        }
    }

    // string region = 1;

    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: ::std::string::String) {
        self.region = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region(&mut self) -> &mut ::std::string::String {
        &mut self.region
    }

    // Take field
    pub fn take_region(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.region, ::std::string::String::new())
    }

    pub fn get_region(&self) -> &str {
        &self.region
    }

    fn get_region_for_reflect(&self) -> &::std::string::String {
        &self.region
    }

    fn mut_region_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.region
    }

    // string zone = 2;

    pub fn clear_zone(&mut self) {
        self.zone.clear();
    }

    // Param is passed by value, moved
    pub fn set_zone(&mut self, v: ::std::string::String) {
        self.zone = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zone(&mut self) -> &mut ::std::string::String {
        &mut self.zone
    }

    // Take field
    pub fn take_zone(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.zone, ::std::string::String::new())
    }

    pub fn get_zone(&self) -> &str {
        &self.zone
    }

    fn get_zone_for_reflect(&self) -> &::std::string::String {
        &self.zone
    }

    fn mut_zone_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.zone
    }

    // string sub_zone = 3;

    pub fn clear_sub_zone(&mut self) {
        self.sub_zone.clear();
    }

    // Param is passed by value, moved
    pub fn set_sub_zone(&mut self, v: ::std::string::String) {
        self.sub_zone = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sub_zone(&mut self) -> &mut ::std::string::String {
        &mut self.sub_zone
    }

    // Take field
    pub fn take_sub_zone(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sub_zone, ::std::string::String::new())
    }

    pub fn get_sub_zone(&self) -> &str {
        &self.sub_zone
    }

    fn get_sub_zone_for_reflect(&self) -> &::std::string::String {
        &self.sub_zone
    }

    fn mut_sub_zone_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sub_zone
    }
}

impl ::protobuf::Message for Locality {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.region)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.zone)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sub_zone)?;
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
        if !self.region.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.region);
        }
        if !self.zone.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.zone);
        }
        if !self.sub_zone.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.sub_zone);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.region.is_empty() {
            os.write_string(1, &self.region)?;
        }
        if !self.zone.is_empty() {
            os.write_string(2, &self.zone)?;
        }
        if !self.sub_zone.is_empty() {
            os.write_string(3, &self.sub_zone)?;
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

impl ::protobuf::MessageStatic for Locality {
    fn new() -> Locality {
        Locality::new()
    }

    fn descriptor_static(_: ::std::option::Option<Locality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "region",
                    Locality::get_region_for_reflect,
                    Locality::mut_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "zone",
                    Locality::get_zone_for_reflect,
                    Locality::mut_zone_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sub_zone",
                    Locality::get_sub_zone_for_reflect,
                    Locality::mut_sub_zone_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Locality>(
                    "Locality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Locality {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_zone();
        self.clear_sub_zone();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Locality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Locality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Node {
    // message fields
    pub id: ::std::string::String,
    pub cluster: ::std::string::String,
    pub metadata: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    pub locality: ::protobuf::SingularPtrField<Locality>,
    pub build_version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(Node::new)
        }
    }

    // string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::string::String {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // string cluster = 2;

    pub fn clear_cluster(&mut self) {
        self.cluster.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: ::std::string::String) {
        self.cluster = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster(&mut self) -> &mut ::std::string::String {
        &mut self.cluster
    }

    // Take field
    pub fn take_cluster(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cluster, ::std::string::String::new())
    }

    pub fn get_cluster(&self) -> &str {
        &self.cluster
    }

    fn get_cluster_for_reflect(&self) -> &::std::string::String {
        &self.cluster
    }

    fn mut_cluster_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cluster
    }

    // .google.protobuf.Struct metadata = 3;

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

    // .envoy.api.v2.Locality locality = 4;

    pub fn clear_locality(&mut self) {
        self.locality.clear();
    }

    pub fn has_locality(&self) -> bool {
        self.locality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locality(&mut self, v: Locality) {
        self.locality = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locality(&mut self) -> &mut Locality {
        if self.locality.is_none() {
            self.locality.set_default();
        }
        self.locality.as_mut().unwrap()
    }

    // Take field
    pub fn take_locality(&mut self) -> Locality {
        self.locality.take().unwrap_or_else(|| Locality::new())
    }

    pub fn get_locality(&self) -> &Locality {
        self.locality.as_ref().unwrap_or_else(|| Locality::default_instance())
    }

    fn get_locality_for_reflect(&self) -> &::protobuf::SingularPtrField<Locality> {
        &self.locality
    }

    fn mut_locality_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Locality> {
        &mut self.locality
    }

    // string build_version = 5;

    pub fn clear_build_version(&mut self) {
        self.build_version.clear();
    }

    // Param is passed by value, moved
    pub fn set_build_version(&mut self, v: ::std::string::String) {
        self.build_version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_version(&mut self) -> &mut ::std::string::String {
        &mut self.build_version
    }

    // Take field
    pub fn take_build_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.build_version, ::std::string::String::new())
    }

    pub fn get_build_version(&self) -> &str {
        &self.build_version
    }

    fn get_build_version_for_reflect(&self) -> &::std::string::String {
        &self.build_version
    }

    fn mut_build_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.build_version
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locality {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cluster)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locality)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.build_version)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cluster);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.locality.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.build_version.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.build_version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.cluster.is_empty() {
            os.write_string(2, &self.cluster)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.locality.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.build_version.is_empty() {
            os.write_string(5, &self.build_version)?;
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

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    Node::get_id_for_reflect,
                    Node::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster",
                    Node::get_cluster_for_reflect,
                    Node::mut_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "metadata",
                    Node::get_metadata_for_reflect,
                    Node::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Locality>>(
                    "locality",
                    Node::get_locality_for_reflect,
                    Node::mut_locality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "build_version",
                    Node::get_build_version_for_reflect,
                    Node::mut_build_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_cluster();
        self.clear_metadata();
        self.clear_locality();
        self.clear_build_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Endpoint {
    // message fields
    pub address: ::protobuf::SingularPtrField<super::address::Address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Endpoint {}

impl Endpoint {
    pub fn new() -> Endpoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Endpoint {
        static mut instance: ::protobuf::lazy::Lazy<Endpoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Endpoint,
        };
        unsafe {
            instance.get(Endpoint::new)
        }
    }

    // .envoy.api.v2.Address address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: super::address::Address) {
        self.address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut super::address::Address {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> super::address::Address {
        self.address.take().unwrap_or_else(|| super::address::Address::new())
    }

    pub fn get_address(&self) -> &super::address::Address {
        self.address.as_ref().unwrap_or_else(|| super::address::Address::default_instance())
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularPtrField<super::address::Address> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::address::Address> {
        &mut self.address
    }
}

impl ::protobuf::Message for Endpoint {
    fn is_initialized(&self) -> bool {
        for v in &self.address {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.address)?;
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
        if let Some(ref v) = self.address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.address.as_ref() {
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

impl ::protobuf::MessageStatic for Endpoint {
    fn new() -> Endpoint {
        Endpoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<Endpoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "address",
                    Endpoint::get_address_for_reflect,
                    Endpoint::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Endpoint>(
                    "Endpoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Endpoint {
    fn clear(&mut self) {
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Endpoint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    pub filter_metadata: ::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // repeated .envoy.api.v2.Metadata.FilterMetadataEntry filter_metadata = 1;

    pub fn clear_filter_metadata(&mut self) {
        self.filter_metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_filter_metadata(&mut self, v: ::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct>) {
        self.filter_metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filter_metadata(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct> {
        &mut self.filter_metadata
    }

    // Take field
    pub fn take_filter_metadata(&mut self) -> ::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct> {
        ::std::mem::replace(&mut self.filter_metadata, ::std::collections::HashMap::new())
    }

    pub fn get_filter_metadata(&self) -> &::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct> {
        &self.filter_metadata
    }

    fn get_filter_metadata_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct> {
        &self.filter_metadata
    }

    fn mut_filter_metadata_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::protobuf::well_known_types::Struct> {
        &mut self.filter_metadata
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(wire_type, is, &mut self.filter_metadata)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(1, &self.filter_metadata);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(1, &self.filter_metadata, os)?;
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

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "filter_metadata",
                    Metadata::get_filter_metadata_for_reflect,
                    Metadata::mut_filter_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_filter_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RuntimeUInt32 {
    // message fields
    pub default_value: u32,
    pub runtime_key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RuntimeUInt32 {}

impl RuntimeUInt32 {
    pub fn new() -> RuntimeUInt32 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RuntimeUInt32 {
        static mut instance: ::protobuf::lazy::Lazy<RuntimeUInt32> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RuntimeUInt32,
        };
        unsafe {
            instance.get(RuntimeUInt32::new)
        }
    }

    // uint32 default_value = 2;

    pub fn clear_default_value(&mut self) {
        self.default_value = 0;
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: u32) {
        self.default_value = v;
    }

    pub fn get_default_value(&self) -> u32 {
        self.default_value
    }

    fn get_default_value_for_reflect(&self) -> &u32 {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut u32 {
        &mut self.default_value
    }

    // string runtime_key = 3;

    pub fn clear_runtime_key(&mut self) {
        self.runtime_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_runtime_key(&mut self, v: ::std::string::String) {
        self.runtime_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_runtime_key(&mut self) -> &mut ::std::string::String {
        &mut self.runtime_key
    }

    // Take field
    pub fn take_runtime_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.runtime_key, ::std::string::String::new())
    }

    pub fn get_runtime_key(&self) -> &str {
        &self.runtime_key
    }

    fn get_runtime_key_for_reflect(&self) -> &::std::string::String {
        &self.runtime_key
    }

    fn mut_runtime_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.runtime_key
    }
}

impl ::protobuf::Message for RuntimeUInt32 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.default_value = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.runtime_key)?;
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
        if self.default_value != 0 {
            my_size += ::protobuf::rt::value_size(2, self.default_value, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.runtime_key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.runtime_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.default_value != 0 {
            os.write_uint32(2, self.default_value)?;
        }
        if !self.runtime_key.is_empty() {
            os.write_string(3, &self.runtime_key)?;
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

impl ::protobuf::MessageStatic for RuntimeUInt32 {
    fn new() -> RuntimeUInt32 {
        RuntimeUInt32::new()
    }

    fn descriptor_static(_: ::std::option::Option<RuntimeUInt32>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "default_value",
                    RuntimeUInt32::get_default_value_for_reflect,
                    RuntimeUInt32::mut_default_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "runtime_key",
                    RuntimeUInt32::get_runtime_key_for_reflect,
                    RuntimeUInt32::mut_runtime_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RuntimeUInt32>(
                    "RuntimeUInt32",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RuntimeUInt32 {
    fn clear(&mut self) {
        self.clear_default_value();
        self.clear_runtime_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RuntimeUInt32 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RuntimeUInt32 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeaderValue {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeaderValue {}

impl HeaderValue {
    pub fn new() -> HeaderValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeaderValue {
        static mut instance: ::protobuf::lazy::Lazy<HeaderValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderValue,
        };
        unsafe {
            instance.get(HeaderValue::new)
        }
    }

    // string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for HeaderValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
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

impl ::protobuf::MessageStatic for HeaderValue {
    fn new() -> HeaderValue {
        HeaderValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeaderValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    HeaderValue::get_key_for_reflect,
                    HeaderValue::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    HeaderValue::get_value_for_reflect,
                    HeaderValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeaderValue>(
                    "HeaderValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeaderValue {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeaderValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeaderValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeaderValueOption {
    // message fields
    pub header: ::protobuf::SingularPtrField<HeaderValue>,
    pub append: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeaderValueOption {}

impl HeaderValueOption {
    pub fn new() -> HeaderValueOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeaderValueOption {
        static mut instance: ::protobuf::lazy::Lazy<HeaderValueOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderValueOption,
        };
        unsafe {
            instance.get(HeaderValueOption::new)
        }
    }

    // .envoy.api.v2.HeaderValue header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: HeaderValue) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut HeaderValue {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> HeaderValue {
        self.header.take().unwrap_or_else(|| HeaderValue::new())
    }

    pub fn get_header(&self) -> &HeaderValue {
        self.header.as_ref().unwrap_or_else(|| HeaderValue::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<HeaderValue> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HeaderValue> {
        &mut self.header
    }

    // .google.protobuf.BoolValue append = 2;

    pub fn clear_append(&mut self) {
        self.append.clear();
    }

    pub fn has_append(&self) -> bool {
        self.append.is_some()
    }

    // Param is passed by value, moved
    pub fn set_append(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.append = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_append(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.append.is_none() {
            self.append.set_default();
        }
        self.append.as_mut().unwrap()
    }

    // Take field
    pub fn take_append(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.append.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_append(&self) -> &::protobuf::well_known_types::BoolValue {
        self.append.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_append_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.append
    }

    fn mut_append_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.append
    }
}

impl ::protobuf::Message for HeaderValueOption {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.append {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.append)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.append.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.append.as_ref() {
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

impl ::protobuf::MessageStatic for HeaderValueOption {
    fn new() -> HeaderValueOption {
        HeaderValueOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeaderValueOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HeaderValue>>(
                    "header",
                    HeaderValueOption::get_header_for_reflect,
                    HeaderValueOption::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "append",
                    HeaderValueOption::get_append_for_reflect,
                    HeaderValueOption::mut_append_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeaderValueOption>(
                    "HeaderValueOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeaderValueOption {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_append();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeaderValueOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeaderValueOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiConfigSource {
    // message fields
    pub api_type: ApiConfigSource_ApiType,
    pub cluster_name: ::protobuf::RepeatedField<::std::string::String>,
    pub refresh_delay: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiConfigSource {}

impl ApiConfigSource {
    pub fn new() -> ApiConfigSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiConfigSource {
        static mut instance: ::protobuf::lazy::Lazy<ApiConfigSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiConfigSource,
        };
        unsafe {
            instance.get(ApiConfigSource::new)
        }
    }

    // .envoy.api.v2.ApiConfigSource.ApiType api_type = 1;

    pub fn clear_api_type(&mut self) {
        self.api_type = ApiConfigSource_ApiType::REST_LEGACY;
    }

    // Param is passed by value, moved
    pub fn set_api_type(&mut self, v: ApiConfigSource_ApiType) {
        self.api_type = v;
    }

    pub fn get_api_type(&self) -> ApiConfigSource_ApiType {
        self.api_type
    }

    fn get_api_type_for_reflect(&self) -> &ApiConfigSource_ApiType {
        &self.api_type
    }

    fn mut_api_type_for_reflect(&mut self) -> &mut ApiConfigSource_ApiType {
        &mut self.api_type
    }

    // repeated string cluster_name = 2;

    pub fn clear_cluster_name(&mut self) {
        self.cluster_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.cluster_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cluster_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cluster_name
    }

    // Take field
    pub fn take_cluster_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.cluster_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_cluster_name(&self) -> &[::std::string::String] {
        &self.cluster_name
    }

    fn get_cluster_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.cluster_name
    }

    fn mut_cluster_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cluster_name
    }

    // .google.protobuf.Duration refresh_delay = 3;

    pub fn clear_refresh_delay(&mut self) {
        self.refresh_delay.clear();
    }

    pub fn has_refresh_delay(&self) -> bool {
        self.refresh_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refresh_delay(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.refresh_delay = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_refresh_delay(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.refresh_delay.is_none() {
            self.refresh_delay.set_default();
        }
        self.refresh_delay.as_mut().unwrap()
    }

    // Take field
    pub fn take_refresh_delay(&mut self) -> ::protobuf::well_known_types::Duration {
        self.refresh_delay.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_refresh_delay(&self) -> &::protobuf::well_known_types::Duration {
        self.refresh_delay.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_refresh_delay_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.refresh_delay
    }

    fn mut_refresh_delay_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.refresh_delay
    }
}

impl ::protobuf::Message for ApiConfigSource {
    fn is_initialized(&self) -> bool {
        for v in &self.refresh_delay {
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
                    self.api_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.cluster_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.refresh_delay)?;
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
        if self.api_type != ApiConfigSource_ApiType::REST_LEGACY {
            my_size += ::protobuf::rt::enum_size(1, self.api_type);
        }
        for value in &self.cluster_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.refresh_delay.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.api_type != ApiConfigSource_ApiType::REST_LEGACY {
            os.write_enum(1, self.api_type.value())?;
        }
        for v in &self.cluster_name {
            os.write_string(2, &v)?;
        };
        if let Some(ref v) = self.refresh_delay.as_ref() {
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

impl ::protobuf::MessageStatic for ApiConfigSource {
    fn new() -> ApiConfigSource {
        ApiConfigSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiConfigSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ApiConfigSource_ApiType>>(
                    "api_type",
                    ApiConfigSource::get_api_type_for_reflect,
                    ApiConfigSource::mut_api_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster_name",
                    ApiConfigSource::get_cluster_name_for_reflect,
                    ApiConfigSource::mut_cluster_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "refresh_delay",
                    ApiConfigSource::get_refresh_delay_for_reflect,
                    ApiConfigSource::mut_refresh_delay_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiConfigSource>(
                    "ApiConfigSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiConfigSource {
    fn clear(&mut self) {
        self.clear_api_type();
        self.clear_cluster_name();
        self.clear_refresh_delay();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiConfigSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiConfigSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ApiConfigSource_ApiType {
    REST_LEGACY = 0,
    REST = 1,
    GRPC = 2,
}

impl ::protobuf::ProtobufEnum for ApiConfigSource_ApiType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ApiConfigSource_ApiType> {
        match value {
            0 => ::std::option::Option::Some(ApiConfigSource_ApiType::REST_LEGACY),
            1 => ::std::option::Option::Some(ApiConfigSource_ApiType::REST),
            2 => ::std::option::Option::Some(ApiConfigSource_ApiType::GRPC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ApiConfigSource_ApiType] = &[
            ApiConfigSource_ApiType::REST_LEGACY,
            ApiConfigSource_ApiType::REST,
            ApiConfigSource_ApiType::GRPC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ApiConfigSource_ApiType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ApiConfigSource_ApiType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ApiConfigSource_ApiType {
}

impl ::std::default::Default for ApiConfigSource_ApiType {
    fn default() -> Self {
        ApiConfigSource_ApiType::REST_LEGACY
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiConfigSource_ApiType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AggregatedConfigSource {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AggregatedConfigSource {}

impl AggregatedConfigSource {
    pub fn new() -> AggregatedConfigSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AggregatedConfigSource {
        static mut instance: ::protobuf::lazy::Lazy<AggregatedConfigSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AggregatedConfigSource,
        };
        unsafe {
            instance.get(AggregatedConfigSource::new)
        }
    }
}

impl ::protobuf::Message for AggregatedConfigSource {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for AggregatedConfigSource {
    fn new() -> AggregatedConfigSource {
        AggregatedConfigSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<AggregatedConfigSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AggregatedConfigSource>(
                    "AggregatedConfigSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AggregatedConfigSource {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AggregatedConfigSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AggregatedConfigSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigSource {
    // message oneof groups
    config_source_specifier: ::std::option::Option<ConfigSource_oneof_config_source_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigSource {}

#[derive(Clone,PartialEq)]
pub enum ConfigSource_oneof_config_source_specifier {
    path(::std::string::String),
    api_config_source(ApiConfigSource),
    ads(AggregatedConfigSource),
}

impl ConfigSource {
    pub fn new() -> ConfigSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigSource {
        static mut instance: ::protobuf::lazy::Lazy<ConfigSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigSource,
        };
        unsafe {
            instance.get(ConfigSource::new)
        }
    }

    // string path = 1;

    pub fn clear_path(&mut self) {
        self.config_source_specifier = ::std::option::Option::None;
    }

    pub fn has_path(&self) -> bool {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(_)) = self.config_source_specifier {
        } else {
            self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(::std::string::String::new()));
        }
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        if self.has_path() {
            match self.config_source_specifier.take() {
                ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path(&self) -> &str {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(ref v)) => v,
            _ => "",
        }
    }

    // .envoy.api.v2.ApiConfigSource api_config_source = 2;

    pub fn clear_api_config_source(&mut self) {
        self.config_source_specifier = ::std::option::Option::None;
    }

    pub fn has_api_config_source(&self) -> bool {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_api_config_source(&mut self, v: ApiConfigSource) {
        self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(v))
    }

    // Mutable pointer to the field.
    pub fn mut_api_config_source(&mut self) -> &mut ApiConfigSource {
        if let ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(_)) = self.config_source_specifier {
        } else {
            self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(ApiConfigSource::new()));
        }
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_api_config_source(&mut self) -> ApiConfigSource {
        if self.has_api_config_source() {
            match self.config_source_specifier.take() {
                ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(v)) => v,
                _ => panic!(),
            }
        } else {
            ApiConfigSource::new()
        }
    }

    pub fn get_api_config_source(&self) -> &ApiConfigSource {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(ref v)) => v,
            _ => ApiConfigSource::default_instance(),
        }
    }

    // .envoy.api.v2.AggregatedConfigSource ads = 3;

    pub fn clear_ads(&mut self) {
        self.config_source_specifier = ::std::option::Option::None;
    }

    pub fn has_ads(&self) -> bool {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ads(&mut self, v: AggregatedConfigSource) {
        self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ads(&mut self) -> &mut AggregatedConfigSource {
        if let ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(_)) = self.config_source_specifier {
        } else {
            self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(AggregatedConfigSource::new()));
        }
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ads(&mut self) -> AggregatedConfigSource {
        if self.has_ads() {
            match self.config_source_specifier.take() {
                ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(v)) => v,
                _ => panic!(),
            }
        } else {
            AggregatedConfigSource::new()
        }
    }

    pub fn get_ads(&self) -> &AggregatedConfigSource {
        match self.config_source_specifier {
            ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(ref v)) => v,
            _ => AggregatedConfigSource::default_instance(),
        }
    }
}

impl ::protobuf::Message for ConfigSource {
    fn is_initialized(&self) -> bool {
        if let Some(ConfigSource_oneof_config_source_specifier::api_config_source(ref v)) = self.config_source_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ConfigSource_oneof_config_source_specifier::ads(ref v)) = self.config_source_specifier {
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
                    self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::path(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::api_config_source(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config_source_specifier = ::std::option::Option::Some(ConfigSource_oneof_config_source_specifier::ads(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.config_source_specifier {
            match v {
                &ConfigSource_oneof_config_source_specifier::path(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &ConfigSource_oneof_config_source_specifier::api_config_source(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ConfigSource_oneof_config_source_specifier::ads(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.config_source_specifier {
            match v {
                &ConfigSource_oneof_config_source_specifier::path(ref v) => {
                    os.write_string(1, v)?;
                },
                &ConfigSource_oneof_config_source_specifier::api_config_source(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ConfigSource_oneof_config_source_specifier::ads(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ConfigSource {
    fn new() -> ConfigSource {
        ConfigSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "path",
                    ConfigSource::has_path,
                    ConfigSource::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ApiConfigSource>(
                    "api_config_source",
                    ConfigSource::has_api_config_source,
                    ConfigSource::get_api_config_source,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AggregatedConfigSource>(
                    "ads",
                    ConfigSource::has_ads,
                    ConfigSource::get_ads,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigSource>(
                    "ConfigSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigSource {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_api_config_source();
        self.clear_ads();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RoutingPriority {
    DEFAULT = 0,
    HIGH = 1,
}

impl ::protobuf::ProtobufEnum for RoutingPriority {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RoutingPriority> {
        match value {
            0 => ::std::option::Option::Some(RoutingPriority::DEFAULT),
            1 => ::std::option::Option::Some(RoutingPriority::HIGH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RoutingPriority] = &[
            RoutingPriority::DEFAULT,
            RoutingPriority::HIGH,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RoutingPriority>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RoutingPriority", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RoutingPriority {
}

impl ::std::default::Default for RoutingPriority {
    fn default() -> Self {
        RoutingPriority::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for RoutingPriority {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RequestMethod {
    METHOD_UNSPECIFIED = 0,
    GET = 1,
    HEAD = 2,
    POST = 3,
    PUT = 4,
    DELETE = 5,
    CONNECT = 6,
    OPTIONS = 7,
    TRACE = 8,
}

impl ::protobuf::ProtobufEnum for RequestMethod {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RequestMethod> {
        match value {
            0 => ::std::option::Option::Some(RequestMethod::METHOD_UNSPECIFIED),
            1 => ::std::option::Option::Some(RequestMethod::GET),
            2 => ::std::option::Option::Some(RequestMethod::HEAD),
            3 => ::std::option::Option::Some(RequestMethod::POST),
            4 => ::std::option::Option::Some(RequestMethod::PUT),
            5 => ::std::option::Option::Some(RequestMethod::DELETE),
            6 => ::std::option::Option::Some(RequestMethod::CONNECT),
            7 => ::std::option::Option::Some(RequestMethod::OPTIONS),
            8 => ::std::option::Option::Some(RequestMethod::TRACE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RequestMethod] = &[
            RequestMethod::METHOD_UNSPECIFIED,
            RequestMethod::GET,
            RequestMethod::HEAD,
            RequestMethod::POST,
            RequestMethod::PUT,
            RequestMethod::DELETE,
            RequestMethod::CONNECT,
            RequestMethod::OPTIONS,
            RequestMethod::TRACE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RequestMethod>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RequestMethod", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RequestMethod {
}

impl ::std::default::Default for RequestMethod {
    fn default() -> Self {
        RequestMethod::METHOD_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestMethod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eapi/base.proto\x12\x0cenvoy.api.v2\x1a\x11api/address.proto\x1a\
    \x1egoogle/protobuf/duration.proto\x1a\x1cgoogle/protobuf/struct.proto\
    \x1a\x1egoogle/protobuf/wrappers.proto\"Q\n\x08Locality\x12\x16\n\x06reg\
    ion\x18\x01\x20\x01(\tR\x06region\x12\x12\n\x04zone\x18\x02\x20\x01(\tR\
    \x04zone\x12\x19\n\x08sub_zone\x18\x03\x20\x01(\tR\x07subZone\"\xbe\x01\
    \n\x04Node\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12\x18\n\x07cluste\
    r\x18\x02\x20\x01(\tR\x07cluster\x123\n\x08metadata\x18\x03\x20\x01(\x0b\
    2\x17.google.protobuf.StructR\x08metadata\x122\n\x08locality\x18\x04\x20\
    \x01(\x0b2\x16.envoy.api.v2.LocalityR\x08locality\x12#\n\rbuild_version\
    \x18\x05\x20\x01(\tR\x0cbuildVersion\";\n\x08Endpoint\x12/\n\x07address\
    \x18\x01\x20\x01(\x0b2\x15.envoy.api.v2.AddressR\x07address\"\xbb\x01\n\
    \x08Metadata\x12S\n\x0ffilter_metadata\x18\x01\x20\x03(\x0b2*.envoy.api.\
    v2.Metadata.FilterMetadataEntryR\x0efilterMetadata\x1aZ\n\x13FilterMetad\
    ataEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12-\n\x05value\x18\
    \x02\x20\x01(\x0b2\x17.google.protobuf.StructR\x05value:\x028\x01\"U\n\r\
    RuntimeUInt32\x12#\n\rdefault_value\x18\x02\x20\x01(\rR\x0cdefaultValue\
    \x12\x1f\n\x0bruntime_key\x18\x03\x20\x01(\tR\nruntimeKey\"5\n\x0bHeader\
    Value\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value\"z\n\x11HeaderValueOption\x121\n\x06header\x18\
    \x01\x20\x01(\x0b2\x19.envoy.api.v2.HeaderValueR\x06header\x122\n\x06app\
    end\x18\x02\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\x06append\"\xe6\
    \x01\n\x0fApiConfigSource\x12@\n\x08api_type\x18\x01\x20\x01(\x0e2%.envo\
    y.api.v2.ApiConfigSource.ApiTypeR\x07apiType\x12!\n\x0ccluster_name\x18\
    \x02\x20\x03(\tR\x0bclusterName\x12>\n\rrefresh_delay\x18\x03\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x0crefreshDelay\".\n\x07ApiType\x12\
    \x0f\n\x0bREST_LEGACY\x10\0\x12\x08\n\x04REST\x10\x01\x12\x08\n\x04GRPC\
    \x10\x02\"\x18\n\x16AggregatedConfigSource\"\xc6\x01\n\x0cConfigSource\
    \x12\x14\n\x04path\x18\x01\x20\x01(\tH\0R\x04path\x12K\n\x11api_config_s\
    ource\x18\x02\x20\x01(\x0b2\x1d.envoy.api.v2.ApiConfigSourceH\0R\x0fapiC\
    onfigSource\x128\n\x03ads\x18\x03\x20\x01(\x0b2$.envoy.api.v2.Aggregated\
    ConfigSourceH\0R\x03adsB\x19\n\x17config_source_specifier*(\n\x0fRouting\
    Priority\x12\x0b\n\x07DEFAULT\x10\0\x12\x08\n\x04HIGH\x10\x01*~\n\rReque\
    stMethod\x12\x16\n\x12METHOD_UNSPECIFIED\x10\0\x12\x07\n\x03GET\x10\x01\
    \x12\x08\n\x04HEAD\x10\x02\x12\x08\n\x04POST\x10\x03\x12\x07\n\x03PUT\
    \x10\x04\x12\n\n\x06DELETE\x10\x05\x12\x0b\n\x07CONNECT\x10\x06\x12\x0b\
    \n\x07OPTIONS\x10\x07\x12\t\n\x05TRACE\x10\x08J\xa4-\n\x07\x12\x05\0\0\
    \x94\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\
    \x08\x14\n\t\n\x02\x03\0\x12\x03\x04\x07\x1a\n\t\n\x02\x03\x01\x12\x03\
    \x06\x07'\n\t\n\x02\x03\x02\x12\x03\x07\x07%\n\t\n\x02\x03\x03\x12\x03\
    \x08\x07'\nY\n\x02\x04\0\x12\x04\x0b\0\x16\x01\x1aM\x20Identifies\x20loc\
    ation\x20of\x20where\x20either\x20Envoy\x20runs\x20or\x20where\x20upstre\
    am\x20hosts\x20run.\n\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x10\n+\n\x04\
    \x04\0\x02\0\x12\x03\r\x02\x14\x1a\x1e\x20Region\x20this\x20zone\x20belo\
    ngs\x20to.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\x02\x0b\x12\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\
    \t\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x12\x13\n:\n\x04\x04\0\x02\
    \x01\x12\x03\x10\x02\x12\x1a-\x20Availability\x20Zone\x20(AZ)\x20in\x20A\
    WS,\x20Zone\x20in\x20GCP.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x10\x02\
    \r\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x10\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x10\t\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x10\
    \x10\x11\n\xa6\x01\n\x04\x04\0\x02\x02\x12\x03\x15\x02\x16\x1a\x98\x01\
    \x20When\x20used\x20for\x20locality\x20of\x20upstream\x20hosts,\x20this\
    \x20field\x20further\x20splits\x20zone\n\x20into\x20smaller\x20chunks\
    \x20of\x20sub_zones\x20so\x20they\x20can\x20be\x20load\x20balanced\n\x20\
    independently\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x15\x02\x10\x12\n\
    \x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x15\t\x11\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x15\x14\
    \x15\nc\n\x02\x04\x01\x12\x04\x19\0#\x01\x1aW\x20Identifies\x20a\x20spec\
    ific\x20Envoy\x20instance.\x20Remote\x20server\x20may\x20have\x20per\x20\
    Envoy\x20configuration.\n\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\x0c\nN\n\
    \x04\x04\x01\x02\0\x12\x03\x1b\x02\x10\x1aA\x20An\x20opaque\x20node\x20i\
    dentifier\x20for\x20the\x20Envoy\x20node.\x20This\x20must\x20be\x20set.\
    \n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x1b\x02\x19\x0e\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\
    \x1b\t\x0b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1b\x0e\x0f\nL\n\x04\
    \x04\x01\x02\x01\x12\x03\x1d\x02\x15\x1a?\x20The\x20cluster\x20that\x20t\
    he\x20Envoy\x20node\x20belongs\x20to.\x20This\x20must\x20be\x20set.\n\n\
    \r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1d\x02\x1b\x10\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x03\x1d\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\
    \x1d\t\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1d\x13\x14\n\x0b\n\
    \x04\x04\x01\x02\x02\x12\x03\x1e\x02&\n\r\n\x05\x04\x01\x02\x02\x04\x12\
    \x04\x1e\x02\x1d\x15\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\x1e\x02\x18\
    \n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1e\x19!\n\x0c\n\x05\x04\x01\
    \x02\x02\x03\x12\x03\x1e$%\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x1f\x02\
    \x18\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04\x1f\x02\x1e&\n\x0c\n\x05\x04\
    \x01\x02\x03\x06\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\
    \x03\x1f\x0b\x13\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1f\x16\x17\n\
    \x92\x01\n\x04\x04\x01\x02\x04\x12\x03\"\x02\x1b\x1a\x84\x01\x20This\x20\
    is\x20motivated\x20by\x20informing\x20a\x20management\x20server\x20durin\
    g\x20canary\x20which\n\x20version\x20of\x20Envoy\x20is\x20being\x20teste\
    d\x20in\x20a\x20heterogeneous\x20fleet.\n\n\r\n\x05\x04\x01\x02\x04\x04\
    \x12\x04\"\x02\x1f\x18\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\"\x02\x08\
    \n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\"\t\x16\n\x0c\n\x05\x04\x01\x02\
    \x04\x03\x12\x03\"\x19\x1a\n\n\n\x02\x04\x02\x12\x04%\0'\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03%\x08\x10\n\x0b\n\x04\x04\x02\x02\0\x12\x03&\x02\x16\
    \n\r\n\x05\x04\x02\x02\0\x04\x12\x04&\x02%\x12\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03&\x02\t\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03&\n\x11\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03&\x14\x15\n\xbf\x07\n\x02\x04\x03\x12\x04:\
    \0=\x01\x1a\xb2\x07\x20Metadata\x20provides\x20additional\x20inputs\x20t\
    o\x20filters\x20based\x20on\x20matched\x20listeners,\n\x20filter\x20chai\
    ns,\x20routes\x20and\x20endpoints.\x20It\x20is\x20structured\x20as\x20a\
    \x20map\x20from\x20filter\n\x20name\x20(in\x20reverse\x20DNS\x20format)\
    \x20to\x20metadata\x20specific\x20to\x20the\x20filter.\x20Metadata\n\x20\
    key-values\x20for\x20a\x20filter\x20are\x20merged\x20as\x20connection\
    \x20and\x20request\x20handling\x20occurs,\n\x20with\x20later\x20values\
    \x20for\x20the\x20same\x20key\x20overriding\x20earlier\x20values.\n\n\
    \x20An\x20example\x20use\x20of\x20metadata\x20is\x20providing\x20additio\
    nal\x20values\x20to\n\x20http_connection_manager\x20in\x20the\x20envoy.h\
    ttp_connection_manager.access_log\n\x20namespace.\n\n\x20For\x20load\x20\
    balancing,\x20Metadata\x20provides\x20a\x20means\x20to\x20subset\x20clus\
    ter\x20endpoints.\n\x20Endpoints\x20have\x20a\x20Metadata\x20object\x20a\
    ssociated\x20and\x20routes\x20contain\x20a\x20Metadata\n\x20object\x20to\
    \x20match\x20against.\x20There\x20are\x20some\x20well\x20defined\x20meta\
    data\x20used\x20today\x20for\n\x20this\x20purpose:\n\x20-\x20{\"envoy.lb\
    \":\x20{\"canary\":\x20<bool>\x20}}.\x20This\x20indicates\x20the\x20cana\
    ry\x20status\x20of\x20an\n\x20\x20\x20endpoint\x20and\x20is\x20also\x20u\
    sed\x20during\x20header\x20processing\n\x20\x20\x20(x-envoy-upstream-can\
    ary)\x20and\x20for\x20stats\x20purposes.\n\n\n\n\x03\x04\x03\x01\x12\x03\
    :\x08\x10\n2\n\x04\x04\x03\x02\0\x12\x03<\x02:\x1a%\x20Key\x20is\x20the\
    \x20reverse\x20DNS\x20filter\x20name.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\
    \x04<\x02:\x12\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03<\x02%\n\x0c\n\x05\
    \x04\x03\x02\0\x01\x12\x03<&5\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03<89\n\
    G\n\x02\x04\x04\x12\x04@\0F\x01\x1a;\x20Runtime\x20derived\x20uint32\x20\
    with\x20a\x20default\x20when\x20not\x20specified.\n\n\n\n\x03\x04\x04\
    \x01\x12\x03@\x08\x15\n?\n\x04\x04\x04\x02\0\x12\x03B\x02\x1b\x1a2\x20De\
    fault\x20value\x20if\x20runtime\x20value\x20is\x20not\x20available.\n\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04B\x02@\x17\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03B\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03B\t\x16\n\x0c\
    \n\x05\x04\x04\x02\0\x03\x12\x03B\x19\x1a\nW\n\x04\x04\x04\x02\x01\x12\
    \x03E\x02\x19\x1aJ\x20Runtime\x20key\x20to\x20get\x20value\x20for\x20com\
    parision.\x20This\x20value\x20is\x20used\x20if\x20defined.\n\n\r\n\x05\
    \x04\x04\x02\x01\x04\x12\x04E\x02B\x1b\n\x0c\n\x05\x04\x04\x02\x01\x05\
    \x12\x03E\x02\x08\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03E\t\x14\n\x0c\n\
    \x05\x04\x04\x02\x01\x03\x12\x03E\x17\x18\n\xac\x03\n\x02\x05\0\x12\x04N\
    \0Q\x01\x1a\x9f\x03\x20Envoy\x20supports\x20upstream\x20priority\x20rout\
    ing\x20both\x20at\x20the\x20route\x20and\x20the\x20virtual\n\x20cluster\
    \x20level.\x20The\x20current\x20priority\x20implementation\x20uses\x20di\
    fferent\x20connection\n\x20pool\x20and\x20circuit\x20breaking\x20setting\
    s\x20for\x20each\x20priority\x20level.\x20This\x20means\x20that\n\x20eve\
    n\x20for\x20HTTP/2\x20requests,\x20two\x20physical\x20connections\x20wil\
    l\x20be\x20used\x20to\x20an\n\x20upstream\x20host.\x20In\x20the\x20futur\
    e\x20Envoy\x20will\x20likely\x20support\x20true\x20HTTP/2\x20priority\n\
    \x20over\x20a\x20single\x20upstream\x20connection.\n\n\n\n\x03\x05\0\x01\
    \x12\x03N\x05\x14\n\x0b\n\x04\x05\0\x02\0\x12\x03O\x02\x0e\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03O\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03O\x0c\
    \r\n\x0b\n\x04\x05\0\x02\x01\x12\x03P\x02\x0b\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03P\x02\x06\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03P\t\n\n!\n\
    \x02\x05\x01\x12\x04T\0^\x01\x1a\x15\x20HTTP\x20request\x20method\n\n\n\
    \n\x03\x05\x01\x01\x12\x03T\x05\x12\n\x0b\n\x04\x05\x01\x02\0\x12\x03U\
    \x02\x19\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03U\x02\x14\n\x0c\n\x05\x05\
    \x01\x02\0\x02\x12\x03U\x17\x18\n\x0b\n\x04\x05\x01\x02\x01\x12\x03V\x02\
    \n\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03V\x02\x05\n\x0c\n\x05\x05\x01\
    \x02\x01\x02\x12\x03V\x08\t\n\x0b\n\x04\x05\x01\x02\x02\x12\x03W\x02\x0b\
    \n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03W\x02\x06\n\x0c\n\x05\x05\x01\
    \x02\x02\x02\x12\x03W\t\n\n\x0b\n\x04\x05\x01\x02\x03\x12\x03X\x02\x0b\n\
    \x0c\n\x05\x05\x01\x02\x03\x01\x12\x03X\x02\x06\n\x0c\n\x05\x05\x01\x02\
    \x03\x02\x12\x03X\t\n\n\x0b\n\x04\x05\x01\x02\x04\x12\x03Y\x02\n\n\x0c\n\
    \x05\x05\x01\x02\x04\x01\x12\x03Y\x02\x05\n\x0c\n\x05\x05\x01\x02\x04\
    \x02\x12\x03Y\x08\t\n\x0b\n\x04\x05\x01\x02\x05\x12\x03Z\x02\r\n\x0c\n\
    \x05\x05\x01\x02\x05\x01\x12\x03Z\x02\x08\n\x0c\n\x05\x05\x01\x02\x05\
    \x02\x12\x03Z\x0b\x0c\n\x0b\n\x04\x05\x01\x02\x06\x12\x03[\x02\x0e\n\x0c\
    \n\x05\x05\x01\x02\x06\x01\x12\x03[\x02\t\n\x0c\n\x05\x05\x01\x02\x06\
    \x02\x12\x03[\x0c\r\n\x0b\n\x04\x05\x01\x02\x07\x12\x03\\\x02\x0e\n\x0c\
    \n\x05\x05\x01\x02\x07\x01\x12\x03\\\x02\t\n\x0c\n\x05\x05\x01\x02\x07\
    \x02\x12\x03\\\x0c\r\n\x0b\n\x04\x05\x01\x02\x08\x12\x03]\x02\x0c\n\x0c\
    \n\x05\x05\x01\x02\x08\x01\x12\x03]\x02\x07\n\x0c\n\x05\x05\x01\x02\x08\
    \x02\x12\x03]\n\x0b\n%\n\x02\x04\x05\x12\x04a\0g\x01\x1a\x19\x20Header\
    \x20name/value\x20pair.\n\n\n\n\x03\x04\x05\x01\x12\x03a\x08\x13\n\x1b\n\
    \x04\x04\x05\x02\0\x12\x03c\x02\x11\x1a\x0e\x20Header\x20name.\n\n\r\n\
    \x05\x04\x05\x02\0\x04\x12\x04c\x02a\x15\n\x0c\n\x05\x04\x05\x02\0\x05\
    \x12\x03c\x02\x08\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03c\t\x0c\n\x0c\n\
    \x05\x04\x05\x02\0\x03\x12\x03c\x0f\x10\n\x1c\n\x04\x04\x05\x02\x01\x12\
    \x03f\x02\x13\x1a\x0f\x20Header\x20value.\n\n\r\n\x05\x04\x05\x02\x01\
    \x04\x12\x04f\x02c\x11\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03f\x02\x08\
    \n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03f\t\x0e\n\x0c\n\x05\x04\x05\x02\
    \x01\x03\x12\x03f\x11\x12\nL\n\x02\x04\x06\x12\x04j\0u\x01\x1a@\x20Heade\
    r\x20name/value\x20pair\x20plus\x20option\x20to\x20control\x20append\x20\
    behavior.\n\n\n\n\x03\x04\x06\x01\x12\x03j\x08\x19\n\xed\x01\n\x04\x04\
    \x06\x02\0\x12\x03p\x02\x19\x1a\xdf\x01\x20Header\x20Name/Value\x20pair\
    \x20that\x20this\x20option\x20applies\x20to.\n\n\x20The\x20same\x20forma\
    t\x20specifier\x20as\x20used\x20for\x20HTTP\x20access\x20logging\x20appl\
    ies\x20here,\n\x20however\x20unknown\x20header\x20values\x20are\x20repla\
    ced\x20with\x20the\x20empty\x20string\x20instead\n\x20of\x20-.\x20[V2-AP\
    I-DIFF].\n\n\r\n\x05\x04\x06\x02\0\x04\x12\x04p\x02j\x1b\n\x0c\n\x05\x04\
    \x06\x02\0\x06\x12\x03p\x02\r\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03p\x0e\
    \x14\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03p\x17\x18\nt\n\x04\x04\x06\x02\
    \x01\x12\x03t\x02'\x1ag\x20Should\x20the\x20value\x20be\x20appended?\x20\
    If\x20false\x20(default),\x20the\x20value\x20overrides\n\x20existing\x20\
    values\x20[V2-API-DIFF].\n\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04t\x02p\
    \x19\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03t\x02\x1b\n\x0c\n\x05\x04\
    \x06\x02\x01\x01\x12\x03t\x1c\"\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03t\
    %&\n\x0b\n\x02\x04\x07\x12\x05w\0\x85\x01\x01\n\n\n\x03\x04\x07\x01\x12\
    \x03w\x08\x17\n<\n\x04\x04\x07\x04\0\x12\x04y\x02~\x03\x1a.\x20APIs\x20m\
    ay\x20be\x20fetched\x20via\x20either\x20REST\x20or\x20gRPC.\n\n\x0c\n\
    \x05\x04\x07\x04\0\x01\x12\x03y\x07\x0e\n7\n\x06\x04\x07\x04\0\x02\0\x12\
    \x03{\x04\x14\x1a(\x20REST\x20legacy\x20corresponds\x20to\x20the\x20v1\
    \x20API.\n\n\x0e\n\x07\x04\x07\x04\0\x02\0\x01\x12\x03{\x04\x0f\n\x0e\n\
    \x07\x04\x07\x04\0\x02\0\x02\x12\x03{\x12\x13\n\r\n\x06\x04\x07\x04\0\
    \x02\x01\x12\x03|\x04\r\n\x0e\n\x07\x04\x07\x04\0\x02\x01\x01\x12\x03|\
    \x04\x08\n\x0e\n\x07\x04\x07\x04\0\x02\x01\x02\x12\x03|\x0b\x0c\n\r\n\
    \x06\x04\x07\x04\0\x02\x02\x12\x03}\x04\r\n\x0e\n\x07\x04\x07\x04\0\x02\
    \x02\x01\x12\x03}\x04\x08\n\x0e\n\x07\x04\x07\x04\0\x02\x02\x02\x12\x03}\
    \x0b\x0c\n\x0b\n\x04\x04\x07\x02\0\x12\x03\x7f\x02\x17\n\r\n\x05\x04\x07\
    \x02\0\x04\x12\x04\x7f\x02~\x03\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03\
    \x7f\x02\t\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03\x7f\n\x12\n\x0c\n\x05\
    \x04\x07\x02\0\x03\x12\x03\x7f\x15\x16\n\x93\x01\n\x04\x04\x07\x02\x01\
    \x12\x04\x82\x01\x02#\x1a\x84\x01\x20Multiple\x20cluster\x20names\x20may\
    \x20be\x20provided.\x20If\x20>\x201\x20cluster\x20is\x20defined,\x20clus\
    ters\n\x20will\x20be\x20cycled\x20through\x20if\x20any\x20kind\x20of\x20\
    failure\x20occurs.\n\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\x82\x01\x02\n\
    \n\r\n\x05\x04\x07\x02\x01\x05\x12\x04\x82\x01\x0b\x11\n\r\n\x05\x04\x07\
    \x02\x01\x01\x12\x04\x82\x01\x12\x1e\n\r\n\x05\x04\x07\x02\x01\x03\x12\
    \x04\x82\x01!\"\nB\n\x04\x04\x07\x02\x02\x12\x04\x84\x01\x02-\x1a4\x20Fo\
    r\x20REST\x20APIs,\x20the\x20delay\x20between\x20successive\x20polls.\n\
    \n\x0f\n\x05\x04\x07\x02\x02\x04\x12\x06\x84\x01\x02\x82\x01#\n\r\n\x05\
    \x04\x07\x02\x02\x06\x12\x04\x84\x01\x02\x1a\n\r\n\x05\x04\x07\x02\x02\
    \x01\x12\x04\x84\x01\x1b(\n\r\n\x05\x04\x07\x02\x02\x03\x12\x04\x84\x01+\
    ,\n4\n\x02\x04\x08\x12\x06\x88\x01\0\x89\x01\x01\x1a&\x20ADS\x20will\x20\
    be\x20used\x20to\x20fetch\x20resources.\n\n\x0b\n\x03\x04\x08\x01\x12\
    \x04\x88\x01\x08\x1e\n\xca\x01\n\x02\x04\t\x12\x06\x8e\x01\0\x94\x01\x01\
    \x1a\xbb\x01\x20Configuration\x20for\x20listeners,\x20clusters,\x20route\
    s,\x20endpoints\x20etc.\x20may\x20either\x20be\n\x20sourced\x20from\x20t\
    he\x20filesystem\x20or\x20from\x20an\x20API\x20source.\x20Filesystem\x20\
    configs\x20are\n\x20watched\x20with\x20inotify\x20for\x20updates.\n\n\
    \x0b\n\x03\x04\t\x01\x12\x04\x8e\x01\x08\x14\n\x0e\n\x04\x04\t\x08\0\x12\
    \x06\x8f\x01\x02\x93\x01\x03\n\r\n\x05\x04\t\x08\0\x01\x12\x04\x8f\x01\
    \x08\x1f\n\x0c\n\x04\x04\t\x02\0\x12\x04\x90\x01\x04\x14\n\r\n\x05\x04\t\
    \x02\0\x05\x12\x04\x90\x01\x04\n\n\r\n\x05\x04\t\x02\0\x01\x12\x04\x90\
    \x01\x0b\x0f\n\r\n\x05\x04\t\x02\0\x03\x12\x04\x90\x01\x12\x13\n\x0c\n\
    \x04\x04\t\x02\x01\x12\x04\x91\x01\x04*\n\r\n\x05\x04\t\x02\x01\x06\x12\
    \x04\x91\x01\x04\x13\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\x91\x01\x14%\n\
    \r\n\x05\x04\t\x02\x01\x03\x12\x04\x91\x01()\n\x0c\n\x04\x04\t\x02\x02\
    \x12\x04\x92\x01\x04#\n\r\n\x05\x04\t\x02\x02\x06\x12\x04\x92\x01\x04\
    \x1a\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\x92\x01\x1b\x1e\n\r\n\x05\x04\t\
    \x02\x02\x03\x12\x04\x92\x01!\"b\x06proto3\
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
