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
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_string(1, &self.path)?;
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
pub struct UnresolvedAddress {
    // message fields
    resolver: ::protobuf::SingularPtrField<UnresolvedAddress_Resolver>,
    // message oneof groups
    address: ::std::option::Option<UnresolvedAddress_oneof_address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnresolvedAddress {}

#[derive(Clone,PartialEq)]
pub enum UnresolvedAddress_oneof_address {
    named_address(UnresolvedAddress_NamedAddress),
    pipe(Pipe),
}

impl UnresolvedAddress {
    pub fn new() -> UnresolvedAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnresolvedAddress {
        static mut instance: ::protobuf::lazy::Lazy<UnresolvedAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnresolvedAddress,
        };
        unsafe {
            instance.get(UnresolvedAddress::new)
        }
    }

    // .envoy.api.v2.UnresolvedAddress.Resolver resolver = 1;

    pub fn clear_resolver(&mut self) {
        self.resolver.clear();
    }

    pub fn has_resolver(&self) -> bool {
        self.resolver.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolver(&mut self, v: UnresolvedAddress_Resolver) {
        self.resolver = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resolver(&mut self) -> &mut UnresolvedAddress_Resolver {
        if self.resolver.is_none() {
            self.resolver.set_default();
        };
        self.resolver.as_mut().unwrap()
    }

    // Take field
    pub fn take_resolver(&mut self) -> UnresolvedAddress_Resolver {
        self.resolver.take().unwrap_or_else(|| UnresolvedAddress_Resolver::new())
    }

    pub fn get_resolver(&self) -> &UnresolvedAddress_Resolver {
        self.resolver.as_ref().unwrap_or_else(|| UnresolvedAddress_Resolver::default_instance())
    }

    fn get_resolver_for_reflect(&self) -> &::protobuf::SingularPtrField<UnresolvedAddress_Resolver> {
        &self.resolver
    }

    fn mut_resolver_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnresolvedAddress_Resolver> {
        &mut self.resolver
    }

    // .envoy.api.v2.UnresolvedAddress.NamedAddress named_address = 2;

    pub fn clear_named_address(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_named_address(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_named_address(&mut self, v: UnresolvedAddress_NamedAddress) {
        self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(v))
    }

    // Mutable pointer to the field.
    pub fn mut_named_address(&mut self) -> &mut UnresolvedAddress_NamedAddress {
        if let ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(UnresolvedAddress_NamedAddress::new()));
        }
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_named_address(&mut self) -> UnresolvedAddress_NamedAddress {
        if self.has_named_address() {
            match self.address.take() {
                ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(v)) => v,
                _ => panic!(),
            }
        } else {
            UnresolvedAddress_NamedAddress::new()
        }
    }

    pub fn get_named_address(&self) -> &UnresolvedAddress_NamedAddress {
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(ref v)) => v,
            _ => UnresolvedAddress_NamedAddress::default_instance(),
        }
    }

    // .envoy.api.v2.Pipe pipe = 3;

    pub fn clear_pipe(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: Pipe) {
        self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut Pipe {
        if let ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(Pipe::new()));
        }
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> Pipe {
        if self.has_pipe() {
            match self.address.take() {
                ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            Pipe::new()
        }
    }

    pub fn get_pipe(&self) -> &Pipe {
        match self.address {
            ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(ref v)) => v,
            _ => Pipe::default_instance(),
        }
    }
}

