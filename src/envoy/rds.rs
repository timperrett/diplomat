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
pub struct WeightedCluster {
    // message fields
    pub clusters: ::protobuf::RepeatedField<WeightedCluster_ClusterWeight>,
    pub runtime_key_prefix: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WeightedCluster {}

impl WeightedCluster {
    pub fn new() -> WeightedCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WeightedCluster {
        static mut instance: ::protobuf::lazy::Lazy<WeightedCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WeightedCluster,
        };
        unsafe {
            instance.get(WeightedCluster::new)
        }
    }

    // repeated .envoy.api.v2.WeightedCluster.ClusterWeight clusters = 1;

    pub fn clear_clusters(&mut self) {
        self.clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_clusters(&mut self, v: ::protobuf::RepeatedField<WeightedCluster_ClusterWeight>) {
        self.clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clusters(&mut self) -> &mut ::protobuf::RepeatedField<WeightedCluster_ClusterWeight> {
        &mut self.clusters
    }

    // Take field
    pub fn take_clusters(&mut self) -> ::protobuf::RepeatedField<WeightedCluster_ClusterWeight> {
        ::std::mem::replace(&mut self.clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_clusters(&self) -> &[WeightedCluster_ClusterWeight] {
        &self.clusters
    }

    fn get_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<WeightedCluster_ClusterWeight> {
        &self.clusters
    }

    fn mut_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<WeightedCluster_ClusterWeight> {
        &mut self.clusters
    }

    // string runtime_key_prefix = 2;

    pub fn clear_runtime_key_prefix(&mut self) {
        self.runtime_key_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_runtime_key_prefix(&mut self, v: ::std::string::String) {
        self.runtime_key_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_runtime_key_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.runtime_key_prefix
    }

    // Take field
    pub fn take_runtime_key_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.runtime_key_prefix, ::std::string::String::new())
    }

    pub fn get_runtime_key_prefix(&self) -> &str {
        &self.runtime_key_prefix
    }

    fn get_runtime_key_prefix_for_reflect(&self) -> &::std::string::String {
        &self.runtime_key_prefix
    }

    fn mut_runtime_key_prefix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.runtime_key_prefix
    }
}

impl ::protobuf::Message for WeightedCluster {
    fn is_initialized(&self) -> bool {
        for v in &self.clusters {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.clusters)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.runtime_key_prefix)?;
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
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.runtime_key_prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.runtime_key_prefix);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.clusters {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.runtime_key_prefix.is_empty() {
            os.write_string(2, &self.runtime_key_prefix)?;
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

impl ::protobuf::MessageStatic for WeightedCluster {
    fn new() -> WeightedCluster {
        WeightedCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<WeightedCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WeightedCluster_ClusterWeight>>(
                    "clusters",
                    WeightedCluster::get_clusters_for_reflect,
                    WeightedCluster::mut_clusters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "runtime_key_prefix",
                    WeightedCluster::get_runtime_key_prefix_for_reflect,
                    WeightedCluster::mut_runtime_key_prefix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WeightedCluster>(
                    "WeightedCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WeightedCluster {
    fn clear(&mut self) {
        self.clear_clusters();
        self.clear_runtime_key_prefix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WeightedCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeightedCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WeightedCluster_ClusterWeight {
    // message fields
    pub name: ::std::string::String,
    pub weight: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub metadata_match: ::protobuf::SingularPtrField<super::base::Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WeightedCluster_ClusterWeight {}

impl WeightedCluster_ClusterWeight {
    pub fn new() -> WeightedCluster_ClusterWeight {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WeightedCluster_ClusterWeight {
        static mut instance: ::protobuf::lazy::Lazy<WeightedCluster_ClusterWeight> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WeightedCluster_ClusterWeight,
        };
        unsafe {
            instance.get(WeightedCluster_ClusterWeight::new)
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

    // .google.protobuf.UInt32Value weight = 2;

    pub fn clear_weight(&mut self) {
        self.weight.clear();
    }

    pub fn has_weight(&self) -> bool {
        self.weight.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weight(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.weight = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_weight(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.weight.is_none() {
            self.weight.set_default();
        }
        self.weight.as_mut().unwrap()
    }

    // Take field
    pub fn take_weight(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.weight.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_weight(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.weight.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_weight_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.weight
    }

    fn mut_weight_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.weight
    }

    // .envoy.api.v2.Metadata metadata_match = 3;

    pub fn clear_metadata_match(&mut self) {
        self.metadata_match.clear();
    }

    pub fn has_metadata_match(&self) -> bool {
        self.metadata_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata_match(&mut self, v: super::base::Metadata) {
        self.metadata_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata_match(&mut self) -> &mut super::base::Metadata {
        if self.metadata_match.is_none() {
            self.metadata_match.set_default();
        }
        self.metadata_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata_match(&mut self) -> super::base::Metadata {
        self.metadata_match.take().unwrap_or_else(|| super::base::Metadata::new())
    }

    pub fn get_metadata_match(&self) -> &super::base::Metadata {
        self.metadata_match.as_ref().unwrap_or_else(|| super::base::Metadata::default_instance())
    }

    fn get_metadata_match_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Metadata> {
        &self.metadata_match
    }

    fn mut_metadata_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Metadata> {
        &mut self.metadata_match
    }
}

impl ::protobuf::Message for WeightedCluster_ClusterWeight {
    fn is_initialized(&self) -> bool {
        for v in &self.weight {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata_match {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.weight)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata_match)?;
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
        if let Some(ref v) = self.weight.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metadata_match.as_ref() {
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
        if let Some(ref v) = self.weight.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metadata_match.as_ref() {
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

impl ::protobuf::MessageStatic for WeightedCluster_ClusterWeight {
    fn new() -> WeightedCluster_ClusterWeight {
        WeightedCluster_ClusterWeight::new()
    }

    fn descriptor_static(_: ::std::option::Option<WeightedCluster_ClusterWeight>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    WeightedCluster_ClusterWeight::get_name_for_reflect,
                    WeightedCluster_ClusterWeight::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "weight",
                    WeightedCluster_ClusterWeight::get_weight_for_reflect,
                    WeightedCluster_ClusterWeight::mut_weight_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata_match",
                    WeightedCluster_ClusterWeight::get_metadata_match_for_reflect,
                    WeightedCluster_ClusterWeight::mut_metadata_match_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WeightedCluster_ClusterWeight>(
                    "WeightedCluster_ClusterWeight",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WeightedCluster_ClusterWeight {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_weight();
        self.clear_metadata_match();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WeightedCluster_ClusterWeight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeightedCluster_ClusterWeight {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteMatch {
    // message fields
    pub case_sensitive: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub runtime: ::protobuf::SingularPtrField<super::base::RuntimeUInt32>,
    pub headers: ::protobuf::RepeatedField<HeaderMatcher>,
    // message oneof groups
    path_specifier: ::std::option::Option<RouteMatch_oneof_path_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteMatch {}

#[derive(Clone,PartialEq)]
pub enum RouteMatch_oneof_path_specifier {
    prefix(::std::string::String),
    path(::std::string::String),
    regex(::std::string::String),
}

impl RouteMatch {
    pub fn new() -> RouteMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteMatch {
        static mut instance: ::protobuf::lazy::Lazy<RouteMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteMatch,
        };
        unsafe {
            instance.get(RouteMatch::new)
        }
    }

    // string prefix = 1;

    pub fn clear_prefix(&mut self) {
        self.path_specifier = ::std::option::Option::None;
    }

    pub fn has_prefix(&self) -> bool {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::string::String) {
        self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(v))
    }

    // Mutable pointer to the field.
    pub fn mut_prefix(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(_)) = self.path_specifier {
        } else {
            self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(::std::string::String::new()));
        }
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::string::String {
        if self.has_prefix() {
            match self.path_specifier.take() {
                ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_prefix(&self) -> &str {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(ref v)) => v,
            _ => "",
        }
    }

    // string path = 2;

    pub fn clear_path(&mut self) {
        self.path_specifier = ::std::option::Option::None;
    }

    pub fn has_path(&self) -> bool {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(_)) = self.path_specifier {
        } else {
            self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(::std::string::String::new()));
        }
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        if self.has_path() {
            match self.path_specifier.take() {
                ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_path(&self) -> &str {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(ref v)) => v,
            _ => "",
        }
    }

    // string regex = 3;

    pub fn clear_regex(&mut self) {
        self.path_specifier = ::std::option::Option::None;
    }

    pub fn has_regex(&self) -> bool {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_regex(&mut self, v: ::std::string::String) {
        self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(v))
    }

    // Mutable pointer to the field.
    pub fn mut_regex(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(_)) = self.path_specifier {
        } else {
            self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(::std::string::String::new()));
        }
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_regex(&mut self) -> ::std::string::String {
        if self.has_regex() {
            match self.path_specifier.take() {
                ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_regex(&self) -> &str {
        match self.path_specifier {
            ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(ref v)) => v,
            _ => "",
        }
    }

    // .google.protobuf.BoolValue case_sensitive = 4;

    pub fn clear_case_sensitive(&mut self) {
        self.case_sensitive.clear();
    }

    pub fn has_case_sensitive(&self) -> bool {
        self.case_sensitive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_case_sensitive(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.case_sensitive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_case_sensitive(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.case_sensitive.is_none() {
            self.case_sensitive.set_default();
        }
        self.case_sensitive.as_mut().unwrap()
    }

    // Take field
    pub fn take_case_sensitive(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.case_sensitive.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_case_sensitive(&self) -> &::protobuf::well_known_types::BoolValue {
        self.case_sensitive.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_case_sensitive_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.case_sensitive
    }

    fn mut_case_sensitive_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.case_sensitive
    }

    // .envoy.api.v2.RuntimeUInt32 runtime = 5;

    pub fn clear_runtime(&mut self) {
        self.runtime.clear();
    }

    pub fn has_runtime(&self) -> bool {
        self.runtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_runtime(&mut self, v: super::base::RuntimeUInt32) {
        self.runtime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_runtime(&mut self) -> &mut super::base::RuntimeUInt32 {
        if self.runtime.is_none() {
            self.runtime.set_default();
        }
        self.runtime.as_mut().unwrap()
    }

    // Take field
    pub fn take_runtime(&mut self) -> super::base::RuntimeUInt32 {
        self.runtime.take().unwrap_or_else(|| super::base::RuntimeUInt32::new())
    }

    pub fn get_runtime(&self) -> &super::base::RuntimeUInt32 {
        self.runtime.as_ref().unwrap_or_else(|| super::base::RuntimeUInt32::default_instance())
    }

    fn get_runtime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::RuntimeUInt32> {
        &self.runtime
    }

    fn mut_runtime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::RuntimeUInt32> {
        &mut self.runtime
    }

    // repeated .envoy.api.v2.HeaderMatcher headers = 6;

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<HeaderMatcher>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<HeaderMatcher> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<HeaderMatcher> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_headers(&self) -> &[HeaderMatcher] {
        &self.headers
    }

    fn get_headers_for_reflect(&self) -> &::protobuf::RepeatedField<HeaderMatcher> {
        &self.headers
    }

    fn mut_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HeaderMatcher> {
        &mut self.headers
    }
}

impl ::protobuf::Message for RouteMatch {
    fn is_initialized(&self) -> bool {
        for v in &self.case_sensitive {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.runtime {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.headers {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::prefix(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::path(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.path_specifier = ::std::option::Option::Some(RouteMatch_oneof_path_specifier::regex(is.read_string()?));
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.case_sensitive)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.runtime)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
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
        if let Some(ref v) = self.case_sensitive.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.runtime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.path_specifier {
            match v {
                &RouteMatch_oneof_path_specifier::prefix(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &RouteMatch_oneof_path_specifier::path(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &RouteMatch_oneof_path_specifier::regex(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.case_sensitive.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.runtime.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.headers {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.path_specifier {
            match v {
                &RouteMatch_oneof_path_specifier::prefix(ref v) => {
                    os.write_string(1, v)?;
                },
                &RouteMatch_oneof_path_specifier::path(ref v) => {
                    os.write_string(2, v)?;
                },
                &RouteMatch_oneof_path_specifier::regex(ref v) => {
                    os.write_string(3, v)?;
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

impl ::protobuf::MessageStatic for RouteMatch {
    fn new() -> RouteMatch {
        RouteMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "prefix",
                    RouteMatch::has_prefix,
                    RouteMatch::get_prefix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "path",
                    RouteMatch::has_path,
                    RouteMatch::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "regex",
                    RouteMatch::has_regex,
                    RouteMatch::get_regex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "case_sensitive",
                    RouteMatch::get_case_sensitive_for_reflect,
                    RouteMatch::mut_case_sensitive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::RuntimeUInt32>>(
                    "runtime",
                    RouteMatch::get_runtime_for_reflect,
                    RouteMatch::mut_runtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HeaderMatcher>>(
                    "headers",
                    RouteMatch::get_headers_for_reflect,
                    RouteMatch::mut_headers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteMatch>(
                    "RouteMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteMatch {
    fn clear(&mut self) {
        self.clear_prefix();
        self.clear_path();
        self.clear_regex();
        self.clear_case_sensitive();
        self.clear_runtime();
        self.clear_headers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CorsPolicy {
    // message fields
    pub allow_origin: ::protobuf::RepeatedField<::std::string::String>,
    pub allow_methods: ::std::string::String,
    pub allow_headers: ::std::string::String,
    pub expose_headers: ::std::string::String,
    pub max_age: ::std::string::String,
    pub allow_credentials: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub enabled: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CorsPolicy {}

impl CorsPolicy {
    pub fn new() -> CorsPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CorsPolicy {
        static mut instance: ::protobuf::lazy::Lazy<CorsPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CorsPolicy,
        };
        unsafe {
            instance.get(CorsPolicy::new)
        }
    }

    // repeated string allow_origin = 1;

    pub fn clear_allow_origin(&mut self) {
        self.allow_origin.clear();
    }

    // Param is passed by value, moved
    pub fn set_allow_origin(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.allow_origin = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allow_origin(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allow_origin
    }

    // Take field
    pub fn take_allow_origin(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.allow_origin, ::protobuf::RepeatedField::new())
    }

    pub fn get_allow_origin(&self) -> &[::std::string::String] {
        &self.allow_origin
    }

    fn get_allow_origin_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.allow_origin
    }

    fn mut_allow_origin_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allow_origin
    }

    // string allow_methods = 2;

    pub fn clear_allow_methods(&mut self) {
        self.allow_methods.clear();
    }

    // Param is passed by value, moved
    pub fn set_allow_methods(&mut self, v: ::std::string::String) {
        self.allow_methods = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allow_methods(&mut self) -> &mut ::std::string::String {
        &mut self.allow_methods
    }

    // Take field
    pub fn take_allow_methods(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allow_methods, ::std::string::String::new())
    }

    pub fn get_allow_methods(&self) -> &str {
        &self.allow_methods
    }

    fn get_allow_methods_for_reflect(&self) -> &::std::string::String {
        &self.allow_methods
    }

    fn mut_allow_methods_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allow_methods
    }

    // string allow_headers = 3;

    pub fn clear_allow_headers(&mut self) {
        self.allow_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_allow_headers(&mut self, v: ::std::string::String) {
        self.allow_headers = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allow_headers(&mut self) -> &mut ::std::string::String {
        &mut self.allow_headers
    }

    // Take field
    pub fn take_allow_headers(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allow_headers, ::std::string::String::new())
    }

    pub fn get_allow_headers(&self) -> &str {
        &self.allow_headers
    }

    fn get_allow_headers_for_reflect(&self) -> &::std::string::String {
        &self.allow_headers
    }

    fn mut_allow_headers_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allow_headers
    }

    // string expose_headers = 4;

    pub fn clear_expose_headers(&mut self) {
        self.expose_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_expose_headers(&mut self, v: ::std::string::String) {
        self.expose_headers = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expose_headers(&mut self) -> &mut ::std::string::String {
        &mut self.expose_headers
    }

    // Take field
    pub fn take_expose_headers(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.expose_headers, ::std::string::String::new())
    }

    pub fn get_expose_headers(&self) -> &str {
        &self.expose_headers
    }

    fn get_expose_headers_for_reflect(&self) -> &::std::string::String {
        &self.expose_headers
    }

    fn mut_expose_headers_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.expose_headers
    }

    // string max_age = 5;

    pub fn clear_max_age(&mut self) {
        self.max_age.clear();
    }

    // Param is passed by value, moved
    pub fn set_max_age(&mut self, v: ::std::string::String) {
        self.max_age = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_age(&mut self) -> &mut ::std::string::String {
        &mut self.max_age
    }

    // Take field
    pub fn take_max_age(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.max_age, ::std::string::String::new())
    }

    pub fn get_max_age(&self) -> &str {
        &self.max_age
    }

    fn get_max_age_for_reflect(&self) -> &::std::string::String {
        &self.max_age
    }

    fn mut_max_age_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.max_age
    }

    // .google.protobuf.BoolValue allow_credentials = 6;

    pub fn clear_allow_credentials(&mut self) {
        self.allow_credentials.clear();
    }

    pub fn has_allow_credentials(&self) -> bool {
        self.allow_credentials.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_credentials(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.allow_credentials = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allow_credentials(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.allow_credentials.is_none() {
            self.allow_credentials.set_default();
        }
        self.allow_credentials.as_mut().unwrap()
    }

    // Take field
    pub fn take_allow_credentials(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.allow_credentials.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_allow_credentials(&self) -> &::protobuf::well_known_types::BoolValue {
        self.allow_credentials.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_allow_credentials_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.allow_credentials
    }

    fn mut_allow_credentials_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.allow_credentials
    }

    // .google.protobuf.BoolValue enabled = 7;

    pub fn clear_enabled(&mut self) {
        self.enabled.clear();
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.enabled = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_enabled(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.enabled.is_none() {
            self.enabled.set_default();
        }
        self.enabled.as_mut().unwrap()
    }

    // Take field
    pub fn take_enabled(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.enabled.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_enabled(&self) -> &::protobuf::well_known_types::BoolValue {
        self.enabled.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_enabled_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.enabled
    }

    fn mut_enabled_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.enabled
    }
}

impl ::protobuf::Message for CorsPolicy {
    fn is_initialized(&self) -> bool {
        for v in &self.allow_credentials {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enabled {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.allow_origin)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allow_methods)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allow_headers)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.expose_headers)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.max_age)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allow_credentials)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.enabled)?;
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
        for value in &self.allow_origin {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if !self.allow_methods.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.allow_methods);
        }
        if !self.allow_headers.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.allow_headers);
        }
        if !self.expose_headers.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.expose_headers);
        }
        if !self.max_age.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.max_age);
        }
        if let Some(ref v) = self.allow_credentials.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.enabled.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.allow_origin {
            os.write_string(1, &v)?;
        };
        if !self.allow_methods.is_empty() {
            os.write_string(2, &self.allow_methods)?;
        }
        if !self.allow_headers.is_empty() {
            os.write_string(3, &self.allow_headers)?;
        }
        if !self.expose_headers.is_empty() {
            os.write_string(4, &self.expose_headers)?;
        }
        if !self.max_age.is_empty() {
            os.write_string(5, &self.max_age)?;
        }
        if let Some(ref v) = self.allow_credentials.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.enabled.as_ref() {
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

impl ::protobuf::MessageStatic for CorsPolicy {
    fn new() -> CorsPolicy {
        CorsPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<CorsPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allow_origin",
                    CorsPolicy::get_allow_origin_for_reflect,
                    CorsPolicy::mut_allow_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allow_methods",
                    CorsPolicy::get_allow_methods_for_reflect,
                    CorsPolicy::mut_allow_methods_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allow_headers",
                    CorsPolicy::get_allow_headers_for_reflect,
                    CorsPolicy::mut_allow_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "expose_headers",
                    CorsPolicy::get_expose_headers_for_reflect,
                    CorsPolicy::mut_expose_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "max_age",
                    CorsPolicy::get_max_age_for_reflect,
                    CorsPolicy::mut_max_age_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "allow_credentials",
                    CorsPolicy::get_allow_credentials_for_reflect,
                    CorsPolicy::mut_allow_credentials_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "enabled",
                    CorsPolicy::get_enabled_for_reflect,
                    CorsPolicy::mut_enabled_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CorsPolicy>(
                    "CorsPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CorsPolicy {
    fn clear(&mut self) {
        self.clear_allow_origin();
        self.clear_allow_methods();
        self.clear_allow_headers();
        self.clear_expose_headers();
        self.clear_max_age();
        self.clear_allow_credentials();
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CorsPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CorsPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction {
    // message fields
    pub metadata_match: ::protobuf::SingularPtrField<super::base::Metadata>,
    pub prefix_rewrite: ::std::string::String,
    pub timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub retry_policy: ::protobuf::SingularPtrField<RouteAction_RetryPolicy>,
    pub request_mirror_policy: ::protobuf::SingularPtrField<RouteAction_RequestMirrorPolicy>,
    pub priority: super::base::RoutingPriority,
    pub request_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub rate_limits: ::protobuf::RepeatedField<RateLimit>,
    pub include_vh_rate_limits: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub hash_policy: ::protobuf::RepeatedField<RouteAction_HashPolicy>,
    pub use_websocket: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub cors: ::protobuf::SingularPtrField<CorsPolicy>,
    // message oneof groups
    cluster_specifier: ::std::option::Option<RouteAction_oneof_cluster_specifier>,
    host_rewrite_specifier: ::std::option::Option<RouteAction_oneof_host_rewrite_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction {}

#[derive(Clone,PartialEq)]
pub enum RouteAction_oneof_cluster_specifier {
    cluster(::std::string::String),
    cluster_header(::std::string::String),
    weighted_clusters(WeightedCluster),
}

#[derive(Clone,PartialEq)]
pub enum RouteAction_oneof_host_rewrite_specifier {
    host_rewrite(::std::string::String),
    auto_host_rewrite(::protobuf::well_known_types::BoolValue),
}

impl RouteAction {
    pub fn new() -> RouteAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction,
        };
        unsafe {
            instance.get(RouteAction::new)
        }
    }

    // string cluster = 1;

    pub fn clear_cluster(&mut self) {
        self.cluster_specifier = ::std::option::Option::None;
    }

    pub fn has_cluster(&self) -> bool {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: ::std::string::String) {
        self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cluster(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(_)) = self.cluster_specifier {
        } else {
            self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(::std::string::String::new()));
        }
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cluster(&mut self) -> ::std::string::String {
        if self.has_cluster() {
            match self.cluster_specifier.take() {
                ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_cluster(&self) -> &str {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(ref v)) => v,
            _ => "",
        }
    }

    // string cluster_header = 2;

    pub fn clear_cluster_header(&mut self) {
        self.cluster_specifier = ::std::option::Option::None;
    }

    pub fn has_cluster_header(&self) -> bool {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cluster_header(&mut self, v: ::std::string::String) {
        self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cluster_header(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(_)) = self.cluster_specifier {
        } else {
            self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(::std::string::String::new()));
        }
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cluster_header(&mut self) -> ::std::string::String {
        if self.has_cluster_header() {
            match self.cluster_specifier.take() {
                ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_cluster_header(&self) -> &str {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(ref v)) => v,
            _ => "",
        }
    }

    // .envoy.api.v2.WeightedCluster weighted_clusters = 3;

    pub fn clear_weighted_clusters(&mut self) {
        self.cluster_specifier = ::std::option::Option::None;
    }

    pub fn has_weighted_clusters(&self) -> bool {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_weighted_clusters(&mut self, v: WeightedCluster) {
        self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(v))
    }

    // Mutable pointer to the field.
    pub fn mut_weighted_clusters(&mut self) -> &mut WeightedCluster {
        if let ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(_)) = self.cluster_specifier {
        } else {
            self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(WeightedCluster::new()));
        }
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_weighted_clusters(&mut self) -> WeightedCluster {
        if self.has_weighted_clusters() {
            match self.cluster_specifier.take() {
                ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(v)) => v,
                _ => panic!(),
            }
        } else {
            WeightedCluster::new()
        }
    }

    pub fn get_weighted_clusters(&self) -> &WeightedCluster {
        match self.cluster_specifier {
            ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(ref v)) => v,
            _ => WeightedCluster::default_instance(),
        }
    }

    // .envoy.api.v2.Metadata metadata_match = 4;

    pub fn clear_metadata_match(&mut self) {
        self.metadata_match.clear();
    }

    pub fn has_metadata_match(&self) -> bool {
        self.metadata_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata_match(&mut self, v: super::base::Metadata) {
        self.metadata_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata_match(&mut self) -> &mut super::base::Metadata {
        if self.metadata_match.is_none() {
            self.metadata_match.set_default();
        }
        self.metadata_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata_match(&mut self) -> super::base::Metadata {
        self.metadata_match.take().unwrap_or_else(|| super::base::Metadata::new())
    }

    pub fn get_metadata_match(&self) -> &super::base::Metadata {
        self.metadata_match.as_ref().unwrap_or_else(|| super::base::Metadata::default_instance())
    }

    fn get_metadata_match_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::Metadata> {
        &self.metadata_match
    }

    fn mut_metadata_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::Metadata> {
        &mut self.metadata_match
    }

    // string prefix_rewrite = 5;

    pub fn clear_prefix_rewrite(&mut self) {
        self.prefix_rewrite.clear();
    }

    // Param is passed by value, moved
    pub fn set_prefix_rewrite(&mut self, v: ::std::string::String) {
        self.prefix_rewrite = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix_rewrite(&mut self) -> &mut ::std::string::String {
        &mut self.prefix_rewrite
    }

    // Take field
    pub fn take_prefix_rewrite(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.prefix_rewrite, ::std::string::String::new())
    }

    pub fn get_prefix_rewrite(&self) -> &str {
        &self.prefix_rewrite
    }

    fn get_prefix_rewrite_for_reflect(&self) -> &::std::string::String {
        &self.prefix_rewrite
    }

    fn mut_prefix_rewrite_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.prefix_rewrite
    }

    // string host_rewrite = 6;

    pub fn clear_host_rewrite(&mut self) {
        self.host_rewrite_specifier = ::std::option::Option::None;
    }

    pub fn has_host_rewrite(&self) -> bool {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_host_rewrite(&mut self, v: ::std::string::String) {
        self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(v))
    }

    // Mutable pointer to the field.
    pub fn mut_host_rewrite(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(_)) = self.host_rewrite_specifier {
        } else {
            self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(::std::string::String::new()));
        }
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_host_rewrite(&mut self) -> ::std::string::String {
        if self.has_host_rewrite() {
            match self.host_rewrite_specifier.take() {
                ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_host_rewrite(&self) -> &str {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(ref v)) => v,
            _ => "",
        }
    }

    // .google.protobuf.BoolValue auto_host_rewrite = 7;

    pub fn clear_auto_host_rewrite(&mut self) {
        self.host_rewrite_specifier = ::std::option::Option::None;
    }

    pub fn has_auto_host_rewrite(&self) -> bool {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_auto_host_rewrite(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(v))
    }

    // Mutable pointer to the field.
    pub fn mut_auto_host_rewrite(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if let ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(_)) = self.host_rewrite_specifier {
        } else {
            self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(::protobuf::well_known_types::BoolValue::new()));
        }
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_auto_host_rewrite(&mut self) -> ::protobuf::well_known_types::BoolValue {
        if self.has_auto_host_rewrite() {
            match self.host_rewrite_specifier.take() {
                ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::BoolValue::new()
        }
    }

    pub fn get_auto_host_rewrite(&self) -> &::protobuf::well_known_types::BoolValue {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(ref v)) => v,
            _ => ::protobuf::well_known_types::BoolValue::default_instance(),
        }
    }

    // .google.protobuf.Duration timeout = 8;

    pub fn clear_timeout(&mut self) {
        self.timeout.clear();
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.timeout.is_none() {
            self.timeout.set_default();
        }
        self.timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.timeout
    }

    fn mut_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.timeout
    }

    // .envoy.api.v2.RouteAction.RetryPolicy retry_policy = 9;

    pub fn clear_retry_policy(&mut self) {
        self.retry_policy.clear();
    }

    pub fn has_retry_policy(&self) -> bool {
        self.retry_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retry_policy(&mut self, v: RouteAction_RetryPolicy) {
        self.retry_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retry_policy(&mut self) -> &mut RouteAction_RetryPolicy {
        if self.retry_policy.is_none() {
            self.retry_policy.set_default();
        }
        self.retry_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_retry_policy(&mut self) -> RouteAction_RetryPolicy {
        self.retry_policy.take().unwrap_or_else(|| RouteAction_RetryPolicy::new())
    }

    pub fn get_retry_policy(&self) -> &RouteAction_RetryPolicy {
        self.retry_policy.as_ref().unwrap_or_else(|| RouteAction_RetryPolicy::default_instance())
    }

    fn get_retry_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<RouteAction_RetryPolicy> {
        &self.retry_policy
    }

    fn mut_retry_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RouteAction_RetryPolicy> {
        &mut self.retry_policy
    }

    // .envoy.api.v2.RouteAction.RequestMirrorPolicy request_mirror_policy = 10;

    pub fn clear_request_mirror_policy(&mut self) {
        self.request_mirror_policy.clear();
    }

    pub fn has_request_mirror_policy(&self) -> bool {
        self.request_mirror_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_mirror_policy(&mut self, v: RouteAction_RequestMirrorPolicy) {
        self.request_mirror_policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_mirror_policy(&mut self) -> &mut RouteAction_RequestMirrorPolicy {
        if self.request_mirror_policy.is_none() {
            self.request_mirror_policy.set_default();
        }
        self.request_mirror_policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_request_mirror_policy(&mut self) -> RouteAction_RequestMirrorPolicy {
        self.request_mirror_policy.take().unwrap_or_else(|| RouteAction_RequestMirrorPolicy::new())
    }

    pub fn get_request_mirror_policy(&self) -> &RouteAction_RequestMirrorPolicy {
        self.request_mirror_policy.as_ref().unwrap_or_else(|| RouteAction_RequestMirrorPolicy::default_instance())
    }

    fn get_request_mirror_policy_for_reflect(&self) -> &::protobuf::SingularPtrField<RouteAction_RequestMirrorPolicy> {
        &self.request_mirror_policy
    }

    fn mut_request_mirror_policy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RouteAction_RequestMirrorPolicy> {
        &mut self.request_mirror_policy
    }

    // .envoy.api.v2.RoutingPriority priority = 11;

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

    // repeated .envoy.api.v2.HeaderValueOption request_headers_to_add = 12;

    pub fn clear_request_headers_to_add(&mut self) {
        self.request_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.request_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // Take field
    pub fn take_request_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.request_headers_to_add, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.request_headers_to_add
    }

    fn get_request_headers_to_add_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &self.request_headers_to_add
    }

    fn mut_request_headers_to_add_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // repeated .envoy.api.v2.RateLimit rate_limits = 13;

    pub fn clear_rate_limits(&mut self) {
        self.rate_limits.clear();
    }

    // Param is passed by value, moved
    pub fn set_rate_limits(&mut self, v: ::protobuf::RepeatedField<RateLimit>) {
        self.rate_limits = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rate_limits(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit> {
        &mut self.rate_limits
    }

    // Take field
    pub fn take_rate_limits(&mut self) -> ::protobuf::RepeatedField<RateLimit> {
        ::std::mem::replace(&mut self.rate_limits, ::protobuf::RepeatedField::new())
    }

    pub fn get_rate_limits(&self) -> &[RateLimit] {
        &self.rate_limits
    }

    fn get_rate_limits_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimit> {
        &self.rate_limits
    }

    fn mut_rate_limits_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit> {
        &mut self.rate_limits
    }

    // .google.protobuf.BoolValue include_vh_rate_limits = 14;

    pub fn clear_include_vh_rate_limits(&mut self) {
        self.include_vh_rate_limits.clear();
    }

    pub fn has_include_vh_rate_limits(&self) -> bool {
        self.include_vh_rate_limits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_include_vh_rate_limits(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.include_vh_rate_limits = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_include_vh_rate_limits(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.include_vh_rate_limits.is_none() {
            self.include_vh_rate_limits.set_default();
        }
        self.include_vh_rate_limits.as_mut().unwrap()
    }

    // Take field
    pub fn take_include_vh_rate_limits(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.include_vh_rate_limits.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_include_vh_rate_limits(&self) -> &::protobuf::well_known_types::BoolValue {
        self.include_vh_rate_limits.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_include_vh_rate_limits_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.include_vh_rate_limits
    }

    fn mut_include_vh_rate_limits_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.include_vh_rate_limits
    }

    // repeated .envoy.api.v2.RouteAction.HashPolicy hash_policy = 15;

    pub fn clear_hash_policy(&mut self) {
        self.hash_policy.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash_policy(&mut self, v: ::protobuf::RepeatedField<RouteAction_HashPolicy>) {
        self.hash_policy = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hash_policy(&mut self) -> &mut ::protobuf::RepeatedField<RouteAction_HashPolicy> {
        &mut self.hash_policy
    }

    // Take field
    pub fn take_hash_policy(&mut self) -> ::protobuf::RepeatedField<RouteAction_HashPolicy> {
        ::std::mem::replace(&mut self.hash_policy, ::protobuf::RepeatedField::new())
    }

    pub fn get_hash_policy(&self) -> &[RouteAction_HashPolicy] {
        &self.hash_policy
    }

    fn get_hash_policy_for_reflect(&self) -> &::protobuf::RepeatedField<RouteAction_HashPolicy> {
        &self.hash_policy
    }

    fn mut_hash_policy_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RouteAction_HashPolicy> {
        &mut self.hash_policy
    }

    // .google.protobuf.BoolValue use_websocket = 16;

    pub fn clear_use_websocket(&mut self) {
        self.use_websocket.clear();
    }

    pub fn has_use_websocket(&self) -> bool {
        self.use_websocket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_websocket(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.use_websocket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_use_websocket(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.use_websocket.is_none() {
            self.use_websocket.set_default();
        }
        self.use_websocket.as_mut().unwrap()
    }

    // Take field
    pub fn take_use_websocket(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.use_websocket.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_use_websocket(&self) -> &::protobuf::well_known_types::BoolValue {
        self.use_websocket.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_use_websocket_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.use_websocket
    }

    fn mut_use_websocket_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.use_websocket
    }

    // .envoy.api.v2.CorsPolicy cors = 17;

    pub fn clear_cors(&mut self) {
        self.cors.clear();
    }

    pub fn has_cors(&self) -> bool {
        self.cors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cors(&mut self, v: CorsPolicy) {
        self.cors = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cors(&mut self) -> &mut CorsPolicy {
        if self.cors.is_none() {
            self.cors.set_default();
        }
        self.cors.as_mut().unwrap()
    }

    // Take field
    pub fn take_cors(&mut self) -> CorsPolicy {
        self.cors.take().unwrap_or_else(|| CorsPolicy::new())
    }

    pub fn get_cors(&self) -> &CorsPolicy {
        self.cors.as_ref().unwrap_or_else(|| CorsPolicy::default_instance())
    }

    fn get_cors_for_reflect(&self) -> &::protobuf::SingularPtrField<CorsPolicy> {
        &self.cors
    }

    fn mut_cors_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CorsPolicy> {
        &mut self.cors
    }
}

impl ::protobuf::Message for RouteAction {
    fn is_initialized(&self) -> bool {
        if let Some(RouteAction_oneof_cluster_specifier::weighted_clusters(ref v)) = self.cluster_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.metadata_match {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(ref v)) = self.host_rewrite_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.retry_policy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_mirror_policy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rate_limits {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.include_vh_rate_limits {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.hash_policy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.use_websocket {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cors {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::cluster_header(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.cluster_specifier = ::std::option::Option::Some(RouteAction_oneof_cluster_specifier::weighted_clusters(is.read_message()?));
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata_match)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.prefix_rewrite)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::host_rewrite(is.read_string()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.host_rewrite_specifier = ::std::option::Option::Some(RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(is.read_message()?));
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timeout)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.retry_policy)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request_mirror_policy)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.priority = tmp;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_headers_to_add)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rate_limits)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.include_vh_rate_limits)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hash_policy)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.use_websocket)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cors)?;
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
        if let Some(ref v) = self.metadata_match.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.prefix_rewrite.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.prefix_rewrite);
        }
        if let Some(ref v) = self.timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.retry_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.request_mirror_policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.priority != super::base::RoutingPriority::DEFAULT {
            my_size += ::protobuf::rt::enum_size(11, self.priority);
        }
        for value in &self.request_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rate_limits {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.include_vh_rate_limits.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.hash_policy {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.use_websocket.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cors.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.cluster_specifier {
            match v {
                &RouteAction_oneof_cluster_specifier::cluster(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &RouteAction_oneof_cluster_specifier::cluster_header(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &RouteAction_oneof_cluster_specifier::weighted_clusters(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.host_rewrite_specifier {
            match v {
                &RouteAction_oneof_host_rewrite_specifier::host_rewrite(ref v) => {
                    my_size += ::protobuf::rt::string_size(6, &v);
                },
                &RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(ref v) => {
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
        if let Some(ref v) = self.metadata_match.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.prefix_rewrite.is_empty() {
            os.write_string(5, &self.prefix_rewrite)?;
        }
        if let Some(ref v) = self.timeout.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.retry_policy.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.request_mirror_policy.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.priority != super::base::RoutingPriority::DEFAULT {
            os.write_enum(11, self.priority.value())?;
        }
        for v in &self.request_headers_to_add {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.rate_limits {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.include_vh_rate_limits.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.hash_policy {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.use_websocket.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cors.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.cluster_specifier {
            match v {
                &RouteAction_oneof_cluster_specifier::cluster(ref v) => {
                    os.write_string(1, v)?;
                },
                &RouteAction_oneof_cluster_specifier::cluster_header(ref v) => {
                    os.write_string(2, v)?;
                },
                &RouteAction_oneof_cluster_specifier::weighted_clusters(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.host_rewrite_specifier {
            match v {
                &RouteAction_oneof_host_rewrite_specifier::host_rewrite(ref v) => {
                    os.write_string(6, v)?;
                },
                &RouteAction_oneof_host_rewrite_specifier::auto_host_rewrite(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RouteAction {
    fn new() -> RouteAction {
        RouteAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "cluster",
                    RouteAction::has_cluster,
                    RouteAction::get_cluster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "cluster_header",
                    RouteAction::has_cluster_header,
                    RouteAction::get_cluster_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, WeightedCluster>(
                    "weighted_clusters",
                    RouteAction::has_weighted_clusters,
                    RouteAction::get_weighted_clusters,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata_match",
                    RouteAction::get_metadata_match_for_reflect,
                    RouteAction::mut_metadata_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "prefix_rewrite",
                    RouteAction::get_prefix_rewrite_for_reflect,
                    RouteAction::mut_prefix_rewrite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "host_rewrite",
                    RouteAction::has_host_rewrite,
                    RouteAction::get_host_rewrite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::BoolValue>(
                    "auto_host_rewrite",
                    RouteAction::has_auto_host_rewrite,
                    RouteAction::get_auto_host_rewrite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "timeout",
                    RouteAction::get_timeout_for_reflect,
                    RouteAction::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RouteAction_RetryPolicy>>(
                    "retry_policy",
                    RouteAction::get_retry_policy_for_reflect,
                    RouteAction::mut_retry_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RouteAction_RequestMirrorPolicy>>(
                    "request_mirror_policy",
                    RouteAction::get_request_mirror_policy_for_reflect,
                    RouteAction::mut_request_mirror_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::base::RoutingPriority>>(
                    "priority",
                    RouteAction::get_priority_for_reflect,
                    RouteAction::mut_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                    "request_headers_to_add",
                    RouteAction::get_request_headers_to_add_for_reflect,
                    RouteAction::mut_request_headers_to_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimit>>(
                    "rate_limits",
                    RouteAction::get_rate_limits_for_reflect,
                    RouteAction::mut_rate_limits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "include_vh_rate_limits",
                    RouteAction::get_include_vh_rate_limits_for_reflect,
                    RouteAction::mut_include_vh_rate_limits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RouteAction_HashPolicy>>(
                    "hash_policy",
                    RouteAction::get_hash_policy_for_reflect,
                    RouteAction::mut_hash_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "use_websocket",
                    RouteAction::get_use_websocket_for_reflect,
                    RouteAction::mut_use_websocket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CorsPolicy>>(
                    "cors",
                    RouteAction::get_cors_for_reflect,
                    RouteAction::mut_cors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction>(
                    "RouteAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction {
    fn clear(&mut self) {
        self.clear_cluster();
        self.clear_cluster_header();
        self.clear_weighted_clusters();
        self.clear_metadata_match();
        self.clear_prefix_rewrite();
        self.clear_host_rewrite();
        self.clear_auto_host_rewrite();
        self.clear_timeout();
        self.clear_retry_policy();
        self.clear_request_mirror_policy();
        self.clear_priority();
        self.clear_request_headers_to_add();
        self.clear_rate_limits();
        self.clear_include_vh_rate_limits();
        self.clear_hash_policy();
        self.clear_use_websocket();
        self.clear_cors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_RetryPolicy {
    // message fields
    pub retry_on: ::std::string::String,
    pub num_retries: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub per_try_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_RetryPolicy {}

impl RouteAction_RetryPolicy {
    pub fn new() -> RouteAction_RetryPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_RetryPolicy {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_RetryPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_RetryPolicy,
        };
        unsafe {
            instance.get(RouteAction_RetryPolicy::new)
        }
    }

    // string retry_on = 1;

    pub fn clear_retry_on(&mut self) {
        self.retry_on.clear();
    }

    // Param is passed by value, moved
    pub fn set_retry_on(&mut self, v: ::std::string::String) {
        self.retry_on = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retry_on(&mut self) -> &mut ::std::string::String {
        &mut self.retry_on
    }

    // Take field
    pub fn take_retry_on(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.retry_on, ::std::string::String::new())
    }

    pub fn get_retry_on(&self) -> &str {
        &self.retry_on
    }

    fn get_retry_on_for_reflect(&self) -> &::std::string::String {
        &self.retry_on
    }

    fn mut_retry_on_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.retry_on
    }

    // .google.protobuf.UInt32Value num_retries = 2;

    pub fn clear_num_retries(&mut self) {
        self.num_retries.clear();
    }

    pub fn has_num_retries(&self) -> bool {
        self.num_retries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_retries(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.num_retries = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_num_retries(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.num_retries.is_none() {
            self.num_retries.set_default();
        }
        self.num_retries.as_mut().unwrap()
    }

    // Take field
    pub fn take_num_retries(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.num_retries.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_num_retries(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.num_retries.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_num_retries_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.num_retries
    }

    fn mut_num_retries_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.num_retries
    }

    // .google.protobuf.Duration per_try_timeout = 3;

    pub fn clear_per_try_timeout(&mut self) {
        self.per_try_timeout.clear();
    }

    pub fn has_per_try_timeout(&self) -> bool {
        self.per_try_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_per_try_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.per_try_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_per_try_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.per_try_timeout.is_none() {
            self.per_try_timeout.set_default();
        }
        self.per_try_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_per_try_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.per_try_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_per_try_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.per_try_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_per_try_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.per_try_timeout
    }

    fn mut_per_try_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.per_try_timeout
    }
}

impl ::protobuf::Message for RouteAction_RetryPolicy {
    fn is_initialized(&self) -> bool {
        for v in &self.num_retries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.per_try_timeout {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.retry_on)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.num_retries)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.per_try_timeout)?;
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
        if !self.retry_on.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.retry_on);
        }
        if let Some(ref v) = self.num_retries.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.per_try_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.retry_on.is_empty() {
            os.write_string(1, &self.retry_on)?;
        }
        if let Some(ref v) = self.num_retries.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.per_try_timeout.as_ref() {
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

impl ::protobuf::MessageStatic for RouteAction_RetryPolicy {
    fn new() -> RouteAction_RetryPolicy {
        RouteAction_RetryPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_RetryPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "retry_on",
                    RouteAction_RetryPolicy::get_retry_on_for_reflect,
                    RouteAction_RetryPolicy::mut_retry_on_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "num_retries",
                    RouteAction_RetryPolicy::get_num_retries_for_reflect,
                    RouteAction_RetryPolicy::mut_num_retries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "per_try_timeout",
                    RouteAction_RetryPolicy::get_per_try_timeout_for_reflect,
                    RouteAction_RetryPolicy::mut_per_try_timeout_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_RetryPolicy>(
                    "RouteAction_RetryPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_RetryPolicy {
    fn clear(&mut self) {
        self.clear_retry_on();
        self.clear_num_retries();
        self.clear_per_try_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_RetryPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_RetryPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_RequestMirrorPolicy {
    // message fields
    pub cluster: ::std::string::String,
    pub runtime_key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_RequestMirrorPolicy {}

impl RouteAction_RequestMirrorPolicy {
    pub fn new() -> RouteAction_RequestMirrorPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_RequestMirrorPolicy {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_RequestMirrorPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_RequestMirrorPolicy,
        };
        unsafe {
            instance.get(RouteAction_RequestMirrorPolicy::new)
        }
    }

    // string cluster = 1;

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

    // string runtime_key = 2;

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

impl ::protobuf::Message for RouteAction_RequestMirrorPolicy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cluster)?;
                },
                2 => {
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
        if !self.cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cluster);
        }
        if !self.runtime_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.runtime_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cluster.is_empty() {
            os.write_string(1, &self.cluster)?;
        }
        if !self.runtime_key.is_empty() {
            os.write_string(2, &self.runtime_key)?;
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

impl ::protobuf::MessageStatic for RouteAction_RequestMirrorPolicy {
    fn new() -> RouteAction_RequestMirrorPolicy {
        RouteAction_RequestMirrorPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_RequestMirrorPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster",
                    RouteAction_RequestMirrorPolicy::get_cluster_for_reflect,
                    RouteAction_RequestMirrorPolicy::mut_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "runtime_key",
                    RouteAction_RequestMirrorPolicy::get_runtime_key_for_reflect,
                    RouteAction_RequestMirrorPolicy::mut_runtime_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_RequestMirrorPolicy>(
                    "RouteAction_RequestMirrorPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_RequestMirrorPolicy {
    fn clear(&mut self) {
        self.clear_cluster();
        self.clear_runtime_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_RequestMirrorPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_RequestMirrorPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_HashPolicy {
    // message oneof groups
    policy_specifier: ::std::option::Option<RouteAction_HashPolicy_oneof_policy_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_HashPolicy {}

#[derive(Clone,PartialEq)]
pub enum RouteAction_HashPolicy_oneof_policy_specifier {
    header(RouteAction_HashPolicy_Header),
    cookie(RouteAction_HashPolicy_Cookie),
    connection_properties(RouteAction_HashPolicy_ConnectionProperties),
}

impl RouteAction_HashPolicy {
    pub fn new() -> RouteAction_HashPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_HashPolicy {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_HashPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_HashPolicy,
        };
        unsafe {
            instance.get(RouteAction_HashPolicy::new)
        }
    }

    // .envoy.api.v2.RouteAction.HashPolicy.Header header = 1;

    pub fn clear_header(&mut self) {
        self.policy_specifier = ::std::option::Option::None;
    }

    pub fn has_header(&self) -> bool {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RouteAction_HashPolicy_Header) {
        self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(v))
    }

    // Mutable pointer to the field.
    pub fn mut_header(&mut self) -> &mut RouteAction_HashPolicy_Header {
        if let ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(_)) = self.policy_specifier {
        } else {
            self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(RouteAction_HashPolicy_Header::new()));
        }
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_header(&mut self) -> RouteAction_HashPolicy_Header {
        if self.has_header() {
            match self.policy_specifier.take() {
                ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(v)) => v,
                _ => panic!(),
            }
        } else {
            RouteAction_HashPolicy_Header::new()
        }
    }

    pub fn get_header(&self) -> &RouteAction_HashPolicy_Header {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(ref v)) => v,
            _ => RouteAction_HashPolicy_Header::default_instance(),
        }
    }

    // .envoy.api.v2.RouteAction.HashPolicy.Cookie cookie = 2;

    pub fn clear_cookie(&mut self) {
        self.policy_specifier = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: RouteAction_HashPolicy_Cookie) {
        self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cookie(&mut self) -> &mut RouteAction_HashPolicy_Cookie {
        if let ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(_)) = self.policy_specifier {
        } else {
            self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(RouteAction_HashPolicy_Cookie::new()));
        }
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cookie(&mut self) -> RouteAction_HashPolicy_Cookie {
        if self.has_cookie() {
            match self.policy_specifier.take() {
                ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(v)) => v,
                _ => panic!(),
            }
        } else {
            RouteAction_HashPolicy_Cookie::new()
        }
    }

    pub fn get_cookie(&self) -> &RouteAction_HashPolicy_Cookie {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(ref v)) => v,
            _ => RouteAction_HashPolicy_Cookie::default_instance(),
        }
    }

    // .envoy.api.v2.RouteAction.HashPolicy.ConnectionProperties connection_properties = 3;

    pub fn clear_connection_properties(&mut self) {
        self.policy_specifier = ::std::option::Option::None;
    }

    pub fn has_connection_properties(&self) -> bool {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_connection_properties(&mut self, v: RouteAction_HashPolicy_ConnectionProperties) {
        self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(v))
    }

    // Mutable pointer to the field.
    pub fn mut_connection_properties(&mut self) -> &mut RouteAction_HashPolicy_ConnectionProperties {
        if let ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(_)) = self.policy_specifier {
        } else {
            self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(RouteAction_HashPolicy_ConnectionProperties::new()));
        }
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_connection_properties(&mut self) -> RouteAction_HashPolicy_ConnectionProperties {
        if self.has_connection_properties() {
            match self.policy_specifier.take() {
                ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(v)) => v,
                _ => panic!(),
            }
        } else {
            RouteAction_HashPolicy_ConnectionProperties::new()
        }
    }

    pub fn get_connection_properties(&self) -> &RouteAction_HashPolicy_ConnectionProperties {
        match self.policy_specifier {
            ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(ref v)) => v,
            _ => RouteAction_HashPolicy_ConnectionProperties::default_instance(),
        }
    }
}

impl ::protobuf::Message for RouteAction_HashPolicy {
    fn is_initialized(&self) -> bool {
        if let Some(RouteAction_HashPolicy_oneof_policy_specifier::header(ref v)) = self.policy_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(ref v)) = self.policy_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(ref v)) = self.policy_specifier {
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
                    self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::header(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::cookie(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.policy_specifier = ::std::option::Option::Some(RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.policy_specifier {
            match v {
                &RouteAction_HashPolicy_oneof_policy_specifier::header(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RouteAction_HashPolicy_oneof_policy_specifier::cookie(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.policy_specifier {
            match v {
                &RouteAction_HashPolicy_oneof_policy_specifier::header(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RouteAction_HashPolicy_oneof_policy_specifier::cookie(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RouteAction_HashPolicy_oneof_policy_specifier::connection_properties(ref v) => {
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

impl ::protobuf::MessageStatic for RouteAction_HashPolicy {
    fn new() -> RouteAction_HashPolicy {
        RouteAction_HashPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_HashPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RouteAction_HashPolicy_Header>(
                    "header",
                    RouteAction_HashPolicy::has_header,
                    RouteAction_HashPolicy::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RouteAction_HashPolicy_Cookie>(
                    "cookie",
                    RouteAction_HashPolicy::has_cookie,
                    RouteAction_HashPolicy::get_cookie,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RouteAction_HashPolicy_ConnectionProperties>(
                    "connection_properties",
                    RouteAction_HashPolicy::has_connection_properties,
                    RouteAction_HashPolicy::get_connection_properties,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_HashPolicy>(
                    "RouteAction_HashPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_HashPolicy {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_cookie();
        self.clear_connection_properties();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_HashPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_HashPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_HashPolicy_Header {
    // message fields
    pub header_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_HashPolicy_Header {}

impl RouteAction_HashPolicy_Header {
    pub fn new() -> RouteAction_HashPolicy_Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_HashPolicy_Header {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_HashPolicy_Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_HashPolicy_Header,
        };
        unsafe {
            instance.get(RouteAction_HashPolicy_Header::new)
        }
    }

    // string header_name = 1;

    pub fn clear_header_name(&mut self) {
        self.header_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_header_name(&mut self, v: ::std::string::String) {
        self.header_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header_name(&mut self) -> &mut ::std::string::String {
        &mut self.header_name
    }

    // Take field
    pub fn take_header_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.header_name, ::std::string::String::new())
    }

    pub fn get_header_name(&self) -> &str {
        &self.header_name
    }

    fn get_header_name_for_reflect(&self) -> &::std::string::String {
        &self.header_name
    }

    fn mut_header_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.header_name
    }
}

impl ::protobuf::Message for RouteAction_HashPolicy_Header {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.header_name)?;
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
        if !self.header_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.header_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.header_name.is_empty() {
            os.write_string(1, &self.header_name)?;
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

impl ::protobuf::MessageStatic for RouteAction_HashPolicy_Header {
    fn new() -> RouteAction_HashPolicy_Header {
        RouteAction_HashPolicy_Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_HashPolicy_Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "header_name",
                    RouteAction_HashPolicy_Header::get_header_name_for_reflect,
                    RouteAction_HashPolicy_Header::mut_header_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_HashPolicy_Header>(
                    "RouteAction_HashPolicy_Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_HashPolicy_Header {
    fn clear(&mut self) {
        self.clear_header_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_HashPolicy_Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_HashPolicy_Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_HashPolicy_Cookie {
    // message fields
    pub name: ::std::string::String,
    pub ttl: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_HashPolicy_Cookie {}

impl RouteAction_HashPolicy_Cookie {
    pub fn new() -> RouteAction_HashPolicy_Cookie {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_HashPolicy_Cookie {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_HashPolicy_Cookie> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_HashPolicy_Cookie,
        };
        unsafe {
            instance.get(RouteAction_HashPolicy_Cookie::new)
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

    // .google.protobuf.Duration ttl = 2;

    pub fn clear_ttl(&mut self) {
        self.ttl.clear();
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.ttl = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ttl(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.ttl.is_none() {
            self.ttl.set_default();
        }
        self.ttl.as_mut().unwrap()
    }

    // Take field
    pub fn take_ttl(&mut self) -> ::protobuf::well_known_types::Duration {
        self.ttl.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_ttl(&self) -> &::protobuf::well_known_types::Duration {
        self.ttl.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_ttl_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.ttl
    }

    fn mut_ttl_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.ttl
    }
}

impl ::protobuf::Message for RouteAction_HashPolicy_Cookie {
    fn is_initialized(&self) -> bool {
        for v in &self.ttl {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ttl)?;
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
        if let Some(ref v) = self.ttl.as_ref() {
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
        if let Some(ref v) = self.ttl.as_ref() {
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

impl ::protobuf::MessageStatic for RouteAction_HashPolicy_Cookie {
    fn new() -> RouteAction_HashPolicy_Cookie {
        RouteAction_HashPolicy_Cookie::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_HashPolicy_Cookie>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    RouteAction_HashPolicy_Cookie::get_name_for_reflect,
                    RouteAction_HashPolicy_Cookie::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "ttl",
                    RouteAction_HashPolicy_Cookie::get_ttl_for_reflect,
                    RouteAction_HashPolicy_Cookie::mut_ttl_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_HashPolicy_Cookie>(
                    "RouteAction_HashPolicy_Cookie",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_HashPolicy_Cookie {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_HashPolicy_Cookie {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_HashPolicy_Cookie {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteAction_HashPolicy_ConnectionProperties {
    // message fields
    pub source_ip: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteAction_HashPolicy_ConnectionProperties {}

impl RouteAction_HashPolicy_ConnectionProperties {
    pub fn new() -> RouteAction_HashPolicy_ConnectionProperties {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteAction_HashPolicy_ConnectionProperties {
        static mut instance: ::protobuf::lazy::Lazy<RouteAction_HashPolicy_ConnectionProperties> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteAction_HashPolicy_ConnectionProperties,
        };
        unsafe {
            instance.get(RouteAction_HashPolicy_ConnectionProperties::new)
        }
    }

    // bool source_ip = 1;

    pub fn clear_source_ip(&mut self) {
        self.source_ip = false;
    }

    // Param is passed by value, moved
    pub fn set_source_ip(&mut self, v: bool) {
        self.source_ip = v;
    }

    pub fn get_source_ip(&self) -> bool {
        self.source_ip
    }

    fn get_source_ip_for_reflect(&self) -> &bool {
        &self.source_ip
    }

    fn mut_source_ip_for_reflect(&mut self) -> &mut bool {
        &mut self.source_ip
    }
}

impl ::protobuf::Message for RouteAction_HashPolicy_ConnectionProperties {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.source_ip = tmp;
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
        if self.source_ip != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.source_ip != false {
            os.write_bool(1, self.source_ip)?;
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

impl ::protobuf::MessageStatic for RouteAction_HashPolicy_ConnectionProperties {
    fn new() -> RouteAction_HashPolicy_ConnectionProperties {
        RouteAction_HashPolicy_ConnectionProperties::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteAction_HashPolicy_ConnectionProperties>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "source_ip",
                    RouteAction_HashPolicy_ConnectionProperties::get_source_ip_for_reflect,
                    RouteAction_HashPolicy_ConnectionProperties::mut_source_ip_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteAction_HashPolicy_ConnectionProperties>(
                    "RouteAction_HashPolicy_ConnectionProperties",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteAction_HashPolicy_ConnectionProperties {
    fn clear(&mut self) {
        self.clear_source_ip();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteAction_HashPolicy_ConnectionProperties {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteAction_HashPolicy_ConnectionProperties {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RedirectAction {
    // message fields
    pub host_redirect: ::std::string::String,
    pub path_redirect: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RedirectAction {}

impl RedirectAction {
    pub fn new() -> RedirectAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RedirectAction {
        static mut instance: ::protobuf::lazy::Lazy<RedirectAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RedirectAction,
        };
        unsafe {
            instance.get(RedirectAction::new)
        }
    }

    // string host_redirect = 1;

    pub fn clear_host_redirect(&mut self) {
        self.host_redirect.clear();
    }

    // Param is passed by value, moved
    pub fn set_host_redirect(&mut self, v: ::std::string::String) {
        self.host_redirect = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_redirect(&mut self) -> &mut ::std::string::String {
        &mut self.host_redirect
    }

    // Take field
    pub fn take_host_redirect(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.host_redirect, ::std::string::String::new())
    }

    pub fn get_host_redirect(&self) -> &str {
        &self.host_redirect
    }

    fn get_host_redirect_for_reflect(&self) -> &::std::string::String {
        &self.host_redirect
    }

    fn mut_host_redirect_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.host_redirect
    }

    // string path_redirect = 2;

    pub fn clear_path_redirect(&mut self) {
        self.path_redirect.clear();
    }

    // Param is passed by value, moved
    pub fn set_path_redirect(&mut self, v: ::std::string::String) {
        self.path_redirect = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path_redirect(&mut self) -> &mut ::std::string::String {
        &mut self.path_redirect
    }

    // Take field
    pub fn take_path_redirect(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.path_redirect, ::std::string::String::new())
    }

    pub fn get_path_redirect(&self) -> &str {
        &self.path_redirect
    }

    fn get_path_redirect_for_reflect(&self) -> &::std::string::String {
        &self.path_redirect
    }

    fn mut_path_redirect_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.path_redirect
    }
}

impl ::protobuf::Message for RedirectAction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.host_redirect)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path_redirect)?;
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
        if !self.host_redirect.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.host_redirect);
        }
        if !self.path_redirect.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.path_redirect);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.host_redirect.is_empty() {
            os.write_string(1, &self.host_redirect)?;
        }
        if !self.path_redirect.is_empty() {
            os.write_string(2, &self.path_redirect)?;
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

impl ::protobuf::MessageStatic for RedirectAction {
    fn new() -> RedirectAction {
        RedirectAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<RedirectAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host_redirect",
                    RedirectAction::get_host_redirect_for_reflect,
                    RedirectAction::mut_host_redirect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path_redirect",
                    RedirectAction::get_path_redirect_for_reflect,
                    RedirectAction::mut_path_redirect_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RedirectAction>(
                    "RedirectAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RedirectAction {
    fn clear(&mut self) {
        self.clear_host_redirect();
        self.clear_path_redirect();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RedirectAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RedirectAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Decorator {
    // message fields
    pub operation: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Decorator {}

impl Decorator {
    pub fn new() -> Decorator {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Decorator {
        static mut instance: ::protobuf::lazy::Lazy<Decorator> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Decorator,
        };
        unsafe {
            instance.get(Decorator::new)
        }
    }

    // string operation = 1;

    pub fn clear_operation(&mut self) {
        self.operation.clear();
    }

    // Param is passed by value, moved
    pub fn set_operation(&mut self, v: ::std::string::String) {
        self.operation = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operation(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }

    // Take field
    pub fn take_operation(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.operation, ::std::string::String::new())
    }

    pub fn get_operation(&self) -> &str {
        &self.operation
    }

    fn get_operation_for_reflect(&self) -> &::std::string::String {
        &self.operation
    }

    fn mut_operation_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.operation
    }
}

impl ::protobuf::Message for Decorator {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.operation)?;
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
        if !self.operation.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.operation);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.operation.is_empty() {
            os.write_string(1, &self.operation)?;
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

impl ::protobuf::MessageStatic for Decorator {
    fn new() -> Decorator {
        Decorator::new()
    }

    fn descriptor_static(_: ::std::option::Option<Decorator>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "operation",
                    Decorator::get_operation_for_reflect,
                    Decorator::mut_operation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Decorator>(
                    "Decorator",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Decorator {
    fn clear(&mut self) {
        self.clear_operation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Decorator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Decorator {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Route {
    // message fields
    pub field_match: ::protobuf::SingularPtrField<RouteMatch>,
    pub metadata: ::protobuf::SingularPtrField<super::base::Metadata>,
    pub decorator: ::protobuf::SingularPtrField<Decorator>,
    // message oneof groups
    action: ::std::option::Option<Route_oneof_action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Route {}

#[derive(Clone,PartialEq)]
pub enum Route_oneof_action {
    route(RouteAction),
    redirect(RedirectAction),
}

impl Route {
    pub fn new() -> Route {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Route {
        static mut instance: ::protobuf::lazy::Lazy<Route> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Route,
        };
        unsafe {
            instance.get(Route::new)
        }
    }

    // .envoy.api.v2.RouteMatch match = 1;

    pub fn clear_field_match(&mut self) {
        self.field_match.clear();
    }

    pub fn has_field_match(&self) -> bool {
        self.field_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_match(&mut self, v: RouteMatch) {
        self.field_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_match(&mut self) -> &mut RouteMatch {
        if self.field_match.is_none() {
            self.field_match.set_default();
        }
        self.field_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_match(&mut self) -> RouteMatch {
        self.field_match.take().unwrap_or_else(|| RouteMatch::new())
    }

    pub fn get_field_match(&self) -> &RouteMatch {
        self.field_match.as_ref().unwrap_or_else(|| RouteMatch::default_instance())
    }

    fn get_field_match_for_reflect(&self) -> &::protobuf::SingularPtrField<RouteMatch> {
        &self.field_match
    }

    fn mut_field_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RouteMatch> {
        &mut self.field_match
    }

    // .envoy.api.v2.RouteAction route = 2;

    pub fn clear_route(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_route(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::route(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_route(&mut self, v: RouteAction) {
        self.action = ::std::option::Option::Some(Route_oneof_action::route(v))
    }

    // Mutable pointer to the field.
    pub fn mut_route(&mut self) -> &mut RouteAction {
        if let ::std::option::Option::Some(Route_oneof_action::route(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(Route_oneof_action::route(RouteAction::new()));
        }
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::route(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_route(&mut self) -> RouteAction {
        if self.has_route() {
            match self.action.take() {
                ::std::option::Option::Some(Route_oneof_action::route(v)) => v,
                _ => panic!(),
            }
        } else {
            RouteAction::new()
        }
    }

    pub fn get_route(&self) -> &RouteAction {
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::route(ref v)) => v,
            _ => RouteAction::default_instance(),
        }
    }

    // .envoy.api.v2.RedirectAction redirect = 3;

    pub fn clear_redirect(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_redirect(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::redirect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_redirect(&mut self, v: RedirectAction) {
        self.action = ::std::option::Option::Some(Route_oneof_action::redirect(v))
    }

    // Mutable pointer to the field.
    pub fn mut_redirect(&mut self) -> &mut RedirectAction {
        if let ::std::option::Option::Some(Route_oneof_action::redirect(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(Route_oneof_action::redirect(RedirectAction::new()));
        }
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::redirect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_redirect(&mut self) -> RedirectAction {
        if self.has_redirect() {
            match self.action.take() {
                ::std::option::Option::Some(Route_oneof_action::redirect(v)) => v,
                _ => panic!(),
            }
        } else {
            RedirectAction::new()
        }
    }

    pub fn get_redirect(&self) -> &RedirectAction {
        match self.action {
            ::std::option::Option::Some(Route_oneof_action::redirect(ref v)) => v,
            _ => RedirectAction::default_instance(),
        }
    }

    // .envoy.api.v2.Metadata metadata = 4;

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

    // .envoy.api.v2.Decorator decorator = 5;

    pub fn clear_decorator(&mut self) {
        self.decorator.clear();
    }

    pub fn has_decorator(&self) -> bool {
        self.decorator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decorator(&mut self, v: Decorator) {
        self.decorator = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_decorator(&mut self) -> &mut Decorator {
        if self.decorator.is_none() {
            self.decorator.set_default();
        }
        self.decorator.as_mut().unwrap()
    }

    // Take field
    pub fn take_decorator(&mut self) -> Decorator {
        self.decorator.take().unwrap_or_else(|| Decorator::new())
    }

    pub fn get_decorator(&self) -> &Decorator {
        self.decorator.as_ref().unwrap_or_else(|| Decorator::default_instance())
    }

    fn get_decorator_for_reflect(&self) -> &::protobuf::SingularPtrField<Decorator> {
        &self.decorator
    }

    fn mut_decorator_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Decorator> {
        &mut self.decorator
    }
}

impl ::protobuf::Message for Route {
    fn is_initialized(&self) -> bool {
        for v in &self.field_match {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(Route_oneof_action::route(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Route_oneof_action::redirect(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.decorator {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_match)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(Route_oneof_action::route(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(Route_oneof_action::redirect(is.read_message()?));
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.decorator)?;
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
        if let Some(ref v) = self.field_match.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.decorator.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &Route_oneof_action::route(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Route_oneof_action::redirect(ref v) => {
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
        if let Some(ref v) = self.field_match.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.decorator.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &Route_oneof_action::route(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Route_oneof_action::redirect(ref v) => {
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

impl ::protobuf::MessageStatic for Route {
    fn new() -> Route {
        Route::new()
    }

    fn descriptor_static(_: ::std::option::Option<Route>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RouteMatch>>(
                    "match",
                    Route::get_field_match_for_reflect,
                    Route::mut_field_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RouteAction>(
                    "route",
                    Route::has_route,
                    Route::get_route,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RedirectAction>(
                    "redirect",
                    Route::has_redirect,
                    Route::get_redirect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Metadata>>(
                    "metadata",
                    Route::get_metadata_for_reflect,
                    Route::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Decorator>>(
                    "decorator",
                    Route::get_decorator_for_reflect,
                    Route::mut_decorator_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Route>(
                    "Route",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Route {
    fn clear(&mut self) {
        self.clear_field_match();
        self.clear_route();
        self.clear_redirect();
        self.clear_metadata();
        self.clear_decorator();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Route {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Route {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VirtualCluster {
    // message fields
    pub pattern: ::std::string::String,
    pub name: ::std::string::String,
    pub method: super::base::RequestMethod,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VirtualCluster {}

impl VirtualCluster {
    pub fn new() -> VirtualCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VirtualCluster {
        static mut instance: ::protobuf::lazy::Lazy<VirtualCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VirtualCluster,
        };
        unsafe {
            instance.get(VirtualCluster::new)
        }
    }

    // string pattern = 1;

    pub fn clear_pattern(&mut self) {
        self.pattern.clear();
    }

    // Param is passed by value, moved
    pub fn set_pattern(&mut self, v: ::std::string::String) {
        self.pattern = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pattern(&mut self) -> &mut ::std::string::String {
        &mut self.pattern
    }

    // Take field
    pub fn take_pattern(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pattern, ::std::string::String::new())
    }

    pub fn get_pattern(&self) -> &str {
        &self.pattern
    }

    fn get_pattern_for_reflect(&self) -> &::std::string::String {
        &self.pattern
    }

    fn mut_pattern_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pattern
    }

    // string name = 2;

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

    // .envoy.api.v2.RequestMethod method = 3;

    pub fn clear_method(&mut self) {
        self.method = super::base::RequestMethod::METHOD_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: super::base::RequestMethod) {
        self.method = v;
    }

    pub fn get_method(&self) -> super::base::RequestMethod {
        self.method
    }

    fn get_method_for_reflect(&self) -> &super::base::RequestMethod {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut super::base::RequestMethod {
        &mut self.method
    }
}

impl ::protobuf::Message for VirtualCluster {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pattern)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.method = tmp;
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
        if !self.pattern.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pattern);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.method != super::base::RequestMethod::METHOD_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(3, self.method);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.pattern.is_empty() {
            os.write_string(1, &self.pattern)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.method != super::base::RequestMethod::METHOD_UNSPECIFIED {
            os.write_enum(3, self.method.value())?;
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

impl ::protobuf::MessageStatic for VirtualCluster {
    fn new() -> VirtualCluster {
        VirtualCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<VirtualCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pattern",
                    VirtualCluster::get_pattern_for_reflect,
                    VirtualCluster::mut_pattern_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    VirtualCluster::get_name_for_reflect,
                    VirtualCluster::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::base::RequestMethod>>(
                    "method",
                    VirtualCluster::get_method_for_reflect,
                    VirtualCluster::mut_method_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VirtualCluster>(
                    "VirtualCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VirtualCluster {
    fn clear(&mut self) {
        self.clear_pattern();
        self.clear_name();
        self.clear_method();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VirtualCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VirtualCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit {
    // message fields
    pub stage: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub disable_key: ::std::string::String,
    pub actions: ::protobuf::RepeatedField<RateLimit_Action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit {}

impl RateLimit {
    pub fn new() -> RateLimit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit,
        };
        unsafe {
            instance.get(RateLimit::new)
        }
    }

    // .google.protobuf.UInt32Value stage = 1;

    pub fn clear_stage(&mut self) {
        self.stage.clear();
    }

    pub fn has_stage(&self) -> bool {
        self.stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.stage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stage(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.stage.is_none() {
            self.stage.set_default();
        }
        self.stage.as_mut().unwrap()
    }

    // Take field
    pub fn take_stage(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.stage.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_stage(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.stage.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_stage_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.stage
    }

    fn mut_stage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.stage
    }

    // string disable_key = 2;

    pub fn clear_disable_key(&mut self) {
        self.disable_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_disable_key(&mut self, v: ::std::string::String) {
        self.disable_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disable_key(&mut self) -> &mut ::std::string::String {
        &mut self.disable_key
    }

    // Take field
    pub fn take_disable_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.disable_key, ::std::string::String::new())
    }

    pub fn get_disable_key(&self) -> &str {
        &self.disable_key
    }

    fn get_disable_key_for_reflect(&self) -> &::std::string::String {
        &self.disable_key
    }

    fn mut_disable_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.disable_key
    }

    // repeated .envoy.api.v2.RateLimit.Action actions = 3;

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_actions(&mut self, v: ::protobuf::RepeatedField<RateLimit_Action>) {
        self.actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_actions(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit_Action> {
        &mut self.actions
    }

    // Take field
    pub fn take_actions(&mut self) -> ::protobuf::RepeatedField<RateLimit_Action> {
        ::std::mem::replace(&mut self.actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_actions(&self) -> &[RateLimit_Action] {
        &self.actions
    }

    fn get_actions_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimit_Action> {
        &self.actions
    }

    fn mut_actions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit_Action> {
        &mut self.actions
    }
}

impl ::protobuf::Message for RateLimit {
    fn is_initialized(&self) -> bool {
        for v in &self.stage {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.actions {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stage)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.disable_key)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.actions)?;
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
        if let Some(ref v) = self.stage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.disable_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.disable_key);
        }
        for value in &self.actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.stage.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.disable_key.is_empty() {
            os.write_string(2, &self.disable_key)?;
        }
        for v in &self.actions {
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

impl ::protobuf::MessageStatic for RateLimit {
    fn new() -> RateLimit {
        RateLimit::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "stage",
                    RateLimit::get_stage_for_reflect,
                    RateLimit::mut_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "disable_key",
                    RateLimit::get_disable_key_for_reflect,
                    RateLimit::mut_disable_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimit_Action>>(
                    "actions",
                    RateLimit::get_actions_for_reflect,
                    RateLimit::mut_actions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit>(
                    "RateLimit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit {
    fn clear(&mut self) {
        self.clear_stage();
        self.clear_disable_key();
        self.clear_actions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action {
    // message oneof groups
    action_specifier: ::std::option::Option<RateLimit_Action_oneof_action_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action {}

#[derive(Clone,PartialEq)]
pub enum RateLimit_Action_oneof_action_specifier {
    source_cluster(RateLimit_Action_SourceCluster),
    destination_cluster(RateLimit_Action_DestinationCluster),
    request_headers(RateLimit_Action_RequestHeaders),
    remote_address(RateLimit_Action_RemoteAddress),
    generic_key(RateLimit_Action_GenericKey),
    header_value_match(RateLimit_Action_HeaderValueMatch),
}

impl RateLimit_Action {
    pub fn new() -> RateLimit_Action {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action,
        };
        unsafe {
            instance.get(RateLimit_Action::new)
        }
    }

    // .envoy.api.v2.RateLimit.Action.SourceCluster source_cluster = 1;

    pub fn clear_source_cluster(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_source_cluster(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_source_cluster(&mut self, v: RateLimit_Action_SourceCluster) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_source_cluster(&mut self) -> &mut RateLimit_Action_SourceCluster {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(RateLimit_Action_SourceCluster::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_source_cluster(&mut self) -> RateLimit_Action_SourceCluster {
        if self.has_source_cluster() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_SourceCluster::new()
        }
    }

    pub fn get_source_cluster(&self) -> &RateLimit_Action_SourceCluster {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(ref v)) => v,
            _ => RateLimit_Action_SourceCluster::default_instance(),
        }
    }

    // .envoy.api.v2.RateLimit.Action.DestinationCluster destination_cluster = 2;

    pub fn clear_destination_cluster(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_destination_cluster(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_destination_cluster(&mut self, v: RateLimit_Action_DestinationCluster) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_destination_cluster(&mut self) -> &mut RateLimit_Action_DestinationCluster {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(RateLimit_Action_DestinationCluster::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_destination_cluster(&mut self) -> RateLimit_Action_DestinationCluster {
        if self.has_destination_cluster() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_DestinationCluster::new()
        }
    }

    pub fn get_destination_cluster(&self) -> &RateLimit_Action_DestinationCluster {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(ref v)) => v,
            _ => RateLimit_Action_DestinationCluster::default_instance(),
        }
    }

    // .envoy.api.v2.RateLimit.Action.RequestHeaders request_headers = 3;

    pub fn clear_request_headers(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_request_headers(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_headers(&mut self, v: RateLimit_Action_RequestHeaders) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(v))
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers(&mut self) -> &mut RateLimit_Action_RequestHeaders {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(RateLimit_Action_RequestHeaders::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_headers(&mut self) -> RateLimit_Action_RequestHeaders {
        if self.has_request_headers() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_RequestHeaders::new()
        }
    }

    pub fn get_request_headers(&self) -> &RateLimit_Action_RequestHeaders {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(ref v)) => v,
            _ => RateLimit_Action_RequestHeaders::default_instance(),
        }
    }

    // .envoy.api.v2.RateLimit.Action.RemoteAddress remote_address = 4;

    pub fn clear_remote_address(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_remote_address(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_remote_address(&mut self, v: RateLimit_Action_RemoteAddress) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(v))
    }

    // Mutable pointer to the field.
    pub fn mut_remote_address(&mut self) -> &mut RateLimit_Action_RemoteAddress {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(RateLimit_Action_RemoteAddress::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_remote_address(&mut self) -> RateLimit_Action_RemoteAddress {
        if self.has_remote_address() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_RemoteAddress::new()
        }
    }

    pub fn get_remote_address(&self) -> &RateLimit_Action_RemoteAddress {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(ref v)) => v,
            _ => RateLimit_Action_RemoteAddress::default_instance(),
        }
    }

    // .envoy.api.v2.RateLimit.Action.GenericKey generic_key = 5;

    pub fn clear_generic_key(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_generic_key(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_generic_key(&mut self, v: RateLimit_Action_GenericKey) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_generic_key(&mut self) -> &mut RateLimit_Action_GenericKey {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(RateLimit_Action_GenericKey::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_generic_key(&mut self) -> RateLimit_Action_GenericKey {
        if self.has_generic_key() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_GenericKey::new()
        }
    }

    pub fn get_generic_key(&self) -> &RateLimit_Action_GenericKey {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(ref v)) => v,
            _ => RateLimit_Action_GenericKey::default_instance(),
        }
    }

    // .envoy.api.v2.RateLimit.Action.HeaderValueMatch header_value_match = 6;

    pub fn clear_header_value_match(&mut self) {
        self.action_specifier = ::std::option::Option::None;
    }

    pub fn has_header_value_match(&self) -> bool {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_header_value_match(&mut self, v: RateLimit_Action_HeaderValueMatch) {
        self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(v))
    }

    // Mutable pointer to the field.
    pub fn mut_header_value_match(&mut self) -> &mut RateLimit_Action_HeaderValueMatch {
        if let ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(_)) = self.action_specifier {
        } else {
            self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(RateLimit_Action_HeaderValueMatch::new()));
        }
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_header_value_match(&mut self) -> RateLimit_Action_HeaderValueMatch {
        if self.has_header_value_match() {
            match self.action_specifier.take() {
                ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(v)) => v,
                _ => panic!(),
            }
        } else {
            RateLimit_Action_HeaderValueMatch::new()
        }
    }

    pub fn get_header_value_match(&self) -> &RateLimit_Action_HeaderValueMatch {
        match self.action_specifier {
            ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(ref v)) => v,
            _ => RateLimit_Action_HeaderValueMatch::default_instance(),
        }
    }
}

impl ::protobuf::Message for RateLimit_Action {
    fn is_initialized(&self) -> bool {
        if let Some(RateLimit_Action_oneof_action_specifier::source_cluster(ref v)) = self.action_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RateLimit_Action_oneof_action_specifier::destination_cluster(ref v)) = self.action_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RateLimit_Action_oneof_action_specifier::request_headers(ref v)) = self.action_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RateLimit_Action_oneof_action_specifier::remote_address(ref v)) = self.action_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RateLimit_Action_oneof_action_specifier::generic_key(ref v)) = self.action_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(RateLimit_Action_oneof_action_specifier::header_value_match(ref v)) = self.action_specifier {
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
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::source_cluster(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::destination_cluster(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::request_headers(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::remote_address(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::generic_key(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action_specifier = ::std::option::Option::Some(RateLimit_Action_oneof_action_specifier::header_value_match(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.action_specifier {
            match v {
                &RateLimit_Action_oneof_action_specifier::source_cluster(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RateLimit_Action_oneof_action_specifier::destination_cluster(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RateLimit_Action_oneof_action_specifier::request_headers(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RateLimit_Action_oneof_action_specifier::remote_address(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RateLimit_Action_oneof_action_specifier::generic_key(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RateLimit_Action_oneof_action_specifier::header_value_match(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.action_specifier {
            match v {
                &RateLimit_Action_oneof_action_specifier::source_cluster(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RateLimit_Action_oneof_action_specifier::destination_cluster(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RateLimit_Action_oneof_action_specifier::request_headers(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RateLimit_Action_oneof_action_specifier::remote_address(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RateLimit_Action_oneof_action_specifier::generic_key(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RateLimit_Action_oneof_action_specifier::header_value_match(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RateLimit_Action {
    fn new() -> RateLimit_Action {
        RateLimit_Action::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_SourceCluster>(
                    "source_cluster",
                    RateLimit_Action::has_source_cluster,
                    RateLimit_Action::get_source_cluster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_DestinationCluster>(
                    "destination_cluster",
                    RateLimit_Action::has_destination_cluster,
                    RateLimit_Action::get_destination_cluster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_RequestHeaders>(
                    "request_headers",
                    RateLimit_Action::has_request_headers,
                    RateLimit_Action::get_request_headers,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_RemoteAddress>(
                    "remote_address",
                    RateLimit_Action::has_remote_address,
                    RateLimit_Action::get_remote_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_GenericKey>(
                    "generic_key",
                    RateLimit_Action::has_generic_key,
                    RateLimit_Action::get_generic_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RateLimit_Action_HeaderValueMatch>(
                    "header_value_match",
                    RateLimit_Action::has_header_value_match,
                    RateLimit_Action::get_header_value_match,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action>(
                    "RateLimit_Action",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action {
    fn clear(&mut self) {
        self.clear_source_cluster();
        self.clear_destination_cluster();
        self.clear_request_headers();
        self.clear_remote_address();
        self.clear_generic_key();
        self.clear_header_value_match();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_SourceCluster {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_SourceCluster {}

impl RateLimit_Action_SourceCluster {
    pub fn new() -> RateLimit_Action_SourceCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_SourceCluster {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_SourceCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_SourceCluster,
        };
        unsafe {
            instance.get(RateLimit_Action_SourceCluster::new)
        }
    }
}

impl ::protobuf::Message for RateLimit_Action_SourceCluster {
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

impl ::protobuf::MessageStatic for RateLimit_Action_SourceCluster {
    fn new() -> RateLimit_Action_SourceCluster {
        RateLimit_Action_SourceCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_SourceCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_SourceCluster>(
                    "RateLimit_Action_SourceCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_SourceCluster {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_SourceCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_SourceCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_DestinationCluster {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_DestinationCluster {}

impl RateLimit_Action_DestinationCluster {
    pub fn new() -> RateLimit_Action_DestinationCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_DestinationCluster {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_DestinationCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_DestinationCluster,
        };
        unsafe {
            instance.get(RateLimit_Action_DestinationCluster::new)
        }
    }
}

impl ::protobuf::Message for RateLimit_Action_DestinationCluster {
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

impl ::protobuf::MessageStatic for RateLimit_Action_DestinationCluster {
    fn new() -> RateLimit_Action_DestinationCluster {
        RateLimit_Action_DestinationCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_DestinationCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_DestinationCluster>(
                    "RateLimit_Action_DestinationCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_DestinationCluster {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_DestinationCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_DestinationCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_RequestHeaders {
    // message fields
    pub header_name: ::std::string::String,
    pub descriptor_key: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_RequestHeaders {}

impl RateLimit_Action_RequestHeaders {
    pub fn new() -> RateLimit_Action_RequestHeaders {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_RequestHeaders {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_RequestHeaders> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_RequestHeaders,
        };
        unsafe {
            instance.get(RateLimit_Action_RequestHeaders::new)
        }
    }

    // string header_name = 1;

    pub fn clear_header_name(&mut self) {
        self.header_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_header_name(&mut self, v: ::std::string::String) {
        self.header_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header_name(&mut self) -> &mut ::std::string::String {
        &mut self.header_name
    }

    // Take field
    pub fn take_header_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.header_name, ::std::string::String::new())
    }

    pub fn get_header_name(&self) -> &str {
        &self.header_name
    }

    fn get_header_name_for_reflect(&self) -> &::std::string::String {
        &self.header_name
    }

    fn mut_header_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.header_name
    }

    // string descriptor_key = 2;

    pub fn clear_descriptor_key(&mut self) {
        self.descriptor_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptor_key(&mut self, v: ::std::string::String) {
        self.descriptor_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptor_key(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_key
    }

    // Take field
    pub fn take_descriptor_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptor_key, ::std::string::String::new())
    }

    pub fn get_descriptor_key(&self) -> &str {
        &self.descriptor_key
    }

    fn get_descriptor_key_for_reflect(&self) -> &::std::string::String {
        &self.descriptor_key
    }

    fn mut_descriptor_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_key
    }
}

impl ::protobuf::Message for RateLimit_Action_RequestHeaders {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.header_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptor_key)?;
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
        if !self.header_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.header_name);
        }
        if !self.descriptor_key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.descriptor_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.header_name.is_empty() {
            os.write_string(1, &self.header_name)?;
        }
        if !self.descriptor_key.is_empty() {
            os.write_string(2, &self.descriptor_key)?;
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

impl ::protobuf::MessageStatic for RateLimit_Action_RequestHeaders {
    fn new() -> RateLimit_Action_RequestHeaders {
        RateLimit_Action_RequestHeaders::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_RequestHeaders>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "header_name",
                    RateLimit_Action_RequestHeaders::get_header_name_for_reflect,
                    RateLimit_Action_RequestHeaders::mut_header_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptor_key",
                    RateLimit_Action_RequestHeaders::get_descriptor_key_for_reflect,
                    RateLimit_Action_RequestHeaders::mut_descriptor_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_RequestHeaders>(
                    "RateLimit_Action_RequestHeaders",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_RequestHeaders {
    fn clear(&mut self) {
        self.clear_header_name();
        self.clear_descriptor_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_RequestHeaders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_RequestHeaders {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_RemoteAddress {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_RemoteAddress {}

impl RateLimit_Action_RemoteAddress {
    pub fn new() -> RateLimit_Action_RemoteAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_RemoteAddress {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_RemoteAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_RemoteAddress,
        };
        unsafe {
            instance.get(RateLimit_Action_RemoteAddress::new)
        }
    }
}

impl ::protobuf::Message for RateLimit_Action_RemoteAddress {
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

impl ::protobuf::MessageStatic for RateLimit_Action_RemoteAddress {
    fn new() -> RateLimit_Action_RemoteAddress {
        RateLimit_Action_RemoteAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_RemoteAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_RemoteAddress>(
                    "RateLimit_Action_RemoteAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_RemoteAddress {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_RemoteAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_RemoteAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_GenericKey {
    // message fields
    pub descriptor_value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_GenericKey {}

impl RateLimit_Action_GenericKey {
    pub fn new() -> RateLimit_Action_GenericKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_GenericKey {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_GenericKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_GenericKey,
        };
        unsafe {
            instance.get(RateLimit_Action_GenericKey::new)
        }
    }

    // string descriptor_value = 1;

    pub fn clear_descriptor_value(&mut self) {
        self.descriptor_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptor_value(&mut self, v: ::std::string::String) {
        self.descriptor_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptor_value(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_value
    }

    // Take field
    pub fn take_descriptor_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptor_value, ::std::string::String::new())
    }

    pub fn get_descriptor_value(&self) -> &str {
        &self.descriptor_value
    }

    fn get_descriptor_value_for_reflect(&self) -> &::std::string::String {
        &self.descriptor_value
    }

    fn mut_descriptor_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_value
    }
}

impl ::protobuf::Message for RateLimit_Action_GenericKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptor_value)?;
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
        if !self.descriptor_value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.descriptor_value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.descriptor_value.is_empty() {
            os.write_string(1, &self.descriptor_value)?;
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

impl ::protobuf::MessageStatic for RateLimit_Action_GenericKey {
    fn new() -> RateLimit_Action_GenericKey {
        RateLimit_Action_GenericKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_GenericKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptor_value",
                    RateLimit_Action_GenericKey::get_descriptor_value_for_reflect,
                    RateLimit_Action_GenericKey::mut_descriptor_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_GenericKey>(
                    "RateLimit_Action_GenericKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_GenericKey {
    fn clear(&mut self) {
        self.clear_descriptor_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_GenericKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_GenericKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit_Action_HeaderValueMatch {
    // message fields
    pub descriptor_value: ::std::string::String,
    pub expect_match: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub headers: ::protobuf::RepeatedField<HeaderMatcher>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimit_Action_HeaderValueMatch {}

impl RateLimit_Action_HeaderValueMatch {
    pub fn new() -> RateLimit_Action_HeaderValueMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit_Action_HeaderValueMatch {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit_Action_HeaderValueMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit_Action_HeaderValueMatch,
        };
        unsafe {
            instance.get(RateLimit_Action_HeaderValueMatch::new)
        }
    }

    // string descriptor_value = 1;

    pub fn clear_descriptor_value(&mut self) {
        self.descriptor_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptor_value(&mut self, v: ::std::string::String) {
        self.descriptor_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_descriptor_value(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_value
    }

    // Take field
    pub fn take_descriptor_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.descriptor_value, ::std::string::String::new())
    }

    pub fn get_descriptor_value(&self) -> &str {
        &self.descriptor_value
    }

    fn get_descriptor_value_for_reflect(&self) -> &::std::string::String {
        &self.descriptor_value
    }

    fn mut_descriptor_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.descriptor_value
    }

    // .google.protobuf.BoolValue expect_match = 2;

    pub fn clear_expect_match(&mut self) {
        self.expect_match.clear();
    }

    pub fn has_expect_match(&self) -> bool {
        self.expect_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expect_match(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.expect_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expect_match(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.expect_match.is_none() {
            self.expect_match.set_default();
        }
        self.expect_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_expect_match(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.expect_match.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_expect_match(&self) -> &::protobuf::well_known_types::BoolValue {
        self.expect_match.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_expect_match_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.expect_match
    }

    fn mut_expect_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.expect_match
    }

    // repeated .envoy.api.v2.HeaderMatcher headers = 3;

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<HeaderMatcher>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<HeaderMatcher> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<HeaderMatcher> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_headers(&self) -> &[HeaderMatcher] {
        &self.headers
    }

    fn get_headers_for_reflect(&self) -> &::protobuf::RepeatedField<HeaderMatcher> {
        &self.headers
    }

    fn mut_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HeaderMatcher> {
        &mut self.headers
    }
}

impl ::protobuf::Message for RateLimit_Action_HeaderValueMatch {
    fn is_initialized(&self) -> bool {
        for v in &self.expect_match {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.headers {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.descriptor_value)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expect_match)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
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
        if !self.descriptor_value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.descriptor_value);
        }
        if let Some(ref v) = self.expect_match.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.descriptor_value.is_empty() {
            os.write_string(1, &self.descriptor_value)?;
        }
        if let Some(ref v) = self.expect_match.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.headers {
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

impl ::protobuf::MessageStatic for RateLimit_Action_HeaderValueMatch {
    fn new() -> RateLimit_Action_HeaderValueMatch {
        RateLimit_Action_HeaderValueMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimit_Action_HeaderValueMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "descriptor_value",
                    RateLimit_Action_HeaderValueMatch::get_descriptor_value_for_reflect,
                    RateLimit_Action_HeaderValueMatch::mut_descriptor_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "expect_match",
                    RateLimit_Action_HeaderValueMatch::get_expect_match_for_reflect,
                    RateLimit_Action_HeaderValueMatch::mut_expect_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HeaderMatcher>>(
                    "headers",
                    RateLimit_Action_HeaderValueMatch::get_headers_for_reflect,
                    RateLimit_Action_HeaderValueMatch::mut_headers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit_Action_HeaderValueMatch>(
                    "RateLimit_Action_HeaderValueMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimit_Action_HeaderValueMatch {
    fn clear(&mut self) {
        self.clear_descriptor_value();
        self.clear_expect_match();
        self.clear_headers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit_Action_HeaderValueMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit_Action_HeaderValueMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeaderMatcher {
    // message fields
    pub name: ::std::string::String,
    pub value: ::std::string::String,
    pub regex: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeaderMatcher {}

impl HeaderMatcher {
    pub fn new() -> HeaderMatcher {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeaderMatcher {
        static mut instance: ::protobuf::lazy::Lazy<HeaderMatcher> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderMatcher,
        };
        unsafe {
            instance.get(HeaderMatcher::new)
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

    // .google.protobuf.BoolValue regex = 3;

    pub fn clear_regex(&mut self) {
        self.regex.clear();
    }

    pub fn has_regex(&self) -> bool {
        self.regex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_regex(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.regex = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_regex(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.regex.is_none() {
            self.regex.set_default();
        }
        self.regex.as_mut().unwrap()
    }

    // Take field
    pub fn take_regex(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.regex.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_regex(&self) -> &::protobuf::well_known_types::BoolValue {
        self.regex.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_regex_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.regex
    }

    fn mut_regex_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.regex
    }
}

impl ::protobuf::Message for HeaderMatcher {
    fn is_initialized(&self) -> bool {
        for v in &self.regex {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.regex)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.value);
        }
        if let Some(ref v) = self.regex.as_ref() {
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
        if !self.value.is_empty() {
            os.write_string(2, &self.value)?;
        }
        if let Some(ref v) = self.regex.as_ref() {
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

impl ::protobuf::MessageStatic for HeaderMatcher {
    fn new() -> HeaderMatcher {
        HeaderMatcher::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeaderMatcher>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    HeaderMatcher::get_name_for_reflect,
                    HeaderMatcher::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    HeaderMatcher::get_value_for_reflect,
                    HeaderMatcher::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "regex",
                    HeaderMatcher::get_regex_for_reflect,
                    HeaderMatcher::mut_regex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeaderMatcher>(
                    "HeaderMatcher",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeaderMatcher {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.clear_regex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeaderMatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeaderMatcher {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VirtualHost {
    // message fields
    pub name: ::std::string::String,
    pub domains: ::protobuf::RepeatedField<::std::string::String>,
    pub routes: ::protobuf::RepeatedField<Route>,
    pub require_tls: VirtualHost_TlsRequirementType,
    pub virtual_clusters: ::protobuf::RepeatedField<VirtualCluster>,
    pub rate_limits: ::protobuf::RepeatedField<RateLimit>,
    pub request_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub cors: ::protobuf::SingularPtrField<CorsPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VirtualHost {}

impl VirtualHost {
    pub fn new() -> VirtualHost {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VirtualHost {
        static mut instance: ::protobuf::lazy::Lazy<VirtualHost> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VirtualHost,
        };
        unsafe {
            instance.get(VirtualHost::new)
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

    // repeated string domains = 2;

    pub fn clear_domains(&mut self) {
        self.domains.clear();
    }

    // Param is passed by value, moved
    pub fn set_domains(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.domains = v;
    }

    // Mutable pointer to the field.
    pub fn mut_domains(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.domains
    }

    // Take field
    pub fn take_domains(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.domains, ::protobuf::RepeatedField::new())
    }

    pub fn get_domains(&self) -> &[::std::string::String] {
        &self.domains
    }

    fn get_domains_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.domains
    }

    fn mut_domains_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.domains
    }

    // repeated .envoy.api.v2.Route routes = 3;

    pub fn clear_routes(&mut self) {
        self.routes.clear();
    }

    // Param is passed by value, moved
    pub fn set_routes(&mut self, v: ::protobuf::RepeatedField<Route>) {
        self.routes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_routes(&mut self) -> &mut ::protobuf::RepeatedField<Route> {
        &mut self.routes
    }

    // Take field
    pub fn take_routes(&mut self) -> ::protobuf::RepeatedField<Route> {
        ::std::mem::replace(&mut self.routes, ::protobuf::RepeatedField::new())
    }

    pub fn get_routes(&self) -> &[Route] {
        &self.routes
    }

    fn get_routes_for_reflect(&self) -> &::protobuf::RepeatedField<Route> {
        &self.routes
    }

    fn mut_routes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Route> {
        &mut self.routes
    }

    // .envoy.api.v2.VirtualHost.TlsRequirementType require_tls = 4;

    pub fn clear_require_tls(&mut self) {
        self.require_tls = VirtualHost_TlsRequirementType::NONE;
    }

    // Param is passed by value, moved
    pub fn set_require_tls(&mut self, v: VirtualHost_TlsRequirementType) {
        self.require_tls = v;
    }

    pub fn get_require_tls(&self) -> VirtualHost_TlsRequirementType {
        self.require_tls
    }

    fn get_require_tls_for_reflect(&self) -> &VirtualHost_TlsRequirementType {
        &self.require_tls
    }

    fn mut_require_tls_for_reflect(&mut self) -> &mut VirtualHost_TlsRequirementType {
        &mut self.require_tls
    }

    // repeated .envoy.api.v2.VirtualCluster virtual_clusters = 5;

    pub fn clear_virtual_clusters(&mut self) {
        self.virtual_clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_virtual_clusters(&mut self, v: ::protobuf::RepeatedField<VirtualCluster>) {
        self.virtual_clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_virtual_clusters(&mut self) -> &mut ::protobuf::RepeatedField<VirtualCluster> {
        &mut self.virtual_clusters
    }

    // Take field
    pub fn take_virtual_clusters(&mut self) -> ::protobuf::RepeatedField<VirtualCluster> {
        ::std::mem::replace(&mut self.virtual_clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_virtual_clusters(&self) -> &[VirtualCluster] {
        &self.virtual_clusters
    }

    fn get_virtual_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<VirtualCluster> {
        &self.virtual_clusters
    }

    fn mut_virtual_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VirtualCluster> {
        &mut self.virtual_clusters
    }

    // repeated .envoy.api.v2.RateLimit rate_limits = 6;

    pub fn clear_rate_limits(&mut self) {
        self.rate_limits.clear();
    }

    // Param is passed by value, moved
    pub fn set_rate_limits(&mut self, v: ::protobuf::RepeatedField<RateLimit>) {
        self.rate_limits = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rate_limits(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit> {
        &mut self.rate_limits
    }

    // Take field
    pub fn take_rate_limits(&mut self) -> ::protobuf::RepeatedField<RateLimit> {
        ::std::mem::replace(&mut self.rate_limits, ::protobuf::RepeatedField::new())
    }

    pub fn get_rate_limits(&self) -> &[RateLimit] {
        &self.rate_limits
    }

    fn get_rate_limits_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimit> {
        &self.rate_limits
    }

    fn mut_rate_limits_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimit> {
        &mut self.rate_limits
    }

    // repeated .envoy.api.v2.HeaderValueOption request_headers_to_add = 7;

    pub fn clear_request_headers_to_add(&mut self) {
        self.request_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.request_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // Take field
    pub fn take_request_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.request_headers_to_add, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.request_headers_to_add
    }

    fn get_request_headers_to_add_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &self.request_headers_to_add
    }

    fn mut_request_headers_to_add_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // .envoy.api.v2.CorsPolicy cors = 8;

    pub fn clear_cors(&mut self) {
        self.cors.clear();
    }

    pub fn has_cors(&self) -> bool {
        self.cors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cors(&mut self, v: CorsPolicy) {
        self.cors = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cors(&mut self) -> &mut CorsPolicy {
        if self.cors.is_none() {
            self.cors.set_default();
        }
        self.cors.as_mut().unwrap()
    }

    // Take field
    pub fn take_cors(&mut self) -> CorsPolicy {
        self.cors.take().unwrap_or_else(|| CorsPolicy::new())
    }

    pub fn get_cors(&self) -> &CorsPolicy {
        self.cors.as_ref().unwrap_or_else(|| CorsPolicy::default_instance())
    }

    fn get_cors_for_reflect(&self) -> &::protobuf::SingularPtrField<CorsPolicy> {
        &self.cors
    }

    fn mut_cors_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CorsPolicy> {
        &mut self.cors
    }
}

impl ::protobuf::Message for VirtualHost {
    fn is_initialized(&self) -> bool {
        for v in &self.routes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.virtual_clusters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rate_limits {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cors {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.domains)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.routes)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.require_tls = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.virtual_clusters)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rate_limits)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_headers_to_add)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cors)?;
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
        for value in &self.domains {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.routes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.require_tls != VirtualHost_TlsRequirementType::NONE {
            my_size += ::protobuf::rt::enum_size(4, self.require_tls);
        }
        for value in &self.virtual_clusters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rate_limits {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.request_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.cors.as_ref() {
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
        for v in &self.domains {
            os.write_string(2, &v)?;
        };
        for v in &self.routes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.require_tls != VirtualHost_TlsRequirementType::NONE {
            os.write_enum(4, self.require_tls.value())?;
        }
        for v in &self.virtual_clusters {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.rate_limits {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.request_headers_to_add {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.cors.as_ref() {
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

impl ::protobuf::MessageStatic for VirtualHost {
    fn new() -> VirtualHost {
        VirtualHost::new()
    }

    fn descriptor_static(_: ::std::option::Option<VirtualHost>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    VirtualHost::get_name_for_reflect,
                    VirtualHost::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "domains",
                    VirtualHost::get_domains_for_reflect,
                    VirtualHost::mut_domains_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Route>>(
                    "routes",
                    VirtualHost::get_routes_for_reflect,
                    VirtualHost::mut_routes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<VirtualHost_TlsRequirementType>>(
                    "require_tls",
                    VirtualHost::get_require_tls_for_reflect,
                    VirtualHost::mut_require_tls_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VirtualCluster>>(
                    "virtual_clusters",
                    VirtualHost::get_virtual_clusters_for_reflect,
                    VirtualHost::mut_virtual_clusters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimit>>(
                    "rate_limits",
                    VirtualHost::get_rate_limits_for_reflect,
                    VirtualHost::mut_rate_limits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                    "request_headers_to_add",
                    VirtualHost::get_request_headers_to_add_for_reflect,
                    VirtualHost::mut_request_headers_to_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CorsPolicy>>(
                    "cors",
                    VirtualHost::get_cors_for_reflect,
                    VirtualHost::mut_cors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VirtualHost>(
                    "VirtualHost",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VirtualHost {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_domains();
        self.clear_routes();
        self.clear_require_tls();
        self.clear_virtual_clusters();
        self.clear_rate_limits();
        self.clear_request_headers_to_add();
        self.clear_cors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VirtualHost {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VirtualHost {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum VirtualHost_TlsRequirementType {
    NONE = 0,
    EXTERNAL_ONLY = 1,
    ALL = 2,
}

impl ::protobuf::ProtobufEnum for VirtualHost_TlsRequirementType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VirtualHost_TlsRequirementType> {
        match value {
            0 => ::std::option::Option::Some(VirtualHost_TlsRequirementType::NONE),
            1 => ::std::option::Option::Some(VirtualHost_TlsRequirementType::EXTERNAL_ONLY),
            2 => ::std::option::Option::Some(VirtualHost_TlsRequirementType::ALL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [VirtualHost_TlsRequirementType] = &[
            VirtualHost_TlsRequirementType::NONE,
            VirtualHost_TlsRequirementType::EXTERNAL_ONLY,
            VirtualHost_TlsRequirementType::ALL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<VirtualHost_TlsRequirementType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("VirtualHost_TlsRequirementType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for VirtualHost_TlsRequirementType {
}

impl ::std::default::Default for VirtualHost_TlsRequirementType {
    fn default() -> Self {
        VirtualHost_TlsRequirementType::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for VirtualHost_TlsRequirementType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RouteConfiguration {
    // message fields
    pub name: ::std::string::String,
    pub virtual_hosts: ::protobuf::RepeatedField<VirtualHost>,
    pub internal_only_headers: ::protobuf::RepeatedField<::std::string::String>,
    pub response_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub response_headers_to_remove: ::protobuf::RepeatedField<::std::string::String>,
    pub request_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub validate_clusters: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RouteConfiguration {}

impl RouteConfiguration {
    pub fn new() -> RouteConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RouteConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<RouteConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RouteConfiguration,
        };
        unsafe {
            instance.get(RouteConfiguration::new)
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

    // repeated .envoy.api.v2.VirtualHost virtual_hosts = 2;

    pub fn clear_virtual_hosts(&mut self) {
        self.virtual_hosts.clear();
    }

    // Param is passed by value, moved
    pub fn set_virtual_hosts(&mut self, v: ::protobuf::RepeatedField<VirtualHost>) {
        self.virtual_hosts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_virtual_hosts(&mut self) -> &mut ::protobuf::RepeatedField<VirtualHost> {
        &mut self.virtual_hosts
    }

    // Take field
    pub fn take_virtual_hosts(&mut self) -> ::protobuf::RepeatedField<VirtualHost> {
        ::std::mem::replace(&mut self.virtual_hosts, ::protobuf::RepeatedField::new())
    }

    pub fn get_virtual_hosts(&self) -> &[VirtualHost] {
        &self.virtual_hosts
    }

    fn get_virtual_hosts_for_reflect(&self) -> &::protobuf::RepeatedField<VirtualHost> {
        &self.virtual_hosts
    }

    fn mut_virtual_hosts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VirtualHost> {
        &mut self.virtual_hosts
    }

    // repeated string internal_only_headers = 3;

    pub fn clear_internal_only_headers(&mut self) {
        self.internal_only_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_internal_only_headers(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.internal_only_headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_internal_only_headers(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.internal_only_headers
    }

    // Take field
    pub fn take_internal_only_headers(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.internal_only_headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_internal_only_headers(&self) -> &[::std::string::String] {
        &self.internal_only_headers
    }

    fn get_internal_only_headers_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.internal_only_headers
    }

    fn mut_internal_only_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.internal_only_headers
    }

    // repeated .envoy.api.v2.HeaderValueOption response_headers_to_add = 4;

    pub fn clear_response_headers_to_add(&mut self) {
        self.response_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.response_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.response_headers_to_add
    }

    // Take field
    pub fn take_response_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.response_headers_to_add, ::protobuf::RepeatedField::new())
    }

    pub fn get_response_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.response_headers_to_add
    }

    fn get_response_headers_to_add_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &self.response_headers_to_add
    }

    fn mut_response_headers_to_add_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.response_headers_to_add
    }

    // repeated string response_headers_to_remove = 5;

    pub fn clear_response_headers_to_remove(&mut self) {
        self.response_headers_to_remove.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_headers_to_remove(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.response_headers_to_remove = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_headers_to_remove(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.response_headers_to_remove
    }

    // Take field
    pub fn take_response_headers_to_remove(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.response_headers_to_remove, ::protobuf::RepeatedField::new())
    }

    pub fn get_response_headers_to_remove(&self) -> &[::std::string::String] {
        &self.response_headers_to_remove
    }

    fn get_response_headers_to_remove_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.response_headers_to_remove
    }

    fn mut_response_headers_to_remove_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.response_headers_to_remove
    }

    // repeated .envoy.api.v2.HeaderValueOption request_headers_to_add = 6;

    pub fn clear_request_headers_to_add(&mut self) {
        self.request_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.request_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // Take field
    pub fn take_request_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.request_headers_to_add, ::protobuf::RepeatedField::new())
    }

    pub fn get_request_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.request_headers_to_add
    }

    fn get_request_headers_to_add_for_reflect(&self) -> &::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &self.request_headers_to_add
    }

    fn mut_request_headers_to_add_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // .google.protobuf.BoolValue validate_clusters = 7;

    pub fn clear_validate_clusters(&mut self) {
        self.validate_clusters.clear();
    }

    pub fn has_validate_clusters(&self) -> bool {
        self.validate_clusters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_validate_clusters(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.validate_clusters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validate_clusters(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.validate_clusters.is_none() {
            self.validate_clusters.set_default();
        }
        self.validate_clusters.as_mut().unwrap()
    }

    // Take field
    pub fn take_validate_clusters(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.validate_clusters.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_validate_clusters(&self) -> &::protobuf::well_known_types::BoolValue {
        self.validate_clusters.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_validate_clusters_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.validate_clusters
    }

    fn mut_validate_clusters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.validate_clusters
    }
}

impl ::protobuf::Message for RouteConfiguration {
    fn is_initialized(&self) -> bool {
        for v in &self.virtual_hosts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.validate_clusters {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.virtual_hosts)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.internal_only_headers)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.response_headers_to_add)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.response_headers_to_remove)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_headers_to_add)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.validate_clusters)?;
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
        for value in &self.virtual_hosts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.internal_only_headers {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.response_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.response_headers_to_remove {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.request_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.validate_clusters.as_ref() {
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
        for v in &self.virtual_hosts {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.internal_only_headers {
            os.write_string(3, &v)?;
        };
        for v in &self.response_headers_to_add {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.response_headers_to_remove {
            os.write_string(5, &v)?;
        };
        for v in &self.request_headers_to_add {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.validate_clusters.as_ref() {
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

impl ::protobuf::MessageStatic for RouteConfiguration {
    fn new() -> RouteConfiguration {
        RouteConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<RouteConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    RouteConfiguration::get_name_for_reflect,
                    RouteConfiguration::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VirtualHost>>(
                    "virtual_hosts",
                    RouteConfiguration::get_virtual_hosts_for_reflect,
                    RouteConfiguration::mut_virtual_hosts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "internal_only_headers",
                    RouteConfiguration::get_internal_only_headers_for_reflect,
                    RouteConfiguration::mut_internal_only_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                    "response_headers_to_add",
                    RouteConfiguration::get_response_headers_to_add_for_reflect,
                    RouteConfiguration::mut_response_headers_to_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "response_headers_to_remove",
                    RouteConfiguration::get_response_headers_to_remove_for_reflect,
                    RouteConfiguration::mut_response_headers_to_remove_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                    "request_headers_to_add",
                    RouteConfiguration::get_request_headers_to_add_for_reflect,
                    RouteConfiguration::mut_request_headers_to_add_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "validate_clusters",
                    RouteConfiguration::get_validate_clusters_for_reflect,
                    RouteConfiguration::mut_validate_clusters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RouteConfiguration>(
                    "RouteConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RouteConfiguration {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_virtual_hosts();
        self.clear_internal_only_headers();
        self.clear_response_headers_to_add();
        self.clear_response_headers_to_remove();
        self.clear_request_headers_to_add();
        self.clear_validate_clusters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/rds.proto\x12\x0cenvoy.api.v2\x1a\x0eapi/base.proto\x1a\x13api/d\
    iscovery.proto\x1a\x1cgoogle/api/annotations.proto\x1a\x1egoogle/protobu\
    f/duration.proto\x1a\x1egoogle/protobuf/wrappers.proto\"\xa3\x02\n\x0fWe\
    ightedCluster\x12G\n\x08clusters\x18\x01\x20\x03(\x0b2+.envoy.api.v2.Wei\
    ghtedCluster.ClusterWeightR\x08clusters\x12,\n\x12runtime_key_prefix\x18\
    \x02\x20\x01(\tR\x10runtimeKeyPrefix\x1a\x98\x01\n\rClusterWeight\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x124\n\x06weight\x18\x02\x20\
    \x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x06weight\x12=\n\x0emetadata\
    _match\x18\x03\x20\x01(\x0b2\x16.envoy.api.v2.MetadataR\rmetadataMatch\"\
    \x97\x02\n\nRouteMatch\x12\x18\n\x06prefix\x18\x01\x20\x01(\tH\0R\x06pre\
    fix\x12\x14\n\x04path\x18\x02\x20\x01(\tH\0R\x04path\x12\x16\n\x05regex\
    \x18\x03\x20\x01(\tH\0R\x05regex\x12A\n\x0ecase_sensitive\x18\x04\x20\
    \x01(\x0b2\x1a.google.protobuf.BoolValueR\rcaseSensitive\x125\n\x07runti\
    me\x18\x05\x20\x01(\x0b2\x1b.envoy.api.v2.RuntimeUInt32R\x07runtime\x125\
    \n\x07headers\x18\x06\x20\x03(\x0b2\x1b.envoy.api.v2.HeaderMatcherR\x07h\
    eadersB\x10\n\x0epath_specifier\"\xb8\x02\n\nCorsPolicy\x12!\n\x0callow_\
    origin\x18\x01\x20\x03(\tR\x0ballowOrigin\x12#\n\rallow_methods\x18\x02\
    \x20\x01(\tR\x0callowMethods\x12#\n\rallow_headers\x18\x03\x20\x01(\tR\
    \x0callowHeaders\x12%\n\x0eexpose_headers\x18\x04\x20\x01(\tR\rexposeHea\
    ders\x12\x17\n\x07max_age\x18\x05\x20\x01(\tR\x06maxAge\x12G\n\x11allow_\
    credentials\x18\x06\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\x10allo\
    wCredentials\x124\n\x07enabled\x18\x07\x20\x01(\x0b2\x1a.google.protobuf\
    .BoolValueR\x07enabled\"\xa5\x0e\n\x0bRouteAction\x12\x1a\n\x07cluster\
    \x18\x01\x20\x01(\tH\0R\x07cluster\x12'\n\x0ecluster_header\x18\x02\x20\
    \x01(\tH\0R\rclusterHeader\x12L\n\x11weighted_clusters\x18\x03\x20\x01(\
    \x0b2\x1d.envoy.api.v2.WeightedClusterH\0R\x10weightedClusters\x12=\n\
    \x0emetadata_match\x18\x04\x20\x01(\x0b2\x16.envoy.api.v2.MetadataR\rmet\
    adataMatch\x12%\n\x0eprefix_rewrite\x18\x05\x20\x01(\tR\rprefixRewrite\
    \x12#\n\x0chost_rewrite\x18\x06\x20\x01(\tH\x01R\x0bhostRewrite\x12H\n\
    \x11auto_host_rewrite\x18\x07\x20\x01(\x0b2\x1a.google.protobuf.BoolValu\
    eH\x01R\x0fautoHostRewrite\x123\n\x07timeout\x18\x08\x20\x01(\x0b2\x19.g\
    oogle.protobuf.DurationR\x07timeout\x12H\n\x0cretry_policy\x18\t\x20\x01\
    (\x0b2%.envoy.api.v2.RouteAction.RetryPolicyR\x0bretryPolicy\x12a\n\x15r\
    equest_mirror_policy\x18\n\x20\x01(\x0b2-.envoy.api.v2.RouteAction.Reque\
    stMirrorPolicyR\x13requestMirrorPolicy\x129\n\x08priority\x18\x0b\x20\
    \x01(\x0e2\x1d.envoy.api.v2.RoutingPriorityR\x08priority\x12T\n\x16reque\
    st_headers_to_add\x18\x0c\x20\x03(\x0b2\x1f.envoy.api.v2.HeaderValueOpti\
    onR\x13requestHeadersToAdd\x128\n\x0brate_limits\x18\r\x20\x03(\x0b2\x17\
    .envoy.api.v2.RateLimitR\nrateLimits\x12O\n\x16include_vh_rate_limits\
    \x18\x0e\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\x13includeVhRateLi\
    mits\x12E\n\x0bhash_policy\x18\x0f\x20\x03(\x0b2$.envoy.api.v2.RouteActi\
    on.HashPolicyR\nhashPolicy\x12?\n\ruse_websocket\x18\x10\x20\x01(\x0b2\
    \x1a.google.protobuf.BoolValueR\x0cuseWebsocket\x12,\n\x04cors\x18\x11\
    \x20\x01(\x0b2\x18.envoy.api.v2.CorsPolicyR\x04cors\x1a\xaa\x01\n\x0bRet\
    ryPolicy\x12\x19\n\x08retry_on\x18\x01\x20\x01(\tR\x07retryOn\x12=\n\x0b\
    num_retries\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\nnumR\
    etries\x12A\n\x0fper_try_timeout\x18\x03\x20\x01(\x0b2\x19.google.protob\
    uf.DurationR\rperTryTimeout\x1aP\n\x13RequestMirrorPolicy\x12\x18\n\x07c\
    luster\x18\x01\x20\x01(\tR\x07cluster\x12\x1f\n\x0bruntime_key\x18\x02\
    \x20\x01(\tR\nruntimeKey\x1a\xcb\x03\n\nHashPolicy\x12E\n\x06header\x18\
    \x01\x20\x01(\x0b2+.envoy.api.v2.RouteAction.HashPolicy.HeaderH\0R\x06he\
    ader\x12E\n\x06cookie\x18\x02\x20\x01(\x0b2+.envoy.api.v2.RouteAction.Ha\
    shPolicy.CookieH\0R\x06cookie\x12p\n\x15connection_properties\x18\x03\
    \x20\x01(\x0b29.envoy.api.v2.RouteAction.HashPolicy.ConnectionProperties\
    H\0R\x14connectionProperties\x1a)\n\x06Header\x12\x1f\n\x0bheader_name\
    \x18\x01\x20\x01(\tR\nheaderName\x1aI\n\x06Cookie\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12+\n\x03ttl\x18\x02\x20\x01(\x0b2\x19.google.\
    protobuf.DurationR\x03ttl\x1a3\n\x14ConnectionProperties\x12\x1b\n\tsour\
    ce_ip\x18\x01\x20\x01(\x08R\x08sourceIpB\x12\n\x10policy_specifierB\x13\
    \n\x11cluster_specifierB\x18\n\x16host_rewrite_specifier\"Z\n\x0eRedirec\
    tAction\x12#\n\rhost_redirect\x18\x01\x20\x01(\tR\x0chostRedirect\x12#\n\
    \rpath_redirect\x18\x02\x20\x01(\tR\x0cpathRedirect\")\n\tDecorator\x12\
    \x1c\n\toperation\x18\x01\x20\x01(\tR\toperation\"\x9b\x02\n\x05Route\
    \x12.\n\x05match\x18\x01\x20\x01(\x0b2\x18.envoy.api.v2.RouteMatchR\x05m\
    atch\x121\n\x05route\x18\x02\x20\x01(\x0b2\x19.envoy.api.v2.RouteActionH\
    \0R\x05route\x12:\n\x08redirect\x18\x03\x20\x01(\x0b2\x1c.envoy.api.v2.R\
    edirectActionH\0R\x08redirect\x122\n\x08metadata\x18\x04\x20\x01(\x0b2\
    \x16.envoy.api.v2.MetadataR\x08metadata\x125\n\tdecorator\x18\x05\x20\
    \x01(\x0b2\x17.envoy.api.v2.DecoratorR\tdecoratorB\x08\n\x06action\"s\n\
    \x0eVirtualCluster\x12\x18\n\x07pattern\x18\x01\x20\x01(\tR\x07pattern\
    \x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x123\n\x06method\x18\x03\
    \x20\x01(\x0e2\x1b.envoy.api.v2.RequestMethodR\x06method\"\xd7\x08\n\tRa\
    teLimit\x122\n\x05stage\x18\x01\x20\x01(\x0b2\x1c.google.protobuf.UInt32\
    ValueR\x05stage\x12\x1f\n\x0bdisable_key\x18\x02\x20\x01(\tR\ndisableKey\
    \x128\n\x07actions\x18\x03\x20\x03(\x0b2\x1e.envoy.api.v2.RateLimit.Acti\
    onR\x07actions\x1a\xba\x07\n\x06Action\x12U\n\x0esource_cluster\x18\x01\
    \x20\x01(\x0b2,.envoy.api.v2.RateLimit.Action.SourceClusterH\0R\rsourceC\
    luster\x12d\n\x13destination_cluster\x18\x02\x20\x01(\x0b21.envoy.api.v2\
    .RateLimit.Action.DestinationClusterH\0R\x12destinationCluster\x12X\n\
    \x0frequest_headers\x18\x03\x20\x01(\x0b2-.envoy.api.v2.RateLimit.Action\
    .RequestHeadersH\0R\x0erequestHeaders\x12U\n\x0eremote_address\x18\x04\
    \x20\x01(\x0b2,.envoy.api.v2.RateLimit.Action.RemoteAddressH\0R\rremoteA\
    ddress\x12L\n\x0bgeneric_key\x18\x05\x20\x01(\x0b2).envoy.api.v2.RateLim\
    it.Action.GenericKeyH\0R\ngenericKey\x12_\n\x12header_value_match\x18\
    \x06\x20\x01(\x0b2/.envoy.api.v2.RateLimit.Action.HeaderValueMatchH\0R\
    \x10headerValueMatch\x1a\x0f\n\rSourceCluster\x1a\x14\n\x12DestinationCl\
    uster\x1aX\n\x0eRequestHeaders\x12\x1f\n\x0bheader_name\x18\x01\x20\x01(\
    \tR\nheaderName\x12%\n\x0edescriptor_key\x18\x02\x20\x01(\tR\rdescriptor\
    Key\x1a\x0f\n\rRemoteAddress\x1a7\n\nGenericKey\x12)\n\x10descriptor_val\
    ue\x18\x01\x20\x01(\tR\x0fdescriptorValue\x1a\xb3\x01\n\x10HeaderValueMa\
    tch\x12)\n\x10descriptor_value\x18\x01\x20\x01(\tR\x0fdescriptorValue\
    \x12=\n\x0cexpect_match\x18\x02\x20\x01(\x0b2\x1a.google.protobuf.BoolVa\
    lueR\x0bexpectMatch\x125\n\x07headers\x18\x03\x20\x03(\x0b2\x1b.envoy.ap\
    i.v2.HeaderMatcherR\x07headersB\x12\n\x10action_specifier\"k\n\rHeaderMa\
    tcher\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\
    \x18\x02\x20\x01(\tR\x05value\x120\n\x05regex\x18\x03\x20\x01(\x0b2\x1a.\
    google.protobuf.BoolValueR\x05regex\"\xfa\x03\n\x0bVirtualHost\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x18\n\x07domains\x18\x02\x20\
    \x03(\tR\x07domains\x12+\n\x06routes\x18\x03\x20\x03(\x0b2\x13.envoy.api\
    .v2.RouteR\x06routes\x12M\n\x0brequire_tls\x18\x04\x20\x01(\x0e2,.envoy.\
    api.v2.VirtualHost.TlsRequirementTypeR\nrequireTls\x12G\n\x10virtual_clu\
    sters\x18\x05\x20\x03(\x0b2\x1c.envoy.api.v2.VirtualClusterR\x0fvirtualC\
    lusters\x128\n\x0brate_limits\x18\x06\x20\x03(\x0b2\x17.envoy.api.v2.Rat\
    eLimitR\nrateLimits\x12T\n\x16request_headers_to_add\x18\x07\x20\x03(\
    \x0b2\x1f.envoy.api.v2.HeaderValueOptionR\x13requestHeadersToAdd\x12,\n\
    \x04cors\x18\x08\x20\x01(\x0b2\x18.envoy.api.v2.CorsPolicyR\x04cors\":\n\
    \x12TlsRequirementType\x12\x08\n\x04NONE\x10\0\x12\x11\n\rEXTERNAL_ONLY\
    \x10\x01\x12\x07\n\x03ALL\x10\x02\"\xd0\x03\n\x12RouteConfiguration\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12>\n\rvirtual_hosts\x18\x02\
    \x20\x03(\x0b2\x19.envoy.api.v2.VirtualHostR\x0cvirtualHosts\x122\n\x15i\
    nternal_only_headers\x18\x03\x20\x03(\tR\x13internalOnlyHeaders\x12V\n\
    \x17response_headers_to_add\x18\x04\x20\x03(\x0b2\x1f.envoy.api.v2.Heade\
    rValueOptionR\x14responseHeadersToAdd\x12;\n\x1aresponse_headers_to_remo\
    ve\x18\x05\x20\x03(\tR\x17responseHeadersToRemove\x12T\n\x16request_head\
    ers_to_add\x18\x06\x20\x03(\x0b2\x1f.envoy.api.v2.HeaderValueOptionR\x13\
    requestHeadersToAdd\x12G\n\x11validate_clusters\x18\x07\x20\x01(\x0b2\
    \x1a.google.protobuf.BoolValueR\x10validateClusters2\xdf\x01\n\x15RouteD\
    iscoveryService\x12U\n\x0cStreamRoutes\x12\x1e.envoy.api.v2.DiscoveryReq\
    uest\x1a\x1f.envoy.api.v2.DiscoveryResponse\"\0(\x010\x01\x12o\n\x0bFetc\
    hRoutes\x12\x1e.envoy.api.v2.DiscoveryRequest\x1a\x1f.envoy.api.v2.Disco\
    veryResponse\"\x1f\x82\xd3\xe4\x93\x02\x19\"\x14/v2/discovery:routes:\
    \x01*J\xa7\xc4\x01\n\x07\x12\x05\x04\0\x86\x04\x01\n\xe5\x01\n\x01\x0c\
    \x12\x03\x04\0\x122\xda\x01\x20This\x20is\x20heavily\x20derived\x20from\
    \n\x20https://lyft.github.io/envoy/docs/configuration/http_conn_man/rout\
    e_config/route_config.html#config-http-conn-man-route-table.\n\x20The\
    \x20v2\x20gRPC\x20API\x20differences\x20are\x20tagged\x20with\x20[V2-API\
    -DIFF].\n\n\x08\n\x01\x02\x12\x03\x06\x08\x14\n\t\n\x02\x03\0\x12\x03\
    \x08\x07\x17\n\t\n\x02\x03\x01\x12\x03\t\x07\x1c\n\t\n\x02\x03\x02\x12\
    \x03\x0b\x07%\n\t\n\x02\x03\x03\x12\x03\x0c\x07'\n\t\n\x02\x03\x04\x12\
    \x03\r\x07'\n\xd9\x02\n\x02\x06\0\x12\x04\x14\0\x20\x01\x1a\xcc\x02\x20T\
    he\x20resource_names\x20field\x20in\x20DiscoveryRequest\x20specifies\x20\
    a\x20route\x20configuration.\n\x20This\x20allows\x20an\x20Envoy\x20confi\
    guration\x20with\x20multiple\x20HTTP\x20listeners\x20(and\n\x20associate\
    d\x20HTTP\x20connection\x20manager\x20filters)\x20to\x20use\x20different\
    \x20route\n\x20configurations.\x20Each\x20listener\x20will\x20bind\x20it\
    s\x20HTTP\x20connection\x20manager\x20filter\x20to\n\x20a\x20route\x20ta\
    ble\x20via\x20this\x20identifier.\n\n\n\n\x03\x06\0\x01\x12\x03\x14\x08\
    \x1d\n\x0c\n\x04\x06\0\x02\0\x12\x04\x15\x02\x17\x03\n\x0c\n\x05\x06\0\
    \x02\0\x01\x12\x03\x15\x06\x12\n\x0c\n\x05\x06\0\x02\0\x05\x12\x03\x15\
    \x13\x19\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x15\x1a*\n\x0c\n\x05\x06\0\
    \x02\0\x06\x12\x03\x16\x0f\x15\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x16\
    \x16'\n\x0c\n\x04\x06\0\x02\x01\x12\x04\x19\x02\x1f\x03\n\x0c\n\x05\x06\
    \0\x02\x01\x01\x12\x03\x19\x06\x11\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\
    \x19\x12\"\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x1a\x0f\x20\n\r\n\x05\
    \x06\0\x02\x01\x04\x12\x04\x1b\x04\x1e\x06\n\x10\n\x08\x06\0\x02\x01\x04\
    \xe7\x07\0\x12\x04\x1b\x04\x1e\x06\n\x10\n\t\x06\0\x02\x01\x04\xe7\x07\0\
    \x02\x12\x03\x1b\x0b\x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x12\
    \x03\x1b\x0b\x1c\n\x12\n\x0b\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\
    \x03\x1b\x0c\x1b\n\x11\n\t\x06\0\x02\x01\x04\xe7\x07\0\x08\x12\x04\x1b\
    \x1f\x1e\x05\n\xe2\x02\n\x02\x04\0\x12\x04'\0@\x01\x1a\xd5\x02\x20Compar\
    ed\x20to\x20the\x20cluster\x20field\x20that\x20specifies\x20a\x20single\
    \x20upstream\x20cluster\x20as\x20the\n\x20target\x20of\x20a\x20request,\
    \x20the\x20weighted_clusters\x20option\x20allows\x20for\x20specification\
    \x20of\n\x20multiple\x20upstream\x20clusters\x20along\x20with\x20weights\
    \x20that\x20indicate\x20the\x20percentage\x20of\n\x20traffic\x20to\x20be\
    \x20forwarded\x20to\x20each\x20cluster.\x20The\x20router\x20selects\x20a\
    n\x20upstream\n\x20cluster\x20based\x20on\x20the\x20weights.\n\n\n\n\x03\
    \x04\0\x01\x12\x03'\x08\x17\n\x0c\n\x04\x04\0\x03\0\x12\x04(\x024\x03\n\
    \x0c\n\x05\x04\0\x03\0\x01\x12\x03(\n\x17\nl\n\x06\x04\0\x03\0\x02\0\x12\
    \x03+\x04\x14\x1a]\x20Name\x20of\x20the\x20upstream\x20cluster.\x20The\
    \x20cluster\x20must\x20exist\x20in\x20the\x20cluster\n\x20manager\x20con\
    figuration.\n\n\x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04+\x04(\x19\n\x0e\
    \n\x07\x04\0\x03\0\x02\0\x05\x12\x03+\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\
    \0\x01\x12\x03+\x0b\x0f\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03+\x12\
    \x13\n\xde\x01\n\x06\x04\0\x03\0\x02\x01\x12\x03/\x04+\x1a\xce\x01\x20An\
    \x20integer\x20between\x200-100.\x20When\x20a\x20request\x20matches\x20t\
    he\x20route,\x20the\x20choice\x20of\n\x20an\x20upstream\x20cluster\x20is\
    \x20determined\x20by\x20its\x20weight.\x20The\x20sum\x20of\x20weights\n\
    \x20across\x20all\x20entries\x20in\x20the\x20clusters\x20array\x20must\
    \x20add\x20up\x20to\x20100.\n\n\x0f\n\x07\x04\0\x03\0\x02\x01\x04\x12\
    \x04/\x04+\x14\n\x0e\n\x07\x04\0\x03\0\x02\x01\x06\x12\x03/\x04\x1f\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03/\x20&\n\x0e\n\x07\x04\0\x03\0\
    \x02\x01\x03\x12\x03/)*\n\xde\x01\n\x06\x04\0\x03\0\x02\x02\x12\x033\x04\
    \x20\x1a\xce\x01\x20Optional\x20endpoint\x20metadata\x20match\x20criteri\
    a.\x20Only\x20endpoints\x20in\x20the\x20upstream\n\x20cluster\x20with\
    \x20metadata\x20matching\x20that\x20set\x20in\x20metadata_match\x20will\
    \x20be\n\x20considered.\x20The\x20filter\x20name\x20should\x20be\x20spec\
    ified\x20as\x20\"envoy.lb\".\n\n\x0f\n\x07\x04\0\x03\0\x02\x02\x04\x12\
    \x043\x04/+\n\x0e\n\x07\x04\0\x03\0\x02\x02\x06\x12\x033\x04\x0c\n\x0e\n\
    \x07\x04\0\x03\0\x02\x02\x01\x12\x033\r\x1b\n\x0e\n\x07\x04\0\x03\0\x02\
    \x02\x03\x12\x033\x1e\x1f\nQ\n\x04\x04\0\x02\0\x12\x036\x02&\x1aD\x20Spe\
    cifies\x20one\x20or\x20more\x20upstream\x20clusters\x20associated\x20wit\
    h\x20the\x20route.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x036\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x036\x0b\x18\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x036\x19!\n\x0c\n\x05\x04\0\x02\0\x03\x12\x036$%\n\xcb\x04\n\x04\x04\0\
    \x02\x01\x12\x03?\x02\x20\x1a\xbd\x04\x20Specifies\x20the\x20runtime\x20\
    key\x20prefix\x20that\x20should\x20be\x20used\x20to\x20construct\x20the\
    \n\x20runtime\x20keys\x20associated\x20with\x20each\x20cluster.\x20When\
    \x20the\x20runtime_key_prefix\x20is\n\x20specified,\x20the\x20router\x20\
    will\x20look\x20for\x20weights\x20associated\x20with\x20each\x20upstream\
    \n\x20cluster\x20under\x20the\x20key\x20runtime_key_prefix\x20+\x20\".\"\
    \x20+\x20cluster[i].name\x20where\n\x20cluster[i]\x20denotes\x20an\x20en\
    try\x20in\x20the\x20clusters\x20array\x20field.\x20If\x20the\x20runtime\
    \n\x20key\x20for\x20the\x20cluster\x20does\x20not\x20exist,\x20the\x20va\
    lue\x20specified\x20in\x20the\n\x20configuration\x20file\x20will\x20be\
    \x20used\x20as\x20the\x20default\x20weight.\x20See\x20the\x20runtime\n\
    \x20documentation\x20for\x20how\x20key\x20names\x20map\x20to\x20the\x20u\
    nderlying\x20implementation.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04?\x026\
    &\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03?\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03?\t\x1b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03?\x1e\x1f\n\
    \n\n\x02\x04\x01\x12\x04B\0f\x01\n\n\n\x03\x04\x01\x01\x12\x03B\x08\x12\
    \n1\n\x04\x04\x01\x08\0\x12\x04D\x02P\x03\x1a#\x20A\x20path\x20specifier\
    \x20must\x20be\x20present.\n\n\x0c\n\x05\x04\x01\x08\0\x01\x12\x03D\x08\
    \x16\n~\n\x04\x04\x01\x02\0\x12\x03G\x04\x16\x1aq\x20If\x20specified,\
    \x20the\x20route\x20is\x20a\x20prefix\x20rule\x20meaning\x20that\x20the\
    \x20prefix\x20must\n\x20match\x20the\x20beginning\x20of\x20the\x20:path\
    \x20header.\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03G\x04\n\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03G\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03G\x14\x15\n\x9a\x01\n\x04\x04\x01\x02\x01\x12\x03K\x04\x14\x1a\x8c\
    \x01\x20If\x20specified,\x20the\x20route\x20is\x20an\x20exact\x20path\
    \x20rule\x20meaning\x20that\x20the\x20path\x20must\n\x20exactly\x20match\
    \x20the\x20:path\x20header\x20once\x20the\x20query\x20string\x20is\x20re\
    moved.\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03K\x04\n\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03K\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\
    \x03K\x12\x13\n\x89\x01\n\x04\x04\x01\x02\x02\x12\x03O\x04\x15\x1a|\x20I\
    f\x20specified,\x20the\x20route\x20is\x20a\x20regular\x20expression\x20m\
    atch\x20on\x20the\x20:path\x20header\n\x20once\x20the\x20query\x20string\
    \x20is\x20removed\x20[V2-API-DIFF].\n\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03O\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03O\x0b\x10\n\x0c\n\
    \x05\x04\x01\x02\x02\x03\x12\x03O\x13\x14\nd\n\x04\x04\x01\x02\x03\x12\
    \x03S\x02/\x1aW\x20Indicates\x20that\x20prefix/path\x20matching\x20shoul\
    d\x20be\x20case\x20insensitive.\x20The\x20default\n\x20is\x20true.\n\n\r\
    \n\x05\x04\x01\x02\x03\x04\x12\x04S\x02P\x03\n\x0c\n\x05\x04\x01\x02\x03\
    \x06\x12\x03S\x02\x1b\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03S\x1c*\n\
    \x0c\n\x05\x04\x01\x02\x03\x03\x12\x03S-.\n\xc1\x04\n\x04\x04\x01\x02\
    \x04\x12\x03^\x02\x1c\x1a\xb3\x04\x20Indicates\x20that\x20the\x20route\
    \x20should\x20additionally\x20match\x20on\x20a\x20runtime\x20key.\x20An\
    \n\x20integer\x20between\x200-100.\x20Every\x20time\x20the\x20route\x20i\
    s\x20considered\x20for\x20a\x20match,\x20a\n\x20random\x20number\x20betw\
    een\x200-99\x20is\x20selected.\x20If\x20the\x20number\x20is\x20<=\x20the\
    \x20value\x20found\n\x20in\x20the\x20key\x20(checked\x20first)\x20or,\
    \x20if\x20the\x20key\x20is\x20not\x20present,\x20the\x20default\n\x20val\
    ue,\x20the\x20route\x20is\x20a\x20match\x20(assuming\x20everything\x20al\
    so\x20about\x20the\x20route\n\x20matches).\n\x20A\x20runtime\x20route\
    \x20configuration\x20can\x20be\x20used\x20to\x20roll\x20out\x20route\x20\
    changes\x20in\x20a\n\x20gradual\x20manner\x20without\x20full\x20code/con\
    fig\x20deploys.\x20Refer\x20to\x20traffic\x20shifting\n\x20docs\x20for\
    \x20additional\x20documentation.\n\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\
    ^\x02S/\n\x0c\n\x05\x04\x01\x02\x04\x06\x12\x03^\x02\x0f\n\x0c\n\x05\x04\
    \x01\x02\x04\x01\x12\x03^\x10\x17\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\
    \x03^\x1a\x1b\n\xd7\x02\n\x04\x04\x01\x02\x05\x12\x03e\x02%\x1a\xc9\x02\
    \x20Specifies\x20a\x20set\x20of\x20headers\x20that\x20the\x20route\x20sh\
    ould\x20match\x20on.\x20The\x20router\x20will\n\x20check\x20the\x20reque\
    st\xe2\x80\x99s\x20headers\x20against\x20all\x20the\x20specified\x20head\
    ers\x20in\x20the\x20route\n\x20config.\x20A\x20match\x20will\x20happen\
    \x20if\x20all\x20the\x20headers\x20in\x20the\x20route\x20are\x20present\
    \x20in\n\x20the\x20request\x20with\x20the\x20same\x20values\x20(or\x20ba\
    sed\x20on\x20presence\x20if\x20the\x20value\x20field\n\x20is\x20not\x20i\
    n\x20the\x20config).\n\n\x0c\n\x05\x04\x01\x02\x05\x04\x12\x03e\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x05\x06\x12\x03e\x0b\x18\n\x0c\n\x05\x04\x01\x02\
    \x05\x01\x12\x03e\x19\x20\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03e#$\n\n\
    \n\x02\x04\x02\x12\x04h\0w\x01\n\n\n\x03\x04\x02\x01\x12\x03h\x08\x12\nN\
    \n\x04\x04\x02\x02\0\x12\x03j\x02#\x1aA\x20Specifies\x20the\x20origins\
    \x20that\x20will\x20be\x20allowed\x20to\x20do\x20CORS\x20requests.\n\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03j\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03j\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03j\x12\x1e\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03j!\"\nQ\n\x04\x04\x02\x02\x01\x12\
    \x03l\x02\x1b\x1aD\x20Specifies\x20the\x20content\x20for\x20the\x20acces\
    s-control-allow-methods\x20header.\n\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04l\x02j#\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03l\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x01\x01\x12\x03l\t\x16\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03l\x19\x1a\nQ\n\x04\x04\x02\x02\x02\x12\x03n\x02\x1b\x1aD\x20Specifie\
    s\x20the\x20content\x20for\x20the\x20access-control-allow-headers\x20hea\
    der.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04n\x02l\x1b\n\x0c\n\x05\x04\
    \x02\x02\x02\x05\x12\x03n\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\
    \x03n\t\x16\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03n\x19\x1a\nR\n\x04\
    \x04\x02\x02\x03\x12\x03p\x02\x1c\x1aE\x20Specifies\x20the\x20content\
    \x20for\x20the\x20access-control-expose-headers\x20header.\n\n\r\n\x05\
    \x04\x02\x02\x03\x04\x12\x04p\x02n\x1b\n\x0c\n\x05\x04\x02\x02\x03\x05\
    \x12\x03p\x02\x08\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03p\t\x17\n\x0c\n\
    \x05\x04\x02\x02\x03\x03\x12\x03p\x1a\x1b\nK\n\x04\x04\x02\x02\x04\x12\
    \x03r\x02\x15\x1a>\x20Specifies\x20the\x20content\x20for\x20the\x20acces\
    s-control-max-age\x20header.\n\n\r\n\x05\x04\x02\x02\x04\x04\x12\x04r\
    \x02p\x1c\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\x03r\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x04\x01\x12\x03r\t\x10\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\
    \x03r\x13\x14\nA\n\x04\x04\x02\x02\x05\x12\x03t\x022\x1a4\x20Specifies\
    \x20whether\x20the\x20resource\x20allows\x20credentials.\n\n\r\n\x05\x04\
    \x02\x02\x05\x04\x12\x04t\x02r\x15\n\x0c\n\x05\x04\x02\x02\x05\x06\x12\
    \x03t\x02\x1b\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03t\x1c-\n\x0c\n\x05\
    \x04\x02\x02\x05\x03\x12\x03t01\nW\n\x04\x04\x02\x02\x06\x12\x03v\x02(\
    \x1aJ\x20Specifies\x20if\x20CORS\x20is\x20enabled.\x20Defaults\x20to\x20\
    true.\x20Only\x20effective\x20on\x20route.\n\n\r\n\x05\x04\x02\x02\x06\
    \x04\x12\x04v\x02t2\n\x0c\n\x05\x04\x02\x02\x06\x06\x12\x03v\x02\x1b\n\
    \x0c\n\x05\x04\x02\x02\x06\x01\x12\x03v\x1c#\n\x0c\n\x05\x04\x02\x02\x06\
    \x03\x12\x03v&'\n\x0b\n\x02\x04\x03\x12\x05y\0\x92\x02\x01\n\n\n\x03\x04\
    \x03\x01\x12\x03y\x08\x13\n\r\n\x04\x04\x03\x08\0\x12\x05z\x02\x88\x01\
    \x03\n\x0c\n\x05\x04\x03\x08\0\x01\x12\x03z\x08\x19\nX\n\x04\x04\x03\x02\
    \0\x12\x03}\x04\x17\x1aK\x20Indicates\x20the\x20upstream\x20cluster\x20t\
    o\x20which\x20the\x20request\x20should\x20be\x20routed\n\x20to.\n\n\x0c\
    \n\x05\x04\x03\x02\0\x05\x12\x03}\x04\n\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x03}\x0b\x12\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03}\x15\x16\n\x81\
    \x02\n\x04\x04\x03\x02\x01\x12\x04\x82\x01\x04\x1e\x1a\xf2\x01\x20Envoy\
    \x20will\x20determine\x20the\x20cluster\x20to\x20route\x20to\x20by\x20re\
    ading\x20the\x20value\x20of\x20the\n\x20HTTP\x20header\x20named\x20by\
    \x20cluster_header\x20from\x20the\x20request\x20headers.\x20If\x20the\n\
    \x20header\x20is\x20not\x20found\x20or\x20the\x20referenced\x20cluster\
    \x20does\x20not\x20exist,\x20Envoy\x20will\n\x20return\x20a\x20404\x20re\
    sponse.\n\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\x82\x01\x04\n\n\r\n\x05\
    \x04\x03\x02\x01\x01\x12\x04\x82\x01\x0b\x19\n\r\n\x05\x04\x03\x02\x01\
    \x03\x12\x04\x82\x01\x1c\x1d\n\xe7\x01\n\x04\x04\x03\x02\x02\x12\x04\x87\
    \x01\x04*\x1a\xd8\x01\x20Multiple\x20upstream\x20clusters\x20can\x20be\
    \x20specified\x20for\x20a\x20given\x20route.\x20The\n\x20request\x20is\
    \x20routed\x20to\x20one\x20of\x20the\x20upstream\x20clusters\x20based\
    \x20on\x20weights\n\x20assigned\x20to\x20each\x20cluster.\x20See\x20traf\
    fic\x20splitting\x20for\x20additional\n\x20documentation.\n\n\r\n\x05\
    \x04\x03\x02\x02\x06\x12\x04\x87\x01\x04\x13\n\r\n\x05\x04\x03\x02\x02\
    \x01\x12\x04\x87\x01\x14%\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\x87\x01(\
    )\n\xdd\x01\n\x04\x04\x03\x02\x03\x12\x04\x8d\x01\x02\x1e\x1a\xce\x01\
    \x20Optional\x20endpoint\x20metadata\x20match\x20criteria.\x20Only\x20en\
    dpoints\x20in\x20the\x20upstream\n\x20cluster\x20with\x20metadata\x20mat\
    ching\x20that\x20set\x20in\x20metadata_match\x20will\x20be\n\x20consider\
    ed.\x20The\x20filter\x20name\x20should\x20be\x20specified\x20as\x20\"env\
    oy.lb\".\n\n\x0f\n\x05\x04\x03\x02\x03\x04\x12\x06\x8d\x01\x02\x88\x01\
    \x03\n\r\n\x05\x04\x03\x02\x03\x06\x12\x04\x8d\x01\x02\n\n\r\n\x05\x04\
    \x03\x02\x03\x01\x12\x04\x8d\x01\x0b\x19\n\r\n\x05\x04\x03\x02\x03\x03\
    \x12\x04\x8d\x01\x1c\x1d\n\xe8\x01\n\x04\x04\x03\x02\x04\x12\x04\x92\x01\
    \x02\x1c\x1a\xd9\x01\x20Indicates\x20that\x20during\x20forwarding,\x20th\
    e\x20matched\x20prefix\x20(or\x20path)\x20should\x20be\n\x20swapped\x20w\
    ith\x20this\x20value.\x20This\x20option\x20allows\x20application\x20URLs\
    \x20to\x20be\x20rooted\n\x20at\x20a\x20different\x20path\x20from\x20thos\
    e\x20exposed\x20at\x20the\x20reverse\x20proxy\x20layer.\n\n\x0f\n\x05\
    \x04\x03\x02\x04\x04\x12\x06\x92\x01\x02\x8d\x01\x1e\n\r\n\x05\x04\x03\
    \x02\x04\x05\x12\x04\x92\x01\x02\x08\n\r\n\x05\x04\x03\x02\x04\x01\x12\
    \x04\x92\x01\t\x17\n\r\n\x05\x04\x03\x02\x04\x03\x12\x04\x92\x01\x1a\x1b\
    \n\x0e\n\x04\x04\x03\x08\x01\x12\x06\x93\x01\x02\x9d\x01\x03\n\r\n\x05\
    \x04\x03\x08\x01\x01\x12\x04\x93\x01\x08\x1e\nc\n\x04\x04\x03\x02\x05\
    \x12\x04\x96\x01\x04\x1c\x1aU\x20Indicates\x20that\x20during\x20forwardi\
    ng,\x20the\x20host\x20header\x20will\x20be\x20swapped\x20with\n\x20this\
    \x20value.\n\n\r\n\x05\x04\x03\x02\x05\x05\x12\x04\x96\x01\x04\n\n\r\n\
    \x05\x04\x03\x02\x05\x01\x12\x04\x96\x01\x0b\x17\n\r\n\x05\x04\x03\x02\
    \x05\x03\x12\x04\x96\x01\x1a\x1b\n\xc7\x02\n\x04\x04\x03\x02\x06\x12\x04\
    \x9c\x01\x044\x1a\xb8\x02\x20Indicates\x20that\x20during\x20forwarding,\
    \x20the\x20host\x20header\x20will\x20be\x20swapped\x20with\n\x20the\x20h\
    ostname\x20of\x20the\x20upstream\x20host\x20chosen\x20by\x20the\x20clust\
    er\x20manager.\x20This\n\x20option\x20is\x20applicable\x20only\x20when\
    \x20the\x20destination\x20cluster\x20for\x20a\x20route\x20is\x20of\n\x20\
    type\x20strict_dns\x20or\x20logical_dns.\x20Setting\x20this\x20to\x20tru\
    e\x20with\x20other\x20cluster\n\x20types\x20has\x20no\x20effect.\n\n\r\n\
    \x05\x04\x03\x02\x06\x06\x12\x04\x9c\x01\x04\x1d\n\r\n\x05\x04\x03\x02\
    \x06\x01\x12\x04\x9c\x01\x1e/\n\r\n\x05\x04\x03\x02\x06\x03\x12\x04\x9c\
    \x0123\n\xf4\x01\n\x04\x04\x03\x02\x07\x12\x04\xa3\x01\x02'\x1a\xe5\x01\
    \x20Specifies\x20the\x20timeout\x20for\x20the\x20route.\x20If\x20not\x20\
    specified,\x20the\x20default\x20is\x2015s.\n\x20Note\x20that\x20this\x20\
    timeout\x20includes\x20all\x20retries.\x20See\x20also\n\x20x-envoy-upstr\
    eam-rq-timeout-ms,\x20x-envoy-upstream-rq-per-try-timeout-ms,\x20and\n\
    \x20the\x20retry\x20overview.\n\n\x0f\n\x05\x04\x03\x02\x07\x04\x12\x06\
    \xa3\x01\x02\x9d\x01\x03\n\r\n\x05\x04\x03\x02\x07\x06\x12\x04\xa3\x01\
    \x02\x1a\n\r\n\x05\x04\x03\x02\x07\x01\x12\x04\xa3\x01\x1b\"\n\r\n\x05\
    \x04\x03\x02\x07\x03\x12\x04\xa3\x01%&\n\x0e\n\x04\x04\x03\x03\0\x12\x06\
    \xa5\x01\x02\xb1\x01\x03\n\r\n\x05\x04\x03\x03\0\x01\x12\x04\xa5\x01\n\
    \x15\n\x89\x01\n\x06\x04\x03\x03\0\x02\0\x12\x04\xa8\x01\x04\x18\x1ay\
    \x20Specifies\x20the\x20conditions\x20under\x20which\x20retry\x20takes\
    \x20place.\x20These\x20are\x20the\n\x20same\x20conditions\x20documented\
    \x20for\x20x-envoy-retry-on.\n\n\x11\n\x07\x04\x03\x03\0\x02\0\x04\x12\
    \x06\xa8\x01\x04\xa5\x01\x17\n\x0f\n\x07\x04\x03\x03\0\x02\0\x05\x12\x04\
    \xa8\x01\x04\n\n\x0f\n\x07\x04\x03\x03\0\x02\0\x01\x12\x04\xa8\x01\x0b\
    \x13\n\x0f\n\x07\x04\x03\x03\0\x02\0\x03\x12\x04\xa8\x01\x16\x17\n\xad\
    \x01\n\x06\x04\x03\x03\0\x02\x01\x12\x04\xac\x01\x040\x1a\x9c\x01\x20Spe\
    cifies\x20the\x20allowed\x20number\x20of\x20retries.\x20This\x20paramete\
    r\x20is\x20optional\x20and\n\x20defaults\x20to\x201.\x20These\x20are\x20\
    the\x20same\x20conditions\x20documented\x20for\n\x20x-envoy-max-retries.\
    \n\n\x11\n\x07\x04\x03\x03\0\x02\x01\x04\x12\x06\xac\x01\x04\xa8\x01\x18\
    \n\x0f\n\x07\x04\x03\x03\0\x02\x01\x06\x12\x04\xac\x01\x04\x1f\n\x0f\n\
    \x07\x04\x03\x03\0\x02\x01\x01\x12\x04\xac\x01\x20+\n\x0f\n\x07\x04\x03\
    \x03\0\x02\x01\x03\x12\x04\xac\x01./\n\xb1\x01\n\x06\x04\x03\x03\0\x02\
    \x02\x12\x04\xb0\x01\x041\x1a\xa0\x01\x20Specifies\x20a\x20non-zero\x20t\
    imeout\x20per\x20retry\x20attempt.\x20This\x20parameter\x20is\n\x20optio\
    nal.\x20The\x20same\x20conditions\x20documented\x20for\n\x20x-envoy-upst\
    ream-rq-per-try-timeout-ms\x20apply.\n\n\x11\n\x07\x04\x03\x03\0\x02\x02\
    \x04\x12\x06\xb0\x01\x04\xac\x010\n\x0f\n\x07\x04\x03\x03\0\x02\x02\x06\
    \x12\x04\xb0\x01\x04\x1c\n\x0f\n\x07\x04\x03\x03\0\x02\x02\x01\x12\x04\
    \xb0\x01\x1d,\n\x0f\n\x07\x04\x03\x03\0\x02\x02\x03\x12\x04\xb0\x01/0\n<\
    \n\x04\x04\x03\x02\x08\x12\x04\xb3\x01\x02\x1f\x1a.\x20Indicates\x20that\
    \x20the\x20route\x20has\x20a\x20retry\x20policy.\n\n\x0f\n\x05\x04\x03\
    \x02\x08\x04\x12\x06\xb3\x01\x02\xb1\x01\x03\n\r\n\x05\x04\x03\x02\x08\
    \x06\x12\x04\xb3\x01\x02\r\n\r\n\x05\x04\x03\x02\x08\x01\x12\x04\xb3\x01\
    \x0e\x1a\n\r\n\x05\x04\x03\x02\x08\x03\x12\x04\xb3\x01\x1d\x1e\nJ\n\x04\
    \x04\x03\x03\x01\x12\x06\xb6\x01\x02\xc1\x01\x03\x1a:\x20Indicates\x20th\
    at\x20the\x20route\x20has\x20a\x20request\x20mirroring\x20policy.\n\n\r\
    \n\x05\x04\x03\x03\x01\x01\x12\x04\xb6\x01\n\x1d\n\x88\x01\n\x06\x04\x03\
    \x03\x01\x02\0\x12\x04\xb9\x01\x04\x17\x1ax\x20Specifies\x20the\x20clust\
    er\x20that\x20requests\x20will\x20be\x20mirrored\x20to.\x20The\x20cluste\
    r\x20must\n\x20exist\x20in\x20the\x20cluster\x20manager\x20configuration\
    .\n\n\x11\n\x07\x04\x03\x03\x01\x02\0\x04\x12\x06\xb9\x01\x04\xb6\x01\
    \x1f\n\x0f\n\x07\x04\x03\x03\x01\x02\0\x05\x12\x04\xb9\x01\x04\n\n\x0f\n\
    \x07\x04\x03\x03\x01\x02\0\x01\x12\x04\xb9\x01\x0b\x12\n\x0f\n\x07\x04\
    \x03\x03\x01\x02\0\x03\x12\x04\xb9\x01\x15\x16\n\x9d\x03\n\x06\x04\x03\
    \x03\x01\x02\x01\x12\x04\xc0\x01\x04\x1b\x1a\x8c\x03\x20If\x20not\x20spe\
    cified,\x20all\x20requests\x20to\x20the\x20target\x20cluster\x20will\x20\
    be\x20mirrored.\x20If\n\x20specified,\x20Envoy\x20will\x20lookup\x20the\
    \x20runtime\x20key\x20to\x20get\x20the\x20%\x20of\x20requests\x20to\n\
    \x20mirror.\x20Valid\x20values\x20are\x20from\x200\x20to\x2010000,\x20al\
    lowing\x20for\x20increments\x20of\n\x200.01%\x20of\x20requests\x20to\x20\
    be\x20mirrored.\x20If\x20the\x20runtime\x20key\x20is\x20specified\x20in\
    \x20the\n\x20configuration\x20but\x20not\x20present\x20in\x20runtime,\
    \x200\x20is\x20the\x20default\x20and\x20thus\x200%\x20of\n\x20requests\
    \x20will\x20be\x20mirrored.\n\n\x11\n\x07\x04\x03\x03\x01\x02\x01\x04\
    \x12\x06\xc0\x01\x04\xb9\x01\x17\n\x0f\n\x07\x04\x03\x03\x01\x02\x01\x05\
    \x12\x04\xc0\x01\x04\n\n\x0f\n\x07\x04\x03\x03\x01\x02\x01\x01\x12\x04\
    \xc0\x01\x0b\x16\n\x0f\n\x07\x04\x03\x03\x01\x02\x01\x03\x12\x04\xc0\x01\
    \x19\x1a\n\x0c\n\x04\x04\x03\x02\t\x12\x04\xc2\x01\x021\n\x0f\n\x05\x04\
    \x03\x02\t\x04\x12\x06\xc2\x01\x02\xc1\x01\x03\n\r\n\x05\x04\x03\x02\t\
    \x06\x12\x04\xc2\x01\x02\x15\n\r\n\x05\x04\x03\x02\t\x01\x12\x04\xc2\x01\
    \x16+\n\r\n\x05\x04\x03\x02\t\x03\x12\x04\xc2\x01.0\n\x0c\n\x04\x04\x03\
    \x02\n\x12\x04\xc4\x01\x02\x20\n\x0f\n\x05\x04\x03\x02\n\x04\x12\x06\xc4\
    \x01\x02\xc2\x011\n\r\n\x05\x04\x03\x02\n\x06\x12\x04\xc4\x01\x02\x11\n\
    \r\n\x05\x04\x03\x02\n\x01\x12\x04\xc4\x01\x12\x1a\n\r\n\x05\x04\x03\x02\
    \n\x03\x12\x04\xc4\x01\x1d\x1f\n_\n\x04\x04\x03\x02\x0b\x12\x04\xc8\x01\
    \x029\x1aQ\x20Specifies\x20a\x20set\x20of\x20headers\x20that\x20will\x20\
    be\x20added\x20to\x20requests\x20matching\x20this\n\x20route.\n\n\r\n\
    \x05\x04\x03\x02\x0b\x04\x12\x04\xc8\x01\x02\n\n\r\n\x05\x04\x03\x02\x0b\
    \x06\x12\x04\xc8\x01\x0b\x1c\n\r\n\x05\x04\x03\x02\x0b\x01\x12\x04\xc8\
    \x01\x1d3\n\r\n\x05\x04\x03\x02\x0b\x03\x12\x04\xc8\x0168\na\n\x04\x04\
    \x03\x02\x0c\x12\x04\xcc\x01\x02&\x1aS\x20Specifies\x20a\x20set\x20of\
    \x20rate\x20limit\x20configurations\x20that\x20could\x20be\x20applied\
    \x20to\x20the\n\x20route.\n\n\r\n\x05\x04\x03\x02\x0c\x04\x12\x04\xcc\
    \x01\x02\n\n\r\n\x05\x04\x03\x02\x0c\x06\x12\x04\xcc\x01\x0b\x14\n\r\n\
    \x05\x04\x03\x02\x0c\x01\x12\x04\xcc\x01\x15\x20\n\r\n\x05\x04\x03\x02\
    \x0c\x03\x12\x04\xcc\x01#%\n\xd0\x01\n\x04\x04\x03\x02\r\x12\x04\xd1\x01\
    \x028\x1a\xc1\x01\x20Specifies\x20if\x20the\x20rate\x20limit\x20filter\
    \x20should\x20include\x20the\x20virtual\x20host\x20rate\n\x20limits.\x20\
    By\x20default,\x20if\x20the\x20route\x20configured\x20rate\x20limits,\
    \x20the\x20virtual\x20host\n\x20rate_limits\x20are\x20not\x20applied\x20\
    to\x20the\x20request.\n\n\x0f\n\x05\x04\x03\x02\r\x04\x12\x06\xd1\x01\
    \x02\xcc\x01&\n\r\n\x05\x04\x03\x02\r\x06\x12\x04\xd1\x01\x02\x1b\n\r\n\
    \x05\x04\x03\x02\r\x01\x12\x04\xd1\x01\x1c2\n\r\n\x05\x04\x03\x02\r\x03\
    \x12\x04\xd1\x0157\n\x0e\n\x04\x04\x03\x03\x02\x12\x06\xd3\x01\x02\xfb\
    \x01\x03\n\r\n\x05\x04\x03\x03\x02\x01\x12\x04\xd3\x01\n\x14\n|\n\x06\
    \x04\x03\x03\x02\x03\0\x12\x06\xd6\x01\x04\xdc\x01\x05\x1aj\x20[V2-API-D\
    IFF]\x20We\x20expect\x20additional\x20hash\x20policies\x20in\x20the\x20f\
    uture,\x20e.g.\n\x20cookie\x20based,\x20originating\x20IP,\x20etc.\n\n\
    \x0f\n\x07\x04\x03\x03\x02\x03\0\x01\x12\x04\xd6\x01\x0c\x12\n\xf3\x01\n\
    \x08\x04\x03\x03\x02\x03\0\x02\0\x12\x04\xdb\x01\x06\x1d\x1a\xe0\x01\x20\
    The\x20name\x20of\x20the\x20request\x20header\x20that\x20will\x20be\x20u\
    sed\x20to\x20obtain\x20the\x20hash\n\x20key.\x20If\x20the\x20request\x20\
    header\x20is\x20not\x20present,\x20the\x20load\x20balancer\x20will\x20us\
    e\x20a\n\x20random\x20number\x20as\x20the\x20hash,\x20effectively\x20mak\
    ing\x20the\x20load\x20balancing\x20policy\n\x20random.\n\n\x13\n\t\x04\
    \x03\x03\x02\x03\0\x02\0\x04\x12\x06\xdb\x01\x06\xd6\x01\x14\n\x11\n\t\
    \x04\x03\x03\x02\x03\0\x02\0\x05\x12\x04\xdb\x01\x06\x0c\n\x11\n\t\x04\
    \x03\x03\x02\x03\0\x02\0\x01\x12\x04\xdb\x01\r\x18\n\x11\n\t\x04\x03\x03\
    \x02\x03\0\x02\0\x03\x12\x04\xdb\x01\x1b\x1c\n\x97\x04\n\x06\x04\x03\x03\
    \x02\x03\x01\x12\x06\xe8\x01\x04\xf1\x01\x05\x1a\x84\x04\x20Envoy\x20sup\
    ports\x20two\x20types\x20of\x20cookie\x20affinity:\n\n\x201.\x20Passive.\
    \x20Envoy\x20takes\x20a\x20cookie\x20that's\x20present\x20in\x20the\x20c\
    ookies\x20header\x20and\n\x20\x20\x20\x20hashes\x20on\x20its\x20value.\n\
    \n\x202.\x20Generated.\x20Envoy\x20generates\x20and\x20sets\x20a\x20cook\
    ie\x20with\x20an\x20expiration\x20(TTL)\n\x20\x20\x20\x20on\x20the\x20fi\
    rst\x20request\x20from\x20the\x20client\x20in\x20its\x20response\x20to\
    \x20the\x20client,\n\x20\x20\x20\x20based\x20on\x20the\x20endpoint\x20th\
    e\x20request\x20gets\x20sent\x20to.\x20The\x20client\x20then\n\x20\x20\
    \x20\x20presents\x20this\x20on\x20the\x20next\x20and\x20all\x20subsequen\
    t\x20requests,\x20the\x20hash\x20of\n\x20\x20\x20\x20this\x20is\x20suffi\
    cient\x20to\x20ensure\x20these\x20requests\x20get\x20sent\x20to\x20the\
    \x20same\n\x20\x20\x20\x20endpoint.\n\n\x0f\n\x07\x04\x03\x03\x02\x03\
    \x01\x01\x12\x04\xe8\x01\x0c\x12\n\xfc\x01\n\x08\x04\x03\x03\x02\x03\x01\
    \x02\0\x12\x04\xed\x01\x06\x16\x1a\xe9\x01\x20The\x20name\x20of\x20the\
    \x20cookie\x20that\x20will\x20be\x20used\x20to\x20obtain\x20the\x20hash\
    \x20key.\x20If\x20the\n\x20cookie\x20is\x20not\x20present\x20and\x20ttl\
    \x20below\x20is\x20not\x20set,\x20the\x20load\x20balancer\x20will\n\x20u\
    se\x20a\x20random\x20number\x20as\x20the\x20hash,\x20effectively\x20maki\
    ng\x20the\x20load\x20balancing\n\x20policy\x20random.\n\n\x13\n\t\x04\
    \x03\x03\x02\x03\x01\x02\0\x04\x12\x06\xed\x01\x06\xe8\x01\x14\n\x11\n\t\
    \x04\x03\x03\x02\x03\x01\x02\0\x05\x12\x04\xed\x01\x06\x0c\n\x11\n\t\x04\
    \x03\x03\x02\x03\x01\x02\0\x01\x12\x04\xed\x01\r\x11\n\x11\n\t\x04\x03\
    \x03\x02\x03\x01\x02\0\x03\x12\x04\xed\x01\x14\x15\nh\n\x08\x04\x03\x03\
    \x02\x03\x01\x02\x01\x12\x04\xf0\x01\x06'\x1aV\x20If\x20specified,\x20a\
    \x20cookie\x20with\x20the\x20TTL\x20will\x20be\x20generated\x20if\x20the\
    \x20cookie\x20is\n\x20not\x20present.\n\n\x13\n\t\x04\x03\x03\x02\x03\
    \x01\x02\x01\x04\x12\x06\xf0\x01\x06\xed\x01\x16\n\x11\n\t\x04\x03\x03\
    \x02\x03\x01\x02\x01\x06\x12\x04\xf0\x01\x06\x1e\n\x11\n\t\x04\x03\x03\
    \x02\x03\x01\x02\x01\x01\x12\x04\xf0\x01\x1f\"\n\x11\n\t\x04\x03\x03\x02\
    \x03\x01\x02\x01\x03\x12\x04\xf0\x01%&\n\x10\n\x06\x04\x03\x03\x02\x03\
    \x02\x12\x06\xf2\x01\x04\xf5\x01\x05\n\x0f\n\x07\x04\x03\x03\x02\x03\x02\
    \x01\x12\x04\xf2\x01\x0c\x20\n.\n\x08\x04\x03\x03\x02\x03\x02\x02\0\x12\
    \x04\xf4\x01\x06\x19\x1a\x1c\x20Hash\x20on\x20source\x20IP\x20address.\n\
    \n\x13\n\t\x04\x03\x03\x02\x03\x02\x02\0\x04\x12\x06\xf4\x01\x06\xf2\x01\
    \"\n\x11\n\t\x04\x03\x03\x02\x03\x02\x02\0\x05\x12\x04\xf4\x01\x06\n\n\
    \x11\n\t\x04\x03\x03\x02\x03\x02\x02\0\x01\x12\x04\xf4\x01\x0b\x14\n\x11\
    \n\t\x04\x03\x03\x02\x03\x02\x02\0\x03\x12\x04\xf4\x01\x17\x18\n\x10\n\
    \x06\x04\x03\x03\x02\x08\0\x12\x06\xf6\x01\x04\xfa\x01\x05\n\x0f\n\x07\
    \x04\x03\x03\x02\x08\0\x01\x12\x04\xf6\x01\n\x1a\n\x0e\n\x06\x04\x03\x03\
    \x02\x02\0\x12\x04\xf7\x01\x06\x18\n\x0f\n\x07\x04\x03\x03\x02\x02\0\x06\
    \x12\x04\xf7\x01\x06\x0c\n\x0f\n\x07\x04\x03\x03\x02\x02\0\x01\x12\x04\
    \xf7\x01\r\x13\n\x0f\n\x07\x04\x03\x03\x02\x02\0\x03\x12\x04\xf7\x01\x16\
    \x17\n\x0e\n\x06\x04\x03\x03\x02\x02\x01\x12\x04\xf8\x01\x06\x18\n\x0f\n\
    \x07\x04\x03\x03\x02\x02\x01\x06\x12\x04\xf8\x01\x06\x0c\n\x0f\n\x07\x04\
    \x03\x03\x02\x02\x01\x01\x12\x04\xf8\x01\r\x13\n\x0f\n\x07\x04\x03\x03\
    \x02\x02\x01\x03\x12\x04\xf8\x01\x16\x17\n\x0e\n\x06\x04\x03\x03\x02\x02\
    \x02\x12\x04\xf9\x01\x065\n\x0f\n\x07\x04\x03\x03\x02\x02\x02\x06\x12\
    \x04\xf9\x01\x06\x1a\n\x0f\n\x07\x04\x03\x03\x02\x02\x02\x01\x12\x04\xf9\
    \x01\x1b0\n\x0f\n\x07\x04\x03\x03\x02\x02\x02\x03\x12\x04\xf9\x0134\n\
    \x0c\n\x04\x04\x03\x02\x0e\x12\x04\xfc\x01\x02'\n\r\n\x05\x04\x03\x02\
    \x0e\x04\x12\x04\xfc\x01\x02\n\n\r\n\x05\x04\x03\x02\x0e\x06\x12\x04\xfc\
    \x01\x0b\x15\n\r\n\x05\x04\x03\x02\x0e\x01\x12\x04\xfc\x01\x16!\n\r\n\
    \x05\x04\x03\x02\x0e\x03\x12\x04\xfc\x01$&\n\x92\x06\n\x04\x04\x03\x02\
    \x0f\x12\x04\x8e\x02\x02/\x1a\x83\x06\x20Indicates\x20that\x20a\x20HTTP/\
    1.1\x20client\x20connection\x20to\x20this\x20particular\x20route\n\x20sh\
    ould\x20be\x20allowed\x20(and\x20expected)\x20to\x20upgrade\x20to\x20a\
    \x20WebSocket\x20connection.\x20The\n\x20default\x20is\x20false.\n\n\x20\
    Attention\n\n\x20If\x20set\x20to\x20true,\x20Envoy\x20will\x20expect\x20\
    the\x20first\x20request\x20matching\x20this\x20route\x20to\n\x20contain\
    \x20WebSocket\x20upgrade\x20headers.\x20If\x20the\x20headers\x20are\x20n\
    ot\x20present,\x20the\n\x20connection\x20will\x20be\x20rejected.\x20If\
    \x20set\x20to\x20true,\x20Envoy\x20will\x20setup\x20plain\x20TCP\n\x20pr\
    oxying\x20between\x20the\x20client\x20and\x20the\x20upstream\x20server.\
    \x20Hence,\x20an\x20upstream\n\x20server\x20that\x20rejects\x20the\x20We\
    bSocket\x20upgrade\x20request\x20is\x20also\x20responsible\x20for\n\x20c\
    losing\x20the\x20associated\x20connection.\x20Until\x20then,\x20Envoy\
    \x20will\x20continue\x20to\n\x20proxy\x20data\x20from\x20the\x20client\
    \x20to\x20the\x20upstream\x20server.\n\n\x20Redirects,\x20timeouts\x20an\
    d\x20retries\x20are\x20not\x20supported\x20on\x20routes\x20where\n\x20we\
    bsocket\x20upgrades\x20are\x20allowed.\n\n\x0f\n\x05\x04\x03\x02\x0f\x04\
    \x12\x06\x8e\x02\x02\xfc\x01'\n\r\n\x05\x04\x03\x02\x0f\x06\x12\x04\x8e\
    \x02\x02\x1b\n\r\n\x05\x04\x03\x02\x0f\x01\x12\x04\x8e\x02\x1c)\n\r\n\
    \x05\x04\x03\x02\x0f\x03\x12\x04\x8e\x02,.\n;\n\x04\x04\x03\x02\x10\x12\
    \x04\x91\x02\x02\x17\x1a-\x20Indicates\x20that\x20the\x20route\x20has\
    \x20a\x20CORS\x20policy.\n\n\x0f\n\x05\x04\x03\x02\x10\x04\x12\x06\x91\
    \x02\x02\x8e\x02/\n\r\n\x05\x04\x03\x02\x10\x06\x12\x04\x91\x02\x02\x0c\
    \n\r\n\x05\x04\x03\x02\x10\x01\x12\x04\x91\x02\r\x11\n\r\n\x05\x04\x03\
    \x02\x10\x03\x12\x04\x91\x02\x14\x16\n\x0c\n\x02\x04\x04\x12\x06\x94\x02\
    \0\x9b\x02\x01\n\x0b\n\x03\x04\x04\x01\x12\x04\x94\x02\x08\x16\nn\n\x04\
    \x04\x04\x02\0\x12\x04\x97\x02\x02\x1b\x1a`\x20A\x20302\x20redirect\x20r\
    esponse\x20will\x20be\x20sent\x20which\x20swaps\x20the\x20host\x20portio\
    n\x20of\x20the\n\x20URL\x20with\x20this\x20value.\n\n\x0f\n\x05\x04\x04\
    \x02\0\x04\x12\x06\x97\x02\x02\x94\x02\x18\n\r\n\x05\x04\x04\x02\0\x05\
    \x12\x04\x97\x02\x02\x08\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\x97\x02\t\
    \x16\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\x97\x02\x19\x1a\nn\n\x04\x04\
    \x04\x02\x01\x12\x04\x9a\x02\x02\x1b\x1a`\x20A\x20302\x20redirect\x20res\
    ponse\x20will\x20be\x20sent\x20which\x20swaps\x20the\x20path\x20portion\
    \x20of\x20the\n\x20URL\x20with\x20this\x20value.\n\n\x0f\n\x05\x04\x04\
    \x02\x01\x04\x12\x06\x9a\x02\x02\x97\x02\x1b\n\r\n\x05\x04\x04\x02\x01\
    \x05\x12\x04\x9a\x02\x02\x08\n\r\n\x05\x04\x04\x02\x01\x01\x12\x04\x9a\
    \x02\t\x16\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\x9a\x02\x19\x1a\n\x0c\n\
    \x02\x04\x05\x12\x06\x9d\x02\0\xa0\x02\x01\n\x0b\n\x03\x04\x05\x01\x12\
    \x04\x9d\x02\x08\x11\nN\n\x04\x04\x05\x02\0\x12\x04\x9f\x02\x02\x17\x1a@\
    \x20The\x20operation\x20(or\x20span\x20name)\x20to\x20be\x20used\x20for\
    \x20the\x20matched\x20route.\n\n\x0f\n\x05\x04\x05\x02\0\x04\x12\x06\x9f\
    \x02\x02\x9d\x02\x13\n\r\n\x05\x04\x05\x02\0\x05\x12\x04\x9f\x02\x02\x08\
    \n\r\n\x05\x04\x05\x02\0\x01\x12\x04\x9f\x02\t\x12\n\r\n\x05\x04\x05\x02\
    \0\x03\x12\x04\x9f\x02\x15\x16\nj\n\x02\x04\x06\x12\x06\xa4\x02\0\xb8\
    \x02\x01\x1a\\\x20The\x20match/action\x20distinction\x20in\x20Route\x20i\
    s\x20surfaced\x20explicitly\x20in\x20the\x20v2\x20API\n\x20[V2-API-DIFF]\
    .\n\n\x0b\n\x03\x04\x06\x01\x12\x04\xa4\x02\x08\r\n\x0c\n\x04\x04\x06\
    \x02\0\x12\x04\xa5\x02\x02\x17\n\x0f\n\x05\x04\x06\x02\0\x04\x12\x06\xa5\
    \x02\x02\xa4\x02\x0f\n\r\n\x05\x04\x06\x02\0\x06\x12\x04\xa5\x02\x02\x0c\
    \n\r\n\x05\x04\x06\x02\0\x01\x12\x04\xa5\x02\r\x12\n\r\n\x05\x04\x06\x02\
    \0\x03\x12\x04\xa5\x02\x15\x16\n\x0e\n\x04\x04\x06\x08\0\x12\x06\xa7\x02\
    \x02\xac\x02\x03\n\r\n\x05\x04\x06\x08\0\x01\x12\x04\xa7\x02\x08\x0e\n7\
    \n\x04\x04\x06\x02\x01\x12\x04\xa9\x02\x04\x1a\x1a)\x20Route\x20request\
    \x20to\x20some\x20upstream\x20cluster.\n\n\r\n\x05\x04\x06\x02\x01\x06\
    \x12\x04\xa9\x02\x04\x0f\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\xa9\x02\
    \x10\x15\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\xa9\x02\x18\x19\n&\n\x04\
    \x04\x06\x02\x02\x12\x04\xab\x02\x04\x20\x1a\x18\x20Return\x20a\x20302\
    \x20redirect.\n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\xab\x02\x04\x12\n\
    \r\n\x05\x04\x06\x02\x02\x01\x12\x04\xab\x02\x13\x1b\n\r\n\x05\x04\x06\
    \x02\x02\x03\x12\x04\xab\x02\x1e\x1f\n\xa1\x03\n\x04\x04\x06\x02\x03\x12\
    \x04\xb4\x02\x02\x18\x1a\x92\x03\x20See\x20base.Metadata\x20description\
    \x20for\x20message\x20structure.\x20The\x20Metadata\x20field\n\x20can\
    \x20be\x20used\x20to\x20provide\x20the\x20Router\x20filter\x20additional\
    \x20information\n\x20about\x20the\x20Route.\x20It\x20can\x20be\x20used\
    \x20for\x20configuration,\x20stats,\x20and\x20logging.\n\x20The\x20metad\
    ata\x20should\x20go\x20under\x20the\x20filter\x20namespace\x20that\x20wi\
    ll\x20need\x20it.\n\x20For\x20instance,\x20if\x20the\x20metadata\x20is\
    \x20intended\x20for\x20the\x20Router\x20filter,\n\x20the\x20filter\x20na\
    me\x20should\x20be\x20specified\x20as\x20\"envoy.router\".\n\n\x0f\n\x05\
    \x04\x06\x02\x03\x04\x12\x06\xb4\x02\x02\xac\x02\x03\n\r\n\x05\x04\x06\
    \x02\x03\x06\x12\x04\xb4\x02\x02\n\n\r\n\x05\x04\x06\x02\x03\x01\x12\x04\
    \xb4\x02\x0b\x13\n\r\n\x05\x04\x06\x02\x03\x03\x12\x04\xb4\x02\x16\x17\n\
    ,\n\x04\x04\x06\x02\x04\x12\x04\xb7\x02\x02\x1a\x1a\x1e\x20Decorator\x20\
    for\x20matched\x20route.\n\n\x0f\n\x05\x04\x06\x02\x04\x04\x12\x06\xb7\
    \x02\x02\xb4\x02\x18\n\r\n\x05\x04\x06\x02\x04\x06\x12\x04\xb7\x02\x02\
    \x0b\n\r\n\x05\x04\x06\x02\x04\x01\x12\x04\xb7\x02\x0c\x15\n\r\n\x05\x04\
    \x06\x02\x04\x03\x12\x04\xb7\x02\x18\x19\n\x9a\x05\n\x02\x04\x07\x12\x06\
    \xc3\x02\0\xce\x02\x01\x1a\x8b\x05\x20A\x20virtual\x20cluster\x20is\x20a\
    \x20way\x20of\x20specifying\x20a\x20regex\x20matching\x20rule\x20against\
    \n\x20certain\x20important\x20endpoints\x20such\x20that\x20statistics\
    \x20are\x20generated\x20explicitly\x20for\n\x20the\x20matched\x20request\
    s.\x20The\x20reason\x20this\x20is\x20useful\x20is\x20that\x20when\x20doi\
    ng\n\x20prefix/path\x20matching\x20Envoy\x20does\x20not\x20always\x20kno\
    w\x20what\x20the\x20application\n\x20considers\x20to\x20be\x20an\x20endp\
    oint.\x20Thus,\x20it\xe2\x80\x99s\x20impossible\x20for\x20Envoy\x20to\
    \x20generically\n\x20emit\x20per\x20endpoint\x20statistics.\x20However,\
    \x20often\x20systems\x20have\x20highly\x20critical\n\x20endpoints\x20tha\
    t\x20they\x20wish\x20to\x20get\x20\xe2\x80\x9cperfect\xe2\x80\x9d\x20sta\
    tistics\x20on.\x20Virtual\x20cluster\n\x20statistics\x20are\x20perfect\
    \x20in\x20the\x20sense\x20that\x20they\x20are\x20emitted\x20on\x20the\
    \x20downstream\n\x20side\x20such\x20that\x20they\x20include\x20network\
    \x20level\x20failures.\n\n\x0b\n\x03\x04\x07\x01\x12\x04\xc3\x02\x08\x16\
    \nG\n\x04\x04\x07\x02\0\x12\x04\xc5\x02\x02\x15\x1a9\x20Specifies\x20a\
    \x20regex\x20pattern\x20to\x20use\x20for\x20matching\x20requests.\n\n\
    \x0f\n\x05\x04\x07\x02\0\x04\x12\x06\xc5\x02\x02\xc3\x02\x18\n\r\n\x05\
    \x04\x07\x02\0\x05\x12\x04\xc5\x02\x02\x08\n\r\n\x05\x04\x07\x02\0\x01\
    \x12\x04\xc5\x02\t\x10\n\r\n\x05\x04\x07\x02\0\x03\x12\x04\xc5\x02\x13\
    \x14\n\x99\x01\n\x04\x04\x07\x02\x01\x12\x04\xc9\x02\x02\x12\x1a\x8a\x01\
    \x20Specifies\x20the\x20name\x20of\x20the\x20virtual\x20cluster.\x20The\
    \x20virtual\x20cluster\x20name\x20as\x20well\n\x20as\x20the\x20virtual\
    \x20host\x20name\x20are\x20used\x20when\x20emitting\x20statistics.\n\n\
    \x0f\n\x05\x04\x07\x02\x01\x04\x12\x06\xc9\x02\x02\xc5\x02\x15\n\r\n\x05\
    \x04\x07\x02\x01\x05\x12\x04\xc9\x02\x02\x08\n\r\n\x05\x04\x07\x02\x01\
    \x01\x12\x04\xc9\x02\t\r\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\xc9\x02\
    \x10\x11\n]\n\x04\x04\x07\x02\x02\x12\x04\xcd\x02\x02\x1b\x1aO\x20Option\
    ally\x20specifies\x20the\x20HTTP\x20method\x20to\x20match\x20on.\x20For\
    \x20example\x20GET,\x20PUT,\n\x20etc.\n\n\x0f\n\x05\x04\x07\x02\x02\x04\
    \x12\x06\xcd\x02\x02\xc9\x02\x12\n\r\n\x05\x04\x07\x02\x02\x06\x12\x04\
    \xcd\x02\x02\x0f\n\r\n\x05\x04\x07\x02\x02\x01\x12\x04\xcd\x02\x10\x16\n\
    \r\n\x05\x04\x07\x02\x02\x03\x12\x04\xcd\x02\x19\x1a\np\n\x02\x04\x08\
    \x12\x06\xd2\x02\0\xa1\x03\x01\x1ab\x20See\n\x20https://lyft.github.io/e\
    nvoy/docs/configuration/http_conn_man/route_config/rate_limits.html\n\n\
    \x0b\n\x03\x04\x08\x01\x12\x04\xd2\x02\x08\x11\n\xf5\x01\n\x04\x04\x08\
    \x02\0\x12\x04\xd7\x02\x02(\x1a\xe6\x01\x20Refers\x20to\x20the\x20stage\
    \x20set\x20in\x20the\x20filter.\x20The\x20rate\x20limit\x20configuration\
    \x20only\n\x20applies\x20to\x20filters\x20with\x20the\x20same\x20stage\
    \x20number.\x20The\x20default\x20stage\x20number\x20is\n\x200.\n\x20NOTE\
    :\x20The\x20filter\x20supports\x20a\x20range\x20of\x200\x20-\x2010\x20in\
    clusively\x20for\x20stage\x20numbers.\n\n\x0f\n\x05\x04\x08\x02\0\x04\
    \x12\x06\xd7\x02\x02\xd2\x02\x13\n\r\n\x05\x04\x08\x02\0\x06\x12\x04\xd7\
    \x02\x02\x1d\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\xd7\x02\x1e#\n\r\n\x05\
    \x04\x08\x02\0\x03\x12\x04\xd7\x02&'\nV\n\x04\x04\x08\x02\x01\x12\x04\
    \xda\x02\x02\x19\x1aH\x20The\x20key\x20to\x20be\x20set\x20in\x20runtime\
    \x20to\x20disable\x20this\x20rate\x20limit\x20configuration.\n\n\x0f\n\
    \x05\x04\x08\x02\x01\x04\x12\x06\xda\x02\x02\xd7\x02(\n\r\n\x05\x04\x08\
    \x02\x01\x05\x12\x04\xda\x02\x02\x08\n\r\n\x05\x04\x08\x02\x01\x01\x12\
    \x04\xda\x02\t\x14\n\r\n\x05\x04\x08\x02\x01\x03\x12\x04\xda\x02\x17\x18\
    \n\x0e\n\x04\x04\x08\x03\0\x12\x06\xdc\x02\x02\x9a\x03\x03\n\r\n\x05\x04\
    \x08\x03\0\x01\x12\x04\xdc\x02\n\x10\n\x80\x01\n\x06\x04\x08\x03\0\x03\0\
    \x12\x06\xdf\x02\x04\xe0\x02\x05\x1an\x20The\x20following\x20descriptor\
    \x20entry\x20is\x20appended\x20to\x20the\x20descriptor:\n\x20(\"source_c\
    luster\",\x20\"<local\x20service\x20cluster>\")\n\n\x0f\n\x07\x04\x08\
    \x03\0\x03\0\x01\x12\x04\xdf\x02\x0c\x19\n\x85\x01\n\x06\x04\x08\x03\0\
    \x03\x01\x12\x06\xe4\x02\x04\xe5\x02\x05\x1as\x20The\x20following\x20des\
    criptor\x20entry\x20is\x20appended\x20to\x20the\x20descriptor:\n\x20(\"d\
    estination_cluster\",\x20\"<routed\x20target\x20cluster>\")\n\n\x0f\n\
    \x07\x04\x08\x03\0\x03\x01\x01\x12\x04\xe4\x02\x0c\x1e\n\xb7\x01\n\x06\
    \x04\x08\x03\0\x03\x02\x12\x06\xea\x02\x04\xf1\x02\x05\x1a\xa4\x01\x20Th\
    e\x20following\x20descriptor\x20entry\x20is\x20appended\x20when\x20a\x20\
    header\x20contains\x20a\x20key\n\x20that\x20matches\x20the\x20header_nam\
    e:\n\x20(\"<descriptor_key>\",\x20\"<header_value_queried_from_header>\"\
    )\n\n\x0f\n\x07\x04\x08\x03\0\x03\x02\x01\x12\x04\xea\x02\x0c\x1a\n\xb1\
    \x01\n\x08\x04\x08\x03\0\x03\x02\x02\0\x12\x04\xee\x02\x06\x1d\x1a\x9e\
    \x01\x20The\x20header\x20name\x20to\x20be\x20queried\x20from\x20the\x20r\
    equest\x20headers.\x20The\x20header\xe2\x80\x99s\n\x20value\x20is\x20use\
    d\x20to\x20populate\x20the\x20value\x20of\x20the\x20descriptor\x20entry\
    \x20for\x20the\n\x20descriptor_key.\n\n\x13\n\t\x04\x08\x03\0\x03\x02\
    \x02\0\x04\x12\x06\xee\x02\x06\xea\x02\x1c\n\x11\n\t\x04\x08\x03\0\x03\
    \x02\x02\0\x05\x12\x04\xee\x02\x06\x0c\n\x11\n\t\x04\x08\x03\0\x03\x02\
    \x02\0\x01\x12\x04\xee\x02\r\x18\n\x11\n\t\x04\x08\x03\0\x03\x02\x02\0\
    \x03\x12\x04\xee\x02\x1b\x1c\n;\n\x08\x04\x08\x03\0\x03\x02\x02\x01\x12\
    \x04\xf0\x02\x06\x20\x1a)\x20The\x20key\x20to\x20use\x20in\x20the\x20des\
    criptor\x20entry.\n\n\x13\n\t\x04\x08\x03\0\x03\x02\x02\x01\x04\x12\x06\
    \xf0\x02\x06\xee\x02\x1d\n\x11\n\t\x04\x08\x03\0\x03\x02\x02\x01\x05\x12\
    \x04\xf0\x02\x06\x0c\n\x11\n\t\x04\x08\x03\0\x03\x02\x02\x01\x01\x12\x04\
    \xf0\x02\r\x1b\n\x11\n\t\x04\x08\x03\0\x03\x02\x02\x01\x03\x12\x04\xf0\
    \x02\x1e\x1f\n\xd1\x01\n\x06\x04\x08\x03\0\x03\x03\x12\x06\xf6\x02\x04\
    \xf7\x02\x05\x1a\xbe\x01\x20The\x20following\x20descriptor\x20entry\x20i\
    s\x20appended\x20to\x20the\x20descriptor\x20and\x20is\n\x20populated\x20\
    using\x20the\x20trusted\x20address\x20from\x20x-forwarded-for:\n\x20(\"r\
    emote_address\",\x20\"<trusted\x20address\x20from\x20x-forwarded-for>\")\
    \n\n\x0f\n\x07\x04\x08\x03\0\x03\x03\x01\x12\x04\xf6\x02\x0c\x19\nx\n\
    \x06\x04\x08\x03\0\x03\x04\x12\x06\xfb\x02\x04\xfe\x02\x05\x1af\x20The\
    \x20following\x20descriptor\x20entry\x20is\x20appended\x20to\x20the\x20d\
    escriptor:\n\x20(\"generic_key\",\x20\"<descriptor_value>\")\n\n\x0f\n\
    \x07\x04\x08\x03\0\x03\x04\x01\x12\x04\xfb\x02\x0c\x16\n=\n\x08\x04\x08\
    \x03\0\x03\x04\x02\0\x12\x04\xfd\x02\x06\"\x1a+\x20The\x20value\x20to\
    \x20use\x20in\x20the\x20descriptor\x20entry.\n\n\x13\n\t\x04\x08\x03\0\
    \x03\x04\x02\0\x04\x12\x06\xfd\x02\x06\xfb\x02\x18\n\x11\n\t\x04\x08\x03\
    \0\x03\x04\x02\0\x05\x12\x04\xfd\x02\x06\x0c\n\x11\n\t\x04\x08\x03\0\x03\
    \x04\x02\0\x01\x12\x04\xfd\x02\r\x1d\n\x11\n\t\x04\x08\x03\0\x03\x04\x02\
    \0\x03\x12\x04\xfd\x02\x20!\n\x81\x01\n\x06\x04\x08\x03\0\x03\x05\x12\
    \x06\x82\x03\x04\x90\x03\x05\x1ao\x20The\x20following\x20descriptor\x20e\
    ntry\x20is\x20appended\x20to\x20the\x20descriptor:\n\x20(\xe2\x80\x9chea\
    der_match\xe2\x80\x9d,\x20\xe2\x80\x9c<descriptor_value>\xe2\x80\x9d)\n\
    \n\x0f\n\x07\x04\x08\x03\0\x03\x05\x01\x12\x04\x82\x03\x0c\x1c\n=\n\x08\
    \x04\x08\x03\0\x03\x05\x02\0\x12\x04\x84\x03\x06\"\x1a+\x20The\x20value\
    \x20to\x20use\x20in\x20the\x20descriptor\x20entry.\n\n\x13\n\t\x04\x08\
    \x03\0\x03\x05\x02\0\x04\x12\x06\x84\x03\x06\x82\x03\x1e\n\x11\n\t\x04\
    \x08\x03\0\x03\x05\x02\0\x05\x12\x04\x84\x03\x06\x0c\n\x11\n\t\x04\x08\
    \x03\0\x03\x05\x02\0\x01\x12\x04\x84\x03\r\x1d\n\x11\n\t\x04\x08\x03\0\
    \x03\x05\x02\0\x03\x12\x04\x84\x03\x20!\n\xfa\x01\n\x08\x04\x08\x03\0\
    \x03\x05\x02\x01\x12\x04\x89\x03\x061\x1a\xe7\x01\x20If\x20set\x20to\x20\
    true,\x20the\x20action\x20will\x20append\x20a\x20descriptor\x20entry\x20\
    when\x20the\n\x20request\x20matches\x20the\x20headers.\x20If\x20set\x20t\
    o\x20false,\x20the\x20action\x20will\x20append\x20a\n\x20descriptor\x20e\
    ntry\x20when\x20the\x20request\x20does\x20not\x20match\x20the\x20headers\
    .\x20The\n\x20default\x20value\x20is\x20true.\n\n\x13\n\t\x04\x08\x03\0\
    \x03\x05\x02\x01\x04\x12\x06\x89\x03\x06\x84\x03\"\n\x11\n\t\x04\x08\x03\
    \0\x03\x05\x02\x01\x06\x12\x04\x89\x03\x06\x1f\n\x11\n\t\x04\x08\x03\0\
    \x03\x05\x02\x01\x01\x12\x04\x89\x03\x20,\n\x11\n\t\x04\x08\x03\0\x03\
    \x05\x02\x01\x03\x12\x04\x89\x03/0\n\xe3\x02\n\x08\x04\x08\x03\0\x03\x05\
    \x02\x02\x12\x04\x8f\x03\x06)\x1a\xd0\x02\x20Specifies\x20a\x20set\x20of\
    \x20headers\x20that\x20the\x20rate\x20limit\x20action\x20should\x20match\
    \n\x20on.\x20The\x20action\x20will\x20check\x20the\x20request\xe2\x80\
    \x99s\x20headers\x20against\x20all\x20the\n\x20specified\x20headers\x20i\
    n\x20the\x20config.\x20A\x20match\x20will\x20happen\x20if\x20all\x20the\
    \n\x20headers\x20in\x20the\x20config\x20are\x20present\x20in\x20the\x20r\
    equest\x20with\x20the\x20same\x20values\n\x20(or\x20based\x20on\x20prese\
    nce\x20if\x20the\x20value\x20field\x20is\x20not\x20in\x20the\x20config).\
    \n\n\x11\n\t\x04\x08\x03\0\x03\x05\x02\x02\x04\x12\x04\x8f\x03\x06\x0e\n\
    \x11\n\t\x04\x08\x03\0\x03\x05\x02\x02\x06\x12\x04\x8f\x03\x0f\x1c\n\x11\
    \n\t\x04\x08\x03\0\x03\x05\x02\x02\x01\x12\x04\x8f\x03\x1d$\n\x11\n\t\
    \x04\x08\x03\0\x03\x05\x02\x02\x03\x12\x04\x8f\x03'(\n\x10\n\x06\x04\x08\
    \x03\0\x08\0\x12\x06\x92\x03\x04\x99\x03\x05\n\x0f\n\x07\x04\x08\x03\0\
    \x08\0\x01\x12\x04\x92\x03\n\x1a\n\x0e\n\x06\x04\x08\x03\0\x02\0\x12\x04\
    \x93\x03\x06'\n\x0f\n\x07\x04\x08\x03\0\x02\0\x06\x12\x04\x93\x03\x06\
    \x13\n\x0f\n\x07\x04\x08\x03\0\x02\0\x01\x12\x04\x93\x03\x14\"\n\x0f\n\
    \x07\x04\x08\x03\0\x02\0\x03\x12\x04\x93\x03%&\n\x0e\n\x06\x04\x08\x03\0\
    \x02\x01\x12\x04\x94\x03\x061\n\x0f\n\x07\x04\x08\x03\0\x02\x01\x06\x12\
    \x04\x94\x03\x06\x18\n\x0f\n\x07\x04\x08\x03\0\x02\x01\x01\x12\x04\x94\
    \x03\x19,\n\x0f\n\x07\x04\x08\x03\0\x02\x01\x03\x12\x04\x94\x03/0\n\x0e\
    \n\x06\x04\x08\x03\0\x02\x02\x12\x04\x95\x03\x06)\n\x0f\n\x07\x04\x08\
    \x03\0\x02\x02\x06\x12\x04\x95\x03\x06\x14\n\x0f\n\x07\x04\x08\x03\0\x02\
    \x02\x01\x12\x04\x95\x03\x15$\n\x0f\n\x07\x04\x08\x03\0\x02\x02\x03\x12\
    \x04\x95\x03'(\n\x0e\n\x06\x04\x08\x03\0\x02\x03\x12\x04\x96\x03\x06'\n\
    \x0f\n\x07\x04\x08\x03\0\x02\x03\x06\x12\x04\x96\x03\x06\x13\n\x0f\n\x07\
    \x04\x08\x03\0\x02\x03\x01\x12\x04\x96\x03\x14\"\n\x0f\n\x07\x04\x08\x03\
    \0\x02\x03\x03\x12\x04\x96\x03%&\n\x0e\n\x06\x04\x08\x03\0\x02\x04\x12\
    \x04\x97\x03\x06!\n\x0f\n\x07\x04\x08\x03\0\x02\x04\x06\x12\x04\x97\x03\
    \x06\x10\n\x0f\n\x07\x04\x08\x03\0\x02\x04\x01\x12\x04\x97\x03\x11\x1c\n\
    \x0f\n\x07\x04\x08\x03\0\x02\x04\x03\x12\x04\x97\x03\x1f\x20\n\x0e\n\x06\
    \x04\x08\x03\0\x02\x05\x12\x04\x98\x03\x06.\n\x0f\n\x07\x04\x08\x03\0\
    \x02\x05\x06\x12\x04\x98\x03\x06\x16\n\x0f\n\x07\x04\x08\x03\0\x02\x05\
    \x01\x12\x04\x98\x03\x17)\n\x0f\n\x07\x04\x08\x03\0\x02\x05\x03\x12\x04\
    \x98\x03,-\n\xfe\x02\n\x04\x04\x08\x02\x02\x12\x04\xa0\x03\x02\x1e\x1a\
    \xef\x02\x20A\x20list\x20of\x20actions\x20that\x20are\x20to\x20be\x20app\
    lied\x20for\x20this\x20rate\x20limit\x20configuration.\n\x20Order\x20mat\
    ters\x20as\x20the\x20actions\x20are\x20processed\x20sequentially\x20and\
    \x20the\x20descriptor\n\x20is\x20composed\x20by\x20appending\x20descript\
    or\x20entries\x20in\x20that\x20sequence.\x20If\x20an\x20action\n\x20cann\
    ot\x20append\x20a\x20descriptor\x20entry,\x20no\x20descriptor\x20is\x20g\
    enerated\x20for\x20the\n\x20configuration.\x20See\x20composing\x20action\
    s\x20for\x20additional\x20documentation.\n\n\r\n\x05\x04\x08\x02\x02\x04\
    \x12\x04\xa0\x03\x02\n\n\r\n\x05\x04\x08\x02\x02\x06\x12\x04\xa0\x03\x0b\
    \x11\n\r\n\x05\x04\x08\x02\x02\x01\x12\x04\xa0\x03\x12\x19\n\r\n\x05\x04\
    \x08\x02\x02\x03\x12\x04\xa0\x03\x1c\x1d\n\x0c\n\x02\x04\t\x12\x06\xa3\
    \x03\0\xac\x03\x01\n\x0b\n\x03\x04\t\x01\x12\x04\xa3\x03\x08\x15\n@\n\
    \x04\x04\t\x02\0\x12\x04\xa5\x03\x02\x12\x1a2\x20Specifies\x20the\x20nam\
    e\x20of\x20the\x20header\x20in\x20the\x20request.\n\n\x0f\n\x05\x04\t\
    \x02\0\x04\x12\x06\xa5\x03\x02\xa3\x03\x17\n\r\n\x05\x04\t\x02\0\x05\x12\
    \x04\xa5\x03\x02\x08\n\r\n\x05\x04\t\x02\0\x01\x12\x04\xa5\x03\t\r\n\r\n\
    \x05\x04\t\x02\0\x03\x12\x04\xa5\x03\x10\x11\n\x9e\x01\n\x04\x04\t\x02\
    \x01\x12\x04\xa8\x03\x02\x13\x1a\x8f\x01\x20Specifies\x20the\x20value\
    \x20of\x20the\x20header.\x20If\x20the\x20value\x20is\x20absent\x20a\x20r\
    equest\x20that\n\x20has\x20the\x20name\x20header\x20will\x20match,\x20re\
    gardless\x20of\x20the\x20header\xe2\x80\x99s\x20value.\n\n\x0f\n\x05\x04\
    \t\x02\x01\x04\x12\x06\xa8\x03\x02\xa5\x03\x12\n\r\n\x05\x04\t\x02\x01\
    \x05\x12\x04\xa8\x03\x02\x08\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\xa8\x03\
    \t\x0e\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\xa8\x03\x11\x12\nf\n\x04\x04\
    \t\x02\x02\x12\x04\xab\x03\x02&\x1aX\x20Specifies\x20whether\x20the\x20h\
    eader\x20value\x20is\x20a\x20regular\x20expression\x20or\x20not.\n\x20De\
    faults\x20to\x20false.\n\n\x0f\n\x05\x04\t\x02\x02\x04\x12\x06\xab\x03\
    \x02\xa8\x03\x13\n\r\n\x05\x04\t\x02\x02\x06\x12\x04\xab\x03\x02\x1b\n\r\
    \n\x05\x04\t\x02\x02\x01\x12\x04\xab\x03\x1c!\n\r\n\x05\x04\t\x02\x02\
    \x03\x12\x04\xab\x03$%\n\x0c\n\x02\x04\n\x12\x06\xae\x03\0\xdd\x03\x01\n\
    \x0b\n\x03\x04\n\x01\x12\x04\xae\x03\x08\x13\n\x85\x01\n\x04\x04\n\x02\0\
    \x12\x04\xb1\x03\x02\x12\x1aw\x20The\x20logical\x20name\x20of\x20the\x20\
    virtual\x20host.\x20This\x20is\x20used\x20when\x20emitting\x20certain\n\
    \x20statistics\x20but\x20is\x20not\x20relevant\x20for\x20routing.\n\n\
    \x0f\n\x05\x04\n\x02\0\x04\x12\x06\xb1\x03\x02\xae\x03\x15\n\r\n\x05\x04\
    \n\x02\0\x05\x12\x04\xb1\x03\x02\x08\n\r\n\x05\x04\n\x02\0\x01\x12\x04\
    \xb1\x03\t\r\n\r\n\x05\x04\n\x02\0\x03\x12\x04\xb1\x03\x10\x11\n\xd0\x04\
    \n\x04\x04\n\x02\x01\x12\x04\xbb\x03\x02\x1e\x1a\xc1\x04\x20A\x20list\
    \x20of\x20domains\x20(host/authority\x20header)\x20that\x20will\x20be\
    \x20matched\x20to\x20this\n\x20virtual\x20host.\x20Wildcard\x20hosts\x20\
    are\x20supported\x20in\x20the\x20form\x20of\x20\xe2\x80\x9c*.foo.com\xe2\
    \x80\x9d\x20or\n\x20\xe2\x80\x9c*-bar.foo.com\xe2\x80\x9d.\x20Note\x20th\
    at\x20the\x20wildcard\x20will\x20not\x20match\x20the\x20empty\x20string.\
    \n\x20e.g.\x20\xe2\x80\x9c*-bar.foo.com\xe2\x80\x9d\x20will\x20match\x20\
    \xe2\x80\x9cbaz-bar.foo.com\xe2\x80\x9d\x20but\x20not\x20\xe2\x80\x9c-ba\
    r.foo.com\xe2\x80\x9d.\n\x20Additionally,\x20a\x20special\x20entry\x20\
    \xe2\x80\x9c*\xe2\x80\x9d\x20is\x20allowed\x20which\x20will\x20match\x20\
    any\n\x20host/authority\x20header.\x20Only\x20a\x20single\x20virtual\x20\
    host\x20in\x20the\x20entire\x20route\n\x20configuration\x20can\x20match\
    \x20on\x20\xe2\x80\x9c*\xe2\x80\x9d.\x20A\x20domain\x20must\x20be\x20uni\
    que\x20across\x20all\x20virtual\n\x20hosts\x20or\x20the\x20config\x20wil\
    l\x20fail\x20to\x20load.\n\n\r\n\x05\x04\n\x02\x01\x04\x12\x04\xbb\x03\
    \x02\n\n\r\n\x05\x04\n\x02\x01\x05\x12\x04\xbb\x03\x0b\x11\n\r\n\x05\x04\
    \n\x02\x01\x01\x12\x04\xbb\x03\x12\x19\n\r\n\x05\x04\n\x02\x01\x03\x12\
    \x04\xbb\x03\x1c\x1d\n\x85\x01\n\x04\x04\n\x02\x02\x12\x04\xbf\x03\x02\
    \x1c\x1aw\x20The\x20list\x20of\x20routes\x20that\x20will\x20be\x20matche\
    d,\x20in\x20order,\x20for\x20incoming\x20requests.\n\x20The\x20first\x20\
    route\x20that\x20matches\x20will\x20be\x20used.\n\n\r\n\x05\x04\n\x02\
    \x02\x04\x12\x04\xbf\x03\x02\n\n\r\n\x05\x04\n\x02\x02\x06\x12\x04\xbf\
    \x03\x0b\x10\n\r\n\x05\x04\n\x02\x02\x01\x12\x04\xbf\x03\x11\x17\n\r\n\
    \x05\x04\n\x02\x02\x03\x12\x04\xbf\x03\x1a\x1b\n\x0e\n\x04\x04\n\x04\0\
    \x12\x06\xc1\x03\x02\xca\x03\x03\n\r\n\x05\x04\n\x04\0\x01\x12\x04\xc1\
    \x03\x07\x19\n:\n\x06\x04\n\x04\0\x02\0\x12\x04\xc3\x03\x04\r\x1a*\x20No\
    \x20TLS\x20requirement\x20for\x20the\x20virtual\x20host.\n\n\x0f\n\x07\
    \x04\n\x04\0\x02\0\x01\x12\x04\xc3\x03\x04\x08\n\x0f\n\x07\x04\n\x04\0\
    \x02\0\x02\x12\x04\xc3\x03\x0b\x0c\n\xa2\x01\n\x06\x04\n\x04\0\x02\x01\
    \x12\x04\xc6\x03\x04\x16\x1a\x91\x01\x20External\x20requests\x20must\x20\
    use\x20TLS.\x20If\x20a\x20request\x20is\x20external\x20and\x20it\x20is\
    \x20not\n\x20using\x20TLS,\x20a\x20302\x20redirect\x20will\x20be\x20sent\
    \x20telling\x20the\x20client\x20to\x20use\x20HTTPS.\n\n\x0f\n\x07\x04\n\
    \x04\0\x02\x01\x01\x12\x04\xc6\x03\x04\x11\n\x0f\n\x07\x04\n\x04\0\x02\
    \x01\x02\x12\x04\xc6\x03\x14\x15\n\x89\x01\n\x06\x04\n\x04\0\x02\x02\x12\
    \x04\xc9\x03\x04\x0c\x1ay\x20All\x20requests\x20must\x20use\x20TLS.\x20I\
    f\x20a\x20request\x20is\x20not\x20using\x20TLS,\x20a\x20302\x20redirect\
    \n\x20will\x20be\x20sent\x20telling\x20the\x20client\x20to\x20use\x20HTT\
    PS.\n\n\x0f\n\x07\x04\n\x04\0\x02\x02\x01\x12\x04\xc9\x03\x04\x07\n\x0f\
    \n\x07\x04\n\x04\0\x02\x02\x02\x12\x04\xc9\x03\n\x0b\nO\n\x04\x04\n\x02\
    \x03\x12\x04\xcc\x03\x02%\x1aA\x20Specifies\x20the\x20type\x20of\x20TLS\
    \x20enforcement\x20the\x20virtual\x20host\x20expects.\n\n\x0f\n\x05\x04\
    \n\x02\x03\x04\x12\x06\xcc\x03\x02\xca\x03\x03\n\r\n\x05\x04\n\x02\x03\
    \x06\x12\x04\xcc\x03\x02\x14\n\r\n\x05\x04\n\x02\x03\x01\x12\x04\xcc\x03\
    \x15\x20\n\r\n\x05\x04\n\x02\x03\x03\x12\x04\xcc\x03#$\n\x89\x01\n\x04\
    \x04\n\x02\x04\x12\x04\xd0\x03\x02/\x1a{\x20A\x20list\x20of\x20virtual\
    \x20clusters\x20defined\x20for\x20this\x20virtual\x20host.\x20Virtual\
    \x20clusters\n\x20are\x20used\x20for\x20additional\x20statistics\x20gath\
    ering.\n\n\r\n\x05\x04\n\x02\x04\x04\x12\x04\xd0\x03\x02\n\n\r\n\x05\x04\
    \n\x02\x04\x06\x12\x04\xd0\x03\x0b\x19\n\r\n\x05\x04\n\x02\x04\x01\x12\
    \x04\xd0\x03\x1a*\n\r\n\x05\x04\n\x02\x04\x03\x12\x04\xd0\x03-.\ng\n\x04\
    \x04\n\x02\x05\x12\x04\xd4\x03\x02%\x1aY\x20Specifies\x20a\x20set\x20of\
    \x20rate\x20limit\x20configurations\x20that\x20will\x20be\x20applied\x20\
    to\x20the\n\x20virtual\x20host.\n\n\r\n\x05\x04\n\x02\x05\x04\x12\x04\
    \xd4\x03\x02\n\n\r\n\x05\x04\n\x02\x05\x06\x12\x04\xd4\x03\x0b\x14\n\r\n\
    \x05\x04\n\x02\x05\x01\x12\x04\xd4\x03\x15\x20\n\r\n\x05\x04\n\x02\x05\
    \x03\x12\x04\xd4\x03#$\n\xb8\x01\n\x04\x04\n\x02\x06\x12\x04\xd9\x03\x02\
    8\x1a\xa9\x01\x20Specifies\x20a\x20list\x20of\x20HTTP\x20headers\x20that\
    \x20should\x20be\x20added\x20to\x20each\x20request\n\x20handled\x20by\
    \x20this\x20virtual\x20host.\x20In\x20the\x20presence\x20of\x20duplicate\
    \x20header\x20keys,\n\x20precedence\x20rules\x20apply.\n\n\r\n\x05\x04\n\
    \x02\x06\x04\x12\x04\xd9\x03\x02\n\n\r\n\x05\x04\n\x02\x06\x06\x12\x04\
    \xd9\x03\x0b\x1c\n\r\n\x05\x04\n\x02\x06\x01\x12\x04\xd9\x03\x1d3\n\r\n\
    \x05\x04\n\x02\x06\x03\x12\x04\xd9\x0367\nB\n\x04\x04\n\x02\x07\x12\x04\
    \xdc\x03\x02\x16\x1a4\x20Indicates\x20that\x20the\x20virtual\x20host\x20\
    has\x20a\x20CORS\x20policy.\n\n\x0f\n\x05\x04\n\x02\x07\x04\x12\x06\xdc\
    \x03\x02\xd9\x038\n\r\n\x05\x04\n\x02\x07\x06\x12\x04\xdc\x03\x02\x0c\n\
    \r\n\x05\x04\n\x02\x07\x01\x12\x04\xdc\x03\r\x11\n\r\n\x05\x04\n\x02\x07\
    \x03\x12\x04\xdc\x03\x14\x15\n\x0c\n\x02\x04\x0b\x12\x06\xdf\x03\0\x86\
    \x04\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\xdf\x03\x08\x1a\n\xa8\x01\n\x04\
    \x04\x0b\x02\0\x12\x04\xe3\x03\x02\x12\x1a\x99\x01\x20The\x20name\x20of\
    \x20the\x20route\x20configuration.\x20For\x20example,\x20it\x20might\x20\
    match\x20the\n\x20router_config_name\x20in\x20the\x20HttpConnectionManag\
    er\x20>\x20route_specifier\x20>\x20rds\n\x20message.\n\n\x0f\n\x05\x04\
    \x0b\x02\0\x04\x12\x06\xe3\x03\x02\xdf\x03\x1c\n\r\n\x05\x04\x0b\x02\0\
    \x05\x12\x04\xe3\x03\x02\x08\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\xe3\x03\
    \t\r\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\xe3\x03\x10\x11\nG\n\x04\x04\
    \x0b\x02\x01\x12\x04\xe6\x03\x02)\x1a9\x20An\x20array\x20of\x20virtual\
    \x20hosts\x20that\x20make\x20up\x20the\x20route\x20table.\n\n\r\n\x05\
    \x04\x0b\x02\x01\x04\x12\x04\xe6\x03\x02\n\n\r\n\x05\x04\x0b\x02\x01\x06\
    \x12\x04\xe6\x03\x0b\x16\n\r\n\x05\x04\x0b\x02\x01\x01\x12\x04\xe6\x03\
    \x17$\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\xe6\x03'(\n\xf6\x01\n\x04\
    \x04\x0b\x02\x02\x12\x04\xec\x03\x02,\x1a\xe7\x01\x20Specifies\x20a\x20l\
    ist\x20of\x20HTTP\x20headers\x20that\x20the\x20connection\x20manager\x20\
    will\x20consider\n\x20to\x20be\x20internal\x20only.\x20If\x20they\x20are\
    \x20found\x20on\x20external\x20requests\x20they\x20will\x20be\n\x20clean\
    ed\x20prior\x20to\x20filter\x20invocation.\x20See\x20x-envoy-internal\
    \x20for\x20more\n\x20information.\n\n\r\n\x05\x04\x0b\x02\x02\x04\x12\
    \x04\xec\x03\x02\n\n\r\n\x05\x04\x0b\x02\x02\x05\x12\x04\xec\x03\x0b\x11\
    \n\r\n\x05\x04\x0b\x02\x02\x01\x12\x04\xec\x03\x12'\n\r\n\x05\x04\x0b\
    \x02\x02\x03\x12\x04\xec\x03*+\n|\n\x04\x04\x0b\x02\x03\x12\x04\xf0\x03\
    \x029\x1an\x20Specifies\x20a\x20list\x20of\x20HTTP\x20headers\x20that\
    \x20should\x20be\x20added\x20to\x20each\x20response\x20that\n\x20the\x20\
    connection\x20manager\x20encodes.\n\n\r\n\x05\x04\x0b\x02\x03\x04\x12\
    \x04\xf0\x03\x02\n\n\r\n\x05\x04\x0b\x02\x03\x06\x12\x04\xf0\x03\x0b\x1c\
    \n\r\n\x05\x04\x0b\x02\x03\x01\x12\x04\xf0\x03\x1d4\n\r\n\x05\x04\x0b\
    \x02\x03\x03\x12\x04\xf0\x0378\n\x80\x01\n\x04\x04\x0b\x02\x04\x12\x04\
    \xf4\x03\x021\x1ar\x20Specifies\x20a\x20list\x20of\x20HTTP\x20headers\
    \x20that\x20should\x20be\x20removed\x20from\x20each\x20response\n\x20tha\
    t\x20the\x20connection\x20manager\x20encodes.\n\n\r\n\x05\x04\x0b\x02\
    \x04\x04\x12\x04\xf4\x03\x02\n\n\r\n\x05\x04\x0b\x02\x04\x05\x12\x04\xf4\
    \x03\x0b\x11\n\r\n\x05\x04\x0b\x02\x04\x01\x12\x04\xf4\x03\x12,\n\r\n\
    \x05\x04\x0b\x02\x04\x03\x12\x04\xf4\x03/0\n\xc2\x01\n\x04\x04\x0b\x02\
    \x05\x12\x04\xf9\x03\x028\x1a\xb3\x01\x20Specifies\x20a\x20list\x20of\
    \x20HTTP\x20headers\x20that\x20should\x20be\x20added\x20to\x20each\x20re\
    quest\n\x20routed\x20by\x20the\x20HTTP\x20connection\x20manager.\x20In\
    \x20the\x20presence\x20of\x20duplicate\n\x20header\x20keys,\x20precenden\
    ce\x20rules\x20apply.\n\n\r\n\x05\x04\x0b\x02\x05\x04\x12\x04\xf9\x03\
    \x02\n\n\r\n\x05\x04\x0b\x02\x05\x06\x12\x04\xf9\x03\x0b\x1c\n\r\n\x05\
    \x04\x0b\x02\x05\x01\x12\x04\xf9\x03\x1d3\n\r\n\x05\x04\x0b\x02\x05\x03\
    \x12\x04\xf9\x0367\n\xd4\x05\n\x04\x04\x0b\x02\x06\x12\x04\x85\x04\x022\
    \x1a\xc5\x05\x20An\x20optional\x20boolean\x20that\x20specifies\x20whethe\
    r\x20the\x20clusters\x20that\x20the\x20route\n\x20table\x20refers\x20to\
    \x20will\x20be\x20validated\x20by\x20the\x20cluster\x20manager.\x20If\
    \x20set\x20to\x20true\n\x20and\x20a\x20route\x20refers\x20to\x20a\x20non\
    -existent\x20cluster,\x20the\x20route\x20table\x20will\x20not\n\x20load.\
    \x20If\x20set\x20to\x20false\x20and\x20a\x20route\x20refers\x20to\x20a\
    \x20non-existent\x20cluster,\x20the\n\x20route\x20table\x20will\x20load\
    \x20and\x20the\x20router\x20filter\x20will\x20return\x20a\x20404\x20if\
    \x20the\x20route\n\x20is\x20selected\x20at\x20runtime.\x20This\x20settin\
    g\x20defaults\x20to\x20true\x20if\x20the\x20route\x20table\n\x20is\x20st\
    atically\x20defined\x20via\x20the\x20route_config\x20option.\x20This\x20\
    setting\x20default\x20to\n\x20false\x20if\x20the\x20route\x20table\x20is\
    \x20loaded\x20dynamically\x20via\x20the\x20rds\x20option.\x20Users\n\x20\
    may\x20which\x20to\x20override\x20the\x20default\x20behavior\x20in\x20ce\
    rtain\x20cases\x20(for\x20example\n\x20when\x20using\x20CDS\x20with\x20a\
    \x20static\x20route\x20table).\n\n\x0f\n\x05\x04\x0b\x02\x06\x04\x12\x06\
    \x85\x04\x02\xf9\x038\n\r\n\x05\x04\x0b\x02\x06\x06\x12\x04\x85\x04\x02\
    \x1b\n\r\n\x05\x04\x0b\x02\x06\x01\x12\x04\x85\x04\x1c-\n\r\n\x05\x04\
    \x0b\x02\x06\x03\x12\x04\x85\x0401b\x06proto3\
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
