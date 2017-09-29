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
pub struct Filter {
    // message fields
    pub name: ::std::string::String,
    pub config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    pub deprecated_v1: ::protobuf::SingularPtrField<Filter_DeprecatedV1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Filter {}

impl Filter {
    pub fn new() -> Filter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Filter {
        static mut instance: ::protobuf::lazy::Lazy<Filter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Filter,
        };
        unsafe {
            instance.get(Filter::new)
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

    // .google.protobuf.Struct config = 2;

    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::protobuf::well_known_types::Struct {
        self.config.take().unwrap_or_else(|| ::protobuf::well_known_types::Struct::new())
    }

    pub fn get_config(&self) -> &::protobuf::well_known_types::Struct {
        self.config.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Struct::default_instance())
    }

    fn get_config_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &self.config
    }

    fn mut_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct> {
        &mut self.config
    }

    // .envoy.api.v2.Filter.DeprecatedV1 deprecated_v1 = 3;

    pub fn clear_deprecated_v1(&mut self) {
        self.deprecated_v1.clear();
    }

    pub fn has_deprecated_v1(&self) -> bool {
        self.deprecated_v1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated_v1(&mut self, v: Filter_DeprecatedV1) {
        self.deprecated_v1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated_v1(&mut self) -> &mut Filter_DeprecatedV1 {
        if self.deprecated_v1.is_none() {
            self.deprecated_v1.set_default();
        }
        self.deprecated_v1.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated_v1(&mut self) -> Filter_DeprecatedV1 {
        self.deprecated_v1.take().unwrap_or_else(|| Filter_DeprecatedV1::new())
    }

    pub fn get_deprecated_v1(&self) -> &Filter_DeprecatedV1 {
        self.deprecated_v1.as_ref().unwrap_or_else(|| Filter_DeprecatedV1::default_instance())
    }

    fn get_deprecated_v1_for_reflect(&self) -> &::protobuf::SingularPtrField<Filter_DeprecatedV1> {
        &self.deprecated_v1
    }

    fn mut_deprecated_v1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Filter_DeprecatedV1> {
        &mut self.deprecated_v1
    }
}

impl ::protobuf::Message for Filter {
    fn is_initialized(&self) -> bool {
        for v in &self.config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deprecated_v1 {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deprecated_v1)?;
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
        if let Some(ref v) = self.config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.deprecated_v1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.deprecated_v1.as_ref() {
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

impl ::protobuf::MessageStatic for Filter {
    fn new() -> Filter {
        Filter::new()
    }

    fn descriptor_static(_: ::std::option::Option<Filter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Filter::get_name_for_reflect,
                    Filter::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "config",
                    Filter::get_config_for_reflect,
                    Filter::mut_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Filter_DeprecatedV1>>(
                    "deprecated_v1",
                    Filter::get_deprecated_v1_for_reflect,
                    Filter::mut_deprecated_v1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Filter>(
                    "Filter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Filter {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_config();
        self.clear_deprecated_v1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Filter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Filter_DeprecatedV1 {
    // message fields
    pub field_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Filter_DeprecatedV1 {}

impl Filter_DeprecatedV1 {
    pub fn new() -> Filter_DeprecatedV1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Filter_DeprecatedV1 {
        static mut instance: ::protobuf::lazy::Lazy<Filter_DeprecatedV1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Filter_DeprecatedV1,
        };
        unsafe {
            instance.get(Filter_DeprecatedV1::new)
        }
    }

    // string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::string::String {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }
}

impl ::protobuf::Message for Filter_DeprecatedV1 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.field_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(1, &self.field_type)?;
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

impl ::protobuf::MessageStatic for Filter_DeprecatedV1 {
    fn new() -> Filter_DeprecatedV1 {
        Filter_DeprecatedV1::new()
    }

    fn descriptor_static(_: ::std::option::Option<Filter_DeprecatedV1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Filter_DeprecatedV1::get_field_type_for_reflect,
                    Filter_DeprecatedV1::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Filter_DeprecatedV1>(
                    "Filter_DeprecatedV1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Filter_DeprecatedV1 {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Filter_DeprecatedV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Filter_DeprecatedV1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FilterChainMatch {
    // message fields
    pub sni_domains: ::protobuf::RepeatedField<::std::string::String>,
    pub prefix_ranges: ::protobuf::RepeatedField<FilterChainMatch_CidrRange>,
    pub address_suffix: ::std::string::String,
    pub suffix_len: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub source_prefix_ranges: ::protobuf::RepeatedField<FilterChainMatch_CidrRange>,
    pub source_ports: ::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value>,
    pub destination_port: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilterChainMatch {}

impl FilterChainMatch {
    pub fn new() -> FilterChainMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilterChainMatch {
        static mut instance: ::protobuf::lazy::Lazy<FilterChainMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilterChainMatch,
        };
        unsafe {
            instance.get(FilterChainMatch::new)
        }
    }

    // repeated string sni_domains = 1;

    pub fn clear_sni_domains(&mut self) {
        self.sni_domains.clear();
    }

    // Param is passed by value, moved
    pub fn set_sni_domains(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.sni_domains = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sni_domains(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.sni_domains
    }

    // Take field
    pub fn take_sni_domains(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.sni_domains, ::protobuf::RepeatedField::new())
    }

    pub fn get_sni_domains(&self) -> &[::std::string::String] {
        &self.sni_domains
    }

    fn get_sni_domains_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.sni_domains
    }

    fn mut_sni_domains_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.sni_domains
    }

    // repeated .envoy.api.v2.FilterChainMatch.CidrRange prefix_ranges = 3;

    pub fn clear_prefix_ranges(&mut self) {
        self.prefix_ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_prefix_ranges(&mut self, v: ::protobuf::RepeatedField<FilterChainMatch_CidrRange>) {
        self.prefix_ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_prefix_ranges(&mut self) -> &mut ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &mut self.prefix_ranges
    }

    // Take field
    pub fn take_prefix_ranges(&mut self) -> ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        ::std::mem::replace(&mut self.prefix_ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_prefix_ranges(&self) -> &[FilterChainMatch_CidrRange] {
        &self.prefix_ranges
    }

    fn get_prefix_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &self.prefix_ranges
    }

    fn mut_prefix_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &mut self.prefix_ranges
    }

    // string address_suffix = 4;

    pub fn clear_address_suffix(&mut self) {
        self.address_suffix.clear();
    }

    // Param is passed by value, moved
    pub fn set_address_suffix(&mut self, v: ::std::string::String) {
        self.address_suffix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address_suffix(&mut self) -> &mut ::std::string::String {
        &mut self.address_suffix
    }

    // Take field
    pub fn take_address_suffix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address_suffix, ::std::string::String::new())
    }

    pub fn get_address_suffix(&self) -> &str {
        &self.address_suffix
    }

    fn get_address_suffix_for_reflect(&self) -> &::std::string::String {
        &self.address_suffix
    }

    fn mut_address_suffix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address_suffix
    }

    // .google.protobuf.UInt32Value suffix_len = 5;

    pub fn clear_suffix_len(&mut self) {
        self.suffix_len.clear();
    }

    pub fn has_suffix_len(&self) -> bool {
        self.suffix_len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suffix_len(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.suffix_len = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suffix_len(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.suffix_len.is_none() {
            self.suffix_len.set_default();
        }
        self.suffix_len.as_mut().unwrap()
    }

    // Take field
    pub fn take_suffix_len(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.suffix_len.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_suffix_len(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.suffix_len.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_suffix_len_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.suffix_len
    }

    fn mut_suffix_len_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.suffix_len
    }

    // repeated .envoy.api.v2.FilterChainMatch.CidrRange source_prefix_ranges = 6;

    pub fn clear_source_prefix_ranges(&mut self) {
        self.source_prefix_ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_source_prefix_ranges(&mut self, v: ::protobuf::RepeatedField<FilterChainMatch_CidrRange>) {
        self.source_prefix_ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_source_prefix_ranges(&mut self) -> &mut ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &mut self.source_prefix_ranges
    }

    // Take field
    pub fn take_source_prefix_ranges(&mut self) -> ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        ::std::mem::replace(&mut self.source_prefix_ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_source_prefix_ranges(&self) -> &[FilterChainMatch_CidrRange] {
        &self.source_prefix_ranges
    }

    fn get_source_prefix_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &self.source_prefix_ranges
    }

    fn mut_source_prefix_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FilterChainMatch_CidrRange> {
        &mut self.source_prefix_ranges
    }

    // repeated .google.protobuf.UInt32Value source_ports = 7;

    pub fn clear_source_ports(&mut self) {
        self.source_ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_source_ports(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value>) {
        self.source_ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_source_ports(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value> {
        &mut self.source_ports
    }

    // Take field
    pub fn take_source_ports(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value> {
        ::std::mem::replace(&mut self.source_ports, ::protobuf::RepeatedField::new())
    }

    pub fn get_source_ports(&self) -> &[::protobuf::well_known_types::UInt32Value] {
        &self.source_ports
    }

    fn get_source_ports_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value> {
        &self.source_ports
    }

    fn mut_source_ports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::UInt32Value> {
        &mut self.source_ports
    }

    // .google.protobuf.UInt32Value destination_port = 8;

    pub fn clear_destination_port(&mut self) {
        self.destination_port.clear();
    }

    pub fn has_destination_port(&self) -> bool {
        self.destination_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destination_port(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.destination_port = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destination_port(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.destination_port.is_none() {
            self.destination_port.set_default();
        }
        self.destination_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_destination_port(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.destination_port.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_destination_port(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.destination_port.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_destination_port_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.destination_port
    }

    fn mut_destination_port_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.destination_port
    }
}

impl ::protobuf::Message for FilterChainMatch {
    fn is_initialized(&self) -> bool {
        for v in &self.prefix_ranges {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.suffix_len {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_prefix_ranges {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_ports {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.destination_port {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.sni_domains)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.prefix_ranges)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address_suffix)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.suffix_len)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.source_prefix_ranges)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.source_ports)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destination_port)?;
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
        for value in &self.sni_domains {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.prefix_ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.address_suffix.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.address_suffix);
        }
        if let Some(ref v) = self.suffix_len.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.source_prefix_ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.source_ports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.destination_port.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.sni_domains {
            os.write_string(1, &v)?;
        };
        for v in &self.prefix_ranges {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.address_suffix.is_empty() {
            os.write_string(4, &self.address_suffix)?;
        }
        if let Some(ref v) = self.suffix_len.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.source_prefix_ranges {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.source_ports {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.destination_port.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for FilterChainMatch {
    fn new() -> FilterChainMatch {
        FilterChainMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilterChainMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sni_domains",
                    FilterChainMatch::get_sni_domains_for_reflect,
                    FilterChainMatch::mut_sni_domains_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FilterChainMatch_CidrRange>>(
                    "prefix_ranges",
                    FilterChainMatch::get_prefix_ranges_for_reflect,
                    FilterChainMatch::mut_prefix_ranges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address_suffix",
                    FilterChainMatch::get_address_suffix_for_reflect,
                    FilterChainMatch::mut_address_suffix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "suffix_len",
                    FilterChainMatch::get_suffix_len_for_reflect,
                    FilterChainMatch::mut_suffix_len_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FilterChainMatch_CidrRange>>(
                    "source_prefix_ranges",
                    FilterChainMatch::get_source_prefix_ranges_for_reflect,
                    FilterChainMatch::mut_source_prefix_ranges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "source_ports",
                    FilterChainMatch::get_source_ports_for_reflect,
                    FilterChainMatch::mut_source_ports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "destination_port",
                    FilterChainMatch::get_destination_port_for_reflect,
                    FilterChainMatch::mut_destination_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FilterChainMatch>(
                    "FilterChainMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilterChainMatch {
    fn clear(&mut self) {
        self.clear_sni_domains();
        self.clear_prefix_ranges();
        self.clear_address_suffix();
        self.clear_suffix_len();
        self.clear_source_prefix_ranges();
        self.clear_source_ports();
        self.clear_destination_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilterChainMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilterChainMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FilterChainMatch_CidrRange {
    // message fields
    pub address_prefix: ::std::string::String,
    pub prefix_len: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilterChainMatch_CidrRange {}

impl FilterChainMatch_CidrRange {
    pub fn new() -> FilterChainMatch_CidrRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilterChainMatch_CidrRange {
        static mut instance: ::protobuf::lazy::Lazy<FilterChainMatch_CidrRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilterChainMatch_CidrRange,
        };
        unsafe {
            instance.get(FilterChainMatch_CidrRange::new)
        }
    }

    // string address_prefix = 1;

    pub fn clear_address_prefix(&mut self) {
        self.address_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_address_prefix(&mut self, v: ::std::string::String) {
        self.address_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.address_prefix
    }

    // Take field
    pub fn take_address_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address_prefix, ::std::string::String::new())
    }

    pub fn get_address_prefix(&self) -> &str {
        &self.address_prefix
    }

    fn get_address_prefix_for_reflect(&self) -> &::std::string::String {
        &self.address_prefix
    }

    fn mut_address_prefix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address_prefix
    }

    // .google.protobuf.UInt32Value prefix_len = 2;

    pub fn clear_prefix_len(&mut self) {
        self.prefix_len.clear();
    }

    pub fn has_prefix_len(&self) -> bool {
        self.prefix_len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix_len(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.prefix_len = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix_len(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.prefix_len.is_none() {
            self.prefix_len.set_default();
        }
        self.prefix_len.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix_len(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.prefix_len.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_prefix_len(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.prefix_len.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_prefix_len_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.prefix_len
    }

    fn mut_prefix_len_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.prefix_len
    }
}

impl ::protobuf::Message for FilterChainMatch_CidrRange {
    fn is_initialized(&self) -> bool {
        for v in &self.prefix_len {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address_prefix)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prefix_len)?;
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
        if !self.address_prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.address_prefix);
        }
        if let Some(ref v) = self.prefix_len.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.address_prefix.is_empty() {
            os.write_string(1, &self.address_prefix)?;
        }
        if let Some(ref v) = self.prefix_len.as_ref() {
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

impl ::protobuf::MessageStatic for FilterChainMatch_CidrRange {
    fn new() -> FilterChainMatch_CidrRange {
        FilterChainMatch_CidrRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilterChainMatch_CidrRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address_prefix",
                    FilterChainMatch_CidrRange::get_address_prefix_for_reflect,
                    FilterChainMatch_CidrRange::mut_address_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "prefix_len",
                    FilterChainMatch_CidrRange::get_prefix_len_for_reflect,
                    FilterChainMatch_CidrRange::mut_prefix_len_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FilterChainMatch_CidrRange>(
                    "FilterChainMatch_CidrRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilterChainMatch_CidrRange {
    fn clear(&mut self) {
        self.clear_address_prefix();
        self.clear_prefix_len();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilterChainMatch_CidrRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilterChainMatch_CidrRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FilterChain {
    // message fields
    pub filter_chain_match: ::protobuf::SingularPtrField<FilterChainMatch>,
    pub tls_context: ::protobuf::SingularPtrField<super::tls_context::DownstreamTlsContext>,
    pub filters: ::protobuf::RepeatedField<Filter>,
    pub use_proxy_proto: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub metadata: ::protobuf::SingularPtrField<super::base::Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilterChain {}

impl FilterChain {
    pub fn new() -> FilterChain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilterChain {
        static mut instance: ::protobuf::lazy::Lazy<FilterChain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilterChain,
        };
        unsafe {
            instance.get(FilterChain::new)
        }
    }

    // .envoy.api.v2.FilterChainMatch filter_chain_match = 1;

    pub fn clear_filter_chain_match(&mut self) {
        self.filter_chain_match.clear();
    }

    pub fn has_filter_chain_match(&self) -> bool {
        self.filter_chain_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter_chain_match(&mut self, v: FilterChainMatch) {
        self.filter_chain_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter_chain_match(&mut self) -> &mut FilterChainMatch {
        if self.filter_chain_match.is_none() {
            self.filter_chain_match.set_default();
        }
        self.filter_chain_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter_chain_match(&mut self) -> FilterChainMatch {
        self.filter_chain_match.take().unwrap_or_else(|| FilterChainMatch::new())
    }

    pub fn get_filter_chain_match(&self) -> &FilterChainMatch {
        self.filter_chain_match.as_ref().unwrap_or_else(|| FilterChainMatch::default_instance())
    }

    fn get_filter_chain_match_for_reflect(&self) -> &::protobuf::SingularPtrField<FilterChainMatch> {
        &self.filter_chain_match
    }

    fn mut_filter_chain_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FilterChainMatch> {
        &mut self.filter_chain_match
    }

    // .envoy.api.v2.DownstreamTlsContext tls_context = 2;

    pub fn clear_tls_context(&mut self) {
        self.tls_context.clear();
    }

    pub fn has_tls_context(&self) -> bool {
        self.tls_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tls_context(&mut self, v: super::tls_context::DownstreamTlsContext) {
        self.tls_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_context(&mut self) -> &mut super::tls_context::DownstreamTlsContext {
        if self.tls_context.is_none() {
            self.tls_context.set_default();
        }
        self.tls_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_tls_context(&mut self) -> super::tls_context::DownstreamTlsContext {
        self.tls_context.take().unwrap_or_else(|| super::tls_context::DownstreamTlsContext::new())
    }

    pub fn get_tls_context(&self) -> &super::tls_context::DownstreamTlsContext {
        self.tls_context.as_ref().unwrap_or_else(|| super::tls_context::DownstreamTlsContext::default_instance())
    }

    fn get_tls_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tls_context::DownstreamTlsContext> {
        &self.tls_context
    }

    fn mut_tls_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tls_context::DownstreamTlsContext> {
        &mut self.tls_context
    }

    // repeated .envoy.api.v2.Filter filters = 3;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: ::protobuf::RepeatedField<Filter>) {
        self.filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filters(&mut self) -> &mut ::protobuf::RepeatedField<Filter> {
        &mut self.filters
    }

    // Take field
    pub fn take_filters(&mut self) -> ::protobuf::RepeatedField<Filter> {
        ::std::mem::replace(&mut self.filters, ::protobuf::RepeatedField::new())
    }

    pub fn get_filters(&self) -> &[Filter] {
        &self.filters
    }

    fn get_filters_for_reflect(&self) -> &::protobuf::RepeatedField<Filter> {
        &self.filters
    }

    fn mut_filters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Filter> {
        &mut self.filters
    }

    // .google.protobuf.BoolValue use_proxy_proto = 4;

    pub fn clear_use_proxy_proto(&mut self) {
        self.use_proxy_proto.clear();
    }

    pub fn has_use_proxy_proto(&self) -> bool {
        self.use_proxy_proto.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_proxy_proto(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.use_proxy_proto = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_use_proxy_proto(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.use_proxy_proto.is_none() {
            self.use_proxy_proto.set_default();
        }
        self.use_proxy_proto.as_mut().unwrap()
    }

    // Take field
    pub fn take_use_proxy_proto(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.use_proxy_proto.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_use_proxy_proto(&self) -> &::protobuf::well_known_types::BoolValue {
        self.use_proxy_proto.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_use_proxy_proto_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.use_proxy_proto
    }

    fn mut_use_proxy_proto_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.use_proxy_proto
    }

    // .envoy.api.v2.Metadata metadata = 5;

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
}

impl ::protobuf::Message for FilterChain {
    fn is_initialized(&self) -> bool {
        for v in &self.filter_chain_match {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tls_context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.use_proxy_proto {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filter_chain_match)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_context)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.filters)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.use_proxy_proto)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.filter_chain_match.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.tls_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.filters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.use_proxy_proto.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.filter_chain_match.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.tls_context.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.filters {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.use_proxy_proto.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
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

impl ::protobuf::MessageStatic for FilterChain {
    fn new() -> FilterChain {
        FilterChain::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilterChain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FilterChainMatch>>(
                    "filter_chain_match",
                    FilterChain::get_filter_chain_match_for_reflect,
                    FilterChain::mut_filter_chain_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tls_context::DownstreamTlsContext>>(
                    "tls_context",
                    FilterChain::get_tls_context_for_reflect,
                    FilterChain::mut_tls_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Filter>>(
                    "filters",
                    FilterChain::get_filters_for_reflect,
                    FilterChain::mut_filters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "use_proxy_proto",
                    FilterChain::get_use_proxy_proto_for_reflect,
                    FilterChain::mut_use_proxy_proto_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata",
                    FilterChain::get_metadata_for_reflect,
                    FilterChain::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FilterChain>(
                    "FilterChain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilterChain {
    fn clear(&mut self) {
        self.clear_filter_chain_match();
        self.clear_tls_context();
        self.clear_filters();
        self.clear_use_proxy_proto();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilterChain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilterChain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Listener {
    // message fields
    pub name: ::std::string::String,
    pub address: ::protobuf::SingularPtrField<super::address::Address>,
    pub filter_chains: ::protobuf::RepeatedField<FilterChain>,
    pub use_original_dst: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub per_connection_buffer_limit_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub metadata: ::protobuf::SingularPtrField<super::base::Metadata>,
    pub deprecated_v1: ::protobuf::SingularPtrField<Listener_DeprecatedV1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Listener {}

impl Listener {
    pub fn new() -> Listener {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Listener {
        static mut instance: ::protobuf::lazy::Lazy<Listener> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Listener,
        };
        unsafe {
            instance.get(Listener::new)
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

    // .envoy.api.v2.Address address = 2;

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

    // repeated .envoy.api.v2.FilterChain filter_chains = 3;

    pub fn clear_filter_chains(&mut self) {
        self.filter_chains.clear();
    }

    // Param is passed by value, moved
    pub fn set_filter_chains(&mut self, v: ::protobuf::RepeatedField<FilterChain>) {
        self.filter_chains = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filter_chains(&mut self) -> &mut ::protobuf::RepeatedField<FilterChain> {
        &mut self.filter_chains
    }

    // Take field
    pub fn take_filter_chains(&mut self) -> ::protobuf::RepeatedField<FilterChain> {
        ::std::mem::replace(&mut self.filter_chains, ::protobuf::RepeatedField::new())
    }

    pub fn get_filter_chains(&self) -> &[FilterChain] {
        &self.filter_chains
    }

    fn get_filter_chains_for_reflect(&self) -> &::protobuf::RepeatedField<FilterChain> {
        &self.filter_chains
    }

    fn mut_filter_chains_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FilterChain> {
        &mut self.filter_chains
    }

    // .google.protobuf.BoolValue use_original_dst = 4;

    pub fn clear_use_original_dst(&mut self) {
        self.use_original_dst.clear();
    }

    pub fn has_use_original_dst(&self) -> bool {
        self.use_original_dst.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_original_dst(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.use_original_dst = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_use_original_dst(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.use_original_dst.is_none() {
            self.use_original_dst.set_default();
        }
        self.use_original_dst.as_mut().unwrap()
    }

    // Take field
    pub fn take_use_original_dst(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.use_original_dst.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_use_original_dst(&self) -> &::protobuf::well_known_types::BoolValue {
        self.use_original_dst.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_use_original_dst_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.use_original_dst
    }

    fn mut_use_original_dst_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.use_original_dst
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

    // .envoy.api.v2.Metadata metadata = 6;

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

    // .envoy.api.v2.Listener.DeprecatedV1 deprecated_v1 = 7;

    pub fn clear_deprecated_v1(&mut self) {
        self.deprecated_v1.clear();
    }

    pub fn has_deprecated_v1(&self) -> bool {
        self.deprecated_v1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated_v1(&mut self, v: Listener_DeprecatedV1) {
        self.deprecated_v1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated_v1(&mut self) -> &mut Listener_DeprecatedV1 {
        if self.deprecated_v1.is_none() {
            self.deprecated_v1.set_default();
        }
        self.deprecated_v1.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated_v1(&mut self) -> Listener_DeprecatedV1 {
        self.deprecated_v1.take().unwrap_or_else(|| Listener_DeprecatedV1::new())
    }

    pub fn get_deprecated_v1(&self) -> &Listener_DeprecatedV1 {
        self.deprecated_v1.as_ref().unwrap_or_else(|| Listener_DeprecatedV1::default_instance())
    }

    fn get_deprecated_v1_for_reflect(&self) -> &::protobuf::SingularPtrField<Listener_DeprecatedV1> {
        &self.deprecated_v1
    }

    fn mut_deprecated_v1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Listener_DeprecatedV1> {
        &mut self.deprecated_v1
    }
}

impl ::protobuf::Message for Listener {
    fn is_initialized(&self) -> bool {
        for v in &self.address {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filter_chains {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.use_original_dst {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.per_connection_buffer_limit_bytes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deprecated_v1 {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.filter_chains)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.use_original_dst)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.per_connection_buffer_limit_bytes)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deprecated_v1)?;
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
        if let Some(ref v) = self.address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.filter_chains {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.use_original_dst.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.per_connection_buffer_limit_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.deprecated_v1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.address.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.filter_chains {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.use_original_dst.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.per_connection_buffer_limit_bytes.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.deprecated_v1.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Listener {
    fn new() -> Listener {
        Listener::new()
    }

    fn descriptor_static(_: ::std::option::Option<Listener>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Listener::get_name_for_reflect,
                    Listener::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "address",
                    Listener::get_address_for_reflect,
                    Listener::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FilterChain>>(
                    "filter_chains",
                    Listener::get_filter_chains_for_reflect,
                    Listener::mut_filter_chains_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "use_original_dst",
                    Listener::get_use_original_dst_for_reflect,
                    Listener::mut_use_original_dst_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "per_connection_buffer_limit_bytes",
                    Listener::get_per_connection_buffer_limit_bytes_for_reflect,
                    Listener::mut_per_connection_buffer_limit_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata",
                    Listener::get_metadata_for_reflect,
                    Listener::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Listener_DeprecatedV1>>(
                    "deprecated_v1",
                    Listener::get_deprecated_v1_for_reflect,
                    Listener::mut_deprecated_v1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Listener>(
                    "Listener",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Listener {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_address();
        self.clear_filter_chains();
        self.clear_use_original_dst();
        self.clear_per_connection_buffer_limit_bytes();
        self.clear_metadata();
        self.clear_deprecated_v1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Listener {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Listener {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Listener_DeprecatedV1 {
    // message fields
    pub bind_to_port: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Listener_DeprecatedV1 {}

impl Listener_DeprecatedV1 {
    pub fn new() -> Listener_DeprecatedV1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Listener_DeprecatedV1 {
        static mut instance: ::protobuf::lazy::Lazy<Listener_DeprecatedV1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Listener_DeprecatedV1,
        };
        unsafe {
            instance.get(Listener_DeprecatedV1::new)
        }
    }

    // .google.protobuf.BoolValue bind_to_port = 1;

    pub fn clear_bind_to_port(&mut self) {
        self.bind_to_port.clear();
    }

    pub fn has_bind_to_port(&self) -> bool {
        self.bind_to_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bind_to_port(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.bind_to_port = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bind_to_port(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.bind_to_port.is_none() {
            self.bind_to_port.set_default();
        }
        self.bind_to_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_bind_to_port(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.bind_to_port.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_bind_to_port(&self) -> &::protobuf::well_known_types::BoolValue {
        self.bind_to_port.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_bind_to_port_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.bind_to_port
    }

    fn mut_bind_to_port_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.bind_to_port
    }
}

impl ::protobuf::Message for Listener_DeprecatedV1 {
    fn is_initialized(&self) -> bool {
        for v in &self.bind_to_port {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bind_to_port)?;
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
        if let Some(ref v) = self.bind_to_port.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.bind_to_port.as_ref() {
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

impl ::protobuf::MessageStatic for Listener_DeprecatedV1 {
    fn new() -> Listener_DeprecatedV1 {
        Listener_DeprecatedV1::new()
    }

    fn descriptor_static(_: ::std::option::Option<Listener_DeprecatedV1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "bind_to_port",
                    Listener_DeprecatedV1::get_bind_to_port_for_reflect,
                    Listener_DeprecatedV1::mut_bind_to_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Listener_DeprecatedV1>(
                    "Listener_DeprecatedV1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Listener_DeprecatedV1 {
    fn clear(&mut self) {
        self.clear_bind_to_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Listener_DeprecatedV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Listener_DeprecatedV1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/lds.proto\x12\x0cenvoy.api.v2\x1a\x11api/address.proto\x1a\x0eap\
    i/base.proto\x1a\x13api/discovery.proto\x1a\x15api/tls_context.proto\x1a\
    \x1cgoogle/api/annotations.proto\x1a\x1cgoogle/protobuf/struct.proto\x1a\
    \x1egoogle/protobuf/wrappers.proto\"\xb9\x01\n\x06Filter\x12\x12\n\x04na\
    me\x18\x01\x20\x01(\tR\x04name\x12/\n\x06config\x18\x02\x20\x01(\x0b2\
    \x17.google.protobuf.StructR\x06config\x12F\n\rdeprecated_v1\x18\x03\x20\
    \x01(\x0b2!.envoy.api.v2.Filter.DeprecatedV1R\x0cdeprecatedV1\x1a\"\n\
    \x0cDeprecatedV1\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\"\xbd\x04\
    \n\x10FilterChainMatch\x12\x1f\n\x0bsni_domains\x18\x01\x20\x03(\tR\nsni\
    Domains\x12M\n\rprefix_ranges\x18\x03\x20\x03(\x0b2(.envoy.api.v2.Filter\
    ChainMatch.CidrRangeR\x0cprefixRanges\x12%\n\x0eaddress_suffix\x18\x04\
    \x20\x01(\tR\raddressSuffix\x12;\n\nsuffix_len\x18\x05\x20\x01(\x0b2\x1c\
    .google.protobuf.UInt32ValueR\tsuffixLen\x12Z\n\x14source_prefix_ranges\
    \x18\x06\x20\x03(\x0b2(.envoy.api.v2.FilterChainMatch.CidrRangeR\x12sour\
    cePrefixRanges\x12?\n\x0csource_ports\x18\x07\x20\x03(\x0b2\x1c.google.p\
    rotobuf.UInt32ValueR\x0bsourcePorts\x12G\n\x10destination_port\x18\x08\
    \x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x0fdestinationPort\x1ao\
    \n\tCidrRange\x12%\n\x0eaddress_prefix\x18\x01\x20\x01(\tR\raddressPrefi\
    x\x12;\n\nprefix_len\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UInt32Val\
    ueR\tprefixLen\"\xc8\x02\n\x0bFilterChain\x12L\n\x12filter_chain_match\
    \x18\x01\x20\x01(\x0b2\x1e.envoy.api.v2.FilterChainMatchR\x10filterChain\
    Match\x12C\n\x0btls_context\x18\x02\x20\x01(\x0b2\".envoy.api.v2.Downstr\
    eamTlsContextR\ntlsContext\x12.\n\x07filters\x18\x03\x20\x03(\x0b2\x14.e\
    nvoy.api.v2.FilterR\x07filters\x12B\n\x0fuse_proxy_proto\x18\x04\x20\x01\
    (\x0b2\x1a.google.protobuf.BoolValueR\ruseProxyProto\x122\n\x08metadata\
    \x18\x05\x20\x01(\x0b2\x16.envoy.api.v2.MetadataR\x08metadata\"\x89\x04\
    \n\x08Listener\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12/\n\x07a\
    ddress\x18\x02\x20\x01(\x0b2\x15.envoy.api.v2.AddressR\x07address\x12>\n\
    \rfilter_chains\x18\x03\x20\x03(\x0b2\x19.envoy.api.v2.FilterChainR\x0cf\
    ilterChains\x12D\n\x10use_original_dst\x18\x04\x20\x01(\x0b2\x1a.google.\
    protobuf.BoolValueR\x0euseOriginalDst\x12f\n!per_connection_buffer_limit\
    _bytes\x18\x05\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x1dperConn\
    ectionBufferLimitBytes\x122\n\x08metadata\x18\x06\x20\x01(\x0b2\x16.envo\
    y.api.v2.MetadataR\x08metadata\x12H\n\rdeprecated_v1\x18\x07\x20\x01(\
    \x0b2#.envoy.api.v2.Listener.DeprecatedV1R\x0cdeprecatedV1\x1aL\n\x0cDep\
    recatedV1\x12<\n\x0cbind_to_port\x18\x01\x20\x01(\x0b2\x1a.google.protob\
    uf.BoolValueR\nbindToPort2\xeb\x01\n\x18ListenerDiscoveryService\x12X\n\
    \x0fStreamListeners\x12\x1e.envoy.api.v2.DiscoveryRequest\x1a\x1f.envoy.\
    api.v2.DiscoveryResponse\"\0(\x010\x01\x12u\n\x0eFetchListeners\x12\x1e.\
    envoy.api.v2.DiscoveryRequest\x1a\x1f.envoy.api.v2.DiscoveryResponse\"\"\
    \x82\xd3\xe4\x93\x02\x1c\"\x17/v2/discovery:listeners:\x01*J\x8a6\n\x07\
    \x12\x05\x04\0\x9c\x01\x01\n\xaf\x01\n\x01\x0c\x12\x03\x04\0\x122\xa4\
    \x01\x20This\x20is\x20heavily\x20derived\x20from\n\x20https://lyft.githu\
    b.io/envoy/docs/configuration/listeners/listeners.html\n\x20The\x20v2\
    \x20gRPC\x20API\x20differences\x20are\x20tagged\x20with\x20[V2-API-DIFF]\
    .\n\n\x08\n\x01\x02\x12\x03\x06\x08\x14\n\t\n\x02\x03\0\x12\x03\x08\x07\
    \x1a\n\t\n\x02\x03\x01\x12\x03\t\x07\x17\n\t\n\x02\x03\x02\x12\x03\n\x07\
    \x1c\n\t\n\x02\x03\x03\x12\x03\x0b\x07\x1e\n\t\n\x02\x03\x04\x12\x03\r\
    \x07%\n\t\n\x02\x03\x05\x12\x03\x0e\x07%\n\t\n\x02\x03\x06\x12\x03\x0f\
    \x07'\n\xa5\x02\n\x02\x06\0\x12\x04\x15\0!\x01\x1a\x98\x02\x20The\x20Env\
    oy\x20instance\x20initiates\x20an\x20RPC\x20at\x20startup\x20to\x20disco\
    ver\x20a\x20list\x20of\n\x20listeners.\x20Updates\x20are\x20delivered\
    \x20via\x20streaming\x20from\x20the\x20LDS\x20server\x20and\n\x20consist\
    \x20of\x20a\x20complete\x20update\x20of\x20all\x20listeners.\x20Existing\
    \x20connections\x20will\x20be\n\x20allowed\x20to\x20drain\x20from\x20lis\
    teners\x20that\x20are\x20no\x20longer\x20present.\n\n\n\n\x03\x06\0\x01\
    \x12\x03\x15\x08\x20\n\x0c\n\x04\x06\0\x02\0\x12\x04\x16\x02\x18\x03\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x16\x06\x15\n\x0c\n\x05\x06\0\x02\0\
    \x05\x12\x03\x16\x16\x1c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x16\x1d-\n\
    \x0c\n\x05\x06\0\x02\0\x06\x12\x03\x17\x0f\x15\n\x0c\n\x05\x06\0\x02\0\
    \x03\x12\x03\x17\x16'\n\x0c\n\x04\x06\0\x02\x01\x12\x04\x1a\x02\x20\x03\
    \n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x1a\x06\x14\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x1a\x15%\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x1b\x0f\
    \x20\n\r\n\x05\x06\0\x02\x01\x04\x12\x04\x1c\x04\x1f\x06\n\x10\n\x08\x06\
    \0\x02\x01\x04\xe7\x07\0\x12\x04\x1c\x04\x1f\x06\n\x10\n\t\x06\0\x02\x01\
    \x04\xe7\x07\0\x02\x12\x03\x1c\x0b\x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\
    \x07\0\x02\0\x12\x03\x1c\x0b\x1c\n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\
    \x02\0\x01\x12\x03\x1c\x0c\x1b\n\x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\
    \x12\x04\x1c\x1f\x1f\x05\n\n\n\x02\x04\0\x12\x04#\0/\x01\n\n\n\x03\x04\0\
    \x01\x12\x03#\x08\x0e\n^\n\x04\x04\0\x02\0\x12\x03&\x02\x12\x1aQ\x20The\
    \x20name\x20of\x20the\x20filter\x20to\x20instantiate.\x20The\x20name\x20\
    must\x20match\x20a\x20supported\n\x20filter.\n\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04&\x02#\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03&\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03&\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03&\
    \x10\x11\n\x93\x01\n\x04\x04\0\x02\x01\x12\x03)\x02$\x1a\x85\x01\x20Filt\
    er\x20specific\x20configuration\x20which\x20depends\x20on\x20the\x20filt\
    er\x20being\n\x20instantiated.\x20See\x20the\x20supported\x20filters\x20\
    for\x20further\x20documentation.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04)\
    \x02&\x12\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03)\x02\x18\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03)\x19\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03)\"\
    #\n\x0c\n\x04\x04\0\x03\0\x12\x04+\x02-\x03\n\x0c\n\x05\x04\0\x03\0\x01\
    \x12\x03+\n\x16\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03,\x04\x14\n\x0f\n\x07\
    \x04\0\x03\0\x02\0\x04\x12\x04,\x04+\x18\n\x0e\n\x07\x04\0\x03\0\x02\0\
    \x05\x12\x03,\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03,\x0b\x0f\n\
    \x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03,\x12\x13\n\x0b\n\x04\x04\0\x02\
    \x02\x12\x03.\x02!\n\r\n\x05\x04\0\x02\x02\x04\x12\x04.\x02-\x03\n\x0c\n\
    \x05\x04\0\x02\x02\x06\x12\x03.\x02\x0e\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03.\x0f\x1c\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03.\x1f\x20\no\n\
    \x02\x04\x01\x12\x043\0S\x01\x1ac\x20Specifies\x20the\x20match\x20criter\
    ia\x20for\x20selecting\x20a\x20specific\x20filter\x20chain\x20for\x20a\n\
    \x20listener\x20[V2-API-DIFF].\n\n\n\n\x03\x04\x01\x01\x12\x033\x08\x18\
    \nm\n\x04\x04\x01\x02\0\x12\x036\x02\"\x1a`\x20If\x20non-empty,\x20the\
    \x20SNI\x20domains\x20to\x20consider.\x20May\x20contain\x20a\x20wildcard\
    \x20prefix,\n\x20e.g.\x20*.example.com.\n\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x036\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x036\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x036\x12\x1d\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x036\x20!\n\xa2\x01\n\x04\x04\x01\x03\0\x12\x04:\x02=\x03\x1a\x93\
    \x01\x20If\x20non-empty,\x20an\x20IP\x20address\x20and\x20prefix\x20leng\
    th\x20to\x20match\x20addresses\x20when\x20the\n\x20listener\x20is\x20bou\
    nd\x20to\x200.0.0.0/::\x20or\x20when\x20use_original_dst\x20is\x20specif\
    ied.\n\n\x0c\n\x05\x04\x01\x03\0\x01\x12\x03:\n\x13\n\r\n\x06\x04\x01\
    \x03\0\x02\0\x12\x03;\x04\x1e\n\x0f\n\x07\x04\x01\x03\0\x02\0\x04\x12\
    \x04;\x04:\x15\n\x0e\n\x07\x04\x01\x03\0\x02\0\x05\x12\x03;\x04\n\n\x0e\
    \n\x07\x04\x01\x03\0\x02\0\x01\x12\x03;\x0b\x19\n\x0e\n\x07\x04\x01\x03\
    \0\x02\0\x03\x12\x03;\x1c\x1d\n\r\n\x06\x04\x01\x03\0\x02\x01\x12\x03<\
    \x04/\n\x0f\n\x07\x04\x01\x03\0\x02\x01\x04\x12\x04<\x04;\x1e\n\x0e\n\
    \x07\x04\x01\x03\0\x02\x01\x06\x12\x03<\x04\x1f\n\x0e\n\x07\x04\x01\x03\
    \0\x02\x01\x01\x12\x03<\x20*\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x03\x12\
    \x03<-.\n\x0b\n\x04\x04\x01\x02\x01\x12\x03>\x02'\n\x0c\n\x05\x04\x01\
    \x02\x01\x04\x12\x03>\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03>\x0b\
    \x14\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03>\x15\"\n\x0c\n\x05\x04\x01\
    \x02\x01\x03\x12\x03>%&\n\xa1\x01\n\x04\x04\x01\x02\x02\x12\x03B\x02\x1c\
    \x1a\x93\x01\x20If\x20non-empty,\x20an\x20IP\x20address\x20and\x20suffix\
    \x20length\x20to\x20match\x20addresses\x20when\x20the\n\x20listener\x20i\
    s\x20bound\x20to\x200.0.0.0/::\x20or\x20when\x20use_original_dst\x20is\
    \x20specified.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04B\x02>'\n\x0c\n\
    \x05\x04\x01\x02\x02\x05\x12\x03B\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03B\t\x17\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03B\x1a\x1b\n\
    \x0b\n\x04\x04\x01\x02\x03\x12\x03C\x02-\n\r\n\x05\x04\x01\x02\x03\x04\
    \x12\x04C\x02B\x1c\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03C\x02\x1d\n\
    \x0c\n\x05\x04\x01\x02\x03\x01\x12\x03C\x1e(\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03C+,\n\xf3\x01\n\x04\x04\x01\x02\x04\x12\x03I\x02.\x1a\xe5\
    \x01\x20The\x20criteria\x20is\x20satisfied\x20if\x20the\x20source\x20IP\
    \x20address\x20of\x20the\x20downstream\n\x20connection\x20is\x20containe\
    d\x20in\x20at\x20least\x20one\x20of\x20the\x20specified\x20subnets.\x20I\
    f\x20the\n\x20parameter\x20is\x20not\x20specified\x20or\x20the\x20list\
    \x20is\x20empty,\x20the\x20source\x20IP\x20address\x20is\n\x20ignored.\n\
    \n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03I\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x04\x06\x12\x03I\x0b\x14\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03I\x15)\
    \n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03I,-\n\xcf\x01\n\x04\x04\x01\x02\
    \x05\x12\x03N\x028\x1a\xc1\x01\x20The\x20criteria\x20is\x20satisfied\x20\
    if\x20the\x20source\x20port\x20of\x20the\x20downstream\x20connection\n\
    \x20is\x20contained\x20in\x20at\x20least\x20one\x20of\x20the\x20specifie\
    d\x20ports.\x20If\x20the\x20parameter\x20is\n\x20not\x20specified,\x20th\
    e\x20source\x20port\x20is\x20ignored.\n\n\x0c\n\x05\x04\x01\x02\x05\x04\
    \x12\x03N\x02\n\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\x03N\x0b&\n\x0c\n\
    \x05\x04\x01\x02\x05\x01\x12\x03N'3\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\
    \x03N67\n\x87\x01\n\x04\x04\x01\x02\x06\x12\x03R\x023\x1az\x20Optional\
    \x20destination\x20port\x20to\x20consider\x20when\x20use_original_dst\
    \x20is\x20set\x20on\x20the\n\x20listener\x20in\x20determining\x20a\x20fi\
    lter\x20chain\x20match.\n\n\r\n\x05\x04\x01\x02\x06\x04\x12\x04R\x02N8\n\
    \x0c\n\x05\x04\x01\x02\x06\x06\x12\x03R\x02\x1d\n\x0c\n\x05\x04\x01\x02\
    \x06\x01\x12\x03R\x1e.\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03R12\n{\n\
    \x02\x04\x02\x12\x04W\0j\x01\x1ao\x20Grouping\x20of\x20FilterChainMatch\
    \x20criteria,\x20DownstreamTlsContext,\x20the\x20actual\x20filter\x20cha\
    in\n\x20and\x20related\x20parameters.\n\n\n\n\x03\x04\x02\x01\x12\x03W\
    \x08\x13\n\x0b\n\x04\x04\x02\x02\0\x12\x03X\x02*\n\r\n\x05\x04\x02\x02\0\
    \x04\x12\x04X\x02W\x15\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03X\x02\x12\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03X\x13%\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03X()\n\x0b\n\x04\x04\x02\x02\x01\x12\x03Y\x02'\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04Y\x02X*\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03Y\x02\
    \x16\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03Y\x17\"\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03Y%&\n\xa3\x02\n\x04\x04\x02\x02\x02\x12\x03^\x02\x1e\
    \x1a\x95\x02\x20A\x20list\x20of\x20individual\x20network\x20filters\x20t\
    hat\x20make\x20up\x20the\x20filter\x20chain\x20for\n\x20connections\x20e\
    stablished\x20with\x20the\x20listener.\x20Order\x20matters\x20as\x20the\
    \x20filters\x20are\n\x20processed\x20sequentially\x20as\x20connection\
    \x20events\x20happen.\x20\x20Note:\x20If\x20the\x20filter\n\x20list\x20i\
    s\x20empty,\x20the\x20connection\x20will\x20close\x20by\x20default.\n\n\
    \x0c\n\x05\x04\x02\x02\x02\x04\x12\x03^\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x02\x06\x12\x03^\x0b\x11\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03^\x12\
    \x19\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03^\x1c\x1d\n\xa3\x03\n\x04\
    \x04\x02\x02\x03\x12\x03f\x020\x1a\x95\x03\x20Whether\x20the\x20listener\
    \x20should\x20expect\x20a\x20PROXY\x20protocol\x20V1\x20header\x20on\x20\
    new\n\x20connections.\x20If\x20this\x20option\x20is\x20enabled,\x20the\
    \x20listener\x20will\x20assume\x20that\x20that\n\x20remote\x20address\
    \x20of\x20the\x20connection\x20is\x20the\x20one\x20specified\x20in\x20th\
    e\x20header.\x20Some\n\x20load\x20balancers\x20including\x20the\x20AWS\
    \x20ELB\x20support\x20this\x20option.\x20If\x20the\x20option\x20is\n\x20\
    absent\x20or\x20set\x20to\x20false,\x20Envoy\x20will\x20use\x20the\x20ph\
    ysical\x20peer\x20address\x20of\x20the\n\x20connection\x20as\x20the\x20r\
    emote\x20address.\n\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04f\x02^\x1e\n\
    \x0c\n\x05\x04\x02\x02\x03\x06\x12\x03f\x02\x1b\n\x0c\n\x05\x04\x02\x02\
    \x03\x01\x12\x03f\x1c+\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03f./\n-\n\
    \x04\x04\x02\x02\x04\x12\x03i\x02\x18\x1a\x20\x20See\x20base.Metadata\
    \x20description.\n\n\r\n\x05\x04\x02\x02\x04\x04\x12\x04i\x02f0\n\x0c\n\
    \x05\x04\x02\x02\x04\x06\x12\x03i\x02\n\n\x0c\n\x05\x04\x02\x02\x04\x01\
    \x12\x03i\x0b\x13\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03i\x16\x17\n\x0b\
    \n\x02\x04\x03\x12\x05l\0\x9c\x01\x01\n\n\n\x03\x04\x03\x01\x12\x03l\x08\
    \x10\n\xc2\x01\n\x04\x04\x03\x02\0\x12\x03p\x02\x12\x1a\xb4\x01\x20The\
    \x20unique\x20name\x20of\x20the\x20listener.\x20If\x20no\x20name\x20is\
    \x20provided,\x20Envoy\x20will\x20generate\x20a\n\x20UUID\x20for\x20inte\
    rnal\x20use.\x20The\x20name\x20is\x20used\x20for\x20dynamic\x20listener\
    \x20update\x20and\x20removal\n\x20via\x20the\x20LDS\x20APIs.\n\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04p\x02l\x12\n\x0c\n\x05\x04\x03\x02\0\x05\x12\
    \x03p\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03p\t\r\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03p\x10\x11\n>\n\x04\x04\x03\x02\x01\x12\x03s\x02\
    \x16\x1a1\x20The\x20address\x20that\x20the\x20listener\x20should\x20list\
    en\x20on.\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04s\x02p\x12\n\x0c\n\x05\
    \x04\x03\x02\x01\x06\x12\x03s\x02\t\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\
    \x03s\n\x11\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03s\x14\x15\n\xcf\x05\n\
    \x04\x04\x03\x02\x02\x12\x04\x81\x01\x02)\x1a\xc0\x05\x20A\x20list\x20of\
    \x20filter\x20chains\x20to\x20consider\x20for\x20this\x20listener.\x20Th\
    e\x20FilterChain\x20with\n\x20the\x20most\x20specific\x20FilterChainMatc\
    h\x20criteria\x20is\x20used\x20on\x20a\x20connection.\x20The\n\x20algori\
    thm\x20works\x20as\x20follows:\n\x201.\x20If\x20SNI\x20information\x20is\
    \x20presented\x20at\x20connection\x20time,\x20only\x20the\n\x20\x20\x20\
    \x20FilterChains\x20matching\x20the\x20SNI\x20are\x20considered.\x20Othe\
    rwise,\x20only\n\x20\x20\x20\x20FilterChains\x20with\x20no\x20SNI\x20dom\
    ains\x20are\x20considered.\n\x202.\x20Of\x20the\x20FilterChains\x20from\
    \x20step\x201,\x20the\x20longest\x20prefix\x20match\x20on\x20the\n\x20\
    \x20\x20\x20bound\x20destination\x20address\x20is\x20used\x20to\x20selec\
    t\x20the\x20next\x20set\x20of\n\x20\x20\x20\x20FilterChains.\x20This\x20\
    may\x20be\x20one\x20FilterChain\x20or\x20multiple\x20if\x20there\x20is\n\
    \x20\x20\x20\x20a\x20tie.\n\x203.\x20The\x20longest\x20suffix\x20match\
    \x20on\x20the\x20bound\x20destination\x20address\x20is\x20used\x20to\n\
    \x20\x20\x20\x20select\x20the\x20FilterChain\x20from\x20step\x202\x20tha\
    t\x20is\x20used.\n\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04\x81\x01\x02\n\n\
    \r\n\x05\x04\x03\x02\x02\x06\x12\x04\x81\x01\x0b\x16\n\r\n\x05\x04\x03\
    \x02\x02\x01\x12\x04\x81\x01\x17$\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\
    \x81\x01'(\n\xaa\x02\n\x04\x04\x03\x02\x03\x12\x04\x87\x01\x021\x1a\x9b\
    \x02\x20If\x20a\x20connection\x20is\x20redirected\x20using\x20iptables,\
    \x20the\x20port\x20on\x20which\x20the\x20proxy\n\x20receives\x20it\x20mi\
    ght\x20be\x20different\x20from\x20the\x20original\x20destination\x20port\
    .\x20When\n\x20this\x20flag\x20is\x20set\x20to\x20true,\x20the\x20listen\
    er\x20uses\x20the\x20original\x20destination\n\x20address\x20and\x20port\
    \x20during\x20FilterChain\x20matching.\x20Default\x20is\x20false.\n\n\
    \x0f\n\x05\x04\x03\x02\x03\x04\x12\x06\x87\x01\x02\x81\x01)\n\r\n\x05\
    \x04\x03\x02\x03\x06\x12\x04\x87\x01\x02\x1b\n\r\n\x05\x04\x03\x02\x03\
    \x01\x12\x04\x87\x01\x1c,\n\r\n\x05\x04\x03\x02\x03\x03\x12\x04\x87\x01/\
    0\n\xa4\x01\n\x04\x04\x03\x02\x04\x12\x04\x8b\x01\x02D\x1a\x95\x01\x20So\
    ft\x20limit\x20on\x20size\x20of\x20the\x20listener\xe2\x80\x99s\x20new\
    \x20connection\x20read\x20and\x20write\x20buffers.\n\x20If\x20unspecifie\
    d,\x20an\x20implementation\x20defined\x20default\x20is\x20applied\x20(1M\
    iB).\n\n\x0f\n\x05\x04\x03\x02\x04\x04\x12\x06\x8b\x01\x02\x87\x011\n\r\
    \n\x05\x04\x03\x02\x04\x06\x12\x04\x8b\x01\x02\x1d\n\r\n\x05\x04\x03\x02\
    \x04\x01\x12\x04\x8b\x01\x1e?\n\r\n\x05\x04\x03\x02\x04\x03\x12\x04\x8b\
    \x01BC\n.\n\x04\x04\x03\x02\x05\x12\x04\x8e\x01\x02\x18\x1a\x20\x20See\
    \x20base.Metadata\x20description.\n\n\x0f\n\x05\x04\x03\x02\x05\x04\x12\
    \x06\x8e\x01\x02\x8b\x01D\n\r\n\x05\x04\x03\x02\x05\x06\x12\x04\x8e\x01\
    \x02\n\n\r\n\x05\x04\x03\x02\x05\x01\x12\x04\x8e\x01\x0b\x13\n\r\n\x05\
    \x04\x03\x02\x05\x03\x12\x04\x8e\x01\x16\x17\n\x0e\n\x04\x04\x03\x03\0\
    \x12\x06\x90\x01\x02\x9a\x01\x03\n\r\n\x05\x04\x03\x03\0\x01\x12\x04\x90\
    \x01\n\x16\n\xf5\x03\n\x06\x04\x03\x03\0\x02\0\x12\x04\x99\x01\x04/\x1a\
    \xe4\x03\x20Whether\x20the\x20listener\x20should\x20bind\x20to\x20the\
    \x20port.\x20A\x20listener\x20that\x20doesn\xe2\x80\x99t\n\x20bind\x20ca\
    n\x20only\x20receive\x20connections\x20redirected\x20from\x20other\x20li\
    steners\x20that\n\x20set\x20use_original_dst\x20parameter\x20to\x20true.\
    \x20Default\x20is\x20true.\n\n\x20[V2-API-DIFF]\x20This\x20is\x20depreca\
    ted\x20in\x20v2,\x20all\x20Listeners\x20will\x20bind\x20to\x20their\n\
    \x20port.\x20An\x20additional\x20filter\x20chain\x20must\x20be\x20create\
    d\x20for\x20every\x20original\n\x20destination\x20port\x20this\x20listen\
    er\x20may\x20redirect\x20to\x20in\x20v2,\x20with\x20the\x20original\n\
    \x20port\x20specified\x20in\x20the\x20FilterChainMatch\x20destination_po\
    rt\x20field.\n\n\x11\n\x07\x04\x03\x03\0\x02\0\x04\x12\x06\x99\x01\x04\
    \x90\x01\x18\n\x0f\n\x07\x04\x03\x03\0\x02\0\x06\x12\x04\x99\x01\x04\x1d\
    \n\x0f\n\x07\x04\x03\x03\0\x02\0\x01\x12\x04\x99\x01\x1e*\n\x0f\n\x07\
    \x04\x03\x03\0\x02\0\x03\x12\x04\x99\x01-.\n\x0c\n\x04\x04\x03\x02\x06\
    \x12\x04\x9b\x01\x02!\n\x0f\n\x05\x04\x03\x02\x06\x04\x12\x06\x9b\x01\
    \x02\x9a\x01\x03\n\r\n\x05\x04\x03\x02\x06\x06\x12\x04\x9b\x01\x02\x0e\n\
    \r\n\x05\x04\x03\x02\x06\x01\x12\x04\x9b\x01\x0f\x1c\n\r\n\x05\x04\x03\
    \x02\x06\x03\x12\x04\x9b\x01\x1f\x20b\x06proto3\
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
