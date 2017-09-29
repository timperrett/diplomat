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
pub struct Pipe {
    // message fields
    pub path: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Pipe {}

impl Pipe {
    pub fn new() -> Pipe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Pipe {
        static mut instance: ::protobuf::lazy::Lazy<Pipe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Pipe,
        };
        unsafe {
            instance.get(Pipe::new)
        }
    }

    // string path = 1;

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
}

impl ::protobuf::Message for Pipe {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
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
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
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

impl ::protobuf::MessageStatic for Pipe {
    fn new() -> Pipe {
        Pipe::new()
    }

    fn descriptor_static(_: ::std::option::Option<Pipe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    Pipe::get_path_for_reflect,
                    Pipe::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Pipe>(
                    "Pipe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Pipe {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Pipe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Pipe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SocketAddress {
    // message fields
    pub protocol: SocketAddress_Protocol,
    pub address: ::std::string::String,
    pub resolver_name: ::std::string::String,
    // message oneof groups
    port_specifier: ::std::option::Option<SocketAddress_oneof_port_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SocketAddress {}

#[derive(Clone,PartialEq)]
pub enum SocketAddress_oneof_port_specifier {
    port_value(u32),
    named_port(::std::string::String),
}

impl SocketAddress {
    pub fn new() -> SocketAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SocketAddress {
        static mut instance: ::protobuf::lazy::Lazy<SocketAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SocketAddress,
        };
        unsafe {
            instance.get(SocketAddress::new)
        }
    }

    // .envoy.api.v2.SocketAddress.Protocol protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = SocketAddress_Protocol::TCP;
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: SocketAddress_Protocol) {
        self.protocol = v;
    }

    pub fn get_protocol(&self) -> SocketAddress_Protocol {
        self.protocol
    }

    fn get_protocol_for_reflect(&self) -> &SocketAddress_Protocol {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut SocketAddress_Protocol {
        &mut self.protocol
    }

    // string address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::string::String {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // uint32 port_value = 3;

    pub fn clear_port_value(&mut self) {
        self.port_specifier = ::std::option::Option::None;
    }

    pub fn has_port_value(&self) -> bool {
        match self.port_specifier {
            ::std::option::Option::Some(SocketAddress_oneof_port_specifier::port_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_port_value(&mut self, v: u32) {
        self.port_specifier = ::std::option::Option::Some(SocketAddress_oneof_port_specifier::port_value(v))
    }

    pub fn get_port_value(&self) -> u32 {
        match self.port_specifier {
            ::std::option::Option::Some(SocketAddress_oneof_port_specifier::port_value(v)) => v,
            _ => 0,
        }
    }

    // string named_port = 4;

    pub fn clear_named_port(&mut self) {
        self.port_specifier = ::std::option::Option::None;
    }

    pub fn has_named_port(&self) -> bool {
        match self.port_specifier {
            ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_named_port(&mut self, v: ::std::string::String) {
        self.port_specifier = ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(v))
    }

    // Mutable pointer to the field.
    pub fn mut_named_port(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(_)) = self.port_specifier {
        } else {
            self.port_specifier = ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(::std::string::String::new()));
        }
        match self.port_specifier {
            ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_named_port(&mut self) -> ::std::string::String {
        if self.has_named_port() {
            match self.port_specifier.take() {
                ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_named_port(&self) -> &str {
        match self.port_specifier {
            ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(ref v)) => v,
            _ => "",
        }
    }

    // string resolver_name = 5;

    pub fn clear_resolver_name(&mut self) {
        self.resolver_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_resolver_name(&mut self, v: ::std::string::String) {
        self.resolver_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resolver_name(&mut self) -> &mut ::std::string::String {
        &mut self.resolver_name
    }

    // Take field
    pub fn take_resolver_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resolver_name, ::std::string::String::new())
    }

    pub fn get_resolver_name(&self) -> &str {
        &self.resolver_name
    }

    fn get_resolver_name_for_reflect(&self) -> &::std::string::String {
        &self.resolver_name
    }

    fn mut_resolver_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.resolver_name
    }
}

impl ::protobuf::Message for SocketAddress {
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
                    let tmp = is.read_enum()?;
                    self.protocol = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.port_specifier = ::std::option::Option::Some(SocketAddress_oneof_port_specifier::port_value(is.read_uint32()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.port_specifier = ::std::option::Option::Some(SocketAddress_oneof_port_specifier::named_port(is.read_string()?));
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resolver_name)?;
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
        if self.protocol != SocketAddress_Protocol::TCP {
            my_size += ::protobuf::rt::enum_size(1, self.protocol);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.address);
        }
        if !self.resolver_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.resolver_name);
        }
        if let ::std::option::Option::Some(ref v) = self.port_specifier {
            match v {
                &SocketAddress_oneof_port_specifier::port_value(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &SocketAddress_oneof_port_specifier::named_port(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.protocol != SocketAddress_Protocol::TCP {
            os.write_enum(1, self.protocol.value())?;
        }
        if !self.address.is_empty() {
            os.write_string(2, &self.address)?;
        }
        if !self.resolver_name.is_empty() {
            os.write_string(5, &self.resolver_name)?;
        }
        if let ::std::option::Option::Some(ref v) = self.port_specifier {
            match v {
                &SocketAddress_oneof_port_specifier::port_value(v) => {
                    os.write_uint32(3, v)?;
                },
                &SocketAddress_oneof_port_specifier::named_port(ref v) => {
                    os.write_string(4, v)?;
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

impl ::protobuf::MessageStatic for SocketAddress {
    fn new() -> SocketAddress {
        SocketAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<SocketAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SocketAddress_Protocol>>(
                    "protocol",
                    SocketAddress::get_protocol_for_reflect,
                    SocketAddress::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    SocketAddress::get_address_for_reflect,
                    SocketAddress::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                    "port_value",
                    SocketAddress::has_port_value,
                    SocketAddress::get_port_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "named_port",
                    SocketAddress::has_named_port,
                    SocketAddress::get_named_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resolver_name",
                    SocketAddress::get_resolver_name_for_reflect,
                    SocketAddress::mut_resolver_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SocketAddress>(
                    "SocketAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SocketAddress {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_address();
        self.clear_port_value();
        self.clear_named_port();
        self.clear_resolver_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SocketAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SocketAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SocketAddress_Protocol {
    TCP = 0,
    UDP = 1,
}

impl ::protobuf::ProtobufEnum for SocketAddress_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SocketAddress_Protocol> {
        match value {
            0 => ::std::option::Option::Some(SocketAddress_Protocol::TCP),
            1 => ::std::option::Option::Some(SocketAddress_Protocol::UDP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SocketAddress_Protocol] = &[
            SocketAddress_Protocol::TCP,
            SocketAddress_Protocol::UDP,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SocketAddress_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SocketAddress_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SocketAddress_Protocol {
}

impl ::std::default::Default for SocketAddress_Protocol {
    fn default() -> Self {
        SocketAddress_Protocol::TCP
    }
}

impl ::protobuf::reflect::ProtobufValue for SocketAddress_Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BindConfig {
    // message fields
    pub source_address: ::protobuf::SingularPtrField<SocketAddress>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BindConfig {}

impl BindConfig {
    pub fn new() -> BindConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BindConfig {
        static mut instance: ::protobuf::lazy::Lazy<BindConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BindConfig,
        };
        unsafe {
            instance.get(BindConfig::new)
        }
    }

    // .envoy.api.v2.SocketAddress source_address = 1;

    pub fn clear_source_address(&mut self) {
        self.source_address.clear();
    }

    pub fn has_source_address(&self) -> bool {
        self.source_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_address(&mut self, v: SocketAddress) {
        self.source_address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_address(&mut self) -> &mut SocketAddress {
        if self.source_address.is_none() {
            self.source_address.set_default();
        }
        self.source_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_address(&mut self) -> SocketAddress {
        self.source_address.take().unwrap_or_else(|| SocketAddress::new())
    }

    pub fn get_source_address(&self) -> &SocketAddress {
        self.source_address.as_ref().unwrap_or_else(|| SocketAddress::default_instance())
    }

    fn get_source_address_for_reflect(&self) -> &::protobuf::SingularPtrField<SocketAddress> {
        &self.source_address
    }

    fn mut_source_address_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SocketAddress> {
        &mut self.source_address
    }
}

impl ::protobuf::Message for BindConfig {
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

impl ::protobuf::MessageStatic for BindConfig {
    fn new() -> BindConfig {
        BindConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<BindConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SocketAddress>>(
                    "source_address",
                    BindConfig::get_source_address_for_reflect,
                    BindConfig::mut_source_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BindConfig>(
                    "BindConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BindConfig {
    fn clear(&mut self) {
        self.clear_source_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BindConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BindConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Address {
    // message oneof groups
    address: ::std::option::Option<Address_oneof_address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Address {}

#[derive(Clone,PartialEq)]
pub enum Address_oneof_address {
    socket_address(SocketAddress),
    pipe(Pipe),
}

impl Address {
    pub fn new() -> Address {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Address {
        static mut instance: ::protobuf::lazy::Lazy<Address> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Address,
        };
        unsafe {
            instance.get(Address::new)
        }
    }

    // .envoy.api.v2.SocketAddress socket_address = 1;

    pub fn clear_socket_address(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_socket_address(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::socket_address(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_socket_address(&mut self, v: SocketAddress) {
        self.address = ::std::option::Option::Some(Address_oneof_address::socket_address(v))
    }

    // Mutable pointer to the field.
    pub fn mut_socket_address(&mut self) -> &mut SocketAddress {
        if let ::std::option::Option::Some(Address_oneof_address::socket_address(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(Address_oneof_address::socket_address(SocketAddress::new()));
        }
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::socket_address(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_socket_address(&mut self) -> SocketAddress {
        if self.has_socket_address() {
            match self.address.take() {
                ::std::option::Option::Some(Address_oneof_address::socket_address(v)) => v,
                _ => panic!(),
            }
        } else {
            SocketAddress::new()
        }
    }

    pub fn get_socket_address(&self) -> &SocketAddress {
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::socket_address(ref v)) => v,
            _ => SocketAddress::default_instance(),
        }
    }

    // .envoy.api.v2.Pipe pipe = 2;

    pub fn clear_pipe(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: Pipe) {
        self.address = ::std::option::Option::Some(Address_oneof_address::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut Pipe {
        if let ::std::option::Option::Some(Address_oneof_address::pipe(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(Address_oneof_address::pipe(Pipe::new()));
        }
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> Pipe {
        if self.has_pipe() {
            match self.address.take() {
                ::std::option::Option::Some(Address_oneof_address::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            Pipe::new()
        }
    }

    pub fn get_pipe(&self) -> &Pipe {
        match self.address {
            ::std::option::Option::Some(Address_oneof_address::pipe(ref v)) => v,
            _ => Pipe::default_instance(),
        }
    }
}

impl ::protobuf::Message for Address {
    fn is_initialized(&self) -> bool {
        if let Some(Address_oneof_address::socket_address(ref v)) = self.address {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Address_oneof_address::pipe(ref v)) = self.address {
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
                    self.address = ::std::option::Option::Some(Address_oneof_address::socket_address(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.address = ::std::option::Option::Some(Address_oneof_address::pipe(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.address {
            match v {
                &Address_oneof_address::socket_address(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Address_oneof_address::pipe(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.address {
            match v {
                &Address_oneof_address::socket_address(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Address_oneof_address::pipe(ref v) => {
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

impl ::protobuf::MessageStatic for Address {
    fn new() -> Address {
        Address::new()
    }

    fn descriptor_static(_: ::std::option::Option<Address>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SocketAddress>(
                    "socket_address",
                    Address::has_socket_address,
                    Address::get_socket_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Pipe>(
                    "pipe",
                    Address::has_pipe,
                    Address::get_pipe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Address>(
                    "Address",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Address {
    fn clear(&mut self) {
        self.clear_socket_address();
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Address {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11api/address.proto\x12\x0cenvoy.api.v2\"\x1a\n\x04Pipe\x12\x12\n\
    \x04path\x18\x01\x20\x01(\tR\x04path\"\x82\x02\n\rSocketAddress\x12@\n\
    \x08protocol\x18\x01\x20\x01(\x0e2$.envoy.api.v2.SocketAddress.ProtocolR\
    \x08protocol\x12\x18\n\x07address\x18\x02\x20\x01(\tR\x07address\x12\x1f\
    \n\nport_value\x18\x03\x20\x01(\rH\0R\tportValue\x12\x1f\n\nnamed_port\
    \x18\x04\x20\x01(\tH\0R\tnamedPort\x12#\n\rresolver_name\x18\x05\x20\x01\
    (\tR\x0cresolverName\"\x1c\n\x08Protocol\x12\x07\n\x03TCP\x10\0\x12\x07\
    \n\x03UDP\x10\x01B\x10\n\x0eport_specifier\"P\n\nBindConfig\x12B\n\x0eso\
    urce_address\x18\x01\x20\x01(\x0b2\x1b.envoy.api.v2.SocketAddressR\rsour\
    ceAddress\"\x84\x01\n\x07Address\x12D\n\x0esocket_address\x18\x01\x20\
    \x01(\x0b2\x1b.envoy.api.v2.SocketAddressH\0R\rsocketAddress\x12(\n\x04p\
    ipe\x18\x02\x20\x01(\x0b2\x12.envoy.api.v2.PipeH\0R\x04pipeB\t\n\x07addr\
    essJ\xeb\x0e\n\x06\x12\x04\0\0/\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x02\x08\x14\n@\n\x02\x04\0\x12\x04\x06\0\x08\x012\
    4\x20[V2-API-DIFF]\x20Addresses\x20now\x20have\x20.proto\x20structure.\n\
    \n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x0c\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x07\x02\x12\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x07\x02\x06\x0e\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x07\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x10\x11\n\n\n\x02\
    \x04\x01\x12\x04\n\0\x20\x01\n\n\n\x03\x04\x01\x01\x12\x03\n\x08\x15\n\
    \x0c\n\x04\x04\x01\x04\0\x12\x04\x0b\x02\x0e\x03\n\x0c\n\x05\x04\x01\x04\
    \0\x01\x12\x03\x0b\x07\x0f\n\r\n\x06\x04\x01\x04\0\x02\0\x12\x03\x0c\x04\
    \x0c\n\x0e\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03\x0c\x04\x07\n\x0e\n\x07\
    \x04\x01\x04\0\x02\0\x02\x12\x03\x0c\n\x0b\n\r\n\x06\x04\x01\x04\0\x02\
    \x01\x12\x03\r\x04\x0c\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x01\x12\x03\r\
    \x04\x07\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03\r\n\x0b\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x0f\x02\x18\n\r\n\x05\x04\x01\x02\0\x04\x12\
    \x04\x0f\x02\x0e\x03\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0f\x02\n\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0f\x0b\x13\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x0f\x16\x17\n\xfe\x02\n\x04\x04\x01\x02\x01\x12\x03\x15\
    \x02\x15\x1a\xf0\x02\x20The\x20address\x20for\x20this\x20socket.\x20\x20\
    Listeners\x20will\x20bind\x20to\x20the\x20address.\x20\x20An\x20empty\n\
    \x20address\x20implies\x20a\x20bind\x20to\x200.0.0.0\x20or\x20::.\x20It'\
    s\x20still\x20possible\x20to\x20distinguish\x20on\n\x20address\x20via\
    \x20the\x20prefix/suffix\x20matching\x20in\x20FilterChainMatch\x20after\
    \x20connection.\n\x20For\x20clusters,\x20an\x20address\x20may\x20be\x20e\
    ither\x20an\x20IP\x20or\x20hostname\x20to\x20be\x20resolved\x20via\n\x20\
    DNS.\x20\x20If\x20it\x20is\x20a\x20hostname,\x20resolve_name\x20should\
    \x20be\x20set.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x15\x02\x0f\x18\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x15\t\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x15\x13\x14\n\x0c\n\x04\x04\x01\x08\0\x12\x04\x16\x02\x1b\x03\n\x0c\n\
    \x05\x04\x01\x08\0\x01\x12\x03\x16\x08\x16\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03\x17\x04\x1a\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x17\x04\n\n\
    \x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x17\x0b\x15\n\x0c\n\x05\x04\x01\
    \x02\x02\x03\x12\x03\x17\x18\x19\n\x91\x01\n\x04\x04\x01\x02\x03\x12\x03\
    \x1a\x04\x1a\x1a\x83\x01\x20This\x20is\x20only\x20valid\x20if\x20DNS\x20\
    SRV\x20or\x20if\x20resolver_name\x20is\x20specified\x20below\n\x20and\
    \x20the\x20named\x20resolver\x20is\x20capable\x20of\x20named\x20port\x20\
    resolution.\n\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x1a\x04\n\n\x0c\n\
    \x05\x04\x01\x02\x03\x01\x12\x03\x1a\x0b\x15\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03\x1a\x18\x19\n\xfe\x01\n\x04\x04\x01\x02\x04\x12\x03\x1f\x02\
    \x1b\x1a\xf0\x01\x20The\x20name\x20of\x20the\x20resolver.\x20This\x20mus\
    t\x20have\x20been\x20registered\x20with\x20Envoy.\x20If\x20this\x20is\n\
    \x20empty,\x20a\x20context\x20dependent\x20default\x20applies.\x20If\x20\
    address\x20is\x20a\x20hostname\x20this\n\x20should\x20be\x20set.\x20\x20\
    If\x20the\x20address\x20is\x20a\x20concrete\x20IP\x20address,\x20no\x20r\
    esolution\x20will\x20occur.\n\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04\x1f\
    \x02\x1b\x03\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03\x1f\x02\x08\n\x0c\n\
    \x05\x04\x01\x02\x04\x01\x12\x03\x1f\t\x16\n\x0c\n\x05\x04\x01\x02\x04\
    \x03\x12\x03\x1f\x19\x1a\n\n\n\x02\x04\x02\x12\x04\"\0%\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03\"\x08\x12\n=\n\x04\x04\x02\x02\0\x12\x03$\x02#\x1a0\
    \x20The\x20address\x20to\x20bind\x20to\x20when\x20creating\x20a\x20socke\
    t.\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04$\x02\"\x14\n\x0c\n\x05\x04\x02\
    \x02\0\x06\x12\x03$\x02\x0f\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03$\x10\
    \x1e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03$!\"\n\xb5\x01\n\x02\x04\x03\
    \x12\x04*\0/\x01\x1a\xa8\x01\x20Addresses\x20specify\x20either\x20a\x20l\
    ogical\x20or\x20physical\x20address\x20and\x20port,\x20which\x20are\n\
    \x20used\x20to\x20tell\x20Envoy\x20where\x20to\x20bind/listen,\x20connec\
    t\x20to\x20upstream\x20and\x20find\n\x20management\x20servers.\n\n\n\n\
    \x03\x04\x03\x01\x12\x03*\x08\x0f\n\x0c\n\x04\x04\x03\x08\0\x12\x04+\x02\
    .\x03\n\x0c\n\x05\x04\x03\x08\0\x01\x12\x03+\x08\x0f\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03,\x04%\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03,\x04\x11\n\
    \x0c\n\x05\x04\x03\x02\0\x01\x12\x03,\x12\x20\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03,#$\n\x0b\n\x04\x04\x03\x02\x01\x12\x03-\x04\x12\n\x0c\n\x05\
    \x04\x03\x02\x01\x06\x12\x03-\x04\x08\n\x0c\n\x05\x04\x03\x02\x01\x01\
    \x12\x03-\t\r\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03-\x10\x11b\x06proto\
    3\
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
