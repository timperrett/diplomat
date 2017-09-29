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
pub struct TcpProtocolOptions {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TcpProtocolOptions {}

impl TcpProtocolOptions {
    pub fn new() -> TcpProtocolOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TcpProtocolOptions {
        static mut instance: ::protobuf::lazy::Lazy<TcpProtocolOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TcpProtocolOptions,
        };
        unsafe {
            instance.get(TcpProtocolOptions::new)
        }
    }
}

impl ::protobuf::Message for TcpProtocolOptions {
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

impl ::protobuf::MessageStatic for TcpProtocolOptions {
    fn new() -> TcpProtocolOptions {
        TcpProtocolOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<TcpProtocolOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TcpProtocolOptions>(
                    "TcpProtocolOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TcpProtocolOptions {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TcpProtocolOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TcpProtocolOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Http1ProtocolOptions {
    // message fields
    pub allow_absolute_url: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Http1ProtocolOptions {}

impl Http1ProtocolOptions {
    pub fn new() -> Http1ProtocolOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Http1ProtocolOptions {
        static mut instance: ::protobuf::lazy::Lazy<Http1ProtocolOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Http1ProtocolOptions,
        };
        unsafe {
            instance.get(Http1ProtocolOptions::new)
        }
    }

    // .google.protobuf.BoolValue allow_absolute_url = 1;

    pub fn clear_allow_absolute_url(&mut self) {
        self.allow_absolute_url.clear();
    }

    pub fn has_allow_absolute_url(&self) -> bool {
        self.allow_absolute_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_absolute_url(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.allow_absolute_url = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allow_absolute_url(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.allow_absolute_url.is_none() {
            self.allow_absolute_url.set_default();
        }
        self.allow_absolute_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_allow_absolute_url(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.allow_absolute_url.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_allow_absolute_url(&self) -> &::protobuf::well_known_types::BoolValue {
        self.allow_absolute_url.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_allow_absolute_url_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.allow_absolute_url
    }

    fn mut_allow_absolute_url_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.allow_absolute_url
    }
}

impl ::protobuf::Message for Http1ProtocolOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.allow_absolute_url {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.allow_absolute_url)?;
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
        if let Some(ref v) = self.allow_absolute_url.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.allow_absolute_url.as_ref() {
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

impl ::protobuf::MessageStatic for Http1ProtocolOptions {
    fn new() -> Http1ProtocolOptions {
        Http1ProtocolOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<Http1ProtocolOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "allow_absolute_url",
                    Http1ProtocolOptions::get_allow_absolute_url_for_reflect,
                    Http1ProtocolOptions::mut_allow_absolute_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Http1ProtocolOptions>(
                    "Http1ProtocolOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Http1ProtocolOptions {
    fn clear(&mut self) {
        self.clear_allow_absolute_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Http1ProtocolOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Http1ProtocolOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Http2ProtocolOptions {
    // message fields
    pub hpack_table_size: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_concurrent_streams: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub initial_stream_window_size: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub initial_connection_window_size: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Http2ProtocolOptions {}

impl Http2ProtocolOptions {
    pub fn new() -> Http2ProtocolOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Http2ProtocolOptions {
        static mut instance: ::protobuf::lazy::Lazy<Http2ProtocolOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Http2ProtocolOptions,
        };
        unsafe {
            instance.get(Http2ProtocolOptions::new)
        }
    }

    // .google.protobuf.UInt32Value hpack_table_size = 1;

    pub fn clear_hpack_table_size(&mut self) {
        self.hpack_table_size.clear();
    }

    pub fn has_hpack_table_size(&self) -> bool {
        self.hpack_table_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hpack_table_size(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.hpack_table_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hpack_table_size(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.hpack_table_size.is_none() {
            self.hpack_table_size.set_default();
        }
        self.hpack_table_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_hpack_table_size(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.hpack_table_size.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_hpack_table_size(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.hpack_table_size.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_hpack_table_size_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.hpack_table_size
    }

    fn mut_hpack_table_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.hpack_table_size
    }

    // .google.protobuf.UInt32Value max_concurrent_streams = 2;

    pub fn clear_max_concurrent_streams(&mut self) {
        self.max_concurrent_streams.clear();
    }

    pub fn has_max_concurrent_streams(&self) -> bool {
        self.max_concurrent_streams.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_concurrent_streams(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_concurrent_streams = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_concurrent_streams(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_concurrent_streams.is_none() {
            self.max_concurrent_streams.set_default();
        }
        self.max_concurrent_streams.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_concurrent_streams(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_concurrent_streams.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_max_concurrent_streams(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_concurrent_streams.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_max_concurrent_streams_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.max_concurrent_streams
    }

    fn mut_max_concurrent_streams_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.max_concurrent_streams
    }

    // .google.protobuf.UInt32Value initial_stream_window_size = 3;

    pub fn clear_initial_stream_window_size(&mut self) {
        self.initial_stream_window_size.clear();
    }

    pub fn has_initial_stream_window_size(&self) -> bool {
        self.initial_stream_window_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial_stream_window_size(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.initial_stream_window_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initial_stream_window_size(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.initial_stream_window_size.is_none() {
            self.initial_stream_window_size.set_default();
        }
        self.initial_stream_window_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_initial_stream_window_size(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.initial_stream_window_size.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_initial_stream_window_size(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.initial_stream_window_size.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_initial_stream_window_size_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.initial_stream_window_size
    }

    fn mut_initial_stream_window_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.initial_stream_window_size
    }

    // .google.protobuf.UInt32Value initial_connection_window_size = 4;

    pub fn clear_initial_connection_window_size(&mut self) {
        self.initial_connection_window_size.clear();
    }

    pub fn has_initial_connection_window_size(&self) -> bool {
        self.initial_connection_window_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial_connection_window_size(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.initial_connection_window_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initial_connection_window_size(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.initial_connection_window_size.is_none() {
            self.initial_connection_window_size.set_default();
        }
        self.initial_connection_window_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_initial_connection_window_size(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.initial_connection_window_size.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_initial_connection_window_size(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.initial_connection_window_size.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_initial_connection_window_size_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.initial_connection_window_size
    }

    fn mut_initial_connection_window_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.initial_connection_window_size
    }
}

impl ::protobuf::Message for Http2ProtocolOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.hpack_table_size {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_concurrent_streams {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.initial_stream_window_size {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.initial_connection_window_size {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hpack_table_size)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_concurrent_streams)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.initial_stream_window_size)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.initial_connection_window_size)?;
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
        if let Some(ref v) = self.hpack_table_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_concurrent_streams.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.initial_stream_window_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.initial_connection_window_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hpack_table_size.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_concurrent_streams.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.initial_stream_window_size.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.initial_connection_window_size.as_ref() {
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

impl ::protobuf::MessageStatic for Http2ProtocolOptions {
    fn new() -> Http2ProtocolOptions {
        Http2ProtocolOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<Http2ProtocolOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "hpack_table_size",
                    Http2ProtocolOptions::get_hpack_table_size_for_reflect,
                    Http2ProtocolOptions::mut_hpack_table_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "max_concurrent_streams",
                    Http2ProtocolOptions::get_max_concurrent_streams_for_reflect,
                    Http2ProtocolOptions::mut_max_concurrent_streams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "initial_stream_window_size",
                    Http2ProtocolOptions::get_initial_stream_window_size_for_reflect,
                    Http2ProtocolOptions::mut_initial_stream_window_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "initial_connection_window_size",
                    Http2ProtocolOptions::get_initial_connection_window_size_for_reflect,
                    Http2ProtocolOptions::mut_initial_connection_window_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Http2ProtocolOptions>(
                    "Http2ProtocolOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Http2ProtocolOptions {
    fn clear(&mut self) {
        self.clear_hpack_table_size();
        self.clear_max_concurrent_streams();
        self.clear_initial_stream_window_size();
        self.clear_initial_connection_window_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Http2ProtocolOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Http2ProtocolOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GrpcProtocolOptions {
    // message fields
    pub http2_protocol_options: ::protobuf::SingularPtrField<Http2ProtocolOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GrpcProtocolOptions {}

impl GrpcProtocolOptions {
    pub fn new() -> GrpcProtocolOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GrpcProtocolOptions {
        static mut instance: ::protobuf::lazy::Lazy<GrpcProtocolOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GrpcProtocolOptions,
        };
        unsafe {
            instance.get(GrpcProtocolOptions::new)
        }
    }

    // .envoy.api.v2.Http2ProtocolOptions http2_protocol_options = 1;

    pub fn clear_http2_protocol_options(&mut self) {
        self.http2_protocol_options.clear();
    }

    pub fn has_http2_protocol_options(&self) -> bool {
        self.http2_protocol_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http2_protocol_options(&mut self, v: Http2ProtocolOptions) {
        self.http2_protocol_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http2_protocol_options(&mut self) -> &mut Http2ProtocolOptions {
        if self.http2_protocol_options.is_none() {
            self.http2_protocol_options.set_default();
        }
        self.http2_protocol_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_http2_protocol_options(&mut self) -> Http2ProtocolOptions {
        self.http2_protocol_options.take().unwrap_or_else(|| Http2ProtocolOptions::new())
    }

    pub fn get_http2_protocol_options(&self) -> &Http2ProtocolOptions {
        self.http2_protocol_options.as_ref().unwrap_or_else(|| Http2ProtocolOptions::default_instance())
    }

    fn get_http2_protocol_options_for_reflect(&self) -> &::protobuf::SingularPtrField<Http2ProtocolOptions> {
        &self.http2_protocol_options
    }

    fn mut_http2_protocol_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Http2ProtocolOptions> {
        &mut self.http2_protocol_options
    }
}

impl ::protobuf::Message for GrpcProtocolOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.http2_protocol_options {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.http2_protocol_options)?;
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
        if let Some(ref v) = self.http2_protocol_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.http2_protocol_options.as_ref() {
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

impl ::protobuf::MessageStatic for GrpcProtocolOptions {
    fn new() -> GrpcProtocolOptions {
        GrpcProtocolOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<GrpcProtocolOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Http2ProtocolOptions>>(
                    "http2_protocol_options",
                    GrpcProtocolOptions::get_http2_protocol_options_for_reflect,
                    GrpcProtocolOptions::mut_http2_protocol_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GrpcProtocolOptions>(
                    "GrpcProtocolOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GrpcProtocolOptions {
    fn clear(&mut self) {
        self.clear_http2_protocol_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcProtocolOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcProtocolOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12api/protocol.proto\x12\x0cenvoy.api.v2\x1a\x1egoogle/protobuf/wrap\
    pers.proto\"\x14\n\x12TcpProtocolOptions\"`\n\x14Http1ProtocolOptions\
    \x12H\n\x12allow_absolute_url\x18\x01\x20\x01(\x0b2\x1a.google.protobuf.\
    BoolValueR\x10allowAbsoluteUrl\"\xf0\x02\n\x14Http2ProtocolOptions\x12F\
    \n\x10hpack_table_size\x18\x01\x20\x01(\x0b2\x1c.google.protobuf.UInt32V\
    alueR\x0ehpackTableSize\x12R\n\x16max_concurrent_streams\x18\x02\x20\x01\
    (\x0b2\x1c.google.protobuf.UInt32ValueR\x14maxConcurrentStreams\x12Y\n\
    \x1ainitial_stream_window_size\x18\x03\x20\x01(\x0b2\x1c.google.protobuf\
    .UInt32ValueR\x17initialStreamWindowSize\x12a\n\x1einitial_connection_wi\
    ndow_size\x18\x04\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x1binit\
    ialConnectionWindowSize\"o\n\x13GrpcProtocolOptions\x12X\n\x16http2_prot\
    ocol_options\x18\x01\x20\x01(\x0b2\".envoy.api.v2.Http2ProtocolOptionsR\
    \x14http2ProtocolOptionsJ\xab\x04\n\x06\x12\x04\0\0\x16\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\n\t\n\x02\x03\0\
    \x12\x03\x04\x07'\n\n\n\x02\x04\0\x12\x04\x06\0\x07\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x06\x08\x1a\n\n\n\x02\x04\x01\x12\x04\t\0\x0b\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\t\x08\x1c\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x023\
    \n\r\n\x05\x04\x01\x02\0\x04\x12\x04\n\x02\t\x1e\n\x0c\n\x05\x04\x01\x02\
    \0\x06\x12\x03\n\x02\x1b\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\x1c.\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n12\n\n\n\x02\x04\x02\x12\x04\r\0\
    \x12\x01\n\n\n\x03\x04\x02\x01\x12\x03\r\x08\x1c\n\x0b\n\x04\x04\x02\x02\
    \0\x12\x03\x0e\x023\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x0e\x02\r\x1e\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x0e\x02\x1d\n\x0c\n\x05\x04\x02\x02\
    \0\x01\x12\x03\x0e\x1e.\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0e12\n\
    \x0b\n\x04\x04\x02\x02\x01\x12\x03\x0f\x029\n\r\n\x05\x04\x02\x02\x01\
    \x04\x12\x04\x0f\x02\x0e3\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x0f\
    \x02\x1d\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x0f\x1e4\n\x0c\n\x05\
    \x04\x02\x02\x01\x03\x12\x03\x0f78\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\
    \x10\x02=\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x10\x02\x0f9\n\x0c\n\x05\
    \x04\x02\x02\x02\x06\x12\x03\x10\x02\x1d\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x10\x1e8\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x10;<\n\x0b\n\
    \x04\x04\x02\x02\x03\x12\x03\x11\x02A\n\r\n\x05\x04\x02\x02\x03\x04\x12\
    \x04\x11\x02\x10=\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03\x11\x02\x1d\n\
    \x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x11\x1e<\n\x0c\n\x05\x04\x02\x02\
    \x03\x03\x12\x03\x11?@\n\n\n\x02\x04\x03\x12\x04\x14\0\x16\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\x14\x08\x1b\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x15\
    \x022\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\x15\x02\x14\x1d\n\x0c\n\x05\
    \x04\x03\x02\0\x06\x12\x03\x15\x02\x16\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x15\x17-\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1501b\x06proto3\
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
