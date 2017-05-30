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
pub struct DataSource {
    // message oneof groups
    specifier: ::std::option::Option<DataSource_oneof_specifier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataSource {}

#[derive(Clone,PartialEq)]
pub enum DataSource_oneof_specifier {
    filename(::std::string::String),
    inline(::std::vec::Vec<u8>),
}

impl DataSource {
    pub fn new() -> DataSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataSource {
        static mut instance: ::protobuf::lazy::Lazy<DataSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataSource,
        };
        unsafe {
            instance.get(DataSource::new)
        }
    }

    // string filename = 1;

    pub fn clear_filename(&mut self) {
        self.specifier = ::std::option::Option::None;
    }

    pub fn has_filename(&self) -> bool {
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::filename(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::filename(v))
    }

    // Mutable pointer to the field.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(DataSource_oneof_specifier::filename(_)) = self.specifier {
        } else {
            self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::filename(::std::string::String::new()));
        }
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::filename(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        if self.has_filename() {
            match self.specifier.take() {
                ::std::option::Option::Some(DataSource_oneof_specifier::filename(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_filename(&self) -> &str {
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::filename(ref v)) => v,
            _ => "",
        }
    }

    // bytes inline = 2;

    pub fn clear_inline(&mut self) {
        self.specifier = ::std::option::Option::None;
    }

    pub fn has_inline(&self) -> bool {
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::inline(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_inline(&mut self, v: ::std::vec::Vec<u8>) {
        self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::inline(v))
    }

    // Mutable pointer to the field.
    pub fn mut_inline(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(DataSource_oneof_specifier::inline(_)) = self.specifier {
        } else {
            self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::inline(::std::vec::Vec::new()));
        }
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::inline(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_inline(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_inline() {
            match self.specifier.take() {
                ::std::option::Option::Some(DataSource_oneof_specifier::inline(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_inline(&self) -> &[u8] {
        match self.specifier {
            ::std::option::Option::Some(DataSource_oneof_specifier::inline(ref v)) => v,
            _ => &[],
        }
    }
}

impl ::protobuf::Message for DataSource {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::filename(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::inline(is.read_bytes()?));
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
        if let ::std::option::Option::Some(ref v) = self.specifier {
            match v {
                &DataSource_oneof_specifier::filename(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &DataSource_oneof_specifier::inline(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.specifier {
            match v {
                &DataSource_oneof_specifier::filename(ref v) => {
                    os.write_string(1, v)?;
                },
                &DataSource_oneof_specifier::inline(ref v) => {
                    os.write_bytes(2, v)?;
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

impl ::protobuf::MessageStatic for DataSource {
    fn new() -> DataSource {
        DataSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "filename",
                    DataSource::has_filename,
                    DataSource::get_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "inline",
                    DataSource::has_inline,
                    DataSource::get_inline,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataSource>(
                    "DataSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataSource {
    fn clear(&mut self) {
        self.clear_filename();
        self.clear_inline();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TlsParameters {
    // message fields
    pub tls_minimum_protocol_version: TlsParameters_TlsProtocol,
    pub tls_maximum_protocol_version: TlsParameters_TlsProtocol,
    cipher_suites: ::protobuf::RepeatedField<::std::string::String>,
    ecdh_curves: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TlsParameters {}

impl TlsParameters {
    pub fn new() -> TlsParameters {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TlsParameters {
        static mut instance: ::protobuf::lazy::Lazy<TlsParameters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TlsParameters,
        };
        unsafe {
            instance.get(TlsParameters::new)
        }
    }

    // .envoy.api.v2.TlsParameters.TlsProtocol tls_minimum_protocol_version = 1;

    pub fn clear_tls_minimum_protocol_version(&mut self) {
        self.tls_minimum_protocol_version = TlsParameters_TlsProtocol::TLS_AUTO;
    }

    // Param is passed by value, moved
    pub fn set_tls_minimum_protocol_version(&mut self, v: TlsParameters_TlsProtocol) {
        self.tls_minimum_protocol_version = v;
    }

    pub fn get_tls_minimum_protocol_version(&self) -> TlsParameters_TlsProtocol {
        self.tls_minimum_protocol_version
    }

    fn get_tls_minimum_protocol_version_for_reflect(&self) -> &TlsParameters_TlsProtocol {
        &self.tls_minimum_protocol_version
    }

    fn mut_tls_minimum_protocol_version_for_reflect(&mut self) -> &mut TlsParameters_TlsProtocol {
        &mut self.tls_minimum_protocol_version
    }

    // .envoy.api.v2.TlsParameters.TlsProtocol tls_maximum_protocol_version = 2;

    pub fn clear_tls_maximum_protocol_version(&mut self) {
        self.tls_maximum_protocol_version = TlsParameters_TlsProtocol::TLS_AUTO;
    }

    // Param is passed by value, moved
    pub fn set_tls_maximum_protocol_version(&mut self, v: TlsParameters_TlsProtocol) {
        self.tls_maximum_protocol_version = v;
    }

    pub fn get_tls_maximum_protocol_version(&self) -> TlsParameters_TlsProtocol {
        self.tls_maximum_protocol_version
    }

    fn get_tls_maximum_protocol_version_for_reflect(&self) -> &TlsParameters_TlsProtocol {
        &self.tls_maximum_protocol_version
    }

    fn mut_tls_maximum_protocol_version_for_reflect(&mut self) -> &mut TlsParameters_TlsProtocol {
        &mut self.tls_maximum_protocol_version
    }

    // repeated string cipher_suites = 3;

    pub fn clear_cipher_suites(&mut self) {
        self.cipher_suites.clear();
    }

    // Param is passed by value, moved
    pub fn set_cipher_suites(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.cipher_suites = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cipher_suites(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cipher_suites
    }

    // Take field
    pub fn take_cipher_suites(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.cipher_suites, ::protobuf::RepeatedField::new())
    }

    pub fn get_cipher_suites(&self) -> &[::std::string::String] {
        &self.cipher_suites
    }

    fn get_cipher_suites_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.cipher_suites
    }

    fn mut_cipher_suites_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cipher_suites
    }

    // repeated string ecdh_curves = 4;

    pub fn clear_ecdh_curves(&mut self) {
        self.ecdh_curves.clear();
    }

    // Param is passed by value, moved
    pub fn set_ecdh_curves(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ecdh_curves = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ecdh_curves(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ecdh_curves
    }

    // Take field
    pub fn take_ecdh_curves(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ecdh_curves, ::protobuf::RepeatedField::new())
    }

    pub fn get_ecdh_curves(&self) -> &[::std::string::String] {
        &self.ecdh_curves
    }

    fn get_ecdh_curves_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.ecdh_curves
    }

    fn mut_ecdh_curves_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ecdh_curves
    }
}

impl ::protobuf::Message for TlsParameters {
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
                    self.tls_minimum_protocol_version = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.tls_maximum_protocol_version = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.cipher_suites)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ecdh_curves)?;
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
        if self.tls_minimum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            my_size += ::protobuf::rt::enum_size(1, self.tls_minimum_protocol_version);
        };
        if self.tls_maximum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            my_size += ::protobuf::rt::enum_size(2, self.tls_maximum_protocol_version);
        };
        for value in &self.cipher_suites {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.ecdh_curves {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.tls_minimum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            os.write_enum(1, self.tls_minimum_protocol_version.value())?;
        };
        if self.tls_maximum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            os.write_enum(2, self.tls_maximum_protocol_version.value())?;
        };
        for v in &self.cipher_suites {
            os.write_string(3, &v)?;
        };
        for v in &self.ecdh_curves {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for TlsParameters {
    fn new() -> TlsParameters {
        TlsParameters::new()
    }

    fn descriptor_static(_: ::std::option::Option<TlsParameters>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TlsParameters_TlsProtocol>>(
                    "tls_minimum_protocol_version",
                    TlsParameters::get_tls_minimum_protocol_version_for_reflect,
                    TlsParameters::mut_tls_minimum_protocol_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TlsParameters_TlsProtocol>>(
                    "tls_maximum_protocol_version",
                    TlsParameters::get_tls_maximum_protocol_version_for_reflect,
                    TlsParameters::mut_tls_maximum_protocol_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cipher_suites",
                    TlsParameters::get_cipher_suites_for_reflect,
                    TlsParameters::mut_cipher_suites_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ecdh_curves",
                    TlsParameters::get_ecdh_curves_for_reflect,
                    TlsParameters::mut_ecdh_curves_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TlsParameters>(
                    "TlsParameters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TlsParameters {
    fn clear(&mut self) {
        self.clear_tls_minimum_protocol_version();
        self.clear_tls_maximum_protocol_version();
        self.clear_cipher_suites();
        self.clear_ecdh_curves();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TlsParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TlsParameters {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TlsParameters_TlsProtocol {
    TLS_AUTO = 0,
    TLSv1_0 = 1,
    TLSv1_1 = 2,
    TLSv1_2 = 3,
    TLSv1_3 = 4,
}

impl ::protobuf::ProtobufEnum for TlsParameters_TlsProtocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TlsParameters_TlsProtocol> {
        match value {
            0 => ::std::option::Option::Some(TlsParameters_TlsProtocol::TLS_AUTO),
            1 => ::std::option::Option::Some(TlsParameters_TlsProtocol::TLSv1_0),
            2 => ::std::option::Option::Some(TlsParameters_TlsProtocol::TLSv1_1),
            3 => ::std::option::Option::Some(TlsParameters_TlsProtocol::TLSv1_2),
            4 => ::std::option::Option::Some(TlsParameters_TlsProtocol::TLSv1_3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TlsParameters_TlsProtocol] = &[
            TlsParameters_TlsProtocol::TLS_AUTO,
            TlsParameters_TlsProtocol::TLSv1_0,
            TlsParameters_TlsProtocol::TLSv1_1,
            TlsParameters_TlsProtocol::TLSv1_2,
            TlsParameters_TlsProtocol::TLSv1_3,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<TlsParameters_TlsProtocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TlsParameters_TlsProtocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for TlsParameters_TlsProtocol {
}

impl ::std::default::Default for TlsParameters_TlsProtocol {
    fn default() -> Self {
        TlsParameters_TlsProtocol::TLS_AUTO
    }
}

impl ::protobuf::reflect::ProtobufValue for TlsParameters_TlsProtocol {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TlsCertificate {
    // message fields
    cert_chain: ::protobuf::SingularPtrField<DataSource>,
    private_key: ::protobuf::SingularPtrField<DataSource>,
    ocsp_staple: ::protobuf::SingularPtrField<DataSource>,
    signed_certificate_timestamp: ::protobuf::RepeatedField<DataSource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TlsCertificate {}

impl TlsCertificate {
    pub fn new() -> TlsCertificate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TlsCertificate {
        static mut instance: ::protobuf::lazy::Lazy<TlsCertificate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TlsCertificate,
        };
        unsafe {
            instance.get(TlsCertificate::new)
        }
    }

    // .envoy.api.v2.DataSource cert_chain = 1;

    pub fn clear_cert_chain(&mut self) {
        self.cert_chain.clear();
    }

    pub fn has_cert_chain(&self) -> bool {
        self.cert_chain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert_chain(&mut self, v: DataSource) {
        self.cert_chain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert_chain(&mut self) -> &mut DataSource {
        if self.cert_chain.is_none() {
            self.cert_chain.set_default();
        };
        self.cert_chain.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert_chain(&mut self) -> DataSource {
        self.cert_chain.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_cert_chain(&self) -> &DataSource {
        self.cert_chain.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_cert_chain_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.cert_chain
    }

    fn mut_cert_chain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.cert_chain
    }

    // .envoy.api.v2.DataSource private_key = 2;

    pub fn clear_private_key(&mut self) {
        self.private_key.clear();
    }

    pub fn has_private_key(&self) -> bool {
        self.private_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_key(&mut self, v: DataSource) {
        self.private_key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_key(&mut self) -> &mut DataSource {
        if self.private_key.is_none() {
            self.private_key.set_default();
        };
        self.private_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_key(&mut self) -> DataSource {
        self.private_key.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_private_key(&self) -> &DataSource {
        self.private_key.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_private_key_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.private_key
    }

    fn mut_private_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.private_key
    }

    // .envoy.api.v2.DataSource ocsp_staple = 3;

    pub fn clear_ocsp_staple(&mut self) {
        self.ocsp_staple.clear();
    }

    pub fn has_ocsp_staple(&self) -> bool {
        self.ocsp_staple.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ocsp_staple(&mut self, v: DataSource) {
        self.ocsp_staple = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ocsp_staple(&mut self) -> &mut DataSource {
        if self.ocsp_staple.is_none() {
            self.ocsp_staple.set_default();
        };
        self.ocsp_staple.as_mut().unwrap()
    }

    // Take field
    pub fn take_ocsp_staple(&mut self) -> DataSource {
        self.ocsp_staple.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_ocsp_staple(&self) -> &DataSource {
        self.ocsp_staple.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_ocsp_staple_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.ocsp_staple
    }

    fn mut_ocsp_staple_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.ocsp_staple
    }

    // repeated .envoy.api.v2.DataSource signed_certificate_timestamp = 4;

    pub fn clear_signed_certificate_timestamp(&mut self) {
        self.signed_certificate_timestamp.clear();
    }

    // Param is passed by value, moved
    pub fn set_signed_certificate_timestamp(&mut self, v: ::protobuf::RepeatedField<DataSource>) {
        self.signed_certificate_timestamp = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signed_certificate_timestamp(&mut self) -> &mut ::protobuf::RepeatedField<DataSource> {
        &mut self.signed_certificate_timestamp
    }

    // Take field
    pub fn take_signed_certificate_timestamp(&mut self) -> ::protobuf::RepeatedField<DataSource> {
        ::std::mem::replace(&mut self.signed_certificate_timestamp, ::protobuf::RepeatedField::new())
    }

    pub fn get_signed_certificate_timestamp(&self) -> &[DataSource] {
        &self.signed_certificate_timestamp
    }

    fn get_signed_certificate_timestamp_for_reflect(&self) -> &::protobuf::RepeatedField<DataSource> {
        &self.signed_certificate_timestamp
    }

    fn mut_signed_certificate_timestamp_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DataSource> {
        &mut self.signed_certificate_timestamp
    }
}

impl ::protobuf::Message for TlsCertificate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert_chain)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.private_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ocsp_staple)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.signed_certificate_timestamp)?;
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
        if let Some(v) = self.cert_chain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.private_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.ocsp_staple.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.signed_certificate_timestamp {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cert_chain.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.private_key.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.ocsp_staple.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.signed_certificate_timestamp {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TlsCertificate {
    fn new() -> TlsCertificate {
        TlsCertificate::new()
    }

    fn descriptor_static(_: ::std::option::Option<TlsCertificate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "cert_chain",
                    TlsCertificate::get_cert_chain_for_reflect,
                    TlsCertificate::mut_cert_chain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "private_key",
                    TlsCertificate::get_private_key_for_reflect,
                    TlsCertificate::mut_private_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "ocsp_staple",
                    TlsCertificate::get_ocsp_staple_for_reflect,
                    TlsCertificate::mut_ocsp_staple_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "signed_certificate_timestamp",
                    TlsCertificate::get_signed_certificate_timestamp_for_reflect,
                    TlsCertificate::mut_signed_certificate_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TlsCertificate>(
                    "TlsCertificate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TlsCertificate {
    fn clear(&mut self) {
        self.clear_cert_chain();
        self.clear_private_key();
        self.clear_ocsp_staple();
        self.clear_signed_certificate_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TlsCertificate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TlsCertificate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateValidationContext {
    // message fields
    ca_cert: ::protobuf::SingularPtrField<DataSource>,
    verify_certificate_hash: ::protobuf::RepeatedField<::std::string::String>,
    verify_spki_sha256: ::protobuf::RepeatedField<::std::string::String>,
    verify_subject_alt_name: ::protobuf::RepeatedField<::std::string::String>,
    require_ocsp_staple: ::protobuf::SingularPtrField<super::wrappers::BoolValue>,
    require_signed_certificate_timestamp: ::protobuf::SingularPtrField<super::wrappers::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateValidationContext {}

impl CertificateValidationContext {
    pub fn new() -> CertificateValidationContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateValidationContext {
        static mut instance: ::protobuf::lazy::Lazy<CertificateValidationContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateValidationContext,
        };
        unsafe {
            instance.get(CertificateValidationContext::new)
        }
    }

    // .envoy.api.v2.DataSource ca_cert = 1;

    pub fn clear_ca_cert(&mut self) {
        self.ca_cert.clear();
    }

    pub fn has_ca_cert(&self) -> bool {
        self.ca_cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ca_cert(&mut self, v: DataSource) {
        self.ca_cert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ca_cert(&mut self) -> &mut DataSource {
        if self.ca_cert.is_none() {
            self.ca_cert.set_default();
        };
        self.ca_cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_ca_cert(&mut self) -> DataSource {
        self.ca_cert.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_ca_cert(&self) -> &DataSource {
        self.ca_cert.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_ca_cert_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.ca_cert
    }

    fn mut_ca_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.ca_cert
    }

    // repeated string verify_certificate_hash = 2;

    pub fn clear_verify_certificate_hash(&mut self) {
        self.verify_certificate_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_verify_certificate_hash(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.verify_certificate_hash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_verify_certificate_hash(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_certificate_hash
    }

    // Take field
    pub fn take_verify_certificate_hash(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.verify_certificate_hash, ::protobuf::RepeatedField::new())
    }

    pub fn get_verify_certificate_hash(&self) -> &[::std::string::String] {
        &self.verify_certificate_hash
    }

    fn get_verify_certificate_hash_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.verify_certificate_hash
    }

    fn mut_verify_certificate_hash_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_certificate_hash
    }

    // repeated string verify_spki_sha256 = 3;

    pub fn clear_verify_spki_sha256(&mut self) {
        self.verify_spki_sha256.clear();
    }

    // Param is passed by value, moved
    pub fn set_verify_spki_sha256(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.verify_spki_sha256 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_verify_spki_sha256(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_spki_sha256
    }

    // Take field
    pub fn take_verify_spki_sha256(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.verify_spki_sha256, ::protobuf::RepeatedField::new())
    }

    pub fn get_verify_spki_sha256(&self) -> &[::std::string::String] {
        &self.verify_spki_sha256
    }

    fn get_verify_spki_sha256_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.verify_spki_sha256
    }

    fn mut_verify_spki_sha256_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_spki_sha256
    }

    // repeated string verify_subject_alt_name = 4;

    pub fn clear_verify_subject_alt_name(&mut self) {
        self.verify_subject_alt_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_verify_subject_alt_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.verify_subject_alt_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_verify_subject_alt_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_subject_alt_name
    }

    // Take field
    pub fn take_verify_subject_alt_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.verify_subject_alt_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_verify_subject_alt_name(&self) -> &[::std::string::String] {
        &self.verify_subject_alt_name
    }

    fn get_verify_subject_alt_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.verify_subject_alt_name
    }

    fn mut_verify_subject_alt_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verify_subject_alt_name
    }

    // .google.protobuf.BoolValue require_ocsp_staple = 5;

    pub fn clear_require_ocsp_staple(&mut self) {
        self.require_ocsp_staple.clear();
    }

    pub fn has_require_ocsp_staple(&self) -> bool {
        self.require_ocsp_staple.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_ocsp_staple(&mut self, v: super::wrappers::BoolValue) {
        self.require_ocsp_staple = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_ocsp_staple(&mut self) -> &mut super::wrappers::BoolValue {
        if self.require_ocsp_staple.is_none() {
            self.require_ocsp_staple.set_default();
        };
        self.require_ocsp_staple.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_ocsp_staple(&mut self) -> super::wrappers::BoolValue {
        self.require_ocsp_staple.take().unwrap_or_else(|| super::wrappers::BoolValue::new())
    }

    pub fn get_require_ocsp_staple(&self) -> &super::wrappers::BoolValue {
        self.require_ocsp_staple.as_ref().unwrap_or_else(|| super::wrappers::BoolValue::default_instance())
    }

    fn get_require_ocsp_staple_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wrappers::BoolValue> {
        &self.require_ocsp_staple
    }

    fn mut_require_ocsp_staple_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wrappers::BoolValue> {
        &mut self.require_ocsp_staple
    }

    // .google.protobuf.BoolValue require_signed_certificate_timestamp = 6;

    pub fn clear_require_signed_certificate_timestamp(&mut self) {
        self.require_signed_certificate_timestamp.clear();
    }

    pub fn has_require_signed_certificate_timestamp(&self) -> bool {
        self.require_signed_certificate_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_signed_certificate_timestamp(&mut self, v: super::wrappers::BoolValue) {
        self.require_signed_certificate_timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_signed_certificate_timestamp(&mut self) -> &mut super::wrappers::BoolValue {
        if self.require_signed_certificate_timestamp.is_none() {
            self.require_signed_certificate_timestamp.set_default();
        };
        self.require_signed_certificate_timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_signed_certificate_timestamp(&mut self) -> super::wrappers::BoolValue {
        self.require_signed_certificate_timestamp.take().unwrap_or_else(|| super::wrappers::BoolValue::new())
    }

    pub fn get_require_signed_certificate_timestamp(&self) -> &super::wrappers::BoolValue {
        self.require_signed_certificate_timestamp.as_ref().unwrap_or_else(|| super::wrappers::BoolValue::default_instance())
    }

    fn get_require_signed_certificate_timestamp_for_reflect(&self) -> &::protobuf::SingularPtrField<super::wrappers::BoolValue> {
        &self.require_signed_certificate_timestamp
    }

    fn mut_require_signed_certificate_timestamp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::wrappers::BoolValue> {
        &mut self.require_signed_certificate_timestamp
    }
}

impl ::protobuf::Message for CertificateValidationContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ca_cert)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.verify_certificate_hash)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.verify_spki_sha256)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.verify_subject_alt_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.require_ocsp_staple)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.require_signed_certificate_timestamp)?;
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
        if let Some(v) = self.ca_cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.verify_certificate_hash {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.verify_spki_sha256 {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.verify_subject_alt_name {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(v) = self.require_ocsp_staple.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.require_signed_certificate_timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ca_cert.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.verify_certificate_hash {
            os.write_string(2, &v)?;
        };
        for v in &self.verify_spki_sha256 {
            os.write_string(3, &v)?;
        };
        for v in &self.verify_subject_alt_name {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.require_ocsp_staple.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.require_signed_certificate_timestamp.as_ref() {
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

impl ::protobuf::MessageStatic for CertificateValidationContext {
    fn new() -> CertificateValidationContext {
        CertificateValidationContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateValidationContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "ca_cert",
                    CertificateValidationContext::get_ca_cert_for_reflect,
                    CertificateValidationContext::mut_ca_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verify_certificate_hash",
                    CertificateValidationContext::get_verify_certificate_hash_for_reflect,
                    CertificateValidationContext::mut_verify_certificate_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verify_spki_sha256",
                    CertificateValidationContext::get_verify_spki_sha256_for_reflect,
                    CertificateValidationContext::mut_verify_spki_sha256_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verify_subject_alt_name",
                    CertificateValidationContext::get_verify_subject_alt_name_for_reflect,
                    CertificateValidationContext::mut_verify_subject_alt_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wrappers::BoolValue>>(
                    "require_ocsp_staple",
                    CertificateValidationContext::get_require_ocsp_staple_for_reflect,
                    CertificateValidationContext::mut_require_ocsp_staple_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::wrappers::BoolValue>>(
                    "require_signed_certificate_timestamp",
                    CertificateValidationContext::get_require_signed_certificate_timestamp_for_reflect,
                    CertificateValidationContext::mut_require_signed_certificate_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateValidationContext>(
                    "CertificateValidationContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateValidationContext {
    fn clear(&mut self) {
        self.clear_ca_cert();
        self.clear_verify_certificate_hash();
        self.clear_verify_spki_sha256();
        self.clear_verify_subject_alt_name();
        self.clear_require_ocsp_staple();
        self.clear_require_signed_certificate_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateValidationContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateValidationContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpstreamTlsContext {
    // message fields
    tls_params: ::protobuf::SingularPtrField<TlsParameters>,
    client_certificate: ::protobuf::SingularPtrField<TlsCertificate>,
    pub sni: ::std::string::String,
    alpn_protocols: ::protobuf::RepeatedField<::std::string::String>,
    server_validation_context: ::protobuf::SingularPtrField<CertificateValidationContext>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpstreamTlsContext {}

impl UpstreamTlsContext {
    pub fn new() -> UpstreamTlsContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpstreamTlsContext {
        static mut instance: ::protobuf::lazy::Lazy<UpstreamTlsContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpstreamTlsContext,
        };
        unsafe {
            instance.get(UpstreamTlsContext::new)
        }
    }

    // .envoy.api.v2.TlsParameters tls_params = 1;

    pub fn clear_tls_params(&mut self) {
        self.tls_params.clear();
    }

    pub fn has_tls_params(&self) -> bool {
        self.tls_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tls_params(&mut self, v: TlsParameters) {
        self.tls_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_params(&mut self) -> &mut TlsParameters {
        if self.tls_params.is_none() {
            self.tls_params.set_default();
        };
        self.tls_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_tls_params(&mut self) -> TlsParameters {
        self.tls_params.take().unwrap_or_else(|| TlsParameters::new())
    }

    pub fn get_tls_params(&self) -> &TlsParameters {
        self.tls_params.as_ref().unwrap_or_else(|| TlsParameters::default_instance())
    }

    fn get_tls_params_for_reflect(&self) -> &::protobuf::SingularPtrField<TlsParameters> {
        &self.tls_params
    }

    fn mut_tls_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TlsParameters> {
        &mut self.tls_params
    }

    // .envoy.api.v2.TlsCertificate client_certificate = 2;

    pub fn clear_client_certificate(&mut self) {
        self.client_certificate.clear();
    }

    pub fn has_client_certificate(&self) -> bool {
        self.client_certificate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_certificate(&mut self, v: TlsCertificate) {
        self.client_certificate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_certificate(&mut self) -> &mut TlsCertificate {
        if self.client_certificate.is_none() {
            self.client_certificate.set_default();
        };
        self.client_certificate.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_certificate(&mut self) -> TlsCertificate {
        self.client_certificate.take().unwrap_or_else(|| TlsCertificate::new())
    }

    pub fn get_client_certificate(&self) -> &TlsCertificate {
        self.client_certificate.as_ref().unwrap_or_else(|| TlsCertificate::default_instance())
    }

    fn get_client_certificate_for_reflect(&self) -> &::protobuf::SingularPtrField<TlsCertificate> {
        &self.client_certificate
    }

    fn mut_client_certificate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TlsCertificate> {
        &mut self.client_certificate
    }

    // string sni = 3;

    pub fn clear_sni(&mut self) {
        self.sni.clear();
    }

    // Param is passed by value, moved
    pub fn set_sni(&mut self, v: ::std::string::String) {
        self.sni = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sni(&mut self) -> &mut ::std::string::String {
        &mut self.sni
    }

    // Take field
    pub fn take_sni(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sni, ::std::string::String::new())
    }

    pub fn get_sni(&self) -> &str {
        &self.sni
    }

    fn get_sni_for_reflect(&self) -> &::std::string::String {
        &self.sni
    }

    fn mut_sni_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sni
    }

    // repeated string alpn_protocols = 4;

    pub fn clear_alpn_protocols(&mut self) {
        self.alpn_protocols.clear();
    }

    // Param is passed by value, moved
    pub fn set_alpn_protocols(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.alpn_protocols = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alpn_protocols(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alpn_protocols
    }

    // Take field
    pub fn take_alpn_protocols(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.alpn_protocols, ::protobuf::RepeatedField::new())
    }

    pub fn get_alpn_protocols(&self) -> &[::std::string::String] {
        &self.alpn_protocols
    }

    fn get_alpn_protocols_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.alpn_protocols
    }

    fn mut_alpn_protocols_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alpn_protocols
    }

    // .envoy.api.v2.CertificateValidationContext server_validation_context = 5;

    pub fn clear_server_validation_context(&mut self) {
        self.server_validation_context.clear();
    }

    pub fn has_server_validation_context(&self) -> bool {
        self.server_validation_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_validation_context(&mut self, v: CertificateValidationContext) {
        self.server_validation_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_validation_context(&mut self) -> &mut CertificateValidationContext {
        if self.server_validation_context.is_none() {
            self.server_validation_context.set_default();
        };
        self.server_validation_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_validation_context(&mut self) -> CertificateValidationContext {
        self.server_validation_context.take().unwrap_or_else(|| CertificateValidationContext::new())
    }

    pub fn get_server_validation_context(&self) -> &CertificateValidationContext {
        self.server_validation_context.as_ref().unwrap_or_else(|| CertificateValidationContext::default_instance())
    }

    fn get_server_validation_context_for_reflect(&self) -> &::protobuf::SingularPtrField<CertificateValidationContext> {
        &self.server_validation_context
    }

    fn mut_server_validation_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CertificateValidationContext> {
        &mut self.server_validation_context
    }
}

impl ::protobuf::Message for UpstreamTlsContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_params)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_certificate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sni)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.alpn_protocols)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.server_validation_context)?;
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
        if let Some(v) = self.tls_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_certificate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.sni.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.sni);
        };
        for value in &self.alpn_protocols {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(v) = self.server_validation_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tls_params.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_certificate.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.sni.is_empty() {
            os.write_string(3, &self.sni)?;
        };
        for v in &self.alpn_protocols {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.server_validation_context.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for UpstreamTlsContext {
    fn new() -> UpstreamTlsContext {
        UpstreamTlsContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpstreamTlsContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsParameters>>(
                    "tls_params",
                    UpstreamTlsContext::get_tls_params_for_reflect,
                    UpstreamTlsContext::mut_tls_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsCertificate>>(
                    "client_certificate",
                    UpstreamTlsContext::get_client_certificate_for_reflect,
                    UpstreamTlsContext::mut_client_certificate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sni",
                    UpstreamTlsContext::get_sni_for_reflect,
                    UpstreamTlsContext::mut_sni_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alpn_protocols",
                    UpstreamTlsContext::get_alpn_protocols_for_reflect,
                    UpstreamTlsContext::mut_alpn_protocols_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateValidationContext>>(
                    "server_validation_context",
                    UpstreamTlsContext::get_server_validation_context_for_reflect,
                    UpstreamTlsContext::mut_server_validation_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpstreamTlsContext>(
                    "UpstreamTlsContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpstreamTlsContext {
    fn clear(&mut self) {
        self.clear_tls_params();
        self.clear_client_certificate();
        self.clear_sni();
        self.clear_alpn_protocols();
        self.clear_server_validation_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpstreamTlsContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpstreamTlsContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DownstreamTlsContext {
    // message fields
    tls_params: ::protobuf::SingularPtrField<TlsParameters>,
    tls_certificates: ::protobuf::RepeatedField<TlsCertificate>,
    alpn_protocols: ::protobuf::RepeatedField<::std::string::String>,
    client_validation_context: ::protobuf::SingularPtrField<CertificateValidationContext>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DownstreamTlsContext {}

impl DownstreamTlsContext {
    pub fn new() -> DownstreamTlsContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DownstreamTlsContext {
        static mut instance: ::protobuf::lazy::Lazy<DownstreamTlsContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DownstreamTlsContext,
        };
        unsafe {
            instance.get(DownstreamTlsContext::new)
        }
    }

    // .envoy.api.v2.TlsParameters tls_params = 1;

    pub fn clear_tls_params(&mut self) {
        self.tls_params.clear();
    }

    pub fn has_tls_params(&self) -> bool {
        self.tls_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tls_params(&mut self, v: TlsParameters) {
        self.tls_params = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tls_params(&mut self) -> &mut TlsParameters {
        if self.tls_params.is_none() {
            self.tls_params.set_default();
        };
        self.tls_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_tls_params(&mut self) -> TlsParameters {
        self.tls_params.take().unwrap_or_else(|| TlsParameters::new())
    }

    pub fn get_tls_params(&self) -> &TlsParameters {
        self.tls_params.as_ref().unwrap_or_else(|| TlsParameters::default_instance())
    }

    fn get_tls_params_for_reflect(&self) -> &::protobuf::SingularPtrField<TlsParameters> {
        &self.tls_params
    }

    fn mut_tls_params_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TlsParameters> {
        &mut self.tls_params
    }

    // repeated .envoy.api.v2.TlsCertificate tls_certificates = 2;

    pub fn clear_tls_certificates(&mut self) {
        self.tls_certificates.clear();
    }

    // Param is passed by value, moved
    pub fn set_tls_certificates(&mut self, v: ::protobuf::RepeatedField<TlsCertificate>) {
        self.tls_certificates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tls_certificates(&mut self) -> &mut ::protobuf::RepeatedField<TlsCertificate> {
        &mut self.tls_certificates
    }

    // Take field
    pub fn take_tls_certificates(&mut self) -> ::protobuf::RepeatedField<TlsCertificate> {
        ::std::mem::replace(&mut self.tls_certificates, ::protobuf::RepeatedField::new())
    }

    pub fn get_tls_certificates(&self) -> &[TlsCertificate] {
        &self.tls_certificates
    }

    fn get_tls_certificates_for_reflect(&self) -> &::protobuf::RepeatedField<TlsCertificate> {
        &self.tls_certificates
    }

    fn mut_tls_certificates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TlsCertificate> {
        &mut self.tls_certificates
    }

    // repeated string alpn_protocols = 3;

    pub fn clear_alpn_protocols(&mut self) {
        self.alpn_protocols.clear();
    }

    // Param is passed by value, moved
    pub fn set_alpn_protocols(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.alpn_protocols = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alpn_protocols(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alpn_protocols
    }

    // Take field
    pub fn take_alpn_protocols(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.alpn_protocols, ::protobuf::RepeatedField::new())
    }

    pub fn get_alpn_protocols(&self) -> &[::std::string::String] {
        &self.alpn_protocols
    }

    fn get_alpn_protocols_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.alpn_protocols
    }

    fn mut_alpn_protocols_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.alpn_protocols
    }

    // .envoy.api.v2.CertificateValidationContext client_validation_context = 4;

    pub fn clear_client_validation_context(&mut self) {
        self.client_validation_context.clear();
    }

    pub fn has_client_validation_context(&self) -> bool {
        self.client_validation_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_validation_context(&mut self, v: CertificateValidationContext) {
        self.client_validation_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_validation_context(&mut self) -> &mut CertificateValidationContext {
        if self.client_validation_context.is_none() {
            self.client_validation_context.set_default();
        };
        self.client_validation_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_validation_context(&mut self) -> CertificateValidationContext {
        self.client_validation_context.take().unwrap_or_else(|| CertificateValidationContext::new())
    }

    pub fn get_client_validation_context(&self) -> &CertificateValidationContext {
        self.client_validation_context.as_ref().unwrap_or_else(|| CertificateValidationContext::default_instance())
    }

    fn get_client_validation_context_for_reflect(&self) -> &::protobuf::SingularPtrField<CertificateValidationContext> {
        &self.client_validation_context
    }

    fn mut_client_validation_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CertificateValidationContext> {
        &mut self.client_validation_context
    }
}

impl ::protobuf::Message for DownstreamTlsContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_params)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tls_certificates)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.alpn_protocols)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_validation_context)?;
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
        if let Some(v) = self.tls_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tls_certificates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alpn_protocols {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.client_validation_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tls_params.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tls_certificates {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.alpn_protocols {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.client_validation_context.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DownstreamTlsContext {
    fn new() -> DownstreamTlsContext {
        DownstreamTlsContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<DownstreamTlsContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsParameters>>(
                    "tls_params",
                    DownstreamTlsContext::get_tls_params_for_reflect,
                    DownstreamTlsContext::mut_tls_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsCertificate>>(
                    "tls_certificates",
                    DownstreamTlsContext::get_tls_certificates_for_reflect,
                    DownstreamTlsContext::mut_tls_certificates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alpn_protocols",
                    DownstreamTlsContext::get_alpn_protocols_for_reflect,
                    DownstreamTlsContext::mut_alpn_protocols_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateValidationContext>>(
                    "client_validation_context",
                    DownstreamTlsContext::get_client_validation_context_for_reflect,
                    DownstreamTlsContext::mut_client_validation_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DownstreamTlsContext>(
                    "DownstreamTlsContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DownstreamTlsContext {
    fn clear(&mut self) {
        self.clear_tls_params();
        self.clear_tls_certificates();
        self.clear_alpn_protocols();
        self.clear_client_validation_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DownstreamTlsContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DownstreamTlsContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x61, 0x70, 0x69, 0x2f, 0x74, 0x6c, 0x73, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61,
    0x70, 0x69, 0x2e, 0x76, 0x32, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x51, 0x0a, 0x0a, 0x44, 0x61, 0x74, 0x61, 0x53, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x12, 0x1c, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x18, 0x0a, 0x06, 0x69, 0x6e, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x48, 0x00, 0x52, 0x06, 0x69, 0x6e, 0x6c, 0x69, 0x6e, 0x65, 0x42, 0x0b, 0x0a, 0x09, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x22, 0xfa, 0x02, 0x0a, 0x0d, 0x54, 0x6c, 0x73,
    0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x12, 0x68, 0x0a, 0x1c, 0x74, 0x6c,
    0x73, 0x5f, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x27, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e,
    0x54, 0x6c, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x54, 0x6c,
    0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x19, 0x74, 0x6c, 0x73, 0x4d, 0x69,
    0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x68, 0x0a, 0x1c, 0x74, 0x6c, 0x73, 0x5f, 0x6d, 0x61, 0x78, 0x69,
    0x6d, 0x75, 0x6d, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x27, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x6c, 0x73, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x54, 0x6c, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x52, 0x19, 0x74, 0x6c, 0x73, 0x4d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x23,
    0x0a, 0x0d, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x5f, 0x73, 0x75, 0x69, 0x74, 0x65, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0c, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75, 0x69,
    0x74, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x63, 0x64, 0x68, 0x5f, 0x63, 0x75, 0x72, 0x76,
    0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0a, 0x65, 0x63, 0x64, 0x68, 0x43, 0x75,
    0x72, 0x76, 0x65, 0x73, 0x22, 0x4f, 0x0a, 0x0b, 0x54, 0x6c, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x12, 0x0c, 0x0a, 0x08, 0x54, 0x4c, 0x53, 0x5f, 0x41, 0x55, 0x54, 0x4f, 0x10,
    0x00, 0x12, 0x0b, 0x0a, 0x07, 0x54, 0x4c, 0x53, 0x76, 0x31, 0x5f, 0x30, 0x10, 0x01, 0x12, 0x0b,
    0x0a, 0x07, 0x54, 0x4c, 0x53, 0x76, 0x31, 0x5f, 0x31, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x54,
    0x4c, 0x53, 0x76, 0x31, 0x5f, 0x32, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x54, 0x4c, 0x53, 0x76,
    0x31, 0x5f, 0x33, 0x10, 0x04, 0x22, 0x9b, 0x02, 0x0a, 0x0e, 0x54, 0x6c, 0x73, 0x43, 0x65, 0x72,
    0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x37, 0x0a, 0x0a, 0x63, 0x65, 0x72, 0x74,
    0x5f, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x09, 0x63, 0x65, 0x72, 0x74, 0x43, 0x68, 0x61, 0x69,
    0x6e, 0x12, 0x39, 0x0a, 0x0b, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61,
    0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x52, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x39, 0x0a, 0x0b,
    0x6f, 0x63, 0x73, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x70, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32,
    0x2e, 0x44, 0x61, 0x74, 0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x0a, 0x6f, 0x63, 0x73,
    0x70, 0x53, 0x74, 0x61, 0x70, 0x6c, 0x65, 0x12, 0x5a, 0x0a, 0x1c, 0x73, 0x69, 0x67, 0x6e, 0x65,
    0x64, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x1a, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x43,
    0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x22, 0xa7, 0x03, 0x0a, 0x1c, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x12, 0x31, 0x0a, 0x07, 0x63, 0x61, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70,
    0x69, 0x2e, 0x76, 0x32, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52,
    0x06, 0x63, 0x61, 0x43, 0x65, 0x72, 0x74, 0x12, 0x36, 0x0a, 0x17, 0x76, 0x65, 0x72, 0x69, 0x66,
    0x79, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x68, 0x61,
    0x73, 0x68, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x15, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79,
    0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x48, 0x61, 0x73, 0x68, 0x12,
    0x2c, 0x0a, 0x12, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x5f, 0x73, 0x70, 0x6b, 0x69, 0x5f, 0x73,
    0x68, 0x61, 0x32, 0x35, 0x36, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x10, 0x76, 0x65, 0x72,
    0x69, 0x66, 0x79, 0x53, 0x70, 0x6b, 0x69, 0x53, 0x68, 0x61, 0x32, 0x35, 0x36, 0x12, 0x35, 0x0a,
    0x17, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x5f, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f,
    0x61, 0x6c, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x14,
    0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x41, 0x6c, 0x74,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x4a, 0x0a, 0x13, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x5f,
    0x6f, 0x63, 0x73, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x70, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x11, 0x72,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x4f, 0x63, 0x73, 0x70, 0x53, 0x74, 0x61, 0x70, 0x6c, 0x65,
    0x12, 0x6b, 0x0a, 0x24, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x5f, 0x73, 0x69, 0x67, 0x6e,
    0x65, 0x64, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2e, 0x42, 0x6f, 0x6f, 0x6c, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x21, 0x72, 0x65, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0xbe, 0x02,
    0x0a, 0x12, 0x55, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x6c, 0x73, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x12, 0x3a, 0x0a, 0x0a, 0x74, 0x6c, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x6c, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x52, 0x09, 0x74, 0x6c, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x12, 0x4b, 0x0a, 0x12, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x6c, 0x73, 0x43,
    0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x52, 0x11, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x10, 0x0a,
    0x03, 0x73, 0x6e, 0x69, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x73, 0x6e, 0x69, 0x12,
    0x25, 0x0a, 0x0e, 0x61, 0x6c, 0x70, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0d, 0x61, 0x6c, 0x70, 0x6e, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x12, 0x66, 0x0a, 0x19, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x5f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x78, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x52, 0x17, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x56, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x22, 0xaa,
    0x02, 0x0a, 0x14, 0x44, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x6c, 0x73,
    0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x3a, 0x0a, 0x0a, 0x74, 0x6c, 0x73, 0x5f, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x6c, 0x73, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x52, 0x09, 0x74, 0x6c, 0x73, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x73, 0x12, 0x47, 0x0a, 0x10, 0x74, 0x6c, 0x73, 0x5f, 0x63, 0x65, 0x72, 0x74, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x54, 0x6c, 0x73,
    0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x52, 0x0f, 0x74, 0x6c, 0x73,
    0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x12, 0x25, 0x0a, 0x0e,
    0x61, 0x6c, 0x70, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x0d, 0x61, 0x6c, 0x70, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x73, 0x12, 0x66, 0x0a, 0x19, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61,
    0x70, 0x69, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x78, 0x74, 0x52, 0x17, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x4a, 0xe1, 0x1f, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x65, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x04, 0x07, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x12, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x07, 0x02, 0x0a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x08, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x09, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d,
    0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x14, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x10, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x12, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x12, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x13, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x13, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x13, 0x0e, 0x0f, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x16, 0x02, 0x2f, 0x1a, 0x18, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x20, 0x54, 0x4c,
    0x53, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x16, 0x02, 0x14, 0x03, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x16, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0e, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x16, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x17, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x17, 0x02,
    0x16, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x17, 0x02, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x0e, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x2d, 0x2e, 0x0a, 0x5a, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x02, 0x24, 0x1a, 0x4d, 0x20, 0x49, 0x66, 0x20, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x4c,
    0x53, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x63, 0x69, 0x70, 0x68, 0x65,
    0x72, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a,
    0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x23,
    0x0a, 0xa1, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x02, 0x22, 0x1a, 0x93,
    0x01, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x73, 0x75, 0x70,
    0x70, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x20, 0x45, 0x43, 0x44, 0x48, 0x0a, 0x20, 0x63, 0x75, 0x72, 0x76, 0x65, 0x73, 0x2e,
    0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x63,
    0x75, 0x72, 0x76, 0x65, 0x73, 0x20, 0x28, 0x58, 0x32, 0x35, 0x35, 0x31, 0x39, 0x2c, 0x20, 0x50,
    0x2d, 0x32, 0x35, 0x36, 0x29, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1e, 0x20, 0x21, 0x0a, 0x80, 0x01, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x28, 0x01, 0x1a, 0x74, 0x20, 0x54, 0x4c, 0x53, 0x20,
    0x63, 0x65, 0x72, 0x74, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f, 0x61,
    0x64, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x64, 0x65, 0x6c, 0x69, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x6c, 0x69, 0x6e,
    0x65, 0x20, 0x5b, 0x56, 0x32, 0x2d, 0x41, 0x50, 0x49, 0x2d, 0x44, 0x49, 0x46, 0x46, 0x5d, 0x2e,
    0x20, 0x49, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x0a, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x65,
    0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x24, 0x02, 0x23, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x24, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x24, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x25, 0x02, 0x24, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x25, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x26, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x26,
    0x02, 0x25, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x26, 0x02,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x0d, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x27, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x27, 0x16, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x27,
    0x35, 0x36, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2a, 0x00, 0x42, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x24, 0x0a, 0xca, 0x01, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x19, 0x1a, 0xbc, 0x01, 0x20, 0x54, 0x4c, 0x53, 0x20,
    0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x65, 0x72, 0x74,
    0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74,
    0x79, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x75, 0x73, 0x65, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20,
    0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x61, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20,
    0x69, 0x73, 0x0a, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x69, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x76, 0x65, 0x72,
    0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x2e, 0x02, 0x2a, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2e, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e,
    0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x17, 0x18,
    0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x32, 0x02, 0x2e, 0x1a, 0x5f, 0x20,
    0x49, 0x66, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x45, 0x6e,
    0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x20,
    0x28, 0x70, 0x69, 0x6e, 0x29, 0x20, 0x68, 0x65, 0x78, 0x2d, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65,
    0x64, 0x20, 0x53, 0x48, 0x41, 0x2d, 0x32, 0x35, 0x36, 0x20, 0x68, 0x61, 0x73, 0x68, 0x20, 0x6f,
    0x66, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x12, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x32, 0x2c, 0x2d, 0x0a, 0xda, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x37, 0x02, 0x29, 0x1a, 0xcc, 0x01, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x65, 0x64, 0x2c, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x20, 0x28, 0x70, 0x69, 0x6e, 0x29, 0x20, 0x62, 0x61,
    0x73, 0x65, 0x36, 0x34, 0x2d, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x53, 0x48, 0x41,
    0x2d, 0x32, 0x35, 0x36, 0x20, 0x68, 0x61, 0x73, 0x68, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63,
    0x20, 0x4b, 0x65, 0x79, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x28, 0x53, 0x50, 0x4b, 0x49, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x61,
    0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x50,
    0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x50, 0x69, 0x6e, 0x6e, 0x69, 0x6e,
    0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x37, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x12, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x37, 0x27, 0x28, 0x0a, 0xa6, 0x01, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x02, 0x2e, 0x1a, 0x98, 0x01, 0x20, 0x41, 0x6e, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6c, 0x74, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x2c, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x76, 0x65, 0x72,
    0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x65,
    0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0xe2, 0x80, 0x99, 0x73, 0x20, 0x73, 0x75,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x61, 0x6c, 0x74, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3b, 0x2c, 0x2d, 0x0a, 0x40, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x34, 0x1a, 0x33, 0x20, 0x4d, 0x75, 0x73, 0x74,
    0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x65,
    0x64, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x2d, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x65, 0x64, 0x20, 0x4f,
    0x43, 0x53, 0x50, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3e, 0x02, 0x3b, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x3e, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3e, 0x1c, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x3e, 0x32, 0x33, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12,
    0x03, 0x41, 0x02, 0x45, 0x1a, 0x2d, 0x20, 0x4d, 0x75, 0x73, 0x74, 0x20, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x2d, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x04, 0x41, 0x02,
    0x3e, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x06, 0x12, 0x03, 0x41, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x41, 0x1c, 0x40, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x41, 0x43, 0x44, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x44, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x44, 0x08, 0x1a, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x46, 0x02,
    0x1f, 0x1a, 0x2b, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2c, 0x20, 0x63, 0x69, 0x70, 0x68, 0x65,
    0x72, 0x20, 0x73, 0x75, 0x69, 0x74, 0x65, 0x73, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x46, 0x02, 0x44, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x46, 0x1d, 0x1e, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x49, 0x02, 0x28, 0x1a, 0x2b, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x65,
    0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x6e, 0x64, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x49, 0x02, 0x46, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x49, 0x02, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x49, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x49, 0x26, 0x27, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x02, 0x12, 0x03, 0x4c, 0x02, 0x11, 0x1a, 0x3a, 0x20, 0x53, 0x4e, 0x49, 0x20, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x68, 0x65, 0x6e,
    0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x62, 0x61,
    0x63, 0x6b, 0x65, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x4c, 0x02,
    0x49, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x09, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4c, 0x0f, 0x10, 0x0a, 0x2f, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x02, 0x25, 0x1a, 0x22, 0x20, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61,
    0x74, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x41, 0x4c, 0x50, 0x4e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x4f, 0x12, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x4f, 0x23, 0x24, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x52,
    0x02, 0x3d, 0x1a, 0x2a, 0x20, 0x48, 0x6f, 0x77, 0x20, 0x74, 0x6f, 0x20, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x6e, 0x64,
    0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x04, 0x52, 0x02, 0x4f, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x52, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x52, 0x1f, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x52, 0x3b, 0x3c, 0x0a, 0xdc, 0x01, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x58, 0x00, 0x65, 0x01, 0x1a, 0xcf, 0x01, 0x20, 0x5b, 0x56, 0x32, 0x2d, 0x41, 0x50, 0x49, 0x2d,
    0x44, 0x49, 0x46, 0x46, 0x5d, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62,
    0x65, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x63, 0x65,
    0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x2f, 0x6b, 0x65, 0x79, 0x20, 0x64, 0x65,
    0x6c, 0x69, 0x76, 0x65, 0x72, 0x79, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65,
    0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x61, 0x73, 0x70, 0x65, 0x63, 0x74,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x58,
    0x08, 0x1c, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x02, 0x1f, 0x1a,
    0x2b, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2c, 0x20, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x20,
    0x73, 0x75, 0x69, 0x74, 0x65, 0x73, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x5a, 0x02, 0x58, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5a, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5a, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x5a, 0x1d, 0x1e, 0x0a, 0xa1, 0x01, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03,
    0x5e, 0x02, 0x2f, 0x1a, 0x93, 0x01, 0x20, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20,
    0x54, 0x4c, 0x53, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73,
    0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x2c, 0x20, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x74,
    0x6f, 0x0a, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x52, 0x53,
    0x41, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x45, 0x43, 0x44, 0x53, 0x41, 0x20, 0x63, 0x65, 0x72, 0x74,
    0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x53, 0x4e, 0x49, 0x20, 0x5b, 0x56, 0x32, 0x2d, 0x41, 0x50,
    0x49, 0x2d, 0x44, 0x49, 0x46, 0x46, 0x5d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x5e, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x5e, 0x1a, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x2d,
    0x2e, 0x0a, 0x53, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x61, 0x02, 0x25, 0x1a, 0x46,
    0x20, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x4c, 0x50, 0x4e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x65, 0x78,
    0x70, 0x6f, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x61, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x61,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x61, 0x12, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x61, 0x23, 0x24, 0x0a, 0x36,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x64, 0x02, 0x3d, 0x1a, 0x29, 0x20, 0x48, 0x6f,
    0x77, 0x20, 0x74, 0x6f, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x64, 0x02, 0x61, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x64, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x64, 0x1f,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x64, 0x3b, 0x3c, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
