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
pub struct LightstepConfig {
    // message fields
    pub collector_cluster: ::std::string::String,
    pub access_token_file: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LightstepConfig {}

impl LightstepConfig {
    pub fn new() -> LightstepConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LightstepConfig {
        static mut instance: ::protobuf::lazy::Lazy<LightstepConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LightstepConfig,
        };
        unsafe {
            instance.get(LightstepConfig::new)
        }
    }

    // string collector_cluster = 1;

    pub fn clear_collector_cluster(&mut self) {
        self.collector_cluster.clear();
    }

    // Param is passed by value, moved
    pub fn set_collector_cluster(&mut self, v: ::std::string::String) {
        self.collector_cluster = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collector_cluster(&mut self) -> &mut ::std::string::String {
        &mut self.collector_cluster
    }

    // Take field
    pub fn take_collector_cluster(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collector_cluster, ::std::string::String::new())
    }

    pub fn get_collector_cluster(&self) -> &str {
        &self.collector_cluster
    }

    fn get_collector_cluster_for_reflect(&self) -> &::std::string::String {
        &self.collector_cluster
    }

    fn mut_collector_cluster_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collector_cluster
    }

    // string access_token_file = 2;

    pub fn clear_access_token_file(&mut self) {
        self.access_token_file.clear();
    }

    // Param is passed by value, moved
    pub fn set_access_token_file(&mut self, v: ::std::string::String) {
        self.access_token_file = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token_file(&mut self) -> &mut ::std::string::String {
        &mut self.access_token_file
    }

    // Take field
    pub fn take_access_token_file(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.access_token_file, ::std::string::String::new())
    }

    pub fn get_access_token_file(&self) -> &str {
        &self.access_token_file
    }

    fn get_access_token_file_for_reflect(&self) -> &::std::string::String {
        &self.access_token_file
    }

    fn mut_access_token_file_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.access_token_file
    }
}