impl ::protobuf::Message for UnresolvedAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resolver)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::named_address(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.address = ::std::option::Option::Some(UnresolvedAddress_oneof_address::pipe(is.read_message()?));
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
        if let Some(v) = self.resolver.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.address {
            match v {
                &UnresolvedAddress_oneof_address::named_address(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &UnresolvedAddress_oneof_address::pipe(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.resolver.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.address {
            match v {
                &UnresolvedAddress_oneof_address::named_address(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &UnresolvedAddress_oneof_address::pipe(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for UnresolvedAddress {
    fn new() -> UnresolvedAddress {
        UnresolvedAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnresolvedAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnresolvedAddress_Resolver>>(
                    "resolver",
                    UnresolvedAddress::get_resolver_for_reflect,
                    UnresolvedAddress::mut_resolver_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, UnresolvedAddress_NamedAddress>(
                    "named_address",
                    UnresolvedAddress::has_named_address,
                    UnresolvedAddress::get_named_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Pipe>(
                    "pipe",
                    UnresolvedAddress::has_pipe,
                    UnresolvedAddress::get_pipe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnresolvedAddress>(
                    "UnresolvedAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnresolvedAddress {
    fn clear(&mut self) {
        self.clear_resolver();
        self.clear_named_address();
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnresolvedAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnresolvedAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnresolvedAddress_NamedAddress {
    // message fields
    pub protocol: UnresolvedAddress_NamedAddress_Protocol,
    pub address: ::std::string::String,
    // message oneof groups
    port_specifier: ::std::option::Option<UnresolvedAddress_NamedAddress_oneof_port_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnresolvedAddress_NamedAddress {}

#[derive(Clone,PartialEq)]
pub enum UnresolvedAddress_NamedAddress_oneof_port_specifier {
    port(super::wrappers::UInt32Value),
    service_name(::std::string::String),
}

impl UnresolvedAddress_NamedAddress {
    pub fn new() -> UnresolvedAddress_NamedAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnresolvedAddress_NamedAddress {
        static mut instance: ::protobuf::lazy::Lazy<UnresolvedAddress_NamedAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnresolvedAddress_NamedAddress,
        };
        unsafe {
            instance.get(UnresolvedAddress_NamedAddress::new)
        }
    }

    // .envoy.api.v2.UnresolvedAddress.NamedAddress.Protocol protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = UnresolvedAddress_NamedAddress_Protocol::TCP;
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: UnresolvedAddress_NamedAddress_Protocol) {
        self.protocol = v;
    }

    pub fn get_protocol(&self) -> UnresolvedAddress_NamedAddress_Protocol {
        self.protocol
    }

    fn get_protocol_for_reflect(&self) -> &UnresolvedAddress_NamedAddress_Protocol {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut UnresolvedAddress_NamedAddress_Protocol {
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

    // .google.protobuf.UInt32Value port = 3;

    pub fn clear_port(&mut self) {
        self.port_specifier = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: super::wrappers::UInt32Value) {
        self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(v))
    }

    // Mutable pointer to the field.
    pub fn mut_port(&mut self) -> &mut super::wrappers::UInt32Value {
        if let ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(_)) = self.port_specifier {
        } else {
            self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(super::wrappers::UInt32Value::new()));
        }
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_port(&mut self) -> super::wrappers::UInt32Value {
        if self.has_port() {
            match self.port_specifier.take() {
                ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(v)) => v,
                _ => panic!(),
            }
        } else {
            super::wrappers::UInt32Value::new()
        }
    }

    pub fn get_port(&self) -> &super::wrappers::UInt32Value {
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(ref v)) => v,
            _ => super::wrappers::UInt32Value::default_instance(),
        }
    }

    // string service_name = 4;

    pub fn clear_service_name(&mut self) {
        self.port_specifier = ::std::option::Option::None;
    }

    pub fn has_service_name(&self) -> bool {
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(_)) = self.port_specifier {
        } else {
            self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(::std::string::String::new()));
        }
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        if self.has_service_name() {
            match self.port_specifier.take() {
                ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_service_name(&self) -> &str {
        match self.port_specifier {
            ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for UnresolvedAddress_NamedAddress {
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
                    };
                    let tmp = is.read_enum()?;
                    self.protocol = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::port(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.port_specifier = ::std::option::Option::Some(UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(is.read_string()?));
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
        if self.protocol != UnresolvedAddress_NamedAddress_Protocol::TCP {
            my_size += ::protobuf::rt::enum_size(1, self.protocol);
        };
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.address);
        };
        if let ::std::option::Option::Some(ref v) = self.port_specifier {
            match v {
                &UnresolvedAddress_NamedAddress_oneof_port_specifier::port(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.protocol != UnresolvedAddress_NamedAddress_Protocol::TCP {
            os.write_enum(1, self.protocol.value())?;
        };
        if !self.address.is_empty() {
            os.write_string(2, &self.address)?;
        };
        if let ::std::option::Option::Some(ref v) = self.port_specifier {
            match v {
                &UnresolvedAddress_NamedAddress_oneof_port_specifier::port(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &UnresolvedAddress_NamedAddress_oneof_port_specifier::service_name(ref v) => {
                    os.write_string(4, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for UnresolvedAddress_NamedAddress {
    fn new() -> UnresolvedAddress_NamedAddress {
        UnresolvedAddress_NamedAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnresolvedAddress_NamedAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<UnresolvedAddress_NamedAddress_Protocol>>(
                    "protocol",
                    UnresolvedAddress_NamedAddress::get_protocol_for_reflect,
                    UnresolvedAddress_NamedAddress::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    UnresolvedAddress_NamedAddress::get_address_for_reflect,
                    UnresolvedAddress_NamedAddress::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::wrappers::UInt32Value>(
                    "port",
                    UnresolvedAddress_NamedAddress::has_port,
                    UnresolvedAddress_NamedAddress::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "service_name",
                    UnresolvedAddress_NamedAddress::has_service_name,
                    UnresolvedAddress_NamedAddress::get_service_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnresolvedAddress_NamedAddress>(
                    "UnresolvedAddress_NamedAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnresolvedAddress_NamedAddress {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_address();
        self.clear_port();
        self.clear_service_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnresolvedAddress_NamedAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnresolvedAddress_NamedAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UnresolvedAddress_NamedAddress_Protocol {
    TCP = 0,
}

impl ::protobuf::ProtobufEnum for UnresolvedAddress_NamedAddress_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UnresolvedAddress_NamedAddress_Protocol> {
        match value {
            0 => ::std::option::Option::Some(UnresolvedAddress_NamedAddress_Protocol::TCP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UnresolvedAddress_NamedAddress_Protocol] = &[
            UnresolvedAddress_NamedAddress_Protocol::TCP,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<UnresolvedAddress_NamedAddress_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("UnresolvedAddress_NamedAddress_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for UnresolvedAddress_NamedAddress_Protocol {
}

impl ::std::default::Default for UnresolvedAddress_NamedAddress_Protocol {
    fn default() -> Self {
        UnresolvedAddress_NamedAddress_Protocol::TCP
    }
}

impl ::protobuf::reflect::ProtobufValue for UnresolvedAddress_NamedAddress_Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnresolvedAddress_Resolver {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnresolvedAddress_Resolver {}

impl UnresolvedAddress_Resolver {
    pub fn new() -> UnresolvedAddress_Resolver {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnresolvedAddress_Resolver {
        static mut instance: ::protobuf::lazy::Lazy<UnresolvedAddress_Resolver> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnresolvedAddress_Resolver,
        };
        unsafe {
            instance.get(UnresolvedAddress_Resolver::new)
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
}

impl ::protobuf::Message for UnresolvedAddress_Resolver {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for UnresolvedAddress_Resolver {
    fn new() -> UnresolvedAddress_Resolver {
        UnresolvedAddress_Resolver::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnresolvedAddress_Resolver>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    UnresolvedAddress_Resolver::get_name_for_reflect,
                    UnresolvedAddress_Resolver::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnresolvedAddress_Resolver>(
                    "UnresolvedAddress_Resolver",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnresolvedAddress_Resolver {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnresolvedAddress_Resolver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnresolvedAddress_Resolver {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnresolvedAddresses {
    // message fields
    addresses: ::protobuf::RepeatedField<UnresolvedAddress>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnresolvedAddresses {}

impl UnresolvedAddresses {
    pub fn new() -> UnresolvedAddresses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnresolvedAddresses {
        static mut instance: ::protobuf::lazy::Lazy<UnresolvedAddresses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnresolvedAddresses,
        };
        unsafe {
            instance.get(UnresolvedAddresses::new)
        }
    }

    // repeated .envoy.api.v2.UnresolvedAddress addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<UnresolvedAddress>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<UnresolvedAddress> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<UnresolvedAddress> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[UnresolvedAddress] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<UnresolvedAddress> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UnresolvedAddress> {
        &mut self.addresses
    }
}

impl ::protobuf::Message for UnresolvedAddresses {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.addresses)?;
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
        for value in &self.addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
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

impl ::protobuf::MessageStatic for UnresolvedAddresses {
    fn new() -> UnresolvedAddresses {
        UnresolvedAddresses::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnresolvedAddresses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnresolvedAddress>>(
                    "addresses",
                    UnresolvedAddresses::get_addresses_for_reflect,
                    UnresolvedAddresses::mut_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnresolvedAddresses>(
                    "UnresolvedAddresses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnresolvedAddresses {
    fn clear(&mut self) {
        self.clear_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnresolvedAddresses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnresolvedAddresses {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolvedAddress {
    // message oneof groups
    address: ::std::option::Option<ResolvedAddress_oneof_address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolvedAddress {}

#[derive(Clone,PartialEq)]
pub enum ResolvedAddress_oneof_address {
    socket_address(ResolvedAddress_SocketAddress),
    pipe(Pipe),
}

impl ResolvedAddress {
    pub fn new() -> ResolvedAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolvedAddress {
        static mut instance: ::protobuf::lazy::Lazy<ResolvedAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolvedAddress,
        };
        unsafe {
            instance.get(ResolvedAddress::new)
        }
    }

    // .envoy.api.v2.ResolvedAddress.SocketAddress socket_address = 2;

    pub fn clear_socket_address(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_socket_address(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_socket_address(&mut self, v: ResolvedAddress_SocketAddress) {
        self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(v))
    }

    // Mutable pointer to the field.
    pub fn mut_socket_address(&mut self) -> &mut ResolvedAddress_SocketAddress {
        if let ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(ResolvedAddress_SocketAddress::new()));
        }
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_socket_address(&mut self) -> ResolvedAddress_SocketAddress {
        if self.has_socket_address() {
            match self.address.take() {
                ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(v)) => v,
                _ => panic!(),
            }
        } else {
            ResolvedAddress_SocketAddress::new()
        }
    }

    pub fn get_socket_address(&self) -> &ResolvedAddress_SocketAddress {
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(ref v)) => v,
            _ => ResolvedAddress_SocketAddress::default_instance(),
        }
    }

    // .envoy.api.v2.Pipe pipe = 3;

    pub fn clear_pipe(&mut self) {
        self.address = ::std::option::Option::None;
    }

    pub fn has_pipe(&self) -> bool {
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_pipe(&mut self, v: Pipe) {
        self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(v))
    }

    // Mutable pointer to the field.
    pub fn mut_pipe(&mut self) -> &mut Pipe {
        if let ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(_)) = self.address {
        } else {
            self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(Pipe::new()));
        }
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_pipe(&mut self) -> Pipe {
        if self.has_pipe() {
            match self.address.take() {
                ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(v)) => v,
                _ => panic!(),
            }
        } else {
            Pipe::new()
        }
    }

    pub fn get_pipe(&self) -> &Pipe {
        match self.address {
            ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(ref v)) => v,
            _ => Pipe::default_instance(),
        }
    }
}

impl ::protobuf::Message for ResolvedAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::socket_address(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.address = ::std::option::Option::Some(ResolvedAddress_oneof_address::pipe(is.read_message()?));
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
                &ResolvedAddress_oneof_address::socket_address(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResolvedAddress_oneof_address::pipe(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.address {
            match v {
                &ResolvedAddress_oneof_address::socket_address(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ResolvedAddress_oneof_address::pipe(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
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

impl ::protobuf::MessageStatic for ResolvedAddress {
    fn new() -> ResolvedAddress {
        ResolvedAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolvedAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResolvedAddress_SocketAddress>(
                    "socket_address",
                    ResolvedAddress::has_socket_address,
                    ResolvedAddress::get_socket_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Pipe>(
                    "pipe",
                    ResolvedAddress::has_pipe,
                    ResolvedAddress::get_pipe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolvedAddress>(
                    "ResolvedAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolvedAddress {
    fn clear(&mut self) {
        self.clear_socket_address();
        self.clear_pipe();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolvedAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolvedAddress_SocketAddress {
    // message fields
    pub protocol: ResolvedAddress_SocketAddress_Protocol,
    pub ip_address: ::std::string::String,
    port: ::protobuf::SingularPtrField<super::wrappers::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolvedAddress_SocketAddress {}

impl ResolvedAddress_SocketAddress {
    pub fn new() -> ResolvedAddress_SocketAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolvedAddress_SocketAddress {
        static mut instance: ::protobuf::lazy::Lazy<ResolvedAddress_SocketAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolvedAddress_SocketAddress,
        };
        unsafe {
            instance.get(ResolvedAddress_SocketAddress::new)
        }
    }

    // .envoy.api.v2.ResolvedAddress.SocketAddress.Protocol protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = ResolvedAddress_SocketAddress_Protocol::TCP;
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ResolvedAddress_SocketAddress_Protocol) {
        self.protocol = v;
    }

    pub fn get_protocol(&self) -> ResolvedAddress_SocketAddress_Protocol {
        self.protocol
    }

    fn get_protocol_for_reflect(&self) -> &ResolvedAddress_SocketAddress_Protocol {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ResolvedAddress_SocketAddress_Protocol {
        &mut self.protocol
    }

    // string ip_address = 2;

    pub fn clear_ip_address(&mut self) {
        self.ip_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip_address(&mut self, v: ::std::string::String) {
        self.ip_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip_address(&mut self) -> &mut ::std::string::String {
        &mut self.ip_address
    }

    // Take field
    pub fn take_ip_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip_address, ::std::string::String::new())
    }

    pub fn get_ip_address(&self) -> &str {
        &self.ip_address
    }

    fn get_ip_address_for_reflect(&self) -> &::std::string::String {
        &self.ip_address
    }

    fn mut_ip_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ip_address
    }

    // .google.protobuf.UInt32Value port = 3;

    pub fn clear_port(&mut self) {
        self.port.clear();
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: super::wrappers::UInt32Value) {
        self.port = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_port(&mut self) -> &mut super::wrappers::UInt32Value {
        if self.port.is_none() {
            self.port.set_default();
        };
        self.port.as_mut().unwrap()
    }

    // Take field
    pub fn take_port(&mut self) -> super::wrappers::UInt32Value {
        self.port.take().unwrap_or_else(|| super::wrappers::UInt32Value::new())
    }

    pub fn get_port(&self) -> &super::wrappers::UInt32Value {
        self.port.as_ref().unwrap_or_else(|| super::wrappers::UInt32Value::default_instance())
    }

    fn get_port_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wrappers::UInt32Value> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wrappers::UInt32Value> {
        &mut self.port
    }
}

impl ::protobuf::Message for ResolvedAddress_SocketAddress {
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
                    };
                    let tmp = is.read_enum()?;
                    self.protocol = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip_address)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.port)?;
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
        if self.protocol != ResolvedAddress_SocketAddress_Protocol::TCP {
            my_size += ::protobuf::rt::enum_size(1, self.protocol);
        };
        if !self.ip_address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ip_address);
        };
        if let Some(v) = self.port.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.protocol != ResolvedAddress_SocketAddress_Protocol::TCP {
            os.write_enum(1, self.protocol.value())?;
        };
        if !self.ip_address.is_empty() {
            os.write_string(2, &self.ip_address)?;
        };
        if let Some(v) = self.port.as_ref() {
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

impl ::protobuf::MessageStatic for ResolvedAddress_SocketAddress {
    fn new() -> ResolvedAddress_SocketAddress {
        ResolvedAddress_SocketAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolvedAddress_SocketAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResolvedAddress_SocketAddress_Protocol>>(
                    "protocol",
                    ResolvedAddress_SocketAddress::get_protocol_for_reflect,
                    ResolvedAddress_SocketAddress::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip_address",
                    ResolvedAddress_SocketAddress::get_ip_address_for_reflect,
                    ResolvedAddress_SocketAddress::mut_ip_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wrappers::UInt32Value>>(
                    "port",
                    ResolvedAddress_SocketAddress::get_port_for_reflect,
                    ResolvedAddress_SocketAddress::mut_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolvedAddress_SocketAddress>(
                    "ResolvedAddress_SocketAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolvedAddress_SocketAddress {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_ip_address();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolvedAddress_SocketAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedAddress_SocketAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResolvedAddress_SocketAddress_Protocol {
    TCP = 0,
}

impl ::protobuf::ProtobufEnum for ResolvedAddress_SocketAddress_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResolvedAddress_SocketAddress_Protocol> {
        match value {
            0 => ::std::option::Option::Some(ResolvedAddress_SocketAddress_Protocol::TCP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResolvedAddress_SocketAddress_Protocol] = &[
            ResolvedAddress_SocketAddress_Protocol::TCP,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ResolvedAddress_SocketAddress_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResolvedAddress_SocketAddress_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResolvedAddress_SocketAddress_Protocol {
}

impl ::std::default::Default for ResolvedAddress_SocketAddress_Protocol {
    fn default() -> Self {
        ResolvedAddress_SocketAddress_Protocol::TCP
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedAddress_SocketAddress_Protocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolvedAddresses {
    // message fields
    addresses: ::protobuf::RepeatedField<ResolvedAddress>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolvedAddresses {}

impl ResolvedAddresses {
    pub fn new() -> ResolvedAddresses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolvedAddresses {
        static mut instance: ::protobuf::lazy::Lazy<ResolvedAddresses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolvedAddresses,
        };
        unsafe {
            instance.get(ResolvedAddresses::new)
        }
    }

    // repeated .envoy.api.v2.ResolvedAddress addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<ResolvedAddress>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedAddress> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<ResolvedAddress> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[ResolvedAddress] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<ResolvedAddress> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResolvedAddress> {
        &mut self.addresses
    }
}

impl ::protobuf::Message for ResolvedAddresses {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.addresses)?;
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
        for value in &self.addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
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

impl ::protobuf::MessageStatic for ResolvedAddresses {
    fn new() -> ResolvedAddresses {
        ResolvedAddresses::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolvedAddresses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResolvedAddress>>(
                    "addresses",
                    ResolvedAddresses::get_addresses_for_reflect,
                    ResolvedAddresses::mut_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolvedAddresses>(
                    "ResolvedAddresses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolvedAddresses {
    fn clear(&mut self) {
        self.clear_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolvedAddresses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolvedAddresses {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76,
    0x32, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x1a, 0x0a, 0x04, 0x50, 0x69, 0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74,
    0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x22, 0x81, 0x04,
    0x0a, 0x11, 0x55, 0x6e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x12, 0x44, 0x0a, 0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70,
    0x69, 0x2e, 0x76, 0x32, 0x2e, 0x55, 0x6e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x52,
    0x08, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x12, 0x53, 0x0a, 0x0d, 0x6e, 0x61, 0x6d,
    0x65, 0x64, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x2c, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e,
    0x55, 0x6e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x48, 0x00,
    0x52, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x28,
    0x0a, 0x04, 0x70, 0x69, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x50, 0x69, 0x70, 0x65,
    0x48, 0x00, 0x52, 0x04, 0x70, 0x69, 0x70, 0x65, 0x1a, 0xfb, 0x01, 0x0a, 0x0c, 0x4e, 0x61, 0x6d,
    0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x51, 0x0a, 0x08, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x35, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x55, 0x6e, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x4e, 0x61, 0x6d,
    0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x18, 0x0a, 0x07,
    0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x32, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x48, 0x00, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x48, 0x00, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x22,
    0x13, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x07, 0x0a, 0x03, 0x54,
    0x43, 0x50, 0x10, 0x00, 0x42, 0x10, 0x0a, 0x0e, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x1a, 0x1e, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76,
    0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x42, 0x09, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x22, 0x54, 0x0a, 0x13, 0x55, 0x6e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x3d, 0x0a, 0x09, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x55, 0x6e, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x09, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x22, 0xe6, 0x02, 0x0a, 0x0f, 0x52, 0x65, 0x73, 0x6f,
    0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x54, 0x0a, 0x0e, 0x73,
    0x6f, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e,
    0x76, 0x32, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x2e, 0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x48, 0x00, 0x52, 0x0d, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x12, 0x28, 0x0a, 0x04, 0x70, 0x69, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x50,
    0x69, 0x70, 0x65, 0x48, 0x00, 0x52, 0x04, 0x70, 0x69, 0x70, 0x65, 0x1a, 0xc7, 0x01, 0x0a, 0x0d,
    0x53, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x50, 0x0a,
    0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x34, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x52,
    0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x53,
    0x6f, 0x63, 0x6b, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12,
    0x1d, 0x0a, 0x0a, 0x69, 0x70, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x09, 0x69, 0x70, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x30,
    0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55,
    0x49, 0x6e, 0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74,
    0x22, 0x13, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x07, 0x0a, 0x03,
    0x54, 0x43, 0x50, 0x10, 0x00, 0x42, 0x09, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x22, 0x50, 0x0a, 0x11, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x3b, 0x0a, 0x09, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x09, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x4a, 0x88, 0x13, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x45, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x27, 0x0a, 0x40, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x32, 0x34, 0x20, 0x5b, 0x56, 0x32, 0x2d, 0x41,
    0x50, 0x49, 0x2d, 0x44, 0x49, 0x46, 0x46, 0x5d, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x20, 0x6e, 0x6f, 0x77, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x09, 0x02, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x10, 0x11,
    0x0a, 0x87, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x2c, 0x01, 0x1a, 0x7b, 0x20,
    0x55, 0x6e, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x65, 0x69, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x73, 0x20,
    0x6f, 0x72, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x76, 0x69, 0x61, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20,
    0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04,
    0x0f, 0x02, 0x1e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0f,
    0x0a, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x12, 0x04, 0x10, 0x04,
    0x12, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10,
    0x09, 0x11, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x11, 0x06, 0x0e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x06, 0x09, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x11, 0x0c, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x13, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x13, 0x04, 0x12, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x13, 0x0d, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x13, 0x18, 0x19, 0x0a, 0x9a, 0x02, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x04, 0x17, 0x1a, 0x8a, 0x02, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70,
    0x74, 0x79, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69,
    0x65, 0x73, 0x20, 0x61, 0x20, 0x62, 0x69, 0x6e, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x30, 0x2e, 0x30,
    0x2e, 0x30, 0x2e, 0x30, 0x20, 0x6f, 0x72, 0x20, 0x3a, 0x3a, 0x2e, 0x20, 0x49, 0x74, 0x27, 0x73,
    0x0a, 0x20, 0x73, 0x74, 0x69, 0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20,
    0x6f, 0x6e, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x76, 0x69, 0x61, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x2f, 0x73, 0x75, 0x66, 0x66, 0x69, 0x78,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x46, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x43, 0x68, 0x61, 0x69, 0x6e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61,
    0x66, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x2c, 0x20,
    0x61, 0x6e, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62,
    0x65, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x50, 0x20, 0x6f,
    0x72, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x0a, 0x20, 0x44,
    0x4e, 0x53, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x19, 0x04, 0x13, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x19, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x19, 0x0b, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x19, 0x15, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x08, 0x00, 0x12,
    0x04, 0x1a, 0x04, 0x1d, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x1a, 0x0a, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x1b, 0x06, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x1b, 0x06, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1b, 0x22, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x1b, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x1c, 0x06, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x1c, 0x06, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x1c, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x1c, 0x1c, 0x1d, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x01, 0x12, 0x04, 0x20, 0x02, 0x25,
    0x03, 0x1a, 0x1e, 0x20, 0x53, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x70, 0x6c, 0x75, 0x67,
    0x67, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x01, 0x01, 0x12, 0x03, 0x20, 0x0a, 0x12, 0x0a,
    0xaf, 0x01, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x14, 0x1a,
    0x42, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x75, 0x73,
    0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x45, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x0a, 0x22, 0x5c, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x68, 0x74, 0x75, 0x63, 0x68,
    0x29, 0x3a, 0x20, 0x44, 0x6f, 0x20, 0x77, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x66, 0x75,
    0x72, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x20, 0x6f, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x77, 0x65,
    0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x70, 0x73, 0x65, 0x0a, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x6c,
    0x76, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3f,
    0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x22, 0x04,
    0x20, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22,
    0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x0b, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22,
    0x12, 0x13, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x18, 0x1a,
    0x35, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20,
    0x44, 0x4e, 0x53, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x27, 0x02, 0x25, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x0b,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x16, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x28, 0x02, 0x2b, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x28, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x29, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x29, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x04, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x04, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x2e, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x1d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2f, 0x29, 0x2a, 0x0a, 0x90, 0x01, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34,
    0x00, 0x41, 0x01, 0x1a, 0x83, 0x01, 0x20, 0x41, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x6c, 0x76, 0x65,
    0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x72, 0x65, 0x74, 0x65, 0x20, 0x73,
    0x6f, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x55, 0x44, 0x53, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x75, 0x72, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x76, 0x69, 0x61, 0x20,
    0x44, 0x4e, 0x53, 0x20, 0x6f, 0x72, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x72, 0x65,
    0x73, 0x6f, 0x6c, 0x76, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x34, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x35,
    0x02, 0x3c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x35, 0x0a,
    0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x04, 0x00, 0x12, 0x04, 0x36, 0x04, 0x38,
    0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x36, 0x09,
    0x11, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x37,
    0x06, 0x0e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x37, 0x06, 0x09, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x37, 0x0c, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x39, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x39, 0x04, 0x38, 0x05, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x39, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x39, 0x0d, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x39, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x3a, 0x04, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x3a, 0x04, 0x39, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3a, 0x0b, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x3a, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x3b, 0x04, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x3b, 0x04, 0x3a, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x3b, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x3b, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x3b, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04,
    0x3d, 0x02, 0x40, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x3d,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x04, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e, 0x04, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x3f, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x3f, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3f, 0x10, 0x11,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x43, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x43, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x44, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x0b,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x1b, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x27, 0x28, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
