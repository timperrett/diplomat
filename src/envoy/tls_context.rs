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
                    }
                    self.specifier = ::std::option::Option::Some(DataSource_oneof_specifier::filename(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
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
    pub cipher_suites: ::protobuf::RepeatedField<::std::string::String>,
    pub ecdh_curves: ::protobuf::RepeatedField<::std::string::String>,
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
                    }
                    let tmp = is.read_enum()?;
                    self.tls_minimum_protocol_version = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if self.tls_maximum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            my_size += ::protobuf::rt::enum_size(2, self.tls_maximum_protocol_version);
        }
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
        }
        if self.tls_maximum_protocol_version != TlsParameters_TlsProtocol::TLS_AUTO {
            os.write_enum(2, self.tls_maximum_protocol_version.value())?;
        }
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

    fn enum_descriptor_static(_: ::std::option::Option<TlsParameters_TlsProtocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
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
    pub certificate_chain: ::protobuf::SingularPtrField<DataSource>,
    pub private_key: ::protobuf::SingularPtrField<DataSource>,
    pub password: ::protobuf::SingularPtrField<DataSource>,
    pub ocsp_staple: ::protobuf::SingularPtrField<DataSource>,
    pub signed_certificate_timestamp: ::protobuf::RepeatedField<DataSource>,
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

    // .envoy.api.v2.DataSource certificate_chain = 1;

    pub fn clear_certificate_chain(&mut self) {
        self.certificate_chain.clear();
    }

    pub fn has_certificate_chain(&self) -> bool {
        self.certificate_chain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_certificate_chain(&mut self, v: DataSource) {
        self.certificate_chain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_certificate_chain(&mut self) -> &mut DataSource {
        if self.certificate_chain.is_none() {
            self.certificate_chain.set_default();
        }
        self.certificate_chain.as_mut().unwrap()
    }

    // Take field
    pub fn take_certificate_chain(&mut self) -> DataSource {
        self.certificate_chain.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_certificate_chain(&self) -> &DataSource {
        self.certificate_chain.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_certificate_chain_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.certificate_chain
    }

    fn mut_certificate_chain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.certificate_chain
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
        }
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

    // .envoy.api.v2.DataSource password = 3;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: DataSource) {
        self.password = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut DataSource {
        if self.password.is_none() {
            self.password.set_default();
        }
        self.password.as_mut().unwrap()
    }

    // Take field
    pub fn take_password(&mut self) -> DataSource {
        self.password.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_password(&self) -> &DataSource {
        self.password.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_password_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.password
    }

    // .envoy.api.v2.DataSource ocsp_staple = 4;

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
        }
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

    // repeated .envoy.api.v2.DataSource signed_certificate_timestamp = 5;

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
        for v in &self.certificate_chain {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.private_key {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.password {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ocsp_staple {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.signed_certificate_timestamp {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.certificate_chain)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.private_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.password)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ocsp_staple)?;
                },
                5 => {
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
        if let Some(ref v) = self.certificate_chain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.private_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.password.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ocsp_staple.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.signed_certificate_timestamp {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.certificate_chain.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.private_key.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.password.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ocsp_staple.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.signed_certificate_timestamp {
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
                    "certificate_chain",
                    TlsCertificate::get_certificate_chain_for_reflect,
                    TlsCertificate::mut_certificate_chain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "private_key",
                    TlsCertificate::get_private_key_for_reflect,
                    TlsCertificate::mut_private_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataSource>>(
                    "password",
                    TlsCertificate::get_password_for_reflect,
                    TlsCertificate::mut_password_for_reflect,
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
        self.clear_certificate_chain();
        self.clear_private_key();
        self.clear_password();
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
    pub trusted_ca: ::protobuf::SingularPtrField<DataSource>,
    pub verify_certificate_hash: ::protobuf::RepeatedField<::std::string::String>,
    pub verify_spki_sha256: ::protobuf::RepeatedField<::std::string::String>,
    pub verify_subject_alt_name: ::protobuf::RepeatedField<::std::string::String>,
    pub require_ocsp_staple: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub require_signed_certificate_timestamp: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
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

    // .envoy.api.v2.DataSource trusted_ca = 1;

    pub fn clear_trusted_ca(&mut self) {
        self.trusted_ca.clear();
    }

    pub fn has_trusted_ca(&self) -> bool {
        self.trusted_ca.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trusted_ca(&mut self, v: DataSource) {
        self.trusted_ca = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_trusted_ca(&mut self) -> &mut DataSource {
        if self.trusted_ca.is_none() {
            self.trusted_ca.set_default();
        }
        self.trusted_ca.as_mut().unwrap()
    }

    // Take field
    pub fn take_trusted_ca(&mut self) -> DataSource {
        self.trusted_ca.take().unwrap_or_else(|| DataSource::new())
    }

    pub fn get_trusted_ca(&self) -> &DataSource {
        self.trusted_ca.as_ref().unwrap_or_else(|| DataSource::default_instance())
    }

    fn get_trusted_ca_for_reflect(&self) -> &::protobuf::SingularPtrField<DataSource> {
        &self.trusted_ca
    }

    fn mut_trusted_ca_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataSource> {
        &mut self.trusted_ca
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
    pub fn set_require_ocsp_staple(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.require_ocsp_staple = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_ocsp_staple(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.require_ocsp_staple.is_none() {
            self.require_ocsp_staple.set_default();
        }
        self.require_ocsp_staple.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_ocsp_staple(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.require_ocsp_staple.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_require_ocsp_staple(&self) -> &::protobuf::well_known_types::BoolValue {
        self.require_ocsp_staple.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_require_ocsp_staple_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.require_ocsp_staple
    }

    fn mut_require_ocsp_staple_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
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
    pub fn set_require_signed_certificate_timestamp(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.require_signed_certificate_timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_signed_certificate_timestamp(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.require_signed_certificate_timestamp.is_none() {
            self.require_signed_certificate_timestamp.set_default();
        }
        self.require_signed_certificate_timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_signed_certificate_timestamp(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.require_signed_certificate_timestamp.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_require_signed_certificate_timestamp(&self) -> &::protobuf::well_known_types::BoolValue {
        self.require_signed_certificate_timestamp.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_require_signed_certificate_timestamp_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.require_signed_certificate_timestamp
    }

    fn mut_require_signed_certificate_timestamp_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.require_signed_certificate_timestamp
    }
}

impl ::protobuf::Message for CertificateValidationContext {
    fn is_initialized(&self) -> bool {
        for v in &self.trusted_ca {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.require_ocsp_staple {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.require_signed_certificate_timestamp {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.trusted_ca)?;
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
        if let Some(ref v) = self.trusted_ca.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.verify_certificate_hash {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.verify_spki_sha256 {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.verify_subject_alt_name {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.require_ocsp_staple.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.require_signed_certificate_timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.trusted_ca.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.verify_certificate_hash {
            os.write_string(2, &v)?;
        };
        for v in &self.verify_spki_sha256 {
            os.write_string(3, &v)?;
        };
        for v in &self.verify_subject_alt_name {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.require_ocsp_staple.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.require_signed_certificate_timestamp.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                    "trusted_ca",
                    CertificateValidationContext::get_trusted_ca_for_reflect,
                    CertificateValidationContext::mut_trusted_ca_for_reflect,
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "require_ocsp_staple",
                    CertificateValidationContext::get_require_ocsp_staple_for_reflect,
                    CertificateValidationContext::mut_require_ocsp_staple_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
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
        self.clear_trusted_ca();
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
pub struct CommonTlsContext {
    // message fields
    pub tls_params: ::protobuf::SingularPtrField<TlsParameters>,
    pub tls_certificates: ::protobuf::RepeatedField<TlsCertificate>,
    pub validation_context: ::protobuf::SingularPtrField<CertificateValidationContext>,
    pub alpn_protocols: ::protobuf::RepeatedField<::std::string::String>,
    pub deprecated_v1: ::protobuf::SingularPtrField<CommonTlsContext_DeprecatedV1>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommonTlsContext {}

impl CommonTlsContext {
    pub fn new() -> CommonTlsContext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommonTlsContext {
        static mut instance: ::protobuf::lazy::Lazy<CommonTlsContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommonTlsContext,
        };
        unsafe {
            instance.get(CommonTlsContext::new)
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
        }
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

    // .envoy.api.v2.CertificateValidationContext validation_context = 3;

    pub fn clear_validation_context(&mut self) {
        self.validation_context.clear();
    }

    pub fn has_validation_context(&self) -> bool {
        self.validation_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_validation_context(&mut self, v: CertificateValidationContext) {
        self.validation_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validation_context(&mut self) -> &mut CertificateValidationContext {
        if self.validation_context.is_none() {
            self.validation_context.set_default();
        }
        self.validation_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_validation_context(&mut self) -> CertificateValidationContext {
        self.validation_context.take().unwrap_or_else(|| CertificateValidationContext::new())
    }

    pub fn get_validation_context(&self) -> &CertificateValidationContext {
        self.validation_context.as_ref().unwrap_or_else(|| CertificateValidationContext::default_instance())
    }

    fn get_validation_context_for_reflect(&self) -> &::protobuf::SingularPtrField<CertificateValidationContext> {
        &self.validation_context
    }

    fn mut_validation_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CertificateValidationContext> {
        &mut self.validation_context
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

    // .envoy.api.v2.CommonTlsContext.DeprecatedV1 deprecated_v1 = 5;

    pub fn clear_deprecated_v1(&mut self) {
        self.deprecated_v1.clear();
    }

    pub fn has_deprecated_v1(&self) -> bool {
        self.deprecated_v1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated_v1(&mut self, v: CommonTlsContext_DeprecatedV1) {
        self.deprecated_v1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deprecated_v1(&mut self) -> &mut CommonTlsContext_DeprecatedV1 {
        if self.deprecated_v1.is_none() {
            self.deprecated_v1.set_default();
        }
        self.deprecated_v1.as_mut().unwrap()
    }

    // Take field
    pub fn take_deprecated_v1(&mut self) -> CommonTlsContext_DeprecatedV1 {
        self.deprecated_v1.take().unwrap_or_else(|| CommonTlsContext_DeprecatedV1::new())
    }

    pub fn get_deprecated_v1(&self) -> &CommonTlsContext_DeprecatedV1 {
        self.deprecated_v1.as_ref().unwrap_or_else(|| CommonTlsContext_DeprecatedV1::default_instance())
    }

    fn get_deprecated_v1_for_reflect(&self) -> &::protobuf::SingularPtrField<CommonTlsContext_DeprecatedV1> {
        &self.deprecated_v1
    }

    fn mut_deprecated_v1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CommonTlsContext_DeprecatedV1> {
        &mut self.deprecated_v1
    }
}

impl ::protobuf::Message for CommonTlsContext {
    fn is_initialized(&self) -> bool {
        for v in &self.tls_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tls_certificates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.validation_context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tls_params)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tls_certificates)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.validation_context)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.alpn_protocols)?;
                },
                5 => {
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
        if let Some(ref v) = self.tls_params.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.tls_certificates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.validation_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.alpn_protocols {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.deprecated_v1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.tls_params.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.tls_certificates {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.validation_context.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.alpn_protocols {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.deprecated_v1.as_ref() {
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

impl ::protobuf::MessageStatic for CommonTlsContext {
    fn new() -> CommonTlsContext {
        CommonTlsContext::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommonTlsContext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsParameters>>(
                    "tls_params",
                    CommonTlsContext::get_tls_params_for_reflect,
                    CommonTlsContext::mut_tls_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TlsCertificate>>(
                    "tls_certificates",
                    CommonTlsContext::get_tls_certificates_for_reflect,
                    CommonTlsContext::mut_tls_certificates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateValidationContext>>(
                    "validation_context",
                    CommonTlsContext::get_validation_context_for_reflect,
                    CommonTlsContext::mut_validation_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alpn_protocols",
                    CommonTlsContext::get_alpn_protocols_for_reflect,
                    CommonTlsContext::mut_alpn_protocols_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommonTlsContext_DeprecatedV1>>(
                    "deprecated_v1",
                    CommonTlsContext::get_deprecated_v1_for_reflect,
                    CommonTlsContext::mut_deprecated_v1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommonTlsContext>(
                    "CommonTlsContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommonTlsContext {
    fn clear(&mut self) {
        self.clear_tls_params();
        self.clear_tls_certificates();
        self.clear_validation_context();
        self.clear_alpn_protocols();
        self.clear_deprecated_v1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommonTlsContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonTlsContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommonTlsContext_DeprecatedV1 {
    // message fields
    pub alt_alpn_protocols: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommonTlsContext_DeprecatedV1 {}

impl CommonTlsContext_DeprecatedV1 {
    pub fn new() -> CommonTlsContext_DeprecatedV1 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommonTlsContext_DeprecatedV1 {
        static mut instance: ::protobuf::lazy::Lazy<CommonTlsContext_DeprecatedV1> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommonTlsContext_DeprecatedV1,
        };
        unsafe {
            instance.get(CommonTlsContext_DeprecatedV1::new)
        }
    }

    // string alt_alpn_protocols = 1;

    pub fn clear_alt_alpn_protocols(&mut self) {
        self.alt_alpn_protocols.clear();
    }

    // Param is passed by value, moved
    pub fn set_alt_alpn_protocols(&mut self, v: ::std::string::String) {
        self.alt_alpn_protocols = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alt_alpn_protocols(&mut self) -> &mut ::std::string::String {
        &mut self.alt_alpn_protocols
    }

    // Take field
    pub fn take_alt_alpn_protocols(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.alt_alpn_protocols, ::std::string::String::new())
    }

    pub fn get_alt_alpn_protocols(&self) -> &str {
        &self.alt_alpn_protocols
    }

    fn get_alt_alpn_protocols_for_reflect(&self) -> &::std::string::String {
        &self.alt_alpn_protocols
    }

    fn mut_alt_alpn_protocols_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.alt_alpn_protocols
    }
}

impl ::protobuf::Message for CommonTlsContext_DeprecatedV1 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.alt_alpn_protocols)?;
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
        if !self.alt_alpn_protocols.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.alt_alpn_protocols);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.alt_alpn_protocols.is_empty() {
            os.write_string(1, &self.alt_alpn_protocols)?;
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

impl ::protobuf::MessageStatic for CommonTlsContext_DeprecatedV1 {
    fn new() -> CommonTlsContext_DeprecatedV1 {
        CommonTlsContext_DeprecatedV1::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommonTlsContext_DeprecatedV1>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "alt_alpn_protocols",
                    CommonTlsContext_DeprecatedV1::get_alt_alpn_protocols_for_reflect,
                    CommonTlsContext_DeprecatedV1::mut_alt_alpn_protocols_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommonTlsContext_DeprecatedV1>(
                    "CommonTlsContext_DeprecatedV1",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommonTlsContext_DeprecatedV1 {
    fn clear(&mut self) {
        self.clear_alt_alpn_protocols();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommonTlsContext_DeprecatedV1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonTlsContext_DeprecatedV1 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpstreamTlsContext {
    // message fields
    pub common_tls_context: ::protobuf::SingularPtrField<CommonTlsContext>,
    pub sni: ::std::string::String,
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

    // .envoy.api.v2.CommonTlsContext common_tls_context = 1;

    pub fn clear_common_tls_context(&mut self) {
        self.common_tls_context.clear();
    }

    pub fn has_common_tls_context(&self) -> bool {
        self.common_tls_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common_tls_context(&mut self, v: CommonTlsContext) {
        self.common_tls_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_tls_context(&mut self) -> &mut CommonTlsContext {
        if self.common_tls_context.is_none() {
            self.common_tls_context.set_default();
        }
        self.common_tls_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_common_tls_context(&mut self) -> CommonTlsContext {
        self.common_tls_context.take().unwrap_or_else(|| CommonTlsContext::new())
    }

    pub fn get_common_tls_context(&self) -> &CommonTlsContext {
        self.common_tls_context.as_ref().unwrap_or_else(|| CommonTlsContext::default_instance())
    }

    fn get_common_tls_context_for_reflect(&self) -> &::protobuf::SingularPtrField<CommonTlsContext> {
        &self.common_tls_context
    }

    fn mut_common_tls_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CommonTlsContext> {
        &mut self.common_tls_context
    }

    // string sni = 2;

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
}

impl ::protobuf::Message for UpstreamTlsContext {
    fn is_initialized(&self) -> bool {
        for v in &self.common_tls_context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common_tls_context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sni)?;
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
        if let Some(ref v) = self.common_tls_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.sni.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.sni);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.common_tls_context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.sni.is_empty() {
            os.write_string(2, &self.sni)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommonTlsContext>>(
                    "common_tls_context",
                    UpstreamTlsContext::get_common_tls_context_for_reflect,
                    UpstreamTlsContext::mut_common_tls_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sni",
                    UpstreamTlsContext::get_sni_for_reflect,
                    UpstreamTlsContext::mut_sni_for_reflect,
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
        self.clear_common_tls_context();
        self.clear_sni();
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
    pub common_tls_context: ::protobuf::SingularPtrField<CommonTlsContext>,
    pub require_client_certificate: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    pub require_sni: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
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

    // .envoy.api.v2.CommonTlsContext common_tls_context = 1;

    pub fn clear_common_tls_context(&mut self) {
        self.common_tls_context.clear();
    }

    pub fn has_common_tls_context(&self) -> bool {
        self.common_tls_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common_tls_context(&mut self, v: CommonTlsContext) {
        self.common_tls_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_tls_context(&mut self) -> &mut CommonTlsContext {
        if self.common_tls_context.is_none() {
            self.common_tls_context.set_default();
        }
        self.common_tls_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_common_tls_context(&mut self) -> CommonTlsContext {
        self.common_tls_context.take().unwrap_or_else(|| CommonTlsContext::new())
    }

    pub fn get_common_tls_context(&self) -> &CommonTlsContext {
        self.common_tls_context.as_ref().unwrap_or_else(|| CommonTlsContext::default_instance())
    }

    fn get_common_tls_context_for_reflect(&self) -> &::protobuf::SingularPtrField<CommonTlsContext> {
        &self.common_tls_context
    }

    fn mut_common_tls_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CommonTlsContext> {
        &mut self.common_tls_context
    }

    // .google.protobuf.BoolValue require_client_certificate = 2;

    pub fn clear_require_client_certificate(&mut self) {
        self.require_client_certificate.clear();
    }

    pub fn has_require_client_certificate(&self) -> bool {
        self.require_client_certificate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_client_certificate(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.require_client_certificate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_client_certificate(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.require_client_certificate.is_none() {
            self.require_client_certificate.set_default();
        }
        self.require_client_certificate.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_client_certificate(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.require_client_certificate.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_require_client_certificate(&self) -> &::protobuf::well_known_types::BoolValue {
        self.require_client_certificate.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_require_client_certificate_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.require_client_certificate
    }

    fn mut_require_client_certificate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.require_client_certificate
    }

    // .google.protobuf.BoolValue require_sni = 3;

    pub fn clear_require_sni(&mut self) {
        self.require_sni.clear();
    }

    pub fn has_require_sni(&self) -> bool {
        self.require_sni.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_sni(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.require_sni = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_require_sni(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.require_sni.is_none() {
            self.require_sni.set_default();
        }
        self.require_sni.as_mut().unwrap()
    }

    // Take field
    pub fn take_require_sni(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.require_sni.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_require_sni(&self) -> &::protobuf::well_known_types::BoolValue {
        self.require_sni.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_require_sni_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.require_sni
    }

    fn mut_require_sni_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.require_sni
    }
}

impl ::protobuf::Message for DownstreamTlsContext {
    fn is_initialized(&self) -> bool {
        for v in &self.common_tls_context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.require_client_certificate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.require_sni {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common_tls_context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.require_client_certificate)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.require_sni)?;
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
        if let Some(ref v) = self.common_tls_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.require_client_certificate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.require_sni.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.common_tls_context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.require_client_certificate.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.require_sni.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommonTlsContext>>(
                    "common_tls_context",
                    DownstreamTlsContext::get_common_tls_context_for_reflect,
                    DownstreamTlsContext::mut_common_tls_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "require_client_certificate",
                    DownstreamTlsContext::get_require_client_certificate_for_reflect,
                    DownstreamTlsContext::mut_require_client_certificate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "require_sni",
                    DownstreamTlsContext::get_require_sni_for_reflect,
                    DownstreamTlsContext::mut_require_sni_for_reflect,
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
        self.clear_common_tls_context();
        self.clear_require_client_certificate();
        self.clear_require_sni();
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15api/tls_context.proto\x12\x0cenvoy.api.v2\x1a\x1egoogle/protobuf/w\
    rappers.proto\"Q\n\nDataSource\x12\x1c\n\x08filename\x18\x01\x20\x01(\tH\
    \0R\x08filename\x12\x18\n\x06inline\x18\x02\x20\x01(\x0cH\0R\x06inlineB\
    \x0b\n\tspecifier\"\xfa\x02\n\rTlsParameters\x12h\n\x1ctls_minimum_proto\
    col_version\x18\x01\x20\x01(\x0e2'.envoy.api.v2.TlsParameters.TlsProtoco\
    lR\x19tlsMinimumProtocolVersion\x12h\n\x1ctls_maximum_protocol_version\
    \x18\x02\x20\x01(\x0e2'.envoy.api.v2.TlsParameters.TlsProtocolR\x19tlsMa\
    ximumProtocolVersion\x12#\n\rcipher_suites\x18\x03\x20\x03(\tR\x0ccipher\
    Suites\x12\x1f\n\x0becdh_curves\x18\x04\x20\x03(\tR\necdhCurves\"O\n\x0b\
    TlsProtocol\x12\x0c\n\x08TLS_AUTO\x10\0\x12\x0b\n\x07TLSv1_0\x10\x01\x12\
    \x0b\n\x07TLSv1_1\x10\x02\x12\x0b\n\x07TLSv1_2\x10\x03\x12\x0b\n\x07TLSv\
    1_3\x10\x04\"\xdf\x02\n\x0eTlsCertificate\x12E\n\x11certificate_chain\
    \x18\x01\x20\x01(\x0b2\x18.envoy.api.v2.DataSourceR\x10certificateChain\
    \x129\n\x0bprivate_key\x18\x02\x20\x01(\x0b2\x18.envoy.api.v2.DataSource\
    R\nprivateKey\x124\n\x08password\x18\x03\x20\x01(\x0b2\x18.envoy.api.v2.\
    DataSourceR\x08password\x129\n\x0bocsp_staple\x18\x04\x20\x01(\x0b2\x18.\
    envoy.api.v2.DataSourceR\nocspStaple\x12Z\n\x1csigned_certificate_timest\
    amp\x18\x05\x20\x03(\x0b2\x18.envoy.api.v2.DataSourceR\x1asignedCertific\
    ateTimestamp\"\xad\x03\n\x1cCertificateValidationContext\x127\n\ntrusted\
    _ca\x18\x01\x20\x01(\x0b2\x18.envoy.api.v2.DataSourceR\ttrustedCa\x126\n\
    \x17verify_certificate_hash\x18\x02\x20\x03(\tR\x15verifyCertificateHash\
    \x12,\n\x12verify_spki_sha256\x18\x03\x20\x03(\tR\x10verifySpkiSha256\
    \x125\n\x17verify_subject_alt_name\x18\x04\x20\x03(\tR\x14verifySubjectA\
    ltName\x12J\n\x13require_ocsp_staple\x18\x05\x20\x01(\x0b2\x1a.google.pr\
    otobuf.BoolValueR\x11requireOcspStaple\x12k\n$require_signed_certificate\
    _timestamp\x18\x06\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR!requireS\
    ignedCertificateTimestamp\"\xa9\x03\n\x10CommonTlsContext\x12:\n\ntls_pa\
    rams\x18\x01\x20\x01(\x0b2\x1b.envoy.api.v2.TlsParametersR\ttlsParams\
    \x12G\n\x10tls_certificates\x18\x02\x20\x03(\x0b2\x1c.envoy.api.v2.TlsCe\
    rtificateR\x0ftlsCertificates\x12Y\n\x12validation_context\x18\x03\x20\
    \x01(\x0b2*.envoy.api.v2.CertificateValidationContextR\x11validationCont\
    ext\x12%\n\x0ealpn_protocols\x18\x04\x20\x03(\tR\ralpnProtocols\x12P\n\r\
    deprecated_v1\x18\x05\x20\x01(\x0b2+.envoy.api.v2.CommonTlsContext.Depre\
    catedV1R\x0cdeprecatedV1\x1a<\n\x0cDeprecatedV1\x12,\n\x12alt_alpn_proto\
    cols\x18\x01\x20\x01(\tR\x10altAlpnProtocols\"t\n\x12UpstreamTlsContext\
    \x12L\n\x12common_tls_context\x18\x01\x20\x01(\x0b2\x1e.envoy.api.v2.Com\
    monTlsContextR\x10commonTlsContext\x12\x10\n\x03sni\x18\x02\x20\x01(\tR\
    \x03sni\"\xfb\x01\n\x14DownstreamTlsContext\x12L\n\x12common_tls_context\
    \x18\x01\x20\x01(\x0b2\x1e.envoy.api.v2.CommonTlsContextR\x10commonTlsCo\
    ntext\x12X\n\x1arequire_client_certificate\x18\x02\x20\x01(\x0b2\x1a.goo\
    gle.protobuf.BoolValueR\x18requireClientCertificate\x12;\n\x0brequire_sn\
    i\x18\x03\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\nrequireSniJ\xa0#\
    \n\x06\x12\x04\0\0p\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x14\n\t\n\x02\x03\0\x12\x03\x04\x07'\n\n\n\x02\x04\0\
    \x12\x04\x06\0\x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x12\n\x0c\n\
    \x04\x04\0\x08\0\x12\x04\x07\x02\n\x03\n\x0c\n\x05\x04\0\x08\0\x01\x12\
    \x03\x07\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\x04\x18\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x08\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x08\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08\x16\x17\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\t\x04\x15\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\t\
    \x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\t\n\x10\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\t\x13\x14\n\n\n\x02\x04\x01\x12\x04\r\0\x1f\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\r\x08\x15\n\x0c\n\x04\x04\x01\x04\0\x12\x04\
    \x0e\x02\x14\x03\n\x0c\n\x05\x04\x01\x04\0\x01\x12\x03\x0e\x07\x12\n\r\n\
    \x06\x04\x01\x04\0\x02\0\x12\x03\x0f\x04\x11\n\x0e\n\x07\x04\x01\x04\0\
    \x02\0\x01\x12\x03\x0f\x04\x0c\n\x0e\n\x07\x04\x01\x04\0\x02\0\x02\x12\
    \x03\x0f\x0f\x10\n\r\n\x06\x04\x01\x04\0\x02\x01\x12\x03\x10\x04\x10\n\
    \x0e\n\x07\x04\x01\x04\0\x02\x01\x01\x12\x03\x10\x04\x0b\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x01\x02\x12\x03\x10\x0e\x0f\n\r\n\x06\x04\x01\x04\0\x02\
    \x02\x12\x03\x11\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x01\x12\x03\
    \x11\x04\x0b\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x02\x12\x03\x11\x0e\x0f\n\
    \r\n\x06\x04\x01\x04\0\x02\x03\x12\x03\x12\x04\x10\n\x0e\n\x07\x04\x01\
    \x04\0\x02\x03\x01\x12\x03\x12\x04\x0b\n\x0e\n\x07\x04\x01\x04\0\x02\x03\
    \x02\x12\x03\x12\x0e\x0f\n\r\n\x06\x04\x01\x04\0\x02\x04\x12\x03\x13\x04\
    \x10\n\x0e\n\x07\x04\x01\x04\0\x02\x04\x01\x12\x03\x13\x04\x0b\n\x0e\n\
    \x07\x04\x01\x04\0\x02\x04\x02\x12\x03\x13\x0e\x0f\n%\n\x04\x04\x01\x02\
    \0\x12\x03\x16\x02/\x1a\x18\x20Allowed\x20TLS\x20protocols.\n\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04\x16\x02\x14\x03\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03\x16\x02\r\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x16\x0e*\n\x0c\
    \n\x05\x04\x01\x02\0\x03\x12\x03\x16-.\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03\x17\x02/\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x17\x02\x16/\n\x0c\n\
    \x05\x04\x01\x02\x01\x06\x12\x03\x17\x02\r\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x17\x0e*\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x17-.\nZ\n\
    \x04\x04\x01\x02\x02\x12\x03\x1a\x02$\x1aM\x20If\x20specified,\x20the\
    \x20TLS\x20listener\x20will\x20only\x20support\x20the\x20specified\x20ci\
    pher\x20list.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x1a\x02\n\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03\x1a\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \x02\x01\x12\x03\x1a\x12\x1f\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1a\
    \"#\n\xa1\x01\n\x04\x04\x01\x02\x03\x12\x03\x1e\x02\"\x1a\x93\x01\x20If\
    \x20specified,\x20the\x20TLS\x20connection\x20will\x20only\x20support\
    \x20the\x20specified\x20ECDH\n\x20curves.\x20If\x20not\x20specified,\x20\
    the\x20default\x20curves\x20(X25519,\x20P-256)\x20will\x20be\x20used.\n\
    \n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x03\x05\x12\x03\x1e\x0b\x11\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\
    \x1e\x12\x1d\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1e\x20!\n\x80\x01\
    \n\x02\x04\x02\x12\x04#\0)\x01\x1at\x20TLS\x20certs\x20can\x20be\x20load\
    ed\x20from\x20file\x20or\x20delivered\x20inline\x20[V2-API-DIFF].\x20Ind\
    ividual\x20fields\x20may\n\x20be\x20loaded\x20from\x20either.\n\n\n\n\
    \x03\x04\x02\x01\x12\x03#\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03$\x02\
    #\n\r\n\x05\x04\x02\x02\0\x04\x12\x04$\x02#\x18\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03$\x02\x0c\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03$\r\x1e\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03$!\"\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03%\x02\x1d\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04%\x02$#\n\x0c\n\x05\
    \x04\x02\x02\x01\x06\x12\x03%\x02\x0c\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x03%\r\x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03%\x1b\x1c\n\x0b\n\
    \x04\x04\x02\x02\x02\x12\x03&\x02\x1a\n\r\n\x05\x04\x02\x02\x02\x04\x12\
    \x04&\x02%\x1d\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03&\x02\x0c\n\x0c\n\
    \x05\x04\x02\x02\x02\x01\x12\x03&\r\x15\n\x0c\n\x05\x04\x02\x02\x02\x03\
    \x12\x03&\x18\x19\n\x0b\n\x04\x04\x02\x02\x03\x12\x03'\x02\x1d\n\r\n\x05\
    \x04\x02\x02\x03\x04\x12\x04'\x02&\x1a\n\x0c\n\x05\x04\x02\x02\x03\x06\
    \x12\x03'\x02\x0c\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03'\r\x18\n\x0c\n\
    \x05\x04\x02\x02\x03\x03\x12\x03'\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x04\
    \x12\x03(\x027\n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03(\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x04\x06\x12\x03(\x0b\x15\n\x0c\n\x05\x04\x02\x02\x04\
    \x01\x12\x03(\x162\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03(56\n\n\n\x02\
    \x04\x03\x12\x04+\0C\x01\n\n\n\x03\x04\x03\x01\x12\x03+\x08$\n\xca\x01\n\
    \x04\x04\x03\x02\0\x12\x03/\x02\x1c\x1a\xbc\x01\x20TLS\x20certificate\
    \x20data\x20containing\x20certificate\x20authority\x20certificates\x20to\
    \x20use\n\x20in\x20verifying\x20a\x20presented\x20certificate.\x20If\x20\
    not\x20specified\x20and\x20a\x20certificate\x20is\n\x20presented\x20it\
    \x20will\x20not\x20be\x20verified.\n\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\
    /\x02+&\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03/\x02\x0c\n\x0c\n\x05\x04\
    \x03\x02\0\x01\x12\x03/\r\x17\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03/\x1a\
    \x1b\nl\n\x04\x04\x03\x02\x01\x12\x033\x02.\x1a_\x20If\x20specified,\x20\
    Envoy\x20will\x20verify\x20(pin)\x20hex-encoded\x20SHA-256\x20hash\x20of\
    \n\x20the\x20presented\x20certificate.\n\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x033\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x033\x0b\x11\n\x0c\n\
    \x05\x04\x03\x02\x01\x01\x12\x033\x12)\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x033,-\n\xda\x01\n\x04\x04\x03\x02\x02\x12\x038\x02)\x1a\xcc\x01\
    \x20If\x20specified,\x20Envoy\x20will\x20verify\x20(pin)\x20base64-encod\
    ed\x20SHA-256\x20hash\x20of\n\x20the\x20Subject\x20Public\x20Key\x20Info\
    rmation\x20(SPKI)\x20of\x20the\x20presented\x20certificate.\n\x20This\
    \x20is\x20the\x20same\x20format\x20as\x20used\x20in\x20HTTP\x20Public\
    \x20Key\x20Pinning.\n\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x038\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x02\x05\x12\x038\x0b\x11\n\x0c\n\x05\x04\x03\x02\
    \x02\x01\x12\x038\x12$\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x038'(\n\xa6\
    \x01\n\x04\x04\x03\x02\x03\x12\x03<\x02.\x1a\x98\x01\x20An\x20optional\
    \x20list\x20of\x20subject\x20alt\x20names.\x20If\x20specified,\x20Envoy\
    \x20will\x20verify\x20that\n\x20the\x20certificate\xe2\x80\x99s\x20subje\
    ct\x20alt\x20name\x20matches\x20one\x20of\x20the\x20specified\x20values.\
    \n\n\x0c\n\x05\x04\x03\x02\x03\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\x03\
    \x02\x03\x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03<\
    \x12)\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03<,-\n@\n\x04\x04\x03\x02\
    \x04\x12\x03?\x024\x1a3\x20Must\x20present\x20a\x20signed\x20time-stampe\
    d\x20OCSP\x20response.\n\n\r\n\x05\x04\x03\x02\x04\x04\x12\x04?\x02<.\n\
    \x0c\n\x05\x04\x03\x02\x04\x06\x12\x03?\x02\x1b\n\x0c\n\x05\x04\x03\x02\
    \x04\x01\x12\x03?\x1c/\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03?23\n:\n\
    \x04\x04\x03\x02\x05\x12\x03B\x02E\x1a-\x20Must\x20present\x20signed\x20\
    certificate\x20time-stamp.\n\n\r\n\x05\x04\x03\x02\x05\x04\x12\x04B\x02?\
    4\n\x0c\n\x05\x04\x03\x02\x05\x06\x12\x03B\x02\x1b\n\x0c\n\x05\x04\x03\
    \x02\x05\x01\x12\x03B\x1c@\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03BCD\nH\
    \n\x02\x04\x04\x12\x04F\0[\x01\x1a<\x20TLS\x20context\x20shared\x20by\
    \x20both\x20client\x20and\x20server\x20TLS\x20contexts.\n\n\n\n\x03\x04\
    \x04\x01\x12\x03F\x08\x18\n8\n\x04\x04\x04\x02\0\x12\x03H\x02\x1f\x1a+\
    \x20TLS\x20protocol\x20versions,\x20cipher\x20suites\x20etc.\n\n\r\n\x05\
    \x04\x04\x02\0\x04\x12\x04H\x02F\x1a\n\x0c\n\x05\x04\x04\x02\0\x06\x12\
    \x03H\x02\x0f\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03H\x10\x1a\n\x0c\n\x05\
    \x04\x04\x02\0\x03\x12\x03H\x1d\x1e\n\x90\x01\n\x04\x04\x04\x02\x01\x12\
    \x03L\x02/\x1a\x82\x01\x20Multiple\x20TLS\x20certificates\x20can\x20be\
    \x20associated\x20with\x20the\x20same\x20context,\n\x20e.g.\x20to\x20all\
    ow\x20both\x20RSA\x20and\x20ECDSA\x20certificates\x20[V2-API-DIFF].\n\n\
    \x0c\n\x05\x04\x04\x02\x01\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\x04\x02\
    \x01\x06\x12\x03L\x0b\x19\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03L\x1a*\
    \n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03L-.\n1\n\x04\x04\x04\x02\x02\x12\
    \x03O\x026\x1a$\x20How\x20to\x20validate\x20peer\x20certificates.\n\n\r\
    \n\x05\x04\x04\x02\x02\x04\x12\x04O\x02L/\n\x0c\n\x05\x04\x04\x02\x02\
    \x06\x12\x03O\x02\x1e\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03O\x1f1\n\
    \x0c\n\x05\x04\x04\x02\x02\x03\x12\x03O45\n/\n\x04\x04\x04\x02\x03\x12\
    \x03R\x02%\x1a\"\x20Protocols\x20to\x20negotiate\x20over\x20ALPN\n\n\x0c\
    \n\x05\x04\x04\x02\x03\x04\x12\x03R\x02\n\n\x0c\n\x05\x04\x04\x02\x03\
    \x05\x12\x03R\x0b\x11\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03R\x12\x20\n\
    \x0c\n\x05\x04\x04\x02\x03\x03\x12\x03R#$\n\xba\x01\n\x04\x04\x04\x03\0\
    \x12\x04W\x02Y\x03\x1a\xab\x01\x20These\x20fields\x20are\x20deprecated\
    \x20and\x20only\x20are\x20used\x20during\x20the\x20interim\x20v1\x20->\
    \x20v2\n\x20transition\x20period\x20for\x20internal\x20purposes.\x20They\
    \x20should\x20not\x20be\x20used\x20outside\x20of\n\x20the\x20Envoy\x20bi\
    nary.\n\n\x0c\n\x05\x04\x04\x03\0\x01\x12\x03W\n\x16\n\r\n\x06\x04\x04\
    \x03\0\x02\0\x12\x03X\x04\"\n\x0f\n\x07\x04\x04\x03\0\x02\0\x04\x12\x04X\
    \x04W\x18\n\x0e\n\x07\x04\x04\x03\0\x02\0\x05\x12\x03X\x04\n\n\x0e\n\x07\
    \x04\x04\x03\0\x02\0\x01\x12\x03X\x0b\x1d\n\x0e\n\x07\x04\x04\x03\0\x02\
    \0\x03\x12\x03X\x20!\n\x0b\n\x04\x04\x04\x02\x04\x12\x03Z\x02!\n\r\n\x05\
    \x04\x04\x02\x04\x04\x12\x04Z\x02Y\x03\n\x0c\n\x05\x04\x04\x02\x04\x06\
    \x12\x03Z\x02\x0e\n\x0c\n\x05\x04\x04\x02\x04\x01\x12\x03Z\x0f\x1c\n\x0c\
    \n\x05\x04\x04\x02\x04\x03\x12\x03Z\x1f\x20\n\n\n\x02\x04\x05\x12\x04]\0\
    b\x01\n\n\n\x03\x04\x05\x01\x12\x03]\x08\x1a\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03^\x02*\n\r\n\x05\x04\x05\x02\0\x04\x12\x04^\x02]\x1c\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x03^\x02\x12\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03^\x13%\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03^()\nG\n\x04\x04\x05\x02\
    \x01\x12\x03a\x02\x11\x1a:\x20SNI\x20string\x20to\x20use\x20when\x20crea\
    ting\x20TLS\x20backend\x20connections.\n\n\r\n\x05\x04\x05\x02\x01\x04\
    \x12\x04a\x02^*\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03a\x02\x08\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x03a\t\x0c\n\x0c\n\x05\x04\x05\x02\x01\x03\
    \x12\x03a\x0f\x10\n\xdc\x01\n\x02\x04\x06\x12\x04g\0p\x01\x1a\xcf\x01\
    \x20[V2-API-DIFF]\x20This\x20has\x20been\x20reworked\x20to\x20support\
    \x20alternative\x20modes\x20of\n\x20certificate/key\x20delivery,\x20for\
    \x20consistency\x20with\x20the\x20upstream\x20TLS\x20context\x20and\n\
    \x20to\x20segregate\x20the\x20client/server\x20aspects\x20of\x20the\x20T\
    LS\x20context.\n\n\n\n\x03\x04\x06\x01\x12\x03g\x08\x1c\n\x0b\n\x04\x04\
    \x06\x02\0\x12\x03h\x02*\n\r\n\x05\x04\x06\x02\0\x04\x12\x04h\x02g\x1e\n\
    \x0c\n\x05\x04\x06\x02\0\x06\x12\x03h\x02\x12\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03h\x13%\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03h()\n_\n\x04\x04\
    \x06\x02\x01\x12\x03l\x02;\x1aR\x20If\x20specified,\x20Envoy\x20will\x20\
    reject\x20connections\x20without\x20a\x20valid\x20client\n\x20certificat\
    e.\n\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04l\x02h*\n\x0c\n\x05\x04\x06\
    \x02\x01\x06\x12\x03l\x02\x1b\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03l\
    \x1c6\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03l9:\n\\\n\x04\x04\x06\x02\
    \x02\x12\x03o\x02,\x1aO\x20If\x20specified,\x20Envoy\x20will\x20reject\
    \x20connections\x20without\x20a\x20valid\x20and\x20matching\x20SNI.\n\n\
    \r\n\x05\x04\x06\x02\x02\x04\x12\x04o\x02l;\n\x0c\n\x05\x04\x06\x02\x02\
    \x06\x12\x03o\x02\x1b\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03o\x1c'\n\
    \x0c\n\x05\x04\x06\x02\x02\x03\x12\x03o*+b\x06proto3\
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