impl ::protobuf::Message for LightstepConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collector_cluster)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.access_token_file)?;
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
        if !self.collector_cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.collector_cluster);
        }
        if !self.access_token_file.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.access_token_file);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.collector_cluster.is_empty() {
            os.write_string(1, &self.collector_cluster)?;
        }
        if !self.access_token_file.is_empty() {
            os.write_string(2, &self.access_token_file)?;
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

impl ::protobuf::MessageStatic for LightstepConfig {
    fn new() -> LightstepConfig {
        LightstepConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<LightstepConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collector_cluster",
                    LightstepConfig::get_collector_cluster_for_reflect,
                    LightstepConfig::mut_collector_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_token_file",
                    LightstepConfig::get_access_token_file_for_reflect,
                    LightstepConfig::mut_access_token_file_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LightstepConfig>(
                    "LightstepConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LightstepConfig {
    fn clear(&mut self) {
        self.clear_collector_cluster();
        self.clear_access_token_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LightstepConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LightstepConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZipkinConfig {
    // message fields
    pub collector_cluster: ::std::string::String,
    pub collector_endpoint: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZipkinConfig {}

impl ZipkinConfig {
    pub fn new() -> ZipkinConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZipkinConfig {
        static mut instance: ::protobuf::lazy::Lazy<ZipkinConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZipkinConfig,
        };
        unsafe {
            instance.get(ZipkinConfig::new)
        }
    }

    // string collector_cluster = 1;

    pub fn clear_collector_cluster(&mut self) {
        self.collector_cluster.clear();
    }

    // Param is passed by value, moved
    pub fn set_collector_cluster(&mut self, v: ::std::string::String) {
        self.collector_cluster = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collector_cluster(&mut self) -> &mut ::std::string::String {
        &mut self.collector_cluster
    }

    // Take field
    pub fn take_collector_cluster(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collector_cluster, ::std::string::String::new())
    }

    pub fn get_collector_cluster(&self) -> &str {
        &self.collector_cluster
    }

    fn get_collector_cluster_for_reflect(&self) -> &::std::string::String {
        &self.collector_cluster
    }

    fn mut_collector_cluster_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collector_cluster
    }

    // string collector_endpoint = 2;

    pub fn clear_collector_endpoint(&mut self) {
        self.collector_endpoint.clear();
    }

    // Param is passed by value, moved
    pub fn set_collector_endpoint(&mut self, v: ::std::string::String) {
        self.collector_endpoint = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collector_endpoint(&mut self) -> &mut ::std::string::String {
        &mut self.collector_endpoint
    }

    // Take field
    pub fn take_collector_endpoint(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collector_endpoint, ::std::string::String::new())
    }

    pub fn get_collector_endpoint(&self) -> &str {
        &self.collector_endpoint
    }

    fn get_collector_endpoint_for_reflect(&self) -> &::std::string::String {
        &self.collector_endpoint
    }

    fn mut_collector_endpoint_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collector_endpoint
    }
}

impl ::protobuf::Message for ZipkinConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collector_cluster)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collector_endpoint)?;
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
        if !self.collector_cluster.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.collector_cluster);
        }
        if !self.collector_endpoint.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.collector_endpoint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.collector_cluster.is_empty() {
            os.write_string(1, &self.collector_cluster)?;
        }
        if !self.collector_endpoint.is_empty() {
            os.write_string(2, &self.collector_endpoint)?;
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

impl ::protobuf::MessageStatic for ZipkinConfig {
    fn new() -> ZipkinConfig {
        ZipkinConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZipkinConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collector_cluster",
                    ZipkinConfig::get_collector_cluster_for_reflect,
                    ZipkinConfig::mut_collector_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collector_endpoint",
                    ZipkinConfig::get_collector_endpoint_for_reflect,
                    ZipkinConfig::mut_collector_endpoint_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZipkinConfig>(
                    "ZipkinConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZipkinConfig {
    fn clear(&mut self) {
        self.clear_collector_cluster();
        self.clear_collector_endpoint();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZipkinConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZipkinConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tracing {
    // message fields
    pub http: ::protobuf::SingularPtrField<Tracing_Http>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tracing {}

impl Tracing {
    pub fn new() -> Tracing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tracing {
        static mut instance: ::protobuf::lazy::Lazy<Tracing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tracing,
        };
        unsafe {
            instance.get(Tracing::new)
        }
    }

    // .envoy.api.v2.Tracing.Http http = 1;

    pub fn clear_http(&mut self) {
        self.http.clear();
    }

    pub fn has_http(&self) -> bool {
        self.http.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http(&mut self, v: Tracing_Http) {
        self.http = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http(&mut self) -> &mut Tracing_Http {
        if self.http.is_none() {
            self.http.set_default();
        }
        self.http.as_mut().unwrap()
    }

    // Take field
    pub fn take_http(&mut self) -> Tracing_Http {
        self.http.take().unwrap_or_else(|| Tracing_Http::new())
    }

    pub fn get_http(&self) -> &Tracing_Http {
        self.http.as_ref().unwrap_or_else(|| Tracing_Http::default_instance())
    }

    fn get_http_for_reflect(&self) -> &::protobuf::SingularPtrField<Tracing_Http> {
        &self.http
    }

    fn mut_http_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Tracing_Http> {
        &mut self.http
    }
}

impl ::protobuf::Message for Tracing {
    fn is_initialized(&self) -> bool {
        for v in &self.http {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.http)?;
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
        if let Some(ref v) = self.http.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.http.as_ref() {
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

impl ::protobuf::MessageStatic for Tracing {
    fn new() -> Tracing {
        Tracing::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tracing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tracing_Http>>(
                    "http",
                    Tracing::get_http_for_reflect,
                    Tracing::mut_http_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tracing>(
                    "Tracing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tracing {
    fn clear(&mut self) {
        self.clear_http();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tracing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tracing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Tracing_Http {
    // message fields
    pub name: ::std::string::String,
    pub config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tracing_Http {}

impl Tracing_Http {
    pub fn new() -> Tracing_Http {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tracing_Http {
        static mut instance: ::protobuf::lazy::Lazy<Tracing_Http> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tracing_Http,
        };
        unsafe {
            instance.get(Tracing_Http::new)
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
}

impl ::protobuf::Message for Tracing_Http {
    fn is_initialized(&self) -> bool {
        for v in &self.config {
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

impl ::protobuf::MessageStatic for Tracing_Http {
    fn new() -> Tracing_Http {
        Tracing_Http::new()
    }

    fn descriptor_static(_: ::std::option::Option<Tracing_Http>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Tracing_Http::get_name_for_reflect,
                    Tracing_Http::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "config",
                    Tracing_Http::get_config_for_reflect,
                    Tracing_Http::mut_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Tracing_Http>(
                    "Tracing_Http",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Tracing_Http {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tracing_Http {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tracing_Http {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Admin {
    // message fields
    pub access_log_path: ::std::string::String,
    pub profile_path: ::std::string::String,
    pub address: ::protobuf::SingularPtrField<super::address::Address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Admin {}

impl Admin {
    pub fn new() -> Admin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Admin {
        static mut instance: ::protobuf::lazy::Lazy<Admin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Admin,
        };
        unsafe {
            instance.get(Admin::new)
        }
    }

    // string access_log_path = 1;

    pub fn clear_access_log_path(&mut self) {
        self.access_log_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_access_log_path(&mut self, v: ::std::string::String) {
        self.access_log_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_log_path(&mut self) -> &mut ::std::string::String {
        &mut self.access_log_path
    }

    // Take field
    pub fn take_access_log_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.access_log_path, ::std::string::String::new())
    }

    pub fn get_access_log_path(&self) -> &str {
        &self.access_log_path
    }

    fn get_access_log_path_for_reflect(&self) -> &::std::string::String {
        &self.access_log_path
    }

    fn mut_access_log_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.access_log_path
    }

    // string profile_path = 2;

    pub fn clear_profile_path(&mut self) {
        self.profile_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_profile_path(&mut self, v: ::std::string::String) {
        self.profile_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_profile_path(&mut self) -> &mut ::std::string::String {
        &mut self.profile_path
    }

    // Take field
    pub fn take_profile_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.profile_path, ::std::string::String::new())
    }

    pub fn get_profile_path(&self) -> &str {
        &self.profile_path
    }

    fn get_profile_path_for_reflect(&self) -> &::std::string::String {
        &self.profile_path
    }

    fn mut_profile_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.profile_path
    }

    // .envoy.api.v2.Address address = 3;

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

impl ::protobuf::Message for Admin {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.access_log_path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.profile_path)?;
                },
                3 => {
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
        if !self.access_log_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.access_log_path);
        }
        if !self.profile_path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.profile_path);
        }
        if let Some(ref v) = self.address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.access_log_path.is_empty() {
            os.write_string(1, &self.access_log_path)?;
        }
        if !self.profile_path.is_empty() {
            os.write_string(2, &self.profile_path)?;
        }
        if let Some(ref v) = self.address.as_ref() {
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

impl ::protobuf::MessageStatic for Admin {
    fn new() -> Admin {
        Admin::new()
    }

    fn descriptor_static(_: ::std::option::Option<Admin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_log_path",
                    Admin::get_access_log_path_for_reflect,
                    Admin::mut_access_log_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "profile_path",
                    Admin::get_profile_path_for_reflect,
                    Admin::mut_profile_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                    "address",
                    Admin::get_address_for_reflect,
                    Admin::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Admin>(
                    "Admin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Admin {
    fn clear(&mut self) {
        self.clear_access_log_path();
        self.clear_profile_path();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Admin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Admin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterManager {
    // message fields
    pub local_cluster_name: ::std::string::String,
    pub outlier_detection: ::protobuf::SingularPtrField<ClusterManager_OutlierDetection>,
    pub upstream_bind_config: ::protobuf::SingularPtrField<super::address::BindConfig>,
    pub load_stats_config: ::protobuf::SingularPtrField<super::base::ApiConfigSource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterManager {}

impl ClusterManager {
    pub fn new() -> ClusterManager {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterManager {
        static mut instance: ::protobuf::lazy::Lazy<ClusterManager> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterManager,
        };
        unsafe {
            instance.get(ClusterManager::new)
        }
    }

    // string local_cluster_name = 1;

    pub fn clear_local_cluster_name(&mut self) {
        self.local_cluster_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_local_cluster_name(&mut self, v: ::std::string::String) {
        self.local_cluster_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_local_cluster_name(&mut self) -> &mut ::std::string::String {
        &mut self.local_cluster_name
    }

    // Take field
    pub fn take_local_cluster_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.local_cluster_name, ::std::string::String::new())
    }

    pub fn get_local_cluster_name(&self) -> &str {
        &self.local_cluster_name
    }

    fn get_local_cluster_name_for_reflect(&self) -> &::std::string::String {
        &self.local_cluster_name
    }

    fn mut_local_cluster_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.local_cluster_name
    }

    // .envoy.api.v2.ClusterManager.OutlierDetection outlier_detection = 2;

    pub fn clear_outlier_detection(&mut self) {
        self.outlier_detection.clear();
    }

    pub fn has_outlier_detection(&self) -> bool {
        self.outlier_detection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outlier_detection(&mut self, v: ClusterManager_OutlierDetection) {
        self.outlier_detection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outlier_detection(&mut self) -> &mut ClusterManager_OutlierDetection {
        if self.outlier_detection.is_none() {
            self.outlier_detection.set_default();
        }
        self.outlier_detection.as_mut().unwrap()
    }

    // Take field
    pub fn take_outlier_detection(&mut self) -> ClusterManager_OutlierDetection {
        self.outlier_detection.take().unwrap_or_else(|| ClusterManager_OutlierDetection::new())
    }

    pub fn get_outlier_detection(&self) -> &ClusterManager_OutlierDetection {
        self.outlier_detection.as_ref().unwrap_or_else(|| ClusterManager_OutlierDetection::default_instance())
    }

    fn get_outlier_detection_for_reflect(&self) -> &::protobuf::SingularPtrField<ClusterManager_OutlierDetection> {
        &self.outlier_detection
    }

    fn mut_outlier_detection_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClusterManager_OutlierDetection> {
        &mut self.outlier_detection
    }

    // .envoy.api.v2.BindConfig upstream_bind_config = 3;

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

    // .envoy.api.v2.ApiConfigSource load_stats_config = 4;

    pub fn clear_load_stats_config(&mut self) {
        self.load_stats_config.clear();
    }

    pub fn has_load_stats_config(&self) -> bool {
        self.load_stats_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_stats_config(&mut self, v: super::base::ApiConfigSource) {
        self.load_stats_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_stats_config(&mut self) -> &mut super::base::ApiConfigSource {
        if self.load_stats_config.is_none() {
            self.load_stats_config.set_default();
        }
        self.load_stats_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_stats_config(&mut self) -> super::base::ApiConfigSource {
        self.load_stats_config.take().unwrap_or_else(|| super::base::ApiConfigSource::new())
    }

    pub fn get_load_stats_config(&self) -> &super::base::ApiConfigSource {
        self.load_stats_config.as_ref().unwrap_or_else(|| super::base::ApiConfigSource::default_instance())
    }

    fn get_load_stats_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ApiConfigSource> {
        &self.load_stats_config
    }

    fn mut_load_stats_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ApiConfigSource> {
        &mut self.load_stats_config
    }
}

impl ::protobuf::Message for ClusterManager {
    fn is_initialized(&self) -> bool {
        for v in &self.outlier_detection {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upstream_bind_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.load_stats_config {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.local_cluster_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.outlier_detection)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upstream_bind_config)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_stats_config)?;
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
        if !self.local_cluster_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.local_cluster_name);
        }
        if let Some(ref v) = self.outlier_detection.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.upstream_bind_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.load_stats_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.local_cluster_name.is_empty() {
            os.write_string(1, &self.local_cluster_name)?;
        }
        if let Some(ref v) = self.outlier_detection.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.upstream_bind_config.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.load_stats_config.as_ref() {
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

impl ::protobuf::MessageStatic for ClusterManager {
    fn new() -> ClusterManager {
        ClusterManager::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterManager>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "local_cluster_name",
                    ClusterManager::get_local_cluster_name_for_reflect,
                    ClusterManager::mut_local_cluster_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterManager_OutlierDetection>>(
                    "outlier_detection",
                    ClusterManager::get_outlier_detection_for_reflect,
                    ClusterManager::mut_outlier_detection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::BindConfig>>(
                    "upstream_bind_config",
                    ClusterManager::get_upstream_bind_config_for_reflect,
                    ClusterManager::mut_upstream_bind_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ApiConfigSource>>(
                    "load_stats_config",
                    ClusterManager::get_load_stats_config_for_reflect,
                    ClusterManager::mut_load_stats_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterManager>(
                    "ClusterManager",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterManager {
    fn clear(&mut self) {
        self.clear_local_cluster_name();
        self.clear_outlier_detection();
        self.clear_upstream_bind_config();
        self.clear_load_stats_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterManager {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterManager_OutlierDetection {
    // message fields
    pub event_log_path: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterManager_OutlierDetection {}

impl ClusterManager_OutlierDetection {
    pub fn new() -> ClusterManager_OutlierDetection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterManager_OutlierDetection {
        static mut instance: ::protobuf::lazy::Lazy<ClusterManager_OutlierDetection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterManager_OutlierDetection,
        };
        unsafe {
            instance.get(ClusterManager_OutlierDetection::new)
        }
    }

    // string event_log_path = 1;

    pub fn clear_event_log_path(&mut self) {
        self.event_log_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_log_path(&mut self, v: ::std::string::String) {
        self.event_log_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_log_path(&mut self) -> &mut ::std::string::String {
        &mut self.event_log_path
    }

    // Take field
    pub fn take_event_log_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.event_log_path, ::std::string::String::new())
    }

    pub fn get_event_log_path(&self) -> &str {
        &self.event_log_path
    }

    fn get_event_log_path_for_reflect(&self) -> &::std::string::String {
        &self.event_log_path
    }

    fn mut_event_log_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.event_log_path
    }
}

impl ::protobuf::Message for ClusterManager_OutlierDetection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.event_log_path)?;
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
        if !self.event_log_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.event_log_path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.event_log_path.is_empty() {
            os.write_string(1, &self.event_log_path)?;
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

impl ::protobuf::MessageStatic for ClusterManager_OutlierDetection {
    fn new() -> ClusterManager_OutlierDetection {
        ClusterManager_OutlierDetection::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterManager_OutlierDetection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_log_path",
                    ClusterManager_OutlierDetection::get_event_log_path_for_reflect,
                    ClusterManager_OutlierDetection::mut_event_log_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterManager_OutlierDetection>(
                    "ClusterManager_OutlierDetection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterManager_OutlierDetection {
    fn clear(&mut self) {
        self.clear_event_log_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterManager_OutlierDetection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterManager_OutlierDetection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatsdSink {
    // message oneof groups
    statsd_specifier: ::std::option::Option<StatsdSink_oneof_statsd_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsdSink {}

#[derive(Clone,PartialEq)]
pub enum StatsdSink_oneof_statsd_specifier {
    address(super::address::Address),
    tcp_cluster_name(::std::string::String),
}

impl StatsdSink {
    pub fn new() -> StatsdSink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsdSink {
        static mut instance: ::protobuf::lazy::Lazy<StatsdSink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsdSink,
        };
        unsafe {
            instance.get(StatsdSink::new)
        }
    }

    // .envoy.api.v2.Address address = 1;

    pub fn clear_address(&mut self) {
        self.statsd_specifier = ::std::option::Option::None;
    }

    pub fn has_address(&self) -> bool {
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: super::address::Address) {
        self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(v))
    }

    // Mutable pointer to the field.
    pub fn mut_address(&mut self) -> &mut super::address::Address {
        if let ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(_)) = self.statsd_specifier {
        } else {
            self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(super::address::Address::new()));
        }
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_address(&mut self) -> super::address::Address {
        if self.has_address() {
            match self.statsd_specifier.take() {
                ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(v)) => v,
                _ => panic!(),
            }
        } else {
            super::address::Address::new()
        }
    }

    pub fn get_address(&self) -> &super::address::Address {
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(ref v)) => v,
            _ => super::address::Address::default_instance(),
        }
    }

    // string tcp_cluster_name = 2;

    pub fn clear_tcp_cluster_name(&mut self) {
        self.statsd_specifier = ::std::option::Option::None;
    }

    pub fn has_tcp_cluster_name(&self) -> bool {
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tcp_cluster_name(&mut self, v: ::std::string::String) {
        self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tcp_cluster_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(_)) = self.statsd_specifier {
        } else {
            self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(::std::string::String::new()));
        }
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tcp_cluster_name(&mut self) -> ::std::string::String {
        if self.has_tcp_cluster_name() {
            match self.statsd_specifier.take() {
                ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_tcp_cluster_name(&self) -> &str {
        match self.statsd_specifier {
            ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for StatsdSink {
    fn is_initialized(&self) -> bool {
        if let Some(StatsdSink_oneof_statsd_specifier::address(ref v)) = self.statsd_specifier {
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
                    self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::address(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.statsd_specifier = ::std::option::Option::Some(StatsdSink_oneof_statsd_specifier::tcp_cluster_name(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.statsd_specifier {
            match v {
                &StatsdSink_oneof_statsd_specifier::address(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StatsdSink_oneof_statsd_specifier::tcp_cluster_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.statsd_specifier {
            match v {
                &StatsdSink_oneof_statsd_specifier::address(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &StatsdSink_oneof_statsd_specifier::tcp_cluster_name(ref v) => {
                    os.write_string(2, v)?;
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

impl ::protobuf::MessageStatic for StatsdSink {
    fn new() -> StatsdSink {
        StatsdSink::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsdSink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::address::Address>(
                    "address",
                    StatsdSink::has_address,
                    StatsdSink::get_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "tcp_cluster_name",
                    StatsdSink::has_tcp_cluster_name,
                    StatsdSink::get_tcp_cluster_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatsdSink>(
                    "StatsdSink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsdSink {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_tcp_cluster_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatsdSink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatsdSink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatsSink {
    // message fields
    pub name: ::std::string::String,
    pub config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsSink {}

impl StatsSink {
    pub fn new() -> StatsSink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsSink {
        static mut instance: ::protobuf::lazy::Lazy<StatsSink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsSink,
        };
        unsafe {
            instance.get(StatsSink::new)
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
}

impl ::protobuf::Message for StatsSink {
    fn is_initialized(&self) -> bool {
        for v in &self.config {
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

impl ::protobuf::MessageStatic for StatsSink {
    fn new() -> StatsSink {
        StatsSink::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsSink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    StatsSink::get_name_for_reflect,
                    StatsSink::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                    "config",
                    StatsSink::get_config_for_reflect,
                    StatsSink::mut_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatsSink>(
                    "StatsSink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsSink {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatsSink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatsSink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TagSpecifier {
    // message fields
    pub tag_name: ::std::string::String,
    pub regex: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TagSpecifier {}

impl TagSpecifier {
    pub fn new() -> TagSpecifier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TagSpecifier {
        static mut instance: ::protobuf::lazy::Lazy<TagSpecifier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TagSpecifier,
        };
        unsafe {
            instance.get(TagSpecifier::new)
        }
    }

    // string tag_name = 1;

    pub fn clear_tag_name(&mut self) {
        self.tag_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_tag_name(&mut self, v: ::std::string::String) {
        self.tag_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag_name(&mut self) -> &mut ::std::string::String {
        &mut self.tag_name
    }

    // Take field
    pub fn take_tag_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tag_name, ::std::string::String::new())
    }

    pub fn get_tag_name(&self) -> &str {
        &self.tag_name
    }

    fn get_tag_name_for_reflect(&self) -> &::std::string::String {
        &self.tag_name
    }

    fn mut_tag_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tag_name
    }

    // string regex = 2;

    pub fn clear_regex(&mut self) {
        self.regex.clear();
    }

    // Param is passed by value, moved
    pub fn set_regex(&mut self, v: ::std::string::String) {
        self.regex = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_regex(&mut self) -> &mut ::std::string::String {
        &mut self.regex
    }

    // Take field
    pub fn take_regex(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.regex, ::std::string::String::new())
    }

    pub fn get_regex(&self) -> &str {
        &self.regex
    }

    fn get_regex_for_reflect(&self) -> &::std::string::String {
        &self.regex
    }

    fn mut_regex_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.regex
    }
}

impl ::protobuf::Message for TagSpecifier {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tag_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.regex)?;
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
        if !self.tag_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tag_name);
        }
        if !self.regex.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.regex);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tag_name.is_empty() {
            os.write_string(1, &self.tag_name)?;
        }
        if !self.regex.is_empty() {
            os.write_string(2, &self.regex)?;
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

impl ::protobuf::MessageStatic for TagSpecifier {
    fn new() -> TagSpecifier {
        TagSpecifier::new()
    }

    fn descriptor_static(_: ::std::option::Option<TagSpecifier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag_name",
                    TagSpecifier::get_tag_name_for_reflect,
                    TagSpecifier::mut_tag_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "regex",
                    TagSpecifier::get_regex_for_reflect,
                    TagSpecifier::mut_regex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TagSpecifier>(
                    "TagSpecifier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TagSpecifier {
    fn clear(&mut self) {
        self.clear_tag_name();
        self.clear_regex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TagSpecifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TagSpecifier {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatsConfig {
    // message fields
    pub stats_tags: ::protobuf::RepeatedField<TagSpecifier>,
    pub use_all_default_tags: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatsConfig {}

impl StatsConfig {
    pub fn new() -> StatsConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatsConfig {
        static mut instance: ::protobuf::lazy::Lazy<StatsConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatsConfig,
        };
        unsafe {
            instance.get(StatsConfig::new)
        }
    }

    // repeated .envoy.api.v2.TagSpecifier stats_tags = 1;

    pub fn clear_stats_tags(&mut self) {
        self.stats_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats_tags(&mut self, v: ::protobuf::RepeatedField<TagSpecifier>) {
        self.stats_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats_tags(&mut self) -> &mut ::protobuf::RepeatedField<TagSpecifier> {
        &mut self.stats_tags
    }

    // Take field
    pub fn take_stats_tags(&mut self) -> ::protobuf::RepeatedField<TagSpecifier> {
        ::std::mem::replace(&mut self.stats_tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_stats_tags(&self) -> &[TagSpecifier] {
        &self.stats_tags
    }

    fn get_stats_tags_for_reflect(&self) -> &::protobuf::RepeatedField<TagSpecifier> {
        &self.stats_tags
    }

    fn mut_stats_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TagSpecifier> {
        &mut self.stats_tags
    }

    // .google.protobuf.BoolValue use_all_default_tags = 2;

    pub fn clear_use_all_default_tags(&mut self) {
        self.use_all_default_tags.clear();
    }

    pub fn has_use_all_default_tags(&self) -> bool {
        self.use_all_default_tags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_all_default_tags(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.use_all_default_tags = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_use_all_default_tags(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.use_all_default_tags.is_none() {
            self.use_all_default_tags.set_default();
        }
        self.use_all_default_tags.as_mut().unwrap()
    }

    // Take field
    pub fn take_use_all_default_tags(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.use_all_default_tags.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_use_all_default_tags(&self) -> &::protobuf::well_known_types::BoolValue {
        self.use_all_default_tags.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_use_all_default_tags_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.use_all_default_tags
    }

    fn mut_use_all_default_tags_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.use_all_default_tags
    }
}

impl ::protobuf::Message for StatsConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.stats_tags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.use_all_default_tags {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats_tags)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.use_all_default_tags)?;
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
        for value in &self.stats_tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.use_all_default_tags.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.stats_tags {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.use_all_default_tags.as_ref() {
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

impl ::protobuf::MessageStatic for StatsConfig {
    fn new() -> StatsConfig {
        StatsConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatsConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TagSpecifier>>(
                    "stats_tags",
                    StatsConfig::get_stats_tags_for_reflect,
                    StatsConfig::mut_stats_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "use_all_default_tags",
                    StatsConfig::get_use_all_default_tags_for_reflect,
                    StatsConfig::mut_use_all_default_tags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatsConfig>(
                    "StatsConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatsConfig {
    fn clear(&mut self) {
        self.clear_stats_tags();
        self.clear_use_all_default_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatsConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatsConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Watchdog {
    // message fields
    pub miss_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub megamiss_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub kill_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub multikill_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Watchdog {}

impl Watchdog {
    pub fn new() -> Watchdog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Watchdog {
        static mut instance: ::protobuf::lazy::Lazy<Watchdog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Watchdog,
        };
        unsafe {
            instance.get(Watchdog::new)
        }
    }

    // .google.protobuf.Duration miss_timeout = 1;

    pub fn clear_miss_timeout(&mut self) {
        self.miss_timeout.clear();
    }

    pub fn has_miss_timeout(&self) -> bool {
        self.miss_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_miss_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.miss_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_miss_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.miss_timeout.is_none() {
            self.miss_timeout.set_default();
        }
        self.miss_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_miss_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.miss_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_miss_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.miss_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_miss_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.miss_timeout
    }

    fn mut_miss_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.miss_timeout
    }

    // .google.protobuf.Duration megamiss_timeout = 2;

    pub fn clear_megamiss_timeout(&mut self) {
        self.megamiss_timeout.clear();
    }

    pub fn has_megamiss_timeout(&self) -> bool {
        self.megamiss_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_megamiss_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.megamiss_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_megamiss_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.megamiss_timeout.is_none() {
            self.megamiss_timeout.set_default();
        }
        self.megamiss_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_megamiss_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.megamiss_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_megamiss_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.megamiss_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_megamiss_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.megamiss_timeout
    }

    fn mut_megamiss_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.megamiss_timeout
    }

    // .google.protobuf.Duration kill_timeout = 3;

    pub fn clear_kill_timeout(&mut self) {
        self.kill_timeout.clear();
    }

    pub fn has_kill_timeout(&self) -> bool {
        self.kill_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kill_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.kill_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kill_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.kill_timeout.is_none() {
            self.kill_timeout.set_default();
        }
        self.kill_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_kill_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.kill_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_kill_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.kill_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_kill_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.kill_timeout
    }

    fn mut_kill_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.kill_timeout
    }

    // .google.protobuf.Duration multikill_timeout = 4;

    pub fn clear_multikill_timeout(&mut self) {
        self.multikill_timeout.clear();
    }

    pub fn has_multikill_timeout(&self) -> bool {
        self.multikill_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_multikill_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.multikill_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_multikill_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.multikill_timeout.is_none() {
            self.multikill_timeout.set_default();
        }
        self.multikill_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_multikill_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.multikill_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_multikill_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.multikill_timeout.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_multikill_timeout_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.multikill_timeout
    }

    fn mut_multikill_timeout_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.multikill_timeout
    }
}

impl ::protobuf::Message for Watchdog {
    fn is_initialized(&self) -> bool {
        for v in &self.miss_timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.megamiss_timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kill_timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.multikill_timeout {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.miss_timeout)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.megamiss_timeout)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kill_timeout)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.multikill_timeout)?;
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
        if let Some(ref v) = self.miss_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.megamiss_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.kill_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.multikill_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.miss_timeout.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.megamiss_timeout.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.kill_timeout.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.multikill_timeout.as_ref() {
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

impl ::protobuf::MessageStatic for Watchdog {
    fn new() -> Watchdog {
        Watchdog::new()
    }

    fn descriptor_static(_: ::std::option::Option<Watchdog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "miss_timeout",
                    Watchdog::get_miss_timeout_for_reflect,
                    Watchdog::mut_miss_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "megamiss_timeout",
                    Watchdog::get_megamiss_timeout_for_reflect,
                    Watchdog::mut_megamiss_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "kill_timeout",
                    Watchdog::get_kill_timeout_for_reflect,
                    Watchdog::mut_kill_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "multikill_timeout",
                    Watchdog::get_multikill_timeout_for_reflect,
                    Watchdog::mut_multikill_timeout_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Watchdog>(
                    "Watchdog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Watchdog {
    fn clear(&mut self) {
        self.clear_miss_timeout();
        self.clear_megamiss_timeout();
        self.clear_kill_timeout();
        self.clear_multikill_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Watchdog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Watchdog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Runtime {
    // message fields
    pub symlink_root: ::std::string::String,
    pub subdirectory: ::std::string::String,
    pub override_subdirectory: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Runtime {}

impl Runtime {
    pub fn new() -> Runtime {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Runtime {
        static mut instance: ::protobuf::lazy::Lazy<Runtime> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Runtime,
        };
        unsafe {
            instance.get(Runtime::new)
        }
    }

    // string symlink_root = 1;

    pub fn clear_symlink_root(&mut self) {
        self.symlink_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_symlink_root(&mut self, v: ::std::string::String) {
        self.symlink_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink_root(&mut self) -> &mut ::std::string::String {
        &mut self.symlink_root
    }

    // Take field
    pub fn take_symlink_root(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.symlink_root, ::std::string::String::new())
    }

    pub fn get_symlink_root(&self) -> &str {
        &self.symlink_root
    }

    fn get_symlink_root_for_reflect(&self) -> &::std::string::String {
        &self.symlink_root
    }

    fn mut_symlink_root_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.symlink_root
    }

    // string subdirectory = 2;

    pub fn clear_subdirectory(&mut self) {
        self.subdirectory.clear();
    }

    // Param is passed by value, moved
    pub fn set_subdirectory(&mut self, v: ::std::string::String) {
        self.subdirectory = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subdirectory(&mut self) -> &mut ::std::string::String {
        &mut self.subdirectory
    }

    // Take field
    pub fn take_subdirectory(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subdirectory, ::std::string::String::new())
    }

    pub fn get_subdirectory(&self) -> &str {
        &self.subdirectory
    }

    fn get_subdirectory_for_reflect(&self) -> &::std::string::String {
        &self.subdirectory
    }

    fn mut_subdirectory_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.subdirectory
    }

    // string override_subdirectory = 3;

    pub fn clear_override_subdirectory(&mut self) {
        self.override_subdirectory.clear();
    }

    // Param is passed by value, moved
    pub fn set_override_subdirectory(&mut self, v: ::std::string::String) {
        self.override_subdirectory = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_override_subdirectory(&mut self) -> &mut ::std::string::String {
        &mut self.override_subdirectory
    }

    // Take field
    pub fn take_override_subdirectory(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.override_subdirectory, ::std::string::String::new())
    }

    pub fn get_override_subdirectory(&self) -> &str {
        &self.override_subdirectory
    }

    fn get_override_subdirectory_for_reflect(&self) -> &::std::string::String {
        &self.override_subdirectory
    }

    fn mut_override_subdirectory_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.override_subdirectory
    }
}

impl ::protobuf::Message for Runtime {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.symlink_root)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subdirectory)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.override_subdirectory)?;
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
        if !self.symlink_root.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.symlink_root);
        }
        if !self.subdirectory.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.subdirectory);
        }
        if !self.override_subdirectory.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.override_subdirectory);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.symlink_root.is_empty() {
            os.write_string(1, &self.symlink_root)?;
        }
        if !self.subdirectory.is_empty() {
            os.write_string(2, &self.subdirectory)?;
        }
        if !self.override_subdirectory.is_empty() {
            os.write_string(3, &self.override_subdirectory)?;
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

impl ::protobuf::MessageStatic for Runtime {
    fn new() -> Runtime {
        Runtime::new()
    }

    fn descriptor_static(_: ::std::option::Option<Runtime>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "symlink_root",
                    Runtime::get_symlink_root_for_reflect,
                    Runtime::mut_symlink_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subdirectory",
                    Runtime::get_subdirectory_for_reflect,
                    Runtime::mut_subdirectory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "override_subdirectory",
                    Runtime::get_override_subdirectory_for_reflect,
                    Runtime::mut_override_subdirectory_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Runtime>(
                    "Runtime",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Runtime {
    fn clear(&mut self) {
        self.clear_symlink_root();
        self.clear_subdirectory();
        self.clear_override_subdirectory();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Runtime {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitServiceConfig {
    // message fields
    pub cluster_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitServiceConfig {}

impl RateLimitServiceConfig {
    pub fn new() -> RateLimitServiceConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitServiceConfig {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitServiceConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitServiceConfig,
        };
        unsafe {
            instance.get(RateLimitServiceConfig::new)
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
}

impl ::protobuf::Message for RateLimitServiceConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cluster_name)?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cluster_name.is_empty() {
            os.write_string(1, &self.cluster_name)?;
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

impl ::protobuf::MessageStatic for RateLimitServiceConfig {
    fn new() -> RateLimitServiceConfig {
        RateLimitServiceConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitServiceConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cluster_name",
                    RateLimitServiceConfig::get_cluster_name_for_reflect,
                    RateLimitServiceConfig::mut_cluster_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitServiceConfig>(
                    "RateLimitServiceConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitServiceConfig {
    fn clear(&mut self) {
        self.clear_cluster_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitServiceConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitServiceConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bootstrap {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub static_resources: ::protobuf::SingularPtrField<Bootstrap_StaticResources>,
    pub dynamic_resources: ::protobuf::SingularPtrField<Bootstrap_DynamicResources>,
    pub cluster_manager: ::protobuf::SingularPtrField<ClusterManager>,
    pub flags_path: ::std::string::String,
    pub stats_sinks: ::protobuf::RepeatedField<StatsSink>,
    pub stats_config: ::protobuf::SingularPtrField<StatsConfig>,
    pub stats_flush_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub watchdog: ::protobuf::SingularPtrField<Watchdog>,
    pub tracing: ::protobuf::SingularPtrField<Tracing>,
    pub rate_limit_service: ::protobuf::SingularPtrField<RateLimitServiceConfig>,
    pub runtime: ::protobuf::SingularPtrField<Runtime>,
    pub admin: ::protobuf::SingularPtrField<Admin>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bootstrap {}

impl Bootstrap {
    pub fn new() -> Bootstrap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bootstrap {
        static mut instance: ::protobuf::lazy::Lazy<Bootstrap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bootstrap,
        };
        unsafe {
            instance.get(Bootstrap::new)
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

    // .envoy.api.v2.Bootstrap.StaticResources static_resources = 2;

    pub fn clear_static_resources(&mut self) {
        self.static_resources.clear();
    }

    pub fn has_static_resources(&self) -> bool {
        self.static_resources.is_some()
    }

    // Param is passed by value, moved
    pub fn set_static_resources(&mut self, v: Bootstrap_StaticResources) {
        self.static_resources = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_static_resources(&mut self) -> &mut Bootstrap_StaticResources {
        if self.static_resources.is_none() {
            self.static_resources.set_default();
        }
        self.static_resources.as_mut().unwrap()
    }

    // Take field
    pub fn take_static_resources(&mut self) -> Bootstrap_StaticResources {
        self.static_resources.take().unwrap_or_else(|| Bootstrap_StaticResources::new())
    }

    pub fn get_static_resources(&self) -> &Bootstrap_StaticResources {
        self.static_resources.as_ref().unwrap_or_else(|| Bootstrap_StaticResources::default_instance())
    }

    fn get_static_resources_for_reflect(&self) -> &::protobuf::SingularPtrField<Bootstrap_StaticResources> {
        &self.static_resources
    }

    fn mut_static_resources_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Bootstrap_StaticResources> {
        &mut self.static_resources
    }

    // .envoy.api.v2.Bootstrap.DynamicResources dynamic_resources = 3;

    pub fn clear_dynamic_resources(&mut self) {
        self.dynamic_resources.clear();
    }

    pub fn has_dynamic_resources(&self) -> bool {
        self.dynamic_resources.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dynamic_resources(&mut self, v: Bootstrap_DynamicResources) {
        self.dynamic_resources = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dynamic_resources(&mut self) -> &mut Bootstrap_DynamicResources {
        if self.dynamic_resources.is_none() {
            self.dynamic_resources.set_default();
        }
        self.dynamic_resources.as_mut().unwrap()
    }

    // Take field
    pub fn take_dynamic_resources(&mut self) -> Bootstrap_DynamicResources {
        self.dynamic_resources.take().unwrap_or_else(|| Bootstrap_DynamicResources::new())
    }

    pub fn get_dynamic_resources(&self) -> &Bootstrap_DynamicResources {
        self.dynamic_resources.as_ref().unwrap_or_else(|| Bootstrap_DynamicResources::default_instance())
    }

    fn get_dynamic_resources_for_reflect(&self) -> &::protobuf::SingularPtrField<Bootstrap_DynamicResources> {
        &self.dynamic_resources
    }

    fn mut_dynamic_resources_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Bootstrap_DynamicResources> {
        &mut self.dynamic_resources
    }

    // .envoy.api.v2.ClusterManager cluster_manager = 4;

    pub fn clear_cluster_manager(&mut self) {
        self.cluster_manager.clear();
    }

    pub fn has_cluster_manager(&self) -> bool {
        self.cluster_manager.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_manager(&mut self, v: ClusterManager) {
        self.cluster_manager = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_manager(&mut self) -> &mut ClusterManager {
        if self.cluster_manager.is_none() {
            self.cluster_manager.set_default();
        }
        self.cluster_manager.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_manager(&mut self) -> ClusterManager {
        self.cluster_manager.take().unwrap_or_else(|| ClusterManager::new())
    }

    pub fn get_cluster_manager(&self) -> &ClusterManager {
        self.cluster_manager.as_ref().unwrap_or_else(|| ClusterManager::default_instance())
    }

    fn get_cluster_manager_for_reflect(&self) -> &::protobuf::SingularPtrField<ClusterManager> {
        &self.cluster_manager
    }

    fn mut_cluster_manager_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClusterManager> {
        &mut self.cluster_manager
    }

    // string flags_path = 5;

    pub fn clear_flags_path(&mut self) {
        self.flags_path.clear();
    }

    // Param is passed by value, moved
    pub fn set_flags_path(&mut self, v: ::std::string::String) {
        self.flags_path = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_flags_path(&mut self) -> &mut ::std::string::String {
        &mut self.flags_path
    }

    // Take field
    pub fn take_flags_path(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.flags_path, ::std::string::String::new())
    }

    pub fn get_flags_path(&self) -> &str {
        &self.flags_path
    }

    fn get_flags_path_for_reflect(&self) -> &::std::string::String {
        &self.flags_path
    }

    fn mut_flags_path_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.flags_path
    }

    // repeated .envoy.api.v2.StatsSink stats_sinks = 6;

    pub fn clear_stats_sinks(&mut self) {
        self.stats_sinks.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats_sinks(&mut self, v: ::protobuf::RepeatedField<StatsSink>) {
        self.stats_sinks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats_sinks(&mut self) -> &mut ::protobuf::RepeatedField<StatsSink> {
        &mut self.stats_sinks
    }

    // Take field
    pub fn take_stats_sinks(&mut self) -> ::protobuf::RepeatedField<StatsSink> {
        ::std::mem::replace(&mut self.stats_sinks, ::protobuf::RepeatedField::new())
    }

    pub fn get_stats_sinks(&self) -> &[StatsSink] {
        &self.stats_sinks
    }

    fn get_stats_sinks_for_reflect(&self) -> &::protobuf::RepeatedField<StatsSink> {
        &self.stats_sinks
    }

    fn mut_stats_sinks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StatsSink> {
        &mut self.stats_sinks
    }

    // .envoy.api.v2.StatsConfig stats_config = 13;

    pub fn clear_stats_config(&mut self) {
        self.stats_config.clear();
    }

    pub fn has_stats_config(&self) -> bool {
        self.stats_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats_config(&mut self, v: StatsConfig) {
        self.stats_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats_config(&mut self) -> &mut StatsConfig {
        if self.stats_config.is_none() {
            self.stats_config.set_default();
        }
        self.stats_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats_config(&mut self) -> StatsConfig {
        self.stats_config.take().unwrap_or_else(|| StatsConfig::new())
    }

    pub fn get_stats_config(&self) -> &StatsConfig {
        self.stats_config.as_ref().unwrap_or_else(|| StatsConfig::default_instance())
    }

    fn get_stats_config_for_reflect(&self) -> &::protobuf::SingularPtrField<StatsConfig> {
        &self.stats_config
    }

    fn mut_stats_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatsConfig> {
        &mut self.stats_config
    }

    // .google.protobuf.Duration stats_flush_interval = 7;

    pub fn clear_stats_flush_interval(&mut self) {
        self.stats_flush_interval.clear();
    }

    pub fn has_stats_flush_interval(&self) -> bool {
        self.stats_flush_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats_flush_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.stats_flush_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats_flush_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.stats_flush_interval.is_none() {
            self.stats_flush_interval.set_default();
        }
        self.stats_flush_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats_flush_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.stats_flush_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_stats_flush_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.stats_flush_interval.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_stats_flush_interval_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.stats_flush_interval
    }

    fn mut_stats_flush_interval_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.stats_flush_interval
    }

    // .envoy.api.v2.Watchdog watchdog = 8;

    pub fn clear_watchdog(&mut self) {
        self.watchdog.clear();
    }

    pub fn has_watchdog(&self) -> bool {
        self.watchdog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watchdog(&mut self, v: Watchdog) {
        self.watchdog = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_watchdog(&mut self) -> &mut Watchdog {
        if self.watchdog.is_none() {
            self.watchdog.set_default();
        }
        self.watchdog.as_mut().unwrap()
    }

    // Take field
    pub fn take_watchdog(&mut self) -> Watchdog {
        self.watchdog.take().unwrap_or_else(|| Watchdog::new())
    }

    pub fn get_watchdog(&self) -> &Watchdog {
        self.watchdog.as_ref().unwrap_or_else(|| Watchdog::default_instance())
    }

    fn get_watchdog_for_reflect(&self) -> &::protobuf::SingularPtrField<Watchdog> {
        &self.watchdog
    }

    fn mut_watchdog_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Watchdog> {
        &mut self.watchdog
    }

    // .envoy.api.v2.Tracing tracing = 9;

    pub fn clear_tracing(&mut self) {
        self.tracing.clear();
    }

    pub fn has_tracing(&self) -> bool {
        self.tracing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tracing(&mut self, v: Tracing) {
        self.tracing = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tracing(&mut self) -> &mut Tracing {
        if self.tracing.is_none() {
            self.tracing.set_default();
        }
        self.tracing.as_mut().unwrap()
    }

    // Take field
    pub fn take_tracing(&mut self) -> Tracing {
        self.tracing.take().unwrap_or_else(|| Tracing::new())
    }

    pub fn get_tracing(&self) -> &Tracing {
        self.tracing.as_ref().unwrap_or_else(|| Tracing::default_instance())
    }

    fn get_tracing_for_reflect(&self) -> &::protobuf::SingularPtrField<Tracing> {
        &self.tracing
    }

    fn mut_tracing_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Tracing> {
        &mut self.tracing
    }

    // .envoy.api.v2.RateLimitServiceConfig rate_limit_service = 10;

    pub fn clear_rate_limit_service(&mut self) {
        self.rate_limit_service.clear();
    }

    pub fn has_rate_limit_service(&self) -> bool {
        self.rate_limit_service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rate_limit_service(&mut self, v: RateLimitServiceConfig) {
        self.rate_limit_service = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rate_limit_service(&mut self) -> &mut RateLimitServiceConfig {
        if self.rate_limit_service.is_none() {
            self.rate_limit_service.set_default();
        }
        self.rate_limit_service.as_mut().unwrap()
    }

    // Take field
    pub fn take_rate_limit_service(&mut self) -> RateLimitServiceConfig {
        self.rate_limit_service.take().unwrap_or_else(|| RateLimitServiceConfig::new())
    }

    pub fn get_rate_limit_service(&self) -> &RateLimitServiceConfig {
        self.rate_limit_service.as_ref().unwrap_or_else(|| RateLimitServiceConfig::default_instance())
    }

    fn get_rate_limit_service_for_reflect(&self) -> &::protobuf::SingularPtrField<RateLimitServiceConfig> {
        &self.rate_limit_service
    }

    fn mut_rate_limit_service_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RateLimitServiceConfig> {
        &mut self.rate_limit_service
    }

    // .envoy.api.v2.Runtime runtime = 11;

    pub fn clear_runtime(&mut self) {
        self.runtime.clear();
    }

    pub fn has_runtime(&self) -> bool {
        self.runtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_runtime(&mut self, v: Runtime) {
        self.runtime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_runtime(&mut self) -> &mut Runtime {
        if self.runtime.is_none() {
            self.runtime.set_default();
        }
        self.runtime.as_mut().unwrap()
    }

    // Take field
    pub fn take_runtime(&mut self) -> Runtime {
        self.runtime.take().unwrap_or_else(|| Runtime::new())
    }

    pub fn get_runtime(&self) -> &Runtime {
        self.runtime.as_ref().unwrap_or_else(|| Runtime::default_instance())
    }

    fn get_runtime_for_reflect(&self) -> &::protobuf::SingularPtrField<Runtime> {
        &self.runtime
    }

    fn mut_runtime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Runtime> {
        &mut self.runtime
    }

    // .envoy.api.v2.Admin admin = 12;

    pub fn clear_admin(&mut self) {
        self.admin.clear();
    }

    pub fn has_admin(&self) -> bool {
        self.admin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin(&mut self, v: Admin) {
        self.admin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin(&mut self) -> &mut Admin {
        if self.admin.is_none() {
            self.admin.set_default();
        }
        self.admin.as_mut().unwrap()
    }

    // Take field
    pub fn take_admin(&mut self) -> Admin {
        self.admin.take().unwrap_or_else(|| Admin::new())
    }

    pub fn get_admin(&self) -> &Admin {
        self.admin.as_ref().unwrap_or_else(|| Admin::default_instance())
    }

    fn get_admin_for_reflect(&self) -> &::protobuf::SingularPtrField<Admin> {
        &self.admin
    }

    fn mut_admin_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Admin> {
        &mut self.admin
    }
}

impl ::protobuf::Message for Bootstrap {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.static_resources {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dynamic_resources {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cluster_manager {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stats_sinks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stats_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stats_flush_interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.watchdog {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tracing {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rate_limit_service {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.runtime {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.admin {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.static_resources)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dynamic_resources)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster_manager)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.flags_path)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats_sinks)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats_config)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats_flush_interval)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.watchdog)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tracing)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rate_limit_service)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.runtime)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.admin)?;
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
        if let Some(ref v) = self.static_resources.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.dynamic_resources.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cluster_manager.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.flags_path.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.flags_path);
        }
        for value in &self.stats_sinks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.stats_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.stats_flush_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.watchdog.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.tracing.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rate_limit_service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.runtime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.admin.as_ref() {
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
        if let Some(ref v) = self.static_resources.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.dynamic_resources.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cluster_manager.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.flags_path.is_empty() {
            os.write_string(5, &self.flags_path)?;
        }
        for v in &self.stats_sinks {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.stats_config.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.stats_flush_interval.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.watchdog.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.tracing.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rate_limit_service.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.runtime.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.admin.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Bootstrap {
    fn new() -> Bootstrap {
        Bootstrap::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bootstrap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                    "node",
                    Bootstrap::get_node_for_reflect,
                    Bootstrap::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bootstrap_StaticResources>>(
                    "static_resources",
                    Bootstrap::get_static_resources_for_reflect,
                    Bootstrap::mut_static_resources_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bootstrap_DynamicResources>>(
                    "dynamic_resources",
                    Bootstrap::get_dynamic_resources_for_reflect,
                    Bootstrap::mut_dynamic_resources_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterManager>>(
                    "cluster_manager",
                    Bootstrap::get_cluster_manager_for_reflect,
                    Bootstrap::mut_cluster_manager_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "flags_path",
                    Bootstrap::get_flags_path_for_reflect,
                    Bootstrap::mut_flags_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatsSink>>(
                    "stats_sinks",
                    Bootstrap::get_stats_sinks_for_reflect,
                    Bootstrap::mut_stats_sinks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatsConfig>>(
                    "stats_config",
                    Bootstrap::get_stats_config_for_reflect,
                    Bootstrap::mut_stats_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "stats_flush_interval",
                    Bootstrap::get_stats_flush_interval_for_reflect,
                    Bootstrap::mut_stats_flush_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Watchdog>>(
                    "watchdog",
                    Bootstrap::get_watchdog_for_reflect,
                    Bootstrap::mut_watchdog_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Tracing>>(
                    "tracing",
                    Bootstrap::get_tracing_for_reflect,
                    Bootstrap::mut_tracing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimitServiceConfig>>(
                    "rate_limit_service",
                    Bootstrap::get_rate_limit_service_for_reflect,
                    Bootstrap::mut_rate_limit_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Runtime>>(
                    "runtime",
                    Bootstrap::get_runtime_for_reflect,
                    Bootstrap::mut_runtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Admin>>(
                    "admin",
                    Bootstrap::get_admin_for_reflect,
                    Bootstrap::mut_admin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bootstrap>(
                    "Bootstrap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bootstrap {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_static_resources();
        self.clear_dynamic_resources();
        self.clear_cluster_manager();
        self.clear_flags_path();
        self.clear_stats_sinks();
        self.clear_stats_config();
        self.clear_stats_flush_interval();
        self.clear_watchdog();
        self.clear_tracing();
        self.clear_rate_limit_service();
        self.clear_runtime();
        self.clear_admin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bootstrap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bootstrap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bootstrap_StaticResources {
    // message fields
    pub listeners: ::protobuf::RepeatedField<super::lds::Listener>,
    pub clusters: ::protobuf::RepeatedField<super::cds::Cluster>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bootstrap_StaticResources {}

impl Bootstrap_StaticResources {
    pub fn new() -> Bootstrap_StaticResources {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bootstrap_StaticResources {
        static mut instance: ::protobuf::lazy::Lazy<Bootstrap_StaticResources> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bootstrap_StaticResources,
        };
        unsafe {
            instance.get(Bootstrap_StaticResources::new)
        }
    }

    // repeated .envoy.api.v2.Listener listeners = 1;

    pub fn clear_listeners(&mut self) {
        self.listeners.clear();
    }

    // Param is passed by value, moved
    pub fn set_listeners(&mut self, v: ::protobuf::RepeatedField<super::lds::Listener>) {
        self.listeners = v;
    }

    // Mutable pointer to the field.
    pub fn mut_listeners(&mut self) -> &mut ::protobuf::RepeatedField<super::lds::Listener> {
        &mut self.listeners
    }

    // Take field
    pub fn take_listeners(&mut self) -> ::protobuf::RepeatedField<super::lds::Listener> {
        ::std::mem::replace(&mut self.listeners, ::protobuf::RepeatedField::new())
    }

    pub fn get_listeners(&self) -> &[super::lds::Listener] {
        &self.listeners
    }

    fn get_listeners_for_reflect(&self) -> &::protobuf::RepeatedField<super::lds::Listener> {
        &self.listeners
    }

    fn mut_listeners_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::lds::Listener> {
        &mut self.listeners
    }

    // repeated .envoy.api.v2.Cluster clusters = 2;

    pub fn clear_clusters(&mut self) {
        self.clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_clusters(&mut self, v: ::protobuf::RepeatedField<super::cds::Cluster>) {
        self.clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clusters(&mut self) -> &mut ::protobuf::RepeatedField<super::cds::Cluster> {
        &mut self.clusters
    }

    // Take field
    pub fn take_clusters(&mut self) -> ::protobuf::RepeatedField<super::cds::Cluster> {
        ::std::mem::replace(&mut self.clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_clusters(&self) -> &[super::cds::Cluster] {
        &self.clusters
    }

    fn get_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<super::cds::Cluster> {
        &self.clusters
    }

    fn mut_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::cds::Cluster> {
        &mut self.clusters
    }
}

impl ::protobuf::Message for Bootstrap_StaticResources {
    fn is_initialized(&self) -> bool {
        for v in &self.listeners {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.listeners)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.clusters)?;
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
        for value in &self.listeners {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.clusters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.listeners {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.clusters {
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

impl ::protobuf::MessageStatic for Bootstrap_StaticResources {
    fn new() -> Bootstrap_StaticResources {
        Bootstrap_StaticResources::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bootstrap_StaticResources>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::lds::Listener>>(
                    "listeners",
                    Bootstrap_StaticResources::get_listeners_for_reflect,
                    Bootstrap_StaticResources::mut_listeners_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cds::Cluster>>(
                    "clusters",
                    Bootstrap_StaticResources::get_clusters_for_reflect,
                    Bootstrap_StaticResources::mut_clusters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bootstrap_StaticResources>(
                    "Bootstrap_StaticResources",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bootstrap_StaticResources {
    fn clear(&mut self) {
        self.clear_listeners();
        self.clear_clusters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bootstrap_StaticResources {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bootstrap_StaticResources {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bootstrap_DynamicResources {
    // message fields
    pub lds_config: ::protobuf::SingularPtrField<super::base::ConfigSource>,
    pub cds_config: ::protobuf::SingularPtrField<super::base::ConfigSource>,
    pub ads_config: ::protobuf::SingularPtrField<super::base::ApiConfigSource>,
    pub deprecated_v1: ::protobuf::SingularPtrField<Bootstrap_DynamicResources_DeprecatedV1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bootstrap_DynamicResources {}

impl Bootstrap_DynamicResources {
    pub fn new() -> Bootstrap_DynamicResources {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bootstrap_DynamicResources {
        static mut instance: ::protobuf::lazy::Lazy<Bootstrap_DynamicResources> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bootstrap_DynamicResources,
        };
        unsafe {
            instance.get(Bootstrap_DynamicResources::new)
        }
    }

    // .envoy.api.v2.ConfigSource lds_config = 1;

    pub fn clear_lds_config(&mut self) {
        self.lds_config.clear();
    }

    pub fn has_lds_config(&self) -> bool {
        self.lds_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lds_config(&mut self, v: super::base::ConfigSource) {
        self.lds_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lds_config(&mut self) -> &mut super::base::ConfigSource {
        if self.lds_config.is_none() {
            self.lds_config.set_default();
        }
        self.lds_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_lds_config(&mut self) -> super::base::ConfigSource {
        self.lds_config.take().unwrap_or_else(|| super::base::ConfigSource::new())
    }

    pub fn get_lds_config(&self) -> &super::base::ConfigSource {
        self.lds_config.as_ref().unwrap_or_else(|| super::base::ConfigSource::default_instance())
    }

    fn get_lds_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ConfigSource> {
        &self.lds_config
    }

    fn mut_lds_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ConfigSource> {
        &mut self.lds_config
    }

    // .envoy.api.v2.ConfigSource cds_config = 2;

    pub fn clear_cds_config(&mut self) {
        self.cds_config.clear();
    }

    pub fn has_cds_config(&self) -> bool {
        self.cds_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cds_config(&mut self, v: super::base::ConfigSource) {
        self.cds_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cds_config(&mut self) -> &mut super::base::ConfigSource {
        if self.cds_config.is_none() {
            self.cds_config.set_default();
        }
        self.cds_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_cds_config(&mut self) -> super::base::ConfigSource {
        self.cds_config.take().unwrap_or_else(|| super::base::ConfigSource::new())
    }

    pub fn get_cds_config(&self) -> &super::base::ConfigSource {
        self.cds_config.as_ref().unwrap_or_else(|| super::base::ConfigSource::default_instance())
    }

    fn get_cds_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ConfigSource> {
        &self.cds_config
    }

    fn mut_cds_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ConfigSource> {
        &mut self.cds_config
    }

    // .envoy.api.v2.ApiConfigSource ads_config = 3;

    pub fn clear_ads_config(&mut self) {
        self.ads_config.clear();
    }

    pub fn has_ads_config(&self) -> bool {
        self.ads_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ads_config(&mut self, v: super::base::ApiConfigSource) {
        self.ads_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ads_config(&mut self) -> &mut super::base::ApiConfigSource {
        if self.ads_config.is_none() {
            self.ads_config.set_default();
        }
        self.ads_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_ads_config(&mut self) -> super::base::ApiConfigSource {
        self.ads_config.take().unwrap_or_else(|| super::base::ApiConfigSource::new())
    }

    pub fn get_ads_config(&self) -> &super::base::ApiConfigSource {
        self.ads_config.as_ref().unwrap_or_else(|| super::base::ApiConfigSource::default_instance())
    }

    fn get_ads_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ApiConfigSource> {
        &self.ads_config
    }

    fn mut_ads_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ApiConfigSource> {
        &mut self.ads_config
    }

    // .envoy.api.v2.Bootstrap.DynamicResources.DeprecatedV1 deprecated_v1 = 4;

    pub fn clear_deprecated_v1(&mut self) {
        self.deprecated_v1.clear();
    }

    pub fn has_deprecated_v1(&self) -> bool {
        self.deprecated_v1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated_v1(&mut self, v: Bootstrap_DynamicResources_DeprecatedV1) {
        self.deprecated_v1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated_v1(&mut self) -> &mut Bootstrap_DynamicResources_DeprecatedV1 {
        if self.deprecated_v1.is_none() {
            self.deprecated_v1.set_default();
        }
        self.deprecated_v1.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated_v1(&mut self) -> Bootstrap_DynamicResources_DeprecatedV1 {
        self.deprecated_v1.take().unwrap_or_else(|| Bootstrap_DynamicResources_DeprecatedV1::new())
    }

    pub fn get_deprecated_v1(&self) -> &Bootstrap_DynamicResources_DeprecatedV1 {
        self.deprecated_v1.as_ref().unwrap_or_else(|| Bootstrap_DynamicResources_DeprecatedV1::default_instance())
    }

    fn get_deprecated_v1_for_reflect(&self) -> &::protobuf::SingularPtrField<Bootstrap_DynamicResources_DeprecatedV1> {
        &self.deprecated_v1
    }

    fn mut_deprecated_v1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Bootstrap_DynamicResources_DeprecatedV1> {
        &mut self.deprecated_v1
    }
}

impl ::protobuf::Message for Bootstrap_DynamicResources {
    fn is_initialized(&self) -> bool {
        for v in &self.lds_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cds_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ads_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lds_config)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cds_config)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ads_config)?;
                },
                4 => {
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
        if let Some(ref v) = self.lds_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cds_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ads_config.as_ref() {
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
        if let Some(ref v) = self.lds_config.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cds_config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ads_config.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.deprecated_v1.as_ref() {
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

impl ::protobuf::MessageStatic for Bootstrap_DynamicResources {
    fn new() -> Bootstrap_DynamicResources {
        Bootstrap_DynamicResources::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bootstrap_DynamicResources>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ConfigSource>>(
                    "lds_config",
                    Bootstrap_DynamicResources::get_lds_config_for_reflect,
                    Bootstrap_DynamicResources::mut_lds_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ConfigSource>>(
                    "cds_config",
                    Bootstrap_DynamicResources::get_cds_config_for_reflect,
                    Bootstrap_DynamicResources::mut_cds_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ApiConfigSource>>(
                    "ads_config",
                    Bootstrap_DynamicResources::get_ads_config_for_reflect,
                    Bootstrap_DynamicResources::mut_ads_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Bootstrap_DynamicResources_DeprecatedV1>>(
                    "deprecated_v1",
                    Bootstrap_DynamicResources::get_deprecated_v1_for_reflect,
                    Bootstrap_DynamicResources::mut_deprecated_v1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bootstrap_DynamicResources>(
                    "Bootstrap_DynamicResources",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bootstrap_DynamicResources {
    fn clear(&mut self) {
        self.clear_lds_config();
        self.clear_cds_config();
        self.clear_ads_config();
        self.clear_deprecated_v1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bootstrap_DynamicResources {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bootstrap_DynamicResources {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Bootstrap_DynamicResources_DeprecatedV1 {
    // message fields
    pub sds_config: ::protobuf::SingularPtrField<super::base::ConfigSource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Bootstrap_DynamicResources_DeprecatedV1 {}

impl Bootstrap_DynamicResources_DeprecatedV1 {
    pub fn new() -> Bootstrap_DynamicResources_DeprecatedV1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Bootstrap_DynamicResources_DeprecatedV1 {
        static mut instance: ::protobuf::lazy::Lazy<Bootstrap_DynamicResources_DeprecatedV1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Bootstrap_DynamicResources_DeprecatedV1,
        };
        unsafe {
            instance.get(Bootstrap_DynamicResources_DeprecatedV1::new)
        }
    }

    // .envoy.api.v2.ConfigSource sds_config = 1;

    pub fn clear_sds_config(&mut self) {
        self.sds_config.clear();
    }

    pub fn has_sds_config(&self) -> bool {
        self.sds_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sds_config(&mut self, v: super::base::ConfigSource) {
        self.sds_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sds_config(&mut self) -> &mut super::base::ConfigSource {
        if self.sds_config.is_none() {
            self.sds_config.set_default();
        }
        self.sds_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_sds_config(&mut self) -> super::base::ConfigSource {
        self.sds_config.take().unwrap_or_else(|| super::base::ConfigSource::new())
    }

    pub fn get_sds_config(&self) -> &super::base::ConfigSource {
        self.sds_config.as_ref().unwrap_or_else(|| super::base::ConfigSource::default_instance())
    }

    fn get_sds_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base::ConfigSource> {
        &self.sds_config
    }

    fn mut_sds_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base::ConfigSource> {
        &mut self.sds_config
    }
}

impl ::protobuf::Message for Bootstrap_DynamicResources_DeprecatedV1 {
    fn is_initialized(&self) -> bool {
        for v in &self.sds_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sds_config)?;
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
        if let Some(ref v) = self.sds_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.sds_config.as_ref() {
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

impl ::protobuf::MessageStatic for Bootstrap_DynamicResources_DeprecatedV1 {
    fn new() -> Bootstrap_DynamicResources_DeprecatedV1 {
        Bootstrap_DynamicResources_DeprecatedV1::new()
    }

    fn descriptor_static(_: ::std::option::Option<Bootstrap_DynamicResources_DeprecatedV1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::ConfigSource>>(
                    "sds_config",
                    Bootstrap_DynamicResources_DeprecatedV1::get_sds_config_for_reflect,
                    Bootstrap_DynamicResources_DeprecatedV1::mut_sds_config_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Bootstrap_DynamicResources_DeprecatedV1>(
                    "Bootstrap_DynamicResources_DeprecatedV1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Bootstrap_DynamicResources_DeprecatedV1 {
    fn clear(&mut self) {
        self.clear_sds_config();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Bootstrap_DynamicResources_DeprecatedV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Bootstrap_DynamicResources_DeprecatedV1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13api/bootstrap.proto\x12\x0cenvoy.api.v2\x1a\x11api/address.proto\
    \x1a\x0eapi/base.proto\x1a\rapi/cds.proto\x1a\rapi/lds.proto\x1a\x1egoog\
    le/protobuf/duration.proto\x1a\x1cgoogle/protobuf/struct.proto\x1a\x1ego\
    ogle/protobuf/wrappers.proto\"j\n\x0fLightstepConfig\x12+\n\x11collector\
    _cluster\x18\x01\x20\x01(\tR\x10collectorCluster\x12*\n\x11access_token_\
    file\x18\x02\x20\x01(\tR\x0faccessTokenFile\"j\n\x0cZipkinConfig\x12+\n\
    \x11collector_cluster\x18\x01\x20\x01(\tR\x10collectorCluster\x12-\n\x12\
    collector_endpoint\x18\x02\x20\x01(\tR\x11collectorEndpoint\"\x86\x01\n\
    \x07Tracing\x12.\n\x04http\x18\x01\x20\x01(\x0b2\x1a.envoy.api.v2.Tracin\
    g.HttpR\x04http\x1aK\n\x04Http\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04\
    name\x12/\n\x06config\x18\x02\x20\x01(\x0b2\x17.google.protobuf.StructR\
    \x06config\"\x83\x01\n\x05Admin\x12&\n\x0faccess_log_path\x18\x01\x20\
    \x01(\tR\raccessLogPath\x12!\n\x0cprofile_path\x18\x02\x20\x01(\tR\x0bpr\
    ofilePath\x12/\n\x07address\x18\x03\x20\x01(\x0b2\x15.envoy.api.v2.Addre\
    ssR\x07address\"\xeb\x02\n\x0eClusterManager\x12,\n\x12local_cluster_nam\
    e\x18\x01\x20\x01(\tR\x10localClusterName\x12Z\n\x11outlier_detection\
    \x18\x02\x20\x01(\x0b2-.envoy.api.v2.ClusterManager.OutlierDetectionR\
    \x10outlierDetection\x12J\n\x14upstream_bind_config\x18\x03\x20\x01(\x0b\
    2\x18.envoy.api.v2.BindConfigR\x12upstreamBindConfig\x12I\n\x11load_stat\
    s_config\x18\x04\x20\x01(\x0b2\x1d.envoy.api.v2.ApiConfigSourceR\x0fload\
    StatsConfig\x1a8\n\x10OutlierDetection\x12$\n\x0eevent_log_path\x18\x01\
    \x20\x01(\tR\x0ceventLogPath\"\x7f\n\nStatsdSink\x121\n\x07address\x18\
    \x01\x20\x01(\x0b2\x15.envoy.api.v2.AddressH\0R\x07address\x12*\n\x10tcp\
    _cluster_name\x18\x02\x20\x01(\tH\0R\x0etcpClusterNameB\x12\n\x10statsd_\
    specifier\"P\n\tStatsSink\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12/\n\x06config\x18\x02\x20\x01(\x0b2\x17.google.protobuf.StructR\x06c\
    onfig\"?\n\x0cTagSpecifier\x12\x19\n\x08tag_name\x18\x01\x20\x01(\tR\x07\
    tagName\x12\x14\n\x05regex\x18\x02\x20\x01(\tR\x05regex\"\x95\x01\n\x0bS\
    tatsConfig\x129\n\nstats_tags\x18\x01\x20\x03(\x0b2\x1a.envoy.api.v2.Tag\
    SpecifierR\tstatsTags\x12K\n\x14use_all_default_tags\x18\x02\x20\x01(\
    \x0b2\x1a.google.protobuf.BoolValueR\x11useAllDefaultTags\"\x94\x02\n\
    \x08Watchdog\x12<\n\x0cmiss_timeout\x18\x01\x20\x01(\x0b2\x19.google.pro\
    tobuf.DurationR\x0bmissTimeout\x12D\n\x10megamiss_timeout\x18\x02\x20\
    \x01(\x0b2\x19.google.protobuf.DurationR\x0fmegamissTimeout\x12<\n\x0cki\
    ll_timeout\x18\x03\x20\x01(\x0b2\x19.google.protobuf.DurationR\x0bkillTi\
    meout\x12F\n\x11multikill_timeout\x18\x04\x20\x01(\x0b2\x19.google.proto\
    buf.DurationR\x10multikillTimeout\"\x85\x01\n\x07Runtime\x12!\n\x0csymli\
    nk_root\x18\x01\x20\x01(\tR\x0bsymlinkRoot\x12\"\n\x0csubdirectory\x18\
    \x02\x20\x01(\tR\x0csubdirectory\x123\n\x15override_subdirectory\x18\x03\
    \x20\x01(\tR\x14overrideSubdirectory\";\n\x16RateLimitServiceConfig\x12!\
    \n\x0ccluster_name\x18\x01\x20\x01(\tR\x0bclusterName\"\x8a\n\n\tBootstr\
    ap\x12&\n\x04node\x18\x01\x20\x01(\x0b2\x12.envoy.api.v2.NodeR\x04node\
    \x12R\n\x10static_resources\x18\x02\x20\x01(\x0b2'.envoy.api.v2.Bootstra\
    p.StaticResourcesR\x0fstaticResources\x12U\n\x11dynamic_resources\x18\
    \x03\x20\x01(\x0b2(.envoy.api.v2.Bootstrap.DynamicResourcesR\x10dynamicR\
    esources\x12E\n\x0fcluster_manager\x18\x04\x20\x01(\x0b2\x1c.envoy.api.v\
    2.ClusterManagerR\x0eclusterManager\x12\x1d\n\nflags_path\x18\x05\x20\
    \x01(\tR\tflagsPath\x128\n\x0bstats_sinks\x18\x06\x20\x03(\x0b2\x17.envo\
    y.api.v2.StatsSinkR\nstatsSinks\x12<\n\x0cstats_config\x18\r\x20\x01(\
    \x0b2\x19.envoy.api.v2.StatsConfigR\x0bstatsConfig\x12K\n\x14stats_flush\
    _interval\x18\x07\x20\x01(\x0b2\x19.google.protobuf.DurationR\x12statsFl\
    ushInterval\x122\n\x08watchdog\x18\x08\x20\x01(\x0b2\x16.envoy.api.v2.Wa\
    tchdogR\x08watchdog\x12/\n\x07tracing\x18\t\x20\x01(\x0b2\x15.envoy.api.\
    v2.TracingR\x07tracing\x12R\n\x12rate_limit_service\x18\n\x20\x01(\x0b2$\
    .envoy.api.v2.RateLimitServiceConfigR\x10rateLimitService\x12/\n\x07runt\
    ime\x18\x0b\x20\x01(\x0b2\x15.envoy.api.v2.RuntimeR\x07runtime\x12)\n\
    \x05admin\x18\x0c\x20\x01(\x0b2\x13.envoy.api.v2.AdminR\x05admin\x1az\n\
    \x0fStaticResources\x124\n\tlisteners\x18\x01\x20\x03(\x0b2\x16.envoy.ap\
    i.v2.ListenerR\tlisteners\x121\n\x08clusters\x18\x02\x20\x03(\x0b2\x15.e\
    nvoy.api.v2.ClusterR\x08clusters\x1a\xed\x02\n\x10DynamicResources\x129\
    \n\nlds_config\x18\x01\x20\x01(\x0b2\x1a.envoy.api.v2.ConfigSourceR\tlds\
    Config\x129\n\ncds_config\x18\x02\x20\x01(\x0b2\x1a.envoy.api.v2.ConfigS\
    ourceR\tcdsConfig\x12<\n\nads_config\x18\x03\x20\x01(\x0b2\x1d.envoy.api\
    .v2.ApiConfigSourceR\tadsConfig\x12Z\n\rdeprecated_v1\x18\x04\x20\x01(\
    \x0b25.envoy.api.v2.Bootstrap.DynamicResources.DeprecatedV1R\x0cdeprecat\
    edV1\x1aI\n\x0cDeprecatedV1\x129\n\nsds_config\x18\x01\x20\x01(\x0b2\x1a\
    .envoy.api.v2.ConfigSourceR\tsdsConfigJ\x83k\n\x07\x12\x05\x04\0\xa2\x02\
    \x01\n\xdc\x01\n\x01\x0c\x12\x03\x04\0\x122\xd1\x01\x20This\x20proto\x20\
    is\x20expected\x20to\x20be\x20provided\x20on\x20disk\x20or\x20via\x20the\
    \x20command-line\x20to\n\x20Envoy.\x20It\x20provides\x20sufficient\x20in\
    formation\x20for\x20Envoy\x20to\x20fetch\x20the\x20rest\x20of\n\x20its\
    \x20configuration\x20from\x20either\x20disk\x20or\x20management\x20serve\
    r(s).\n\n\x08\n\x01\x02\x12\x03\x06\x08\x14\n\t\n\x02\x03\0\x12\x03\x08\
    \x07\x1a\n\t\n\x02\x03\x01\x12\x03\t\x07\x17\n\t\n\x02\x03\x02\x12\x03\n\
    \x07\x16\n\t\n\x02\x03\x03\x12\x03\x0b\x07\x16\n\t\n\x02\x03\x04\x12\x03\
    \r\x07'\n\t\n\x02\x03\x05\x12\x03\x0e\x07%\n\t\n\x02\x03\x06\x12\x03\x0f\
    \x07'\n\n\n\x02\x04\0\x12\x04\x11\0\x16\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x11\x08\x17\nO\n\x04\x04\0\x02\0\x12\x03\x13\x02\x1f\x1aB\x20The\x20clu\
    ster\x20manager\x20cluster\x20that\x20hosts\x20the\x20LightStep\x20colle\
    ctors.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x13\x02\x11\x19\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x13\t\x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x13\x1d\x1e\nE\n\x04\x04\
    \0\x02\x01\x12\x03\x15\x02\x1f\x1a8\x20File\x20containing\x20the\x20acce\
    ss\x20token\x20to\x20the\x20LightStep\x20API.\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x15\x02\x13\x1f\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x15\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x15\t\x1a\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x15\x1d\x1e\n\n\n\x02\x04\x01\x12\x04\x18\0!\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x18\x08\x14\n\xb9\x01\n\x04\x04\x01\x02\0\
    \x12\x03\x1c\x02\x1f\x1a\xab\x01\x20The\x20cluster\x20manager\x20cluster\
    \x20that\x20hosts\x20the\x20Zipkin\x20collectors.\x20Note\x20that\x20the\
    \n\x20Zipkin\x20cluster\x20must\x20be\x20defined\x20under\x20clusters\
    \x20in\x20the\x20cluster\x20manager\n\x20configuration\x20section.\n\n\r\
    \n\x05\x04\x01\x02\0\x04\x12\x04\x1c\x02\x18\x16\n\x0c\n\x05\x04\x01\x02\
    \0\x05\x12\x03\x1c\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1c\t\
    \x1a\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1c\x1d\x1e\n\xca\x01\n\x04\
    \x04\x01\x02\x01\x12\x03\x20\x02\x20\x1a\xbc\x01\x20The\x20API\x20endpoi\
    nt\x20of\x20the\x20Zipkin\x20service\x20where\x20the\x20spans\x20will\
    \x20be\x20sent.\x20When\n\x20using\x20a\x20standard\x20Zipkin\x20install\
    ation,\x20the\x20API\x20endpoint\x20is\x20typically\n\x20/api/v1/spans,\
    \x20which\x20is\x20the\x20default\x20value.\n\n\r\n\x05\x04\x01\x02\x01\
    \x04\x12\x04\x20\x02\x1c\x1f\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x20\
    \x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x20\t\x1b\n\x0c\n\x05\
    \x04\x01\x02\x01\x03\x12\x03\x20\x1e\x1f\n\n\n\x02\x04\x02\x12\x04#\0/\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03#\x08\x0f\n;\n\x04\x04\x02\x03\0\x12\
    \x04%\x02-\x03\x1a-\x20Provides\x20configuration\x20for\x20the\x20HTTP\
    \x20tracer.\n\n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03%\n\x0e\n\xb9\x01\n\
    \x06\x04\x02\x03\0\x02\0\x12\x03)\x04\x14\x1a\xa9\x01\x20The\x20name\x20\
    of\x20the\x20HTTP\x20trace\x20driver\x20to\x20instantiate.\x20The\x20nam\
    e\x20must\x20match\x20a\n\x20supported\x20HTTP\x20trace\x20driver.\x20\"\
    envoy.lightstep\"\x20and\x20\"envoy.zipkin\"\x20are\x20built-in\n\x20tra\
    ce\x20drivers.\n\n\x0f\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04)\x04%\x10\n\
    \x0e\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03)\x04\n\n\x0e\n\x07\x04\x02\
    \x03\0\x02\0\x01\x12\x03)\x0b\x0f\n\x0e\n\x07\x04\x02\x03\0\x02\0\x03\
    \x12\x03)\x12\x13\n\x97\x01\n\x06\x04\x02\x03\0\x02\x01\x12\x03,\x04&\
    \x1a\x87\x01\x20Trace\x20driver\x20specific\x20configuration\x20which\
    \x20depends\x20on\x20the\x20driver\x20being\n\x20instantiated.\x20See\
    \x20the\x20trace\x20drivers\x20for\x20further\x20documentation.\n\n\x0f\
    \n\x07\x04\x02\x03\0\x02\x01\x04\x12\x04,\x04)\x14\n\x0e\n\x07\x04\x02\
    \x03\0\x02\x01\x06\x12\x03,\x04\x1a\n\x0e\n\x07\x04\x02\x03\0\x02\x01\
    \x01\x12\x03,\x1b!\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03,$%\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x03.\x02\x10\n\r\n\x05\x04\x02\x02\0\x04\
    \x12\x04.\x02-\x03\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03.\x02\x06\n\x0c\
    \n\x05\x04\x02\x02\0\x01\x12\x03.\x07\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03.\x0e\x0f\n\n\n\x02\x04\x03\x12\x041\0:\x01\n\n\n\x03\x04\x03\
    \x01\x12\x031\x08\r\n\x84\x01\n\x04\x04\x03\x02\0\x12\x034\x02\x1d\x1aw\
    \x20The\x20path\x20to\x20write\x20the\x20access\x20log\x20for\x20the\x20\
    administration\x20server.\x20If\x20no\n\x20access\x20log\x20is\x20desire\
    d\x20specify\x20\xe2\x80\x98/dev/null\xe2\x80\x99.\n\n\r\n\x05\x04\x03\
    \x02\0\x04\x12\x044\x021\x0f\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x034\x02\
    \x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x034\t\x18\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x034\x1b\x1c\n\x9d\x01\n\x04\x04\x03\x02\x01\x12\x037\x02\
    \x1a\x1a\x8f\x01\x20The\x20cpu\x20profiler\x20output\x20path\x20for\x20t\
    he\x20administration\x20server.\x20If\x20no\x20profile\n\x20path\x20is\
    \x20specified,\x20the\x20default\x20is\x20\xe2\x80\x98/var/log/envoy/env\
    oy.prof\xe2\x80\x99.\n\n\r\n\x05\x04\x03\x02\x01\x04\x12\x047\x024\x1d\n\
    \x0c\n\x05\x04\x03\x02\x01\x05\x12\x037\x02\x08\n\x0c\n\x05\x04\x03\x02\
    \x01\x01\x12\x037\t\x15\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x037\x18\x19\
    \nM\n\x04\x04\x03\x02\x02\x12\x039\x02\x16\x1a@\x20The\x20TCP\x20address\
    \x20that\x20the\x20administration\x20server\x20will\x20listen\x20on.\n\n\
    \r\n\x05\x04\x03\x02\x02\x04\x12\x049\x027\x1a\n\x0c\n\x05\x04\x03\x02\
    \x02\x06\x12\x039\x02\t\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x039\n\x11\n\
    \x0c\n\x05\x04\x03\x02\x02\x03\x12\x039\x14\x15\n\n\n\x02\x04\x04\x12\
    \x04<\0P\x01\n\n\n\x03\x04\x04\x01\x12\x03<\x08\x16\n\x99\x02\n\x04\x04\
    \x04\x02\0\x12\x03A\x02\x20\x1a\x8b\x02\x20Name\x20of\x20the\x20local\
    \x20cluster\x20(i.e.,\x20the\x20cluster\x20that\x20owns\x20the\x20Envoy\
    \x20running\n\x20this\x20configuration).\x20In\x20order\x20to\x20enable\
    \x20zone\x20aware\x20routing\x20this\x20option\x20must\n\x20be\x20set.\
    \x20If\x20local_cluster_name\x20is\x20defined\x20then\x20clusters\x20mus\
    t\x20contain\x20a\n\x20definition\x20of\x20a\x20cluster\x20with\x20the\
    \x20same\x20name.\n\n\r\n\x05\x04\x04\x02\0\x04\x12\x04A\x02<\x18\n\x0c\
    \n\x05\x04\x04\x02\0\x05\x12\x03A\x02\x08\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x03A\t\x1b\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03A\x1e\x1f\nD\n\x04\
    \x04\x04\x03\0\x12\x04D\x02F\x03\x1a6\x20Optional\x20global\x20configura\
    tion\x20for\x20outlier\x20detection.\n\n\x0c\n\x05\x04\x04\x03\0\x01\x12\
    \x03D\n\x1a\n\r\n\x06\x04\x04\x03\0\x02\0\x12\x03E\x04\x1e\n\x0f\n\x07\
    \x04\x04\x03\0\x02\0\x04\x12\x04E\x04D\x1c\n\x0e\n\x07\x04\x04\x03\0\x02\
    \0\x05\x12\x03E\x04\n\n\x0e\n\x07\x04\x04\x03\0\x02\0\x01\x12\x03E\x0b\
    \x19\n\x0e\n\x07\x04\x04\x03\0\x02\0\x03\x12\x03E\x1c\x1d\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03G\x02)\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04G\
    \x02F\x03\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03G\x02\x12\n\x0c\n\x05\
    \x04\x04\x02\x01\x01\x12\x03G\x13$\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03G'(\n\xb5\x01\n\x04\x04\x04\x02\x02\x12\x03K\x02&\x1a\xa7\x01\x20Opt\
    ional\x20configuration\x20used\x20to\x20bind\x20newly\x20established\x20\
    upstream\x20connections.\n\x20This\x20may\x20be\x20overridden\x20on\x20a\
    \x20per-cluster\x20basis\x20by\x20upstream_bind_config\x20in\x20the\x20c\
    ds_config.\n\n\r\n\x05\x04\x04\x02\x02\x04\x12\x04K\x02G)\n\x0c\n\x05\
    \x04\x04\x02\x02\x06\x12\x03K\x02\x0c\n\x0c\n\x05\x04\x04\x02\x02\x01\
    \x12\x03K\r!\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03K$%\n\x88\x01\n\x04\
    \x04\x04\x02\x03\x12\x03O\x02(\x1a{\x20A\x20management\x20server\x20endp\
    oint\x20to\x20stream\x20load\x20stats\x20to,\x20as\x20per\n\x20StreamLoa\
    dStats\x20in\x20eds.proto.\x20This\x20must\x20have\x20api_type\x20GRPC.\
    \n\n\r\n\x05\x04\x04\x02\x03\x04\x12\x04O\x02K&\n\x0c\n\x05\x04\x04\x02\
    \x03\x06\x12\x03O\x02\x11\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03O\x12#\
    \n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03O&'\n;\n\x02\x04\x05\x12\x04S\0]\
    \x01\x1a/\x20Stats\x20proto\x20for\x20built-in\x20\"envoy.statsd\"\x20si\
    nk.\n\n\n\n\x03\x04\x05\x01\x12\x03S\x08\x12\n\x0c\n\x04\x04\x05\x08\0\
    \x12\x04T\x02\\\x03\n\x0c\n\x05\x04\x05\x08\0\x01\x12\x03T\x08\x18\n\x81\
    \x01\n\x04\x04\x05\x02\0\x12\x03W\x04\x18\x1at\x20The\x20UDP\x20address\
    \x20of\x20a\x20running\x20statsd\x20compliant\x20listener.\x20If\x20spec\
    ified,\n\x20statistics\x20will\x20be\x20flushed\x20to\x20this\x20address\
    .\n\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03W\x04\x0b\n\x0c\n\x05\x04\x05\
    \x02\0\x01\x12\x03W\x0c\x13\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03W\x16\
    \x17\n\xae\x01\n\x04\x04\x05\x02\x01\x12\x03[\x04\x20\x1a\xa0\x01\x20The\
    \x20name\x20of\x20a\x20cluster\x20manager\x20cluster\x20that\x20is\x20ru\
    nning\x20a\x20TCP\x20statsd\n\x20compliant\x20listener.\x20If\x20specifi\
    ed,\x20Envoy\x20will\x20connect\x20to\x20this\x20cluster\x20to\n\x20flus\
    h\x20statistics.\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03[\x04\n\n\x0c\
    \n\x05\x04\x05\x02\x01\x01\x12\x03[\x0b\x1b\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x03[\x1e\x1f\n\n\n\x02\x04\x06\x12\x04_\0g\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03_\x08\x11\n\xab\x01\n\x04\x04\x06\x02\0\x12\x03c\x02\x12\
    \x1a\x9d\x01\x20The\x20name\x20of\x20the\x20stats\x20sink\x20to\x20insta\
    ntiate.\x20The\x20name\x20must\x20match\x20a\x20supported\n\x20stats\x20\
    sink.\x20\"envoy.statsd\"\x20is\x20a\x20built-in\x20sink\x20suitable\x20\
    for\x20emitting\x20to\n\x20statsd.\n\n\r\n\x05\x04\x06\x02\0\x04\x12\x04\
    c\x02_\x13\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03c\x02\x08\n\x0c\n\x05\
    \x04\x06\x02\0\x01\x12\x03c\t\r\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03c\
    \x10\x11\n\x93\x01\n\x04\x04\x06\x02\x01\x12\x03f\x02$\x1a\x85\x01\x20St\
    ats\x20sink\x20specific\x20configuration\x20which\x20depends\x20on\x20th\
    e\x20sink\x20being\n\x20instantiated.\x20See\x20the\x20supported\x20sink\
    s\x20for\x20further\x20documentation.\n\n\r\n\x05\x04\x06\x02\x01\x04\
    \x12\x04f\x02c\x12\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03f\x02\x18\n\
    \x0c\n\x05\x04\x06\x02\x01\x01\x12\x03f\x19\x1f\n\x0c\n\x05\x04\x06\x02\
    \x01\x03\x12\x03f\"#\n\xe5\x01\n\x02\x04\x07\x12\x05l\0\x9d\x01\x01\x1a\
    \xd7\x01\x20Designates\x20a\x20tag\x20to\x20strip\x20from\x20the\x20tag\
    \x20extracted\x20name\x20and\x20provide\x20as\x20a\x20named\n\x20tag\x20\
    value\x20for\x20all\x20statistics.\x20This\x20will\x20only\x20occur\x20i\
    f\x20any\x20part\x20of\x20the\x20name\n\x20matches\x20the\x20regex\x20pr\
    ovided\x20with\x20one\x20or\x20more\x20capture\x20groups.\n\n\n\n\x03\
    \x04\x07\x01\x12\x03l\x08\x14\n\x88\x04\n\x04\x04\x07\x02\0\x12\x03u\x02\
    \x16\x1a\xfa\x03\x20Attaches\x20an\x20identifier\x20to\x20the\x20tag\x20\
    values\x20to\x20identify\x20the\x20tag\x20being\x20in\x20the\n\x20sink.\
    \x20Envoy\x20has\x20a\x20set\x20of\x20default\x20names\x20and\x20regexes\
    \x20to\x20extract\x20dynamic\n\x20portions\x20of\x20existing\x20stats,\
    \x20which\x20can\x20be\x20found\x20in\n\x20source/common/config/well_kno\
    wn_names.h\x20in\x20the\x20Envoy\x20repository.\x20If\x20a\n\x20tag_name\
    \x20is\x20provided\x20in\x20the\x20config\x20with\x20an\x20empty\x20rege\
    x,\x20Envoy\x20will\x20attempt\n\x20to\x20find\x20that\x20name\x20in\x20\
    its\x20set\x20of\x20defaults\x20and\x20use\x20the\x20accompanying\x20reg\
    ex.\n\x20Note:\x20if\x20any\x20default\x20tags\x20are\x20specified\x20tw\
    ice,\x20the\x20config\x20will\x20be\n\x20considered\x20invalid.\n\n\r\n\
    \x05\x04\x07\x02\0\x04\x12\x04u\x02l\x16\n\x0c\n\x05\x04\x07\x02\0\x05\
    \x12\x03u\x02\x08\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03u\t\x11\n\x0c\n\
    \x05\x04\x07\x02\0\x03\x12\x03u\x14\x15\n\xcf\x0c\n\x04\x04\x07\x02\x01\
    \x12\x04\x9c\x01\x02\x13\x1a\xc0\x0c\x20The\x20first\x20capture\x20group\
    \x20identifies\x20the\x20portion\x20of\x20the\x20name\x20to\x20remove.\
    \x20The\n\x20second\x20capture\x20group\x20(which\x20will\x20normally\
    \x20be\x20nested\x20inside\x20the\x20first)\n\x20will\x20designate\x20th\
    e\x20value\x20of\x20the\x20tag\x20for\x20the\x20statistic.\x20If\x20no\
    \x20second\x20\n\x20capture\x20group\x20is\x20provided,\x20the\x20first\
    \x20will\x20also\x20be\x20used\x20to\x20set\x20the\x20value\x20of\n\x20t\
    he\x20tag.\x20All\x20other\x20capture\x20groups\x20will\x20be\x20ignored\
    .\n\n\x20For\x20example:\n\n\x20stat\x20name:\x20\"cluster.foo_cluster.u\
    pstream_rq_timeout\"\n\n\x20tag\x20name:\x20\"envoy.cluster_name\"\x20(o\
    ne\x20of\x20many\x20default\x20stat\x20regexes\x20provided\n\x20in\x20En\
    voy)\n\n\x20regex:\x20\"^cluster\\.((.+?)\\.)\"\n\n\x20Notice\x20the\x20\
    regex\x20will\x20remove\x20\"foo_cluster.\"\x20making\x20the\x20tag\x20e\
    xtracted\x20name\n\x20\"cluster.upstream_rq_timeout\"\x20and\x20the\x20t\
    ag\x20value\x20for\x20\"envoy.cluster_name\"\n\x20\"foo_cluster\"\x20(no\
    te:\x20no\x20'.'\x20character\x20because\x20of\x20the\x20second\x20captu\
    re\x20group).\n\n\x20An\x20example\x20with\x20two\x20regexes:\n\n\x20Fir\
    st\x20tag\x20name:\x20\"envoy.http_user_agent\"\n\x20First\x20regex:\x20\
    \"^http(?=\\.).*?\\.user_agent\\.((.+?)\\.)\\w+?$\"\n\x20Second\x20tag\
    \x20name:\x20\"envoy.http_conn_manager_prefix\"\n\x20Second\x20regex:\
    \x20\"^http\\.((.*?)\\.)\"\n\x20Stat\x20name:\x20\"http.connection_manag\
    er_1.user_agent.ios.downstream_cx_total\"\n\n\x20The\x20first\x20regex\
    \x20will\x20remove\x20\"ios.\"\x20leaving\x20the\x20tag\x20extracted\x20\
    name:\n\x20\"http.connection_manager_1.user_agent.downstream_cx_total\".\
    \n\x20The\x20tag\x20\"envoy.http_user_agent\"\x20will\x20be\x20added\x20\
    with\x20tag\x20value\x20\"ios\".\n\n\x20The\x20second\x20regex\x20will\
    \x20remove\x20\"connection_manager_1.\"\x20from\x20the\x20tag\x20extract\
    ed\n\x20name\x20produced\x20by\x20the\x20first\x20regex\n\x20(\"http.con\
    nection_manager_1.user_agent.downstream_cx_total\"),\x20leaving\n\x20\"h\
    ttp.user_agent.downstream_cx_total\"\x20as\x20the\x20tag\x20extracted\
    \x20name.\x20\x20The\x20tag\n\x20\"envoy.http_conn_manager_prefix\"\x20w\
    ill\x20be\x20added\x20with\x20the\x20tag\x20value\n\x20\"connection_mana\
    ger_1\".\n\n\x0e\n\x05\x04\x07\x02\x01\x04\x12\x05\x9c\x01\x02u\x16\n\r\
    \n\x05\x04\x07\x02\x01\x05\x12\x04\x9c\x01\x02\x08\n\r\n\x05\x04\x07\x02\
    \x01\x01\x12\x04\x9c\x01\t\x0e\n\r\n\x05\x04\x07\x02\x01\x03\x12\x04\x9c\
    \x01\x11\x12\n\x0c\n\x02\x04\x08\x12\x06\x9f\x01\0\xac\x01\x01\n\x0b\n\
    \x03\x04\x08\x01\x12\x04\x9f\x01\x08\x13\n\xe4\x01\n\x04\x04\x08\x02\0\
    \x12\x04\xa3\x01\x02'\x1a\xd5\x01\x20Each\x20stat\x20name\x20is\x20itera\
    tively\x20processed\x20through\x20these\x20tag\x20specifiers.\n\x20When\
    \x20a\x20tag\x20is\x20matched,\x20the\x20first\x20capture\x20group\x20is\
    \x20removed\x20from\x20the\x20name\x20so\n\x20later\x20TagSpecifiers\x20\
    cannot\x20match\x20that\x20same\x20portion\x20of\x20the\x20match.\n\n\r\
    \n\x05\x04\x08\x02\0\x04\x12\x04\xa3\x01\x02\n\n\r\n\x05\x04\x08\x02\0\
    \x06\x12\x04\xa3\x01\x0b\x17\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\xa3\x01\
    \x18\"\n\r\n\x05\x04\x08\x02\0\x03\x12\x04\xa3\x01%&\n\x93\x03\n\x04\x04\
    \x08\x02\x01\x12\x04\xab\x01\x025\x1a\x84\x03\x20Use\x20all\x20default\
    \x20tag\x20regexes\x20specified\x20in\x20Envoy.\x20These\x20can\x20be\
    \x20combined\x20with\n\x20custom\x20tags\x20specified\x20in\x20stats_tag\
    s.\x20They\x20will\x20be\x20processed\x20before\x20the\n\x20custom\x20ta\
    gs.\x20Note:\x20if\x20any\x20default\x20tags\x20are\x20specified\x20twic\
    e,\x20the\x20config\x20will\n\x20be\x20considered\x20invalid.\x20See\x20\
    source/common/config/well_known_names.h\x20for\x20a\n\x20a\x20list\x20of\
    \x20the\x20default\x20tags\x20in\x20Envoy.\x20If\x20not\x20provided,\x20\
    the\x20value\x20is\x20assumed\n\x20to\x20be\x20true.\n\n\x0f\n\x05\x04\
    \x08\x02\x01\x04\x12\x06\xab\x01\x02\xa3\x01'\n\r\n\x05\x04\x08\x02\x01\
    \x06\x12\x04\xab\x01\x02\x1b\n\r\n\x05\x04\x08\x02\x01\x01\x12\x04\xab\
    \x01\x1c0\n\r\n\x05\x04\x08\x02\x01\x03\x12\x04\xab\x0134\n\x0c\n\x02\
    \x04\t\x12\x06\xae\x01\0\xbe\x01\x01\n\x0b\n\x03\x04\t\x01\x12\x04\xae\
    \x01\x08\x10\n\xa2\x01\n\x04\x04\t\x02\0\x12\x04\xb1\x01\x02,\x1a\x93\
    \x01\x20The\x20duration\x20after\x20which\x20Envoy\x20counts\x20a\x20non\
    responsive\x20thread\x20in\x20the\n\x20\xe2\x80\x9cserver.watchdog_miss\
    \xe2\x80\x9d\x20statistic.\x20If\x20not\x20specified\x20the\x20default\
    \x20is\x20200ms.\n\n\x0f\n\x05\x04\t\x02\0\x04\x12\x06\xb1\x01\x02\xae\
    \x01\x12\n\r\n\x05\x04\t\x02\0\x06\x12\x04\xb1\x01\x02\x1a\n\r\n\x05\x04\
    \t\x02\0\x01\x12\x04\xb1\x01\x1b'\n\r\n\x05\x04\t\x02\0\x03\x12\x04\xb1\
    \x01*+\n\xa9\x01\n\x04\x04\t\x02\x01\x12\x04\xb5\x01\x020\x1a\x9a\x01\
    \x20The\x20duration\x20after\x20which\x20Envoy\x20counts\x20a\x20nonresp\
    onsive\x20thread\x20in\x20the\n\x20\xe2\x80\x9cserver.watchdog_mega_miss\
    \xe2\x80\x9d\x20statistic.\x20If\x20not\x20specified\x20the\x20default\
    \x20is\n\x201000ms.\n\n\x0f\n\x05\x04\t\x02\x01\x04\x12\x06\xb5\x01\x02\
    \xb1\x01,\n\r\n\x05\x04\t\x02\x01\x06\x12\x04\xb5\x01\x02\x1a\n\r\n\x05\
    \x04\t\x02\x01\x01\x12\x04\xb5\x01\x1b+\n\r\n\x05\x04\t\x02\x01\x03\x12\
    \x04\xb5\x01./\n\xdf\x01\n\x04\x04\t\x02\x02\x12\x04\xb9\x01\x02,\x1a\
    \xd0\x01\x20If\x20a\x20watched\x20thread\x20has\x20been\x20nonresponsive\
    \x20for\x20this\x20duration,\x20assume\x20a\n\x20programming\x20error\
    \x20and\x20kill\x20the\x20entire\x20Envoy\x20process.\x20Set\x20to\x200\
    \x20to\x20disable\n\x20kill\x20behavior.\x20If\x20not\x20specified\x20th\
    e\x20default\x20is\x200\x20(disabled).\n\n\x0f\n\x05\x04\t\x02\x02\x04\
    \x12\x06\xb9\x01\x02\xb5\x010\n\r\n\x05\x04\t\x02\x02\x06\x12\x04\xb9\
    \x01\x02\x1a\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\xb9\x01\x1b'\n\r\n\x05\
    \x04\t\x02\x02\x03\x12\x04\xb9\x01*+\n\xf0\x01\n\x04\x04\t\x02\x03\x12\
    \x04\xbd\x01\x021\x1a\xe1\x01\x20If\x20at\x20least\x20two\x20watched\x20\
    threads\x20have\x20been\x20nonresponsive\x20for\x20at\x20least\x20this\n\
    \x20duration\x20assume\x20a\x20true\x20deadlock\x20and\x20kill\x20the\
    \x20entire\x20Envoy\x20process.\x20Set\x20to\x200\n\x20to\x20disable\x20\
    this\x20behavior.\x20If\x20not\x20specified\x20the\x20default\x20is\x200\
    \x20(disabled).\n\n\x0f\n\x05\x04\t\x02\x03\x04\x12\x06\xbd\x01\x02\xb9\
    \x01,\n\r\n\x05\x04\t\x02\x03\x06\x12\x04\xbd\x01\x02\x1a\n\r\n\x05\x04\
    \t\x02\x03\x01\x12\x04\xbd\x01\x1b,\n\r\n\x05\x04\t\x02\x03\x03\x12\x04\
    \xbd\x01/0\n\x0c\n\x02\x04\n\x12\x06\xc0\x01\0\xd2\x01\x01\n\x0b\n\x03\
    \x04\n\x01\x12\x04\xc0\x01\x08\x0f\n\xc1\x02\n\x04\x04\n\x02\0\x12\x04\
    \xc6\x01\x02\x1a\x1a\xb2\x02\x20The\x20implementation\x20assumes\x20that\
    \x20the\x20file\x20system\x20tree\x20is\x20accessed\x20via\x20a\n\x20sym\
    bolic\x20link.\x20An\x20atomic\x20link\x20swap\x20is\x20used\x20when\x20\
    a\x20new\x20tree\x20should\x20be\n\x20switched\x20to.\x20This\x20paramet\
    er\x20specifies\x20the\x20path\x20to\x20the\x20symbolic\x20link.\x20Envo\
    y\n\x20will\x20watch\x20the\x20location\x20for\x20changes\x20and\x20relo\
    ad\x20the\x20file\x20system\x20tree\x20when\n\x20they\x20happen.\n\n\x0f\
    \n\x05\x04\n\x02\0\x04\x12\x06\xc6\x01\x02\xc0\x01\x11\n\r\n\x05\x04\n\
    \x02\0\x05\x12\x04\xc6\x01\x02\x08\n\r\n\x05\x04\n\x02\0\x01\x12\x04\xc6\
    \x01\t\x15\n\r\n\x05\x04\n\x02\0\x03\x12\x04\xc6\x01\x18\x19\n\xe1\x01\n\
    \x04\x04\n\x02\x01\x12\x04\xca\x01\x02\x1a\x1a\xd2\x01\x20Specifies\x20t\
    he\x20subdirectory\x20to\x20load\x20within\x20the\x20root\x20directory.\
    \x20This\x20is\n\x20useful\x20if\x20multiple\x20systems\x20share\x20the\
    \x20same\x20delivery\x20mechanism.\x20Envoy\n\x20configuration\x20elemen\
    ts\x20can\x20be\x20contained\x20in\x20a\x20dedicated\x20subdirectory.\n\
    \n\x0f\n\x05\x04\n\x02\x01\x04\x12\x06\xca\x01\x02\xc6\x01\x1a\n\r\n\x05\
    \x04\n\x02\x01\x05\x12\x04\xca\x01\x02\x08\n\r\n\x05\x04\n\x02\x01\x01\
    \x12\x04\xca\x01\t\x15\n\r\n\x05\x04\n\x02\x01\x03\x12\x04\xca\x01\x18\
    \x19\n\xc6\x03\n\x04\x04\n\x02\x02\x12\x04\xd1\x01\x02#\x1a\xb7\x03\x20S\
    pecifies\x20an\x20optional\x20subdirectory\x20to\x20load\x20within\x20th\
    e\x20root\x20directory.\x20If\n\x20specified\x20and\x20the\x20directory\
    \x20exists,\x20configuration\x20values\x20within\x20this\n\x20directory\
    \x20will\x20override\x20those\x20found\x20in\x20the\x20primary\x20subdir\
    ectory.\x20This\x20is\n\x20useful\x20when\x20Envoy\x20is\x20deployed\x20\
    across\x20many\x20different\x20types\x20of\x20servers.\n\x20Sometimes\
    \x20it\x20is\x20useful\x20to\x20have\x20a\x20per\x20service\x20cluster\
    \x20directory\x20for\x20runtime\n\x20configuration.\x20See\x20below\x20f\
    or\x20exactly\x20how\x20the\x20override\x20directory\x20is\x20used.\n\n\
    \x0f\n\x05\x04\n\x02\x02\x04\x12\x06\xd1\x01\x02\xca\x01\x1a\n\r\n\x05\
    \x04\n\x02\x02\x05\x12\x04\xd1\x01\x02\x08\n\r\n\x05\x04\n\x02\x02\x01\
    \x12\x04\xd1\x01\t\x1e\n\r\n\x05\x04\n\x02\x02\x03\x12\x04\xd1\x01!\"\n\
    \x0c\n\x02\x04\x0b\x12\x06\xd4\x01\0\xd9\x01\x01\n\x0b\n\x03\x04\x0b\x01\
    \x12\x04\xd4\x01\x08\x1e\n\xbb\x01\n\x04\x04\x0b\x02\0\x12\x04\xd8\x01\
    \x02\x1a\x1a\xac\x01\x20Specifies\x20the\x20cluster\x20manager\x20cluste\
    r\x20name\x20that\x20hosts\x20the\x20rate\x20limit\n\x20service.\x20The\
    \x20client\x20will\x20connect\x20to\x20this\x20cluster\x20when\x20it\x20\
    needs\x20to\x20make\x20rate\n\x20limit\x20service\x20requests.\n\n\x0f\n\
    \x05\x04\x0b\x02\0\x04\x12\x06\xd8\x01\x02\xd4\x01\x20\n\r\n\x05\x04\x0b\
    \x02\0\x05\x12\x04\xd8\x01\x02\x08\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\
    \xd8\x01\t\x15\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\xd8\x01\x18\x19\n\x0c\
    \n\x02\x04\x0c\x12\x06\xdb\x01\0\xa2\x02\x01\n\x0b\n\x03\x04\x0c\x01\x12\
    \x04\xdb\x01\x08\x11\n\x88\x01\n\x04\x04\x0c\x02\0\x12\x04\xde\x01\x02\
    \x10\x1az\x20Node\x20identity\x20to\x20present\x20to\x20the\x20managemen\
    t\x20server\x20and\x20for\x20instance\n\x20identification\x20purposes\
    \x20(e.g.\x20in\x20generated\x20headers).\n\n\x0f\n\x05\x04\x0c\x02\0\
    \x04\x12\x06\xde\x01\x02\xdb\x01\x13\n\r\n\x05\x04\x0c\x02\0\x06\x12\x04\
    \xde\x01\x02\x06\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\xde\x01\x07\x0b\n\r\
    \n\x05\x04\x0c\x02\0\x03\x12\x04\xde\x01\x0e\x0f\n1\n\x04\x04\x0c\x03\0\
    \x12\x06\xe1\x01\x02\xe8\x01\x03\x1a!\x20Statically\x20specified\x20reso\
    urces.\n\n\r\n\x05\x04\x0c\x03\0\x01\x12\x04\xe1\x01\n\x19\n\x0e\n\x06\
    \x04\x0c\x03\0\x02\0\x12\x04\xe2\x01\x04$\n\x0f\n\x07\x04\x0c\x03\0\x02\
    \0\x04\x12\x04\xe2\x01\x04\x0c\n\x0f\n\x07\x04\x0c\x03\0\x02\0\x06\x12\
    \x04\xe2\x01\r\x15\n\x0f\n\x07\x04\x0c\x03\0\x02\0\x01\x12\x04\xe2\x01\
    \x16\x1f\n\x0f\n\x07\x04\x0c\x03\0\x02\0\x03\x12\x04\xe2\x01\"#\n\xb2\
    \x02\n\x06\x04\x0c\x03\0\x02\x01\x12\x04\xe7\x01\x04\"\x1a\xa1\x02\x20If\
    \x20a\x20network\x20based\x20configuration\x20source\x20is\x20specified\
    \x20for\x20cds_config,\x20it's\n\x20necessary\x20to\x20have\x20some\x20i\
    nitial\x20cluster\x20definitions\x20available\x20to\x20allow\x20Envoy\n\
    \x20to\x20know\x20how\x20to\x20speak\x20to\x20the\x20management\x20serve\
    r.\x20These\x20cluster\x20definitions\n\x20may\x20not\x20use\x20EDS\x20(\
    i.e.\x20they\x20should\x20be\x20static\x20IP\x20or\x20DNS-based).\n\n\
    \x0f\n\x07\x04\x0c\x03\0\x02\x01\x04\x12\x04\xe7\x01\x04\x0c\n\x0f\n\x07\
    \x04\x0c\x03\0\x02\x01\x06\x12\x04\xe7\x01\r\x14\n\x0f\n\x07\x04\x0c\x03\
    \0\x02\x01\x01\x12\x04\xe7\x01\x15\x1d\n\x0f\n\x07\x04\x0c\x03\0\x02\x01\
    \x03\x12\x04\xe7\x01\x20!\n\x0c\n\x04\x04\x0c\x02\x01\x12\x04\xe9\x01\
    \x02'\n\x0f\n\x05\x04\x0c\x02\x01\x04\x12\x06\xe9\x01\x02\xe8\x01\x03\n\
    \r\n\x05\x04\x0c\x02\x01\x06\x12\x04\xe9\x01\x02\x11\n\r\n\x05\x04\x0c\
    \x02\x01\x01\x12\x04\xe9\x01\x12\"\n\r\n\x05\x04\x0c\x02\x01\x03\x12\x04\
    \xe9\x01%&\n,\n\x04\x04\x0c\x03\x01\x12\x06\xec\x01\x02\xfa\x01\x03\x1a\
    \x1c\x20xDS\x20configuration\x20sources.\n\n\r\n\x05\x04\x0c\x03\x01\x01\
    \x12\x04\xec\x01\n\x1a\nR\n\x06\x04\x0c\x03\x01\x02\0\x12\x04\xee\x01\
    \x04\x20\x1aB\x20All\x20Listeners\x20are\x20provided\x20by\x20a\x20singl\
    e\x20LDS\x20configuration\x20source.\n\n\x11\n\x07\x04\x0c\x03\x01\x02\0\
    \x04\x12\x06\xee\x01\x04\xec\x01\x1c\n\x0f\n\x07\x04\x0c\x03\x01\x02\0\
    \x06\x12\x04\xee\x01\x04\x10\n\x0f\n\x07\x04\x0c\x03\x01\x02\0\x01\x12\
    \x04\xee\x01\x11\x1b\n\x0f\n\x07\x04\x0c\x03\x01\x02\0\x03\x12\x04\xee\
    \x01\x1e\x1f\nl\n\x06\x04\x0c\x03\x01\x02\x01\x12\x04\xf1\x01\x04\x20\
    \x1a\\\x20All\x20post-bootstrap\x20Cluster\x20definitions\x20are\x20prov\
    ided\x20by\x20a\x20single\x20CDS\n\x20configuration\x20source.\n\n\x11\n\
    \x07\x04\x0c\x03\x01\x02\x01\x04\x12\x06\xf1\x01\x04\xee\x01\x20\n\x0f\n\
    \x07\x04\x0c\x03\x01\x02\x01\x06\x12\x04\xf1\x01\x04\x10\n\x0f\n\x07\x04\
    \x0c\x03\x01\x02\x01\x01\x12\x04\xf1\x01\x11\x1b\n\x0f\n\x07\x04\x0c\x03\
    \x01\x02\x01\x03\x12\x04\xf1\x01\x1e\x1f\n`\n\x06\x04\x0c\x03\x01\x02\
    \x02\x12\x04\xf3\x01\x04#\x1aP\x20A\x20single\x20ADS\x20source\x20may\
    \x20be\x20optionally\x20specified.\x20This\x20must\x20have\x20api_type\
    \x20GRPC.\n\n\x11\n\x07\x04\x0c\x03\x01\x02\x02\x04\x12\x06\xf3\x01\x04\
    \xf1\x01\x20\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x06\x12\x04\xf3\x01\x04\
    \x13\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x01\x12\x04\xf3\x01\x14\x1e\n\
    \x0f\n\x07\x04\x0c\x03\x01\x02\x02\x03\x12\x04\xf3\x01!\"\n\x10\n\x06\
    \x04\x0c\x03\x01\x03\0\x12\x06\xf5\x01\x04\xf8\x01\x05\n\x0f\n\x07\x04\
    \x0c\x03\x01\x03\0\x01\x12\x04\xf5\x01\x0c\x18\nQ\n\x08\x04\x0c\x03\x01\
    \x03\0\x02\0\x12\x04\xf7\x01\x06\"\x1a?\x20This\x20is\x20the\x20global\
    \x20SDS\x20config\x20when\x20using\x20v1\x20REST\x20for\x20CDS/EDS.\n\n\
    \x13\n\t\x04\x0c\x03\x01\x03\0\x02\0\x04\x12\x06\xf7\x01\x06\xf5\x01\x1a\
    \n\x11\n\t\x04\x0c\x03\x01\x03\0\x02\0\x06\x12\x04\xf7\x01\x06\x12\n\x11\
    \n\t\x04\x0c\x03\x01\x03\0\x02\0\x01\x12\x04\xf7\x01\x13\x1d\n\x11\n\t\
    \x04\x0c\x03\x01\x03\0\x02\0\x03\x12\x04\xf7\x01\x20!\n\x0e\n\x06\x04\
    \x0c\x03\x01\x02\x03\x12\x04\xf9\x01\x04#\n\x11\n\x07\x04\x0c\x03\x01\
    \x02\x03\x04\x12\x06\xf9\x01\x04\xf8\x01\x05\n\x0f\n\x07\x04\x0c\x03\x01\
    \x02\x03\x06\x12\x04\xf9\x01\x04\x10\n\x0f\n\x07\x04\x0c\x03\x01\x02\x03\
    \x01\x12\x04\xf9\x01\x11\x1e\n\x0f\n\x07\x04\x0c\x03\x01\x02\x03\x03\x12\
    \x04\xf9\x01!\"\n\x0c\n\x04\x04\x0c\x02\x02\x12\x04\xfb\x01\x02)\n\x0f\n\
    \x05\x04\x0c\x02\x02\x04\x12\x06\xfb\x01\x02\xfa\x01\x03\n\r\n\x05\x04\
    \x0c\x02\x02\x06\x12\x04\xfb\x01\x02\x12\n\r\n\x05\x04\x0c\x02\x02\x01\
    \x12\x04\xfb\x01\x13$\n\r\n\x05\x04\x0c\x02\x02\x03\x12\x04\xfb\x01'(\nj\
    \n\x04\x04\x0c\x02\x03\x12\x04\xff\x01\x02%\x1a\\\x20Configuration\x20fo\
    r\x20the\x20cluster\x20manager\x20which\x20owns\x20all\x20upstream\x20cl\
    usters\n\x20within\x20the\x20server.\n\n\x0f\n\x05\x04\x0c\x02\x03\x04\
    \x12\x06\xff\x01\x02\xfb\x01)\n\r\n\x05\x04\x0c\x02\x03\x06\x12\x04\xff\
    \x01\x02\x10\n\r\n\x05\x04\x0c\x02\x03\x01\x12\x04\xff\x01\x11\x20\n\r\n\
    \x05\x04\x0c\x02\x03\x03\x12\x04\xff\x01#$\nK\n\x04\x04\x0c\x02\x04\x12\
    \x04\x82\x02\x02\x18\x1a=\x20Optional\x20file\x20system\x20path\x20to\
    \x20search\x20for\x20startup\x20flag\x20files.\n\n\x0f\n\x05\x04\x0c\x02\
    \x04\x04\x12\x06\x82\x02\x02\xff\x01%\n\r\n\x05\x04\x0c\x02\x04\x05\x12\
    \x04\x82\x02\x02\x08\n\r\n\x05\x04\x0c\x02\x04\x01\x12\x04\x82\x02\t\x13\
    \n\r\n\x05\x04\x0c\x02\x04\x03\x12\x04\x82\x02\x16\x17\n,\n\x04\x04\x0c\
    \x02\x05\x12\x04\x85\x02\x02%\x1a\x1e\x20Optional\x20set\x20of\x20stats\
    \x20sinks.\n\n\r\n\x05\x04\x0c\x02\x05\x04\x12\x04\x85\x02\x02\n\n\r\n\
    \x05\x04\x0c\x02\x05\x06\x12\x04\x85\x02\x0b\x14\n\r\n\x05\x04\x0c\x02\
    \x05\x01\x12\x04\x85\x02\x15\x20\n\r\n\x05\x04\x0c\x02\x05\x03\x12\x04\
    \x85\x02#$\n?\n\x04\x04\x0c\x02\x06\x12\x04\x88\x02\x02\x20\x1a1\x20Conf\
    iguration\x20for\x20internal\x20processing\x20of\x20stats.\n\n\x0f\n\x05\
    \x04\x0c\x02\x06\x04\x12\x06\x88\x02\x02\x85\x02%\n\r\n\x05\x04\x0c\x02\
    \x06\x06\x12\x04\x88\x02\x02\r\n\r\n\x05\x04\x0c\x02\x06\x01\x12\x04\x88\
    \x02\x0e\x1a\n\r\n\x05\x04\x0c\x02\x06\x03\x12\x04\x88\x02\x1d\x1f\n\xf0\
    \x01\n\x04\x04\x0c\x02\x07\x12\x04\x8e\x02\x024\x1a\xe1\x01\x20Optional\
    \x20duration\x20between\x20flushes\x20to\x20configured\x20stats\x20sinks\
    .\x20For\n\x20performance\x20reasons\x20Envoy\x20latches\x20counters\x20\
    and\x20only\x20flushes\x20counters\x20and\n\x20gauges\x20at\x20a\x20peri\
    odic\x20interval.\x20If\x20not\x20specified\x20the\x20default\x20is\x205\
    000ms\x20(5\n\x20seconds).\n\n\x0f\n\x05\x04\x0c\x02\x07\x04\x12\x06\x8e\
    \x02\x02\x88\x02\x20\n\r\n\x05\x04\x0c\x02\x07\x06\x12\x04\x8e\x02\x02\
    \x1a\n\r\n\x05\x04\x0c\x02\x07\x01\x12\x04\x8e\x02\x1b/\n\r\n\x05\x04\
    \x0c\x02\x07\x03\x12\x04\x8e\x0223\n0\n\x04\x04\x0c\x02\x08\x12\x04\x91\
    \x02\x02\x18\x1a\"\x20Optional\x20watchdog\x20configuration.\n\n\x0f\n\
    \x05\x04\x0c\x02\x08\x04\x12\x06\x91\x02\x02\x8e\x024\n\r\n\x05\x04\x0c\
    \x02\x08\x06\x12\x04\x91\x02\x02\n\n\r\n\x05\x04\x0c\x02\x08\x01\x12\x04\
    \x91\x02\x0b\x13\n\r\n\x05\x04\x0c\x02\x08\x03\x12\x04\x91\x02\x16\x17\n\
    p\n\x04\x04\x0c\x02\t\x12\x04\x95\x02\x02\x16\x1ab\x20Configuration\x20f\
    or\x20an\x20external\x20tracing\x20provider.\x20If\x20not\x20specified,\
    \x20no\n\x20tracing\x20will\x20be\x20performed.\n\n\x0f\n\x05\x04\x0c\
    \x02\t\x04\x12\x06\x95\x02\x02\x91\x02\x18\n\r\n\x05\x04\x0c\x02\t\x06\
    \x12\x04\x95\x02\x02\t\n\r\n\x05\x04\x0c\x02\t\x01\x12\x04\x95\x02\n\x11\
    \n\r\n\x05\x04\x0c\x02\t\x03\x12\x04\x95\x02\x14\x15\n\xa4\x01\n\x04\x04\
    \x0c\x02\n\x12\x04\x9a\x02\x021\x1a\x95\x01\x20Configuration\x20for\x20a\
    n\x20external\x20rate\x20limit\x20service\x20provider.\x20If\x20not\n\
    \x20specified,\x20any\x20calls\x20to\x20the\x20rate\x20limit\x20service\
    \x20will\x20immediately\x20return\n\x20success.\n\n\x0f\n\x05\x04\x0c\
    \x02\n\x04\x12\x06\x9a\x02\x02\x95\x02\x16\n\r\n\x05\x04\x0c\x02\n\x06\
    \x12\x04\x9a\x02\x02\x18\n\r\n\x05\x04\x0c\x02\n\x01\x12\x04\x9a\x02\x19\
    +\n\r\n\x05\x04\x0c\x02\n\x03\x12\x04\x9a\x02.0\n\xaa\x01\n\x04\x04\x0c\
    \x02\x0b\x12\x04\x9e\x02\x02\x17\x1a\x9b\x01\x20Configuration\x20for\x20\
    the\x20runtime\x20configuration\x20provider.\x20If\x20not\x20specified,\
    \x20a\n\x20\xe2\x80\x9cnull\xe2\x80\x9d\x20provider\x20will\x20be\x20use\
    d\x20which\x20will\x20result\x20in\x20all\x20defaults\x20being\x20used.\
    \n\n\x0f\n\x05\x04\x0c\x02\x0b\x04\x12\x06\x9e\x02\x02\x9a\x021\n\r\n\
    \x05\x04\x0c\x02\x0b\x06\x12\x04\x9e\x02\x02\t\n\r\n\x05\x04\x0c\x02\x0b\
    \x01\x12\x04\x9e\x02\n\x11\n\r\n\x05\x04\x0c\x02\x0b\x03\x12\x04\x9e\x02\
    \x14\x16\nG\n\x04\x04\x0c\x02\x0c\x12\x04\xa1\x02\x02\x13\x1a9\x20Config\
    uration\x20for\x20the\x20local\x20administration\x20HTTP\x20server.\n\n\
    \x0f\n\x05\x04\x0c\x02\x0c\x04\x12\x06\xa1\x02\x02\x9e\x02\x17\n\r\n\x05\
    \x04\x0c\x02\x0c\x06\x12\x04\xa1\x02\x02\x07\n\r\n\x05\x04\x0c\x02\x0c\
    \x01\x12\x04\xa1\x02\x08\r\n\r\n\x05\x04\x0c\x02\x0c\x03\x12\x04\xa1\x02\
    \x10\x12b\x06proto3\
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
