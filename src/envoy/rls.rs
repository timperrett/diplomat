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
pub struct RateLimitRequest {
    // message fields
    pub domain: ::std::string::String,
    pub descriptors: ::protobuf::RepeatedField<RateLimitDescriptor>,
    pub hits_addend: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitRequest {}

impl RateLimitRequest {
    pub fn new() -> RateLimitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitRequest {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitRequest,
        };
        unsafe {
            instance.get(RateLimitRequest::new)
        }
    }

    // string domain = 1;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        &mut self.domain
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.domain, ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }

    fn get_domain_for_reflect(&self) -> &::std::string::String {
        &self.domain
    }

    fn mut_domain_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.domain
    }

    // repeated .envoy.api.v2.RateLimitDescriptor descriptors = 2;

    pub fn clear_descriptors(&mut self) {
        self.descriptors.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptors(&mut self, v: ::protobuf::RepeatedField<RateLimitDescriptor>) {
        self.descriptors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptors(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitDescriptor> {
        &mut self.descriptors
    }

    // Take field
    pub fn take_descriptors(&mut self) -> ::protobuf::RepeatedField<RateLimitDescriptor> {
        ::std::mem::replace(&mut self.descriptors, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptors(&self) -> &[RateLimitDescriptor] {
        &self.descriptors
    }

    fn get_descriptors_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimitDescriptor> {
        &self.descriptors
    }

    fn mut_descriptors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitDescriptor> {
        &mut self.descriptors
    }

    // uint32 hits_addend = 3;

    pub fn clear_hits_addend(&mut self) {
        self.hits_addend = 0;
    }

    // Param is passed by value, moved
    pub fn set_hits_addend(&mut self, v: u32) {
        self.hits_addend = v;
    }

    pub fn get_hits_addend(&self) -> u32 {
        self.hits_addend
    }

    fn get_hits_addend_for_reflect(&self) -> &u32 {
        &self.hits_addend
    }

    fn mut_hits_addend_for_reflect(&mut self) -> &mut u32 {
        &mut self.hits_addend
    }
}

impl ::protobuf::Message for RateLimitRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.descriptors {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.domain)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptors)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hits_addend = tmp;
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
        if !self.domain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.domain);
        }
        for value in &self.descriptors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.hits_addend != 0 {
            my_size += ::protobuf::rt::value_size(3, self.hits_addend, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.domain.is_empty() {
            os.write_string(1, &self.domain)?;
        }
        for v in &self.descriptors {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.hits_addend != 0 {
            os.write_uint32(3, self.hits_addend)?;
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

impl ::protobuf::MessageStatic for RateLimitRequest {
    fn new() -> RateLimitRequest {
        RateLimitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "domain",
                    RateLimitRequest::get_domain_for_reflect,
                    RateLimitRequest::mut_domain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimitDescriptor>>(
                    "descriptors",
                    RateLimitRequest::get_descriptors_for_reflect,
                    RateLimitRequest::mut_descriptors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hits_addend",
                    RateLimitRequest::get_hits_addend_for_reflect,
                    RateLimitRequest::mut_hits_addend_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitRequest>(
                    "RateLimitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitRequest {
    fn clear(&mut self) {
        self.clear_domain();
        self.clear_descriptors();
        self.clear_hits_addend();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitDescriptor {
    // message fields
    pub entries: ::protobuf::RepeatedField<RateLimitDescriptor_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitDescriptor {}

impl RateLimitDescriptor {
    pub fn new() -> RateLimitDescriptor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitDescriptor {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitDescriptor,
        };
        unsafe {
            instance.get(RateLimitDescriptor::new)
        }
    }

    // repeated .envoy.api.v2.RateLimitDescriptor.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<RateLimitDescriptor_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitDescriptor_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<RateLimitDescriptor_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[RateLimitDescriptor_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimitDescriptor_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitDescriptor_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for RateLimitDescriptor {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
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
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
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

impl ::protobuf::MessageStatic for RateLimitDescriptor {
    fn new() -> RateLimitDescriptor {
        RateLimitDescriptor::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitDescriptor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimitDescriptor_Entry>>(
                    "entries",
                    RateLimitDescriptor::get_entries_for_reflect,
                    RateLimitDescriptor::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitDescriptor>(
                    "RateLimitDescriptor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitDescriptor {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitDescriptor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitDescriptor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitDescriptor_Entry {
    // message fields
    pub key: ::std::string::String,
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitDescriptor_Entry {}

impl RateLimitDescriptor_Entry {
    pub fn new() -> RateLimitDescriptor_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitDescriptor_Entry {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitDescriptor_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitDescriptor_Entry,
        };
        unsafe {
            instance.get(RateLimitDescriptor_Entry::new)
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

impl ::protobuf::Message for RateLimitDescriptor_Entry {
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

impl ::protobuf::MessageStatic for RateLimitDescriptor_Entry {
    fn new() -> RateLimitDescriptor_Entry {
        RateLimitDescriptor_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitDescriptor_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    RateLimitDescriptor_Entry::get_key_for_reflect,
                    RateLimitDescriptor_Entry::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    RateLimitDescriptor_Entry::get_value_for_reflect,
                    RateLimitDescriptor_Entry::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitDescriptor_Entry>(
                    "RateLimitDescriptor_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitDescriptor_Entry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitDescriptor_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitDescriptor_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitResponse {
    // message fields
    pub overall_code: RateLimitResponse_Code,
    pub statuses: ::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitResponse {}

impl RateLimitResponse {
    pub fn new() -> RateLimitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitResponse {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitResponse,
        };
        unsafe {
            instance.get(RateLimitResponse::new)
        }
    }

    // .envoy.api.v2.RateLimitResponse.Code overall_code = 1;

    pub fn clear_overall_code(&mut self) {
        self.overall_code = RateLimitResponse_Code::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_overall_code(&mut self, v: RateLimitResponse_Code) {
        self.overall_code = v;
    }

    pub fn get_overall_code(&self) -> RateLimitResponse_Code {
        self.overall_code
    }

    fn get_overall_code_for_reflect(&self) -> &RateLimitResponse_Code {
        &self.overall_code
    }

    fn mut_overall_code_for_reflect(&mut self) -> &mut RateLimitResponse_Code {
        &mut self.overall_code
    }

    // repeated .envoy.api.v2.RateLimitResponse.DescriptorStatus statuses = 2;

    pub fn clear_statuses(&mut self) {
        self.statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_statuses(&mut self, v: ::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus>) {
        self.statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statuses(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus> {
        &mut self.statuses
    }

    // Take field
    pub fn take_statuses(&mut self) -> ::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus> {
        ::std::mem::replace(&mut self.statuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_statuses(&self) -> &[RateLimitResponse_DescriptorStatus] {
        &self.statuses
    }

    fn get_statuses_for_reflect(&self) -> &::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus> {
        &self.statuses
    }

    fn mut_statuses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RateLimitResponse_DescriptorStatus> {
        &mut self.statuses
    }
}

impl ::protobuf::Message for RateLimitResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.statuses {
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
                    self.overall_code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.statuses)?;
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
        if self.overall_code != RateLimitResponse_Code::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.overall_code);
        }
        for value in &self.statuses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.overall_code != RateLimitResponse_Code::UNKNOWN {
            os.write_enum(1, self.overall_code.value())?;
        }
        for v in &self.statuses {
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

impl ::protobuf::MessageStatic for RateLimitResponse {
    fn new() -> RateLimitResponse {
        RateLimitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RateLimitResponse_Code>>(
                    "overall_code",
                    RateLimitResponse::get_overall_code_for_reflect,
                    RateLimitResponse::mut_overall_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimitResponse_DescriptorStatus>>(
                    "statuses",
                    RateLimitResponse::get_statuses_for_reflect,
                    RateLimitResponse::mut_statuses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitResponse>(
                    "RateLimitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitResponse {
    fn clear(&mut self) {
        self.clear_overall_code();
        self.clear_statuses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitResponse_RateLimit {
    // message fields
    pub requests_per_unit: u32,
    pub unit: RateLimitResponse_RateLimit_Unit,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitResponse_RateLimit {}

impl RateLimitResponse_RateLimit {
    pub fn new() -> RateLimitResponse_RateLimit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitResponse_RateLimit {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitResponse_RateLimit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitResponse_RateLimit,
        };
        unsafe {
            instance.get(RateLimitResponse_RateLimit::new)
        }
    }

    // uint32 requests_per_unit = 1;

    pub fn clear_requests_per_unit(&mut self) {
        self.requests_per_unit = 0;
    }

    // Param is passed by value, moved
    pub fn set_requests_per_unit(&mut self, v: u32) {
        self.requests_per_unit = v;
    }

    pub fn get_requests_per_unit(&self) -> u32 {
        self.requests_per_unit
    }

    fn get_requests_per_unit_for_reflect(&self) -> &u32 {
        &self.requests_per_unit
    }

    fn mut_requests_per_unit_for_reflect(&mut self) -> &mut u32 {
        &mut self.requests_per_unit
    }

    // .envoy.api.v2.RateLimitResponse.RateLimit.Unit unit = 2;

    pub fn clear_unit(&mut self) {
        self.unit = RateLimitResponse_RateLimit_Unit::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_unit(&mut self, v: RateLimitResponse_RateLimit_Unit) {
        self.unit = v;
    }

    pub fn get_unit(&self) -> RateLimitResponse_RateLimit_Unit {
        self.unit
    }

    fn get_unit_for_reflect(&self) -> &RateLimitResponse_RateLimit_Unit {
        &self.unit
    }

    fn mut_unit_for_reflect(&mut self) -> &mut RateLimitResponse_RateLimit_Unit {
        &mut self.unit
    }
}

impl ::protobuf::Message for RateLimitResponse_RateLimit {
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
                    let tmp = is.read_uint32()?;
                    self.requests_per_unit = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.unit = tmp;
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
        if self.requests_per_unit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.requests_per_unit, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.unit != RateLimitResponse_RateLimit_Unit::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(2, self.unit);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.requests_per_unit != 0 {
            os.write_uint32(1, self.requests_per_unit)?;
        }
        if self.unit != RateLimitResponse_RateLimit_Unit::UNKNOWN {
            os.write_enum(2, self.unit.value())?;
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

impl ::protobuf::MessageStatic for RateLimitResponse_RateLimit {
    fn new() -> RateLimitResponse_RateLimit {
        RateLimitResponse_RateLimit::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitResponse_RateLimit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "requests_per_unit",
                    RateLimitResponse_RateLimit::get_requests_per_unit_for_reflect,
                    RateLimitResponse_RateLimit::mut_requests_per_unit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RateLimitResponse_RateLimit_Unit>>(
                    "unit",
                    RateLimitResponse_RateLimit::get_unit_for_reflect,
                    RateLimitResponse_RateLimit::mut_unit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitResponse_RateLimit>(
                    "RateLimitResponse_RateLimit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitResponse_RateLimit {
    fn clear(&mut self) {
        self.clear_requests_per_unit();
        self.clear_unit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitResponse_RateLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitResponse_RateLimit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RateLimitResponse_RateLimit_Unit {
    UNKNOWN = 0,
    SECOND = 1,
    MINUTE = 2,
    HOUR = 3,
    DAY = 4,
}

impl ::protobuf::ProtobufEnum for RateLimitResponse_RateLimit_Unit {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RateLimitResponse_RateLimit_Unit> {
        match value {
            0 => ::std::option::Option::Some(RateLimitResponse_RateLimit_Unit::UNKNOWN),
            1 => ::std::option::Option::Some(RateLimitResponse_RateLimit_Unit::SECOND),
            2 => ::std::option::Option::Some(RateLimitResponse_RateLimit_Unit::MINUTE),
            3 => ::std::option::Option::Some(RateLimitResponse_RateLimit_Unit::HOUR),
            4 => ::std::option::Option::Some(RateLimitResponse_RateLimit_Unit::DAY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RateLimitResponse_RateLimit_Unit] = &[
            RateLimitResponse_RateLimit_Unit::UNKNOWN,
            RateLimitResponse_RateLimit_Unit::SECOND,
            RateLimitResponse_RateLimit_Unit::MINUTE,
            RateLimitResponse_RateLimit_Unit::HOUR,
            RateLimitResponse_RateLimit_Unit::DAY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RateLimitResponse_RateLimit_Unit>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RateLimitResponse_RateLimit_Unit", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RateLimitResponse_RateLimit_Unit {
}

impl ::std::default::Default for RateLimitResponse_RateLimit_Unit {
    fn default() -> Self {
        RateLimitResponse_RateLimit_Unit::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitResponse_RateLimit_Unit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RateLimitResponse_DescriptorStatus {
    // message fields
    pub code: RateLimitResponse_Code,
    pub current_limit: ::protobuf::SingularPtrField<RateLimitResponse_RateLimit>,
    pub limit_remaining: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RateLimitResponse_DescriptorStatus {}

impl RateLimitResponse_DescriptorStatus {
    pub fn new() -> RateLimitResponse_DescriptorStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimitResponse_DescriptorStatus {
        static mut instance: ::protobuf::lazy::Lazy<RateLimitResponse_DescriptorStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimitResponse_DescriptorStatus,
        };
        unsafe {
            instance.get(RateLimitResponse_DescriptorStatus::new)
        }
    }

    // .envoy.api.v2.RateLimitResponse.Code code = 1;

    pub fn clear_code(&mut self) {
        self.code = RateLimitResponse_Code::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: RateLimitResponse_Code) {
        self.code = v;
    }

    pub fn get_code(&self) -> RateLimitResponse_Code {
        self.code
    }

    fn get_code_for_reflect(&self) -> &RateLimitResponse_Code {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut RateLimitResponse_Code {
        &mut self.code
    }

    // .envoy.api.v2.RateLimitResponse.RateLimit current_limit = 2;

    pub fn clear_current_limit(&mut self) {
        self.current_limit.clear();
    }

    pub fn has_current_limit(&self) -> bool {
        self.current_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_limit(&mut self, v: RateLimitResponse_RateLimit) {
        self.current_limit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_current_limit(&mut self) -> &mut RateLimitResponse_RateLimit {
        if self.current_limit.is_none() {
            self.current_limit.set_default();
        }
        self.current_limit.as_mut().unwrap()
    }

    // Take field
    pub fn take_current_limit(&mut self) -> RateLimitResponse_RateLimit {
        self.current_limit.take().unwrap_or_else(|| RateLimitResponse_RateLimit::new())
    }

    pub fn get_current_limit(&self) -> &RateLimitResponse_RateLimit {
        self.current_limit.as_ref().unwrap_or_else(|| RateLimitResponse_RateLimit::default_instance())
    }

    fn get_current_limit_for_reflect(&self) -> &::protobuf::SingularPtrField<RateLimitResponse_RateLimit> {
        &self.current_limit
    }

    fn mut_current_limit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RateLimitResponse_RateLimit> {
        &mut self.current_limit
    }

    // uint32 limit_remaining = 3;

    pub fn clear_limit_remaining(&mut self) {
        self.limit_remaining = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit_remaining(&mut self, v: u32) {
        self.limit_remaining = v;
    }

    pub fn get_limit_remaining(&self) -> u32 {
        self.limit_remaining
    }

    fn get_limit_remaining_for_reflect(&self) -> &u32 {
        &self.limit_remaining
    }

    fn mut_limit_remaining_for_reflect(&mut self) -> &mut u32 {
        &mut self.limit_remaining
    }
}

impl ::protobuf::Message for RateLimitResponse_DescriptorStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.current_limit {
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
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.current_limit)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.limit_remaining = tmp;
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
        if self.code != RateLimitResponse_Code::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if let Some(ref v) = self.current_limit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.limit_remaining != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit_remaining, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != RateLimitResponse_Code::UNKNOWN {
            os.write_enum(1, self.code.value())?;
        }
        if let Some(ref v) = self.current_limit.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.limit_remaining != 0 {
            os.write_uint32(3, self.limit_remaining)?;
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

impl ::protobuf::MessageStatic for RateLimitResponse_DescriptorStatus {
    fn new() -> RateLimitResponse_DescriptorStatus {
        RateLimitResponse_DescriptorStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<RateLimitResponse_DescriptorStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RateLimitResponse_Code>>(
                    "code",
                    RateLimitResponse_DescriptorStatus::get_code_for_reflect,
                    RateLimitResponse_DescriptorStatus::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RateLimitResponse_RateLimit>>(
                    "current_limit",
                    RateLimitResponse_DescriptorStatus::get_current_limit_for_reflect,
                    RateLimitResponse_DescriptorStatus::mut_current_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit_remaining",
                    RateLimitResponse_DescriptorStatus::get_limit_remaining_for_reflect,
                    RateLimitResponse_DescriptorStatus::mut_limit_remaining_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RateLimitResponse_DescriptorStatus>(
                    "RateLimitResponse_DescriptorStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RateLimitResponse_DescriptorStatus {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_current_limit();
        self.clear_limit_remaining();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimitResponse_DescriptorStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitResponse_DescriptorStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RateLimitResponse_Code {
    UNKNOWN = 0,
    OK = 1,
    OVER_LIMIT = 2,
}

impl ::protobuf::ProtobufEnum for RateLimitResponse_Code {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RateLimitResponse_Code> {
        match value {
            0 => ::std::option::Option::Some(RateLimitResponse_Code::UNKNOWN),
            1 => ::std::option::Option::Some(RateLimitResponse_Code::OK),
            2 => ::std::option::Option::Some(RateLimitResponse_Code::OVER_LIMIT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RateLimitResponse_Code] = &[
            RateLimitResponse_Code::UNKNOWN,
            RateLimitResponse_Code::OK,
            RateLimitResponse_Code::OVER_LIMIT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RateLimitResponse_Code>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RateLimitResponse_Code", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RateLimitResponse_Code {
}

impl ::std::default::Default for RateLimitResponse_Code {
    fn default() -> Self {
        RateLimitResponse_Code::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitResponse_Code {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rapi/rls.proto\x12\x0cenvoy.api.v2\"\x90\x01\n\x10RateLimitRequest\
    \x12\x16\n\x06domain\x18\x01\x20\x01(\tR\x06domain\x12C\n\x0bdescriptors\
    \x18\x02\x20\x03(\x0b2!.envoy.api.v2.RateLimitDescriptorR\x0bdescriptors\
    \x12\x1f\n\x0bhits_addend\x18\x03\x20\x01(\rR\nhitsAddend\"\x89\x01\n\
    \x13RateLimitDescriptor\x12A\n\x07entries\x18\x01\x20\x03(\x0b2'.envoy.a\
    pi.v2.RateLimitDescriptor.EntryR\x07entries\x1a/\n\x05Entry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\t\
    R\x05value\"\xdd\x04\n\x11RateLimitResponse\x12G\n\x0coverall_code\x18\
    \x01\x20\x01(\x0e2$.envoy.api.v2.RateLimitResponse.CodeR\x0boverallCode\
    \x12L\n\x08statuses\x18\x02\x20\x03(\x0b20.envoy.api.v2.RateLimitRespons\
    e.DescriptorStatusR\x08statuses\x1a\xbb\x01\n\tRateLimit\x12*\n\x11reque\
    sts_per_unit\x18\x01\x20\x01(\rR\x0frequestsPerUnit\x12B\n\x04unit\x18\
    \x02\x20\x01(\x0e2..envoy.api.v2.RateLimitResponse.RateLimit.UnitR\x04un\
    it\">\n\x04Unit\x12\x0b\n\x07UNKNOWN\x10\0\x12\n\n\x06SECOND\x10\x01\x12\
    \n\n\x06MINUTE\x10\x02\x12\x08\n\x04HOUR\x10\x03\x12\x07\n\x03DAY\x10\
    \x04\x1a\xc5\x01\n\x10DescriptorStatus\x128\n\x04code\x18\x01\x20\x01(\
    \x0e2$.envoy.api.v2.RateLimitResponse.CodeR\x04code\x12N\n\rcurrent_limi\
    t\x18\x02\x20\x01(\x0b2).envoy.api.v2.RateLimitResponse.RateLimitR\x0ccu\
    rrentLimit\x12'\n\x0flimit_remaining\x18\x03\x20\x01(\rR\x0elimitRemaini\
    ng\"+\n\x04Code\x12\x0b\n\x07UNKNOWN\x10\0\x12\x06\n\x02OK\x10\x01\x12\
    \x0e\n\nOVER_LIMIT\x10\x022h\n\x10RateLimitService\x12T\n\x0fShouldRateL\
    imit\x12\x1e.envoy.api.v2.RateLimitRequest\x1a\x1f.envoy.api.v2.RateLimi\
    tResponse\"\0J\xee'\n\x06\x12\x04\0\0a\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\n\n\n\x02\x06\0\x12\x04\x04\0\
    \x07\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\x08\x18\nA\n\x04\x06\0\x02\0\
    \x12\x03\x06\x02G\x1a4\x20Determine\x20whether\x20rate\x20limiting\x20sh\
    ould\x20take\x20place.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x06\x06\x15\
    \n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x06\x17'\n\x0c\n\x05\x06\0\x02\0\
    \x03\x12\x03\x062C\n\xc9\x04\n\x02\x04\0\x12\x04\x10\0\x1b\x01\x1a\xbc\
    \x04\x20Main\x20message\x20for\x20a\x20rate\x20limit\x20request.\x20The\
    \x20rate\x20limit\x20service\x20is\x20designed\x20to\x20be\x20fully\x20g\
    eneric\n\x20in\x20the\x20sense\x20that\x20it\x20can\x20operate\x20on\x20\
    arbitrary\x20hierarchical\x20key/value\x20pairs.\x20The\x20loaded\n\x20c\
    onfiguration\x20will\x20parse\x20the\x20request\x20and\x20find\x20the\
    \x20most\x20specific\x20limit\x20to\x20apply.\x20In\x20addition,\n\x20a\
    \x20RateLimitRequest\x20can\x20contain\x20multiple\x20\"descriptors\"\
    \x20to\x20limit\x20on.\x20When\x20multiple\x20descriptors\n\x20are\x20pr\
    ovided,\x20the\x20server\x20will\x20limit\x20on\x20*ALL*\x20of\x20them\
    \x20and\x20return\x20an\x20OVER_LIMIT\x20response\x20if\x20any\n\x20of\
    \x20them\x20are\x20over\x20limit.\x20This\x20enables\x20more\x20complex\
    \x20application\x20level\x20rate\x20limiting\x20scenarios\n\x20if\x20des\
    ired.\n\n\n\n\x03\x04\0\x01\x12\x03\x10\x08\x18\n\x9c\x01\n\x04\x04\0\
    \x02\0\x12\x03\x13\x02\x14\x1a\x8e\x01\x20All\x20rate\x20limit\x20reques\
    ts\x20must\x20specify\x20a\x20domain.\x20This\x20enables\x20the\x20confi\
    guration\x20to\x20be\x20per\n\x20application\x20without\x20fear\x20of\
    \x20overlap.\x20E.g.,\x20\"envoy\".\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x13\x02\x10\x1a\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x13\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x13\t\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x13\x12\x13\n\xee\x01\n\x04\x04\0\x02\x01\x12\x03\x17\x02/\x1a\xe0\
    \x01\x20All\x20rate\x20limit\x20requests\x20must\x20specify\x20at\x20lea\
    st\x20one\x20RateLimitDescriptor.\x20Each\x20descriptor\x20is\n\x20proce\
    ssed\x20by\x20the\x20service\x20(see\x20below).\x20If\x20any\x20of\x20th\
    e\x20descriptors\x20are\x20over\x20limit,\x20the\x20entire\n\x20request\
    \x20is\x20considered\x20to\x20be\x20over\x20limit.\n\n\x0c\n\x05\x04\0\
    \x02\x01\x04\x12\x03\x17\x02\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x17\
    \x0b\x1e\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x17\x1f*\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x17-.\n\xc7\x01\n\x04\x04\0\x02\x02\x12\x03\x1a\
    \x02\x19\x1a\xb9\x01\x20Rate\x20limit\x20requests\x20can\x20optionally\
    \x20specify\x20the\x20number\x20of\x20hits\x20a\x20request\x20adds\x20to\
    \x20the\x20matched\x20limit.\x20If\x20the\n\x20value\x20is\x20not\x20set\
    \x20in\x20the\x20message,\x20a\x20request\x20increases\x20the\x20matched\
    \x20limit\x20by\x201.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x1a\x02\x17/\
    \n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1a\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x1a\t\x14\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1a\x17\
    \x18\n\xe0\x0b\n\x02\x04\x01\x12\x042\09\x01\x1a\xd3\x0b\x20A\x20RateLim\
    itDescriptor\x20is\x20a\x20list\x20of\x20hierarchical\x20entries\x20that\
    \x20are\x20used\x20by\x20the\x20service\x20to\n\x20determine\x20the\x20f\
    inal\x20rate\x20limit\x20key\x20and\x20overall\x20allowed\x20limit.\x20H\
    ere\x20are\x20some\x20examples\x20of\x20how\n\x20they\x20might\x20be\x20\
    used\x20for\x20the\x20domain\x20\"envoy\".\n\x201)\x20[\"authenticated\"\
    :\x20\"false\"],\x20[\"ip_address\":\x20\"10.0.0.1\"]\n\x20\x20\x20\x20W\
    hat\x20it\x20does:\x20Limits\x20all\x20unauthenticated\x20traffic\x20for\
    \x20the\x20IP\x20address\x2010.0.0.1.\x20The\n\x20\x20\x20\x20configurat\
    ion\x20supplies\x20a\x20default\x20limit\x20for\x20the\x20ip_address\x20\
    field.\x20If\x20there\x20is\x20a\x20desire\x20to\x20raise\n\x20\x20\x20\
    \x20the\x20limit\x20for\x2010.0.0.1\x20or\x20block\x20it\x20entirely\x20\
    it\x20can\x20be\x20specified\x20directly\x20in\x20the\n\x20\x20\x20\x20c\
    onfiguration.\n\x202)\x20[\"authenticated\":\x20\"false\"],\x20[\"path\"\
    :\x20\"/foo/bar\"]\n\x20\x20\x20\x20What\x20it\x20does:\x20Limits\x20all\
    \x20unauthenticated\x20traffic\x20globally\x20for\x20a\x20specific\x20pa\
    th\x20(or\x20prefix\x20if\n\x20\x20\x20\x20configured\x20that\x20way\x20\
    in\x20the\x20service).\n\x203)\x20[\"authenticated\":\x20\"false\"],\x20\
    [\"path\":\x20\"/foo/bar\"],\x20[\"ip_address\":\x20\"10.0.0.1\"]\n\x20\
    \x20\x20\x20What\x20it\x20does:\x20Limits\x20unauthenticated\x20traffic\
    \x20to\x20a\x20specific\x20path\x20for\x20a\x20specific\x20IP\x20address\
    .\n\x20\x20\x20\x20Like\x20(1)\x20we\x20can\x20raise/block\x20specific\
    \x20IP\x20addresses\x20if\x20we\x20want\x20with\x20an\x20override\x20con\
    figuration.\n\x204)\x20[\"authenticated\":\x20\"true\"],\x20[\"client_id\
    \":\x20\"foo\"]\n\x20\x20\x20\x20What\x20it\x20does:\x20Limits\x20all\
    \x20traffic\x20for\x20an\x20authenticated\x20client\x20\"foo\"\n\x205)\
    \x20[\"authenticated\":\x20\"true\"],\x20[\"client_id\":\x20\"foo\"],\
    \x20[\"path\":\x20\"/foo/bar\"]\n\x20\x20\x20\x20What\x20it\x20does:\x20\
    Limits\x20traffic\x20to\x20a\x20specific\x20path\x20for\x20an\x20authent\
    icated\x20client\x20\"foo\"\n\n\x20The\x20idea\x20behind\x20the\x20API\
    \x20is\x20that\x20(1)/(2)/(3)\x20and\x20(4)/(5)\x20can\x20be\x20sent\x20\
    in\x201\x20request\x20if\x20desired.\n\x20This\x20enables\x20building\
    \x20complex\x20application\x20scenarios\x20with\x20a\x20generic\x20backe\
    nd.\n\n\n\n\x03\x04\x01\x01\x12\x032\x08\x1b\n\x0c\n\x04\x04\x01\x03\0\
    \x12\x043\x026\x03\n\x0c\n\x05\x04\x01\x03\0\x01\x12\x033\n\x0f\n\r\n\
    \x06\x04\x01\x03\0\x02\0\x12\x034\x04\x13\n\x0f\n\x07\x04\x01\x03\0\x02\
    \0\x04\x12\x044\x043\x11\n\x0e\n\x07\x04\x01\x03\0\x02\0\x05\x12\x034\
    \x04\n\n\x0e\n\x07\x04\x01\x03\0\x02\0\x01\x12\x034\x0b\x0e\n\x0e\n\x07\
    \x04\x01\x03\0\x02\0\x03\x12\x034\x11\x12\n\r\n\x06\x04\x01\x03\0\x02\
    \x01\x12\x035\x04\x15\n\x0f\n\x07\x04\x01\x03\0\x02\x01\x04\x12\x045\x04\
    4\x13\n\x0e\n\x07\x04\x01\x03\0\x02\x01\x05\x12\x035\x04\n\n\x0e\n\x07\
    \x04\x01\x03\0\x02\x01\x01\x12\x035\x0b\x10\n\x0e\n\x07\x04\x01\x03\0\
    \x02\x01\x03\x12\x035\x13\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x038\x02\x1d\
    \n\x0c\n\x05\x04\x01\x02\0\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x038\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x038\x11\x18\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x038\x1b\x1c\n5\n\x02\x04\x02\x12\x04<\
    \0a\x01\x1a)\x20A\x20response\x20from\x20a\x20ShouldRateLimit\x20call.\n\
    \n\n\n\x03\x04\x02\x01\x12\x03<\x08\x19\n\x0c\n\x04\x04\x02\x04\0\x12\
    \x04=\x02A\x03\n\x0c\n\x05\x04\x02\x04\0\x01\x12\x03=\x07\x0b\n\r\n\x06\
    \x04\x02\x04\0\x02\0\x12\x03>\x04\x10\n\x0e\n\x07\x04\x02\x04\0\x02\0\
    \x01\x12\x03>\x04\x0b\n\x0e\n\x07\x04\x02\x04\0\x02\0\x02\x12\x03>\x0e\
    \x0f\n\r\n\x06\x04\x02\x04\0\x02\x01\x12\x03?\x04\x0b\n\x0e\n\x07\x04\
    \x02\x04\0\x02\x01\x01\x12\x03?\x04\x06\n\x0e\n\x07\x04\x02\x04\0\x02\
    \x01\x02\x12\x03?\t\n\n\r\n\x06\x04\x02\x04\0\x02\x02\x12\x03@\x04\x13\n\
    \x0e\n\x07\x04\x02\x04\0\x02\x02\x01\x12\x03@\x04\x0e\n\x0e\n\x07\x04\
    \x02\x04\0\x02\x02\x02\x12\x03@\x11\x12\ng\n\x04\x04\x02\x03\0\x12\x04D\
    \x02O\x03\x1aY\x20Defines\x20an\x20actual\x20rate\x20limit\x20in\x20term\
    s\x20of\x20requests\x20per\x20unit\x20of\x20time\x20and\x20the\x20unit\
    \x20itself.\n\n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03D\n\x13\n\x0e\n\x06\
    \x04\x02\x03\0\x04\0\x12\x04E\x04K\x05\n\x0e\n\x07\x04\x02\x03\0\x04\0\
    \x01\x12\x03E\t\r\n\x0f\n\x08\x04\x02\x03\0\x04\0\x02\0\x12\x03F\x06\x12\
    \n\x10\n\t\x04\x02\x03\0\x04\0\x02\0\x01\x12\x03F\x06\r\n\x10\n\t\x04\
    \x02\x03\0\x04\0\x02\0\x02\x12\x03F\x10\x11\n\x0f\n\x08\x04\x02\x03\0\
    \x04\0\x02\x01\x12\x03G\x06\x12\n\x10\n\t\x04\x02\x03\0\x04\0\x02\x01\
    \x01\x12\x03G\x06\x0c\n\x10\n\t\x04\x02\x03\0\x04\0\x02\x01\x02\x12\x03G\
    \x10\x11\n\x0f\n\x08\x04\x02\x03\0\x04\0\x02\x02\x12\x03H\x06\x12\n\x10\
    \n\t\x04\x02\x03\0\x04\0\x02\x02\x01\x12\x03H\x06\x0c\n\x10\n\t\x04\x02\
    \x03\0\x04\0\x02\x02\x02\x12\x03H\x10\x11\n\x0f\n\x08\x04\x02\x03\0\x04\
    \0\x02\x03\x12\x03I\x06\x12\n\x10\n\t\x04\x02\x03\0\x04\0\x02\x03\x01\
    \x12\x03I\x06\n\n\x10\n\t\x04\x02\x03\0\x04\0\x02\x03\x02\x12\x03I\x10\
    \x11\n\x0f\n\x08\x04\x02\x03\0\x04\0\x02\x04\x12\x03J\x06\x12\n\x10\n\t\
    \x04\x02\x03\0\x04\0\x02\x04\x01\x12\x03J\x06\t\n\x10\n\t\x04\x02\x03\0\
    \x04\0\x02\x04\x02\x12\x03J\x10\x11\n\r\n\x06\x04\x02\x03\0\x02\0\x12\
    \x03M\x04!\n\x0f\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04M\x04K\x05\n\x0e\n\
    \x07\x04\x02\x03\0\x02\0\x05\x12\x03M\x04\n\n\x0e\n\x07\x04\x02\x03\0\
    \x02\0\x01\x12\x03M\x0b\x1c\n\x0e\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03M\
    \x1f\x20\n\r\n\x06\x04\x02\x03\0\x02\x01\x12\x03N\x04\x12\n\x0f\n\x07\
    \x04\x02\x03\0\x02\x01\x04\x12\x04N\x04M!\n\x0e\n\x07\x04\x02\x03\0\x02\
    \x01\x06\x12\x03N\x04\x08\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x03N\
    \t\r\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03N\x10\x11\n\x0c\n\x04\
    \x04\x02\x03\x01\x12\x04Q\x02X\x03\n\x0c\n\x05\x04\x02\x03\x01\x01\x12\
    \x03Q\n\x1a\n@\n\x06\x04\x02\x03\x01\x02\0\x12\x03S\x04\x12\x1a1\x20The\
    \x20response\x20code\x20for\x20an\x20individual\x20descriptor.\n\n\x0f\n\
    \x07\x04\x02\x03\x01\x02\0\x04\x12\x04S\x04Q\x1c\n\x0e\n\x07\x04\x02\x03\
    \x01\x02\0\x06\x12\x03S\x04\x08\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x01\
    \x12\x03S\t\r\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x03\x12\x03S\x10\x11\nZ\
    \n\x06\x04\x02\x03\x01\x02\x01\x12\x03U\x04\x20\x1aK\x20The\x20current\
    \x20limit\x20as\x20configured\x20by\x20the\x20server.\x20Useful\x20for\
    \x20debugging,\x20etc.\n\n\x0f\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x04\
    U\x04S\x12\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x06\x12\x03U\x04\r\n\x0e\
    \n\x07\x04\x02\x03\x01\x02\x01\x01\x12\x03U\x0e\x1b\n\x0e\n\x07\x04\x02\
    \x03\x01\x02\x01\x03\x12\x03U\x1e\x1f\n>\n\x06\x04\x02\x03\x01\x02\x02\
    \x12\x03W\x04\x1f\x1a/\x20The\x20limit\x20remaining\x20in\x20the\x20curr\
    ent\x20time\x20unit.\n\n\x0f\n\x07\x04\x02\x03\x01\x02\x02\x04\x12\x04W\
    \x04U\x20\n\x0e\n\x07\x04\x02\x03\x01\x02\x02\x05\x12\x03W\x04\n\n\x0e\n\
    \x07\x04\x02\x03\x01\x02\x02\x01\x12\x03W\x0b\x1a\n\x0e\n\x07\x04\x02\
    \x03\x01\x02\x02\x03\x12\x03W\x1d\x1e\n\x8b\x01\n\x04\x04\x02\x02\0\x12\
    \x03\\\x02\x18\x1a~\x20The\x20overall\x20response\x20code\x20which\x20ta\
    kes\x20into\x20account\x20all\x20of\x20the\x20descriptors\x20that\x20wer\
    e\x20passed\n\x20in\x20the\x20RateLimitRequest\x20message.\n\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04\\\x02X\x03\n\x0c\n\x05\x04\x02\x02\0\x06\x12\
    \x03\\\x02\x06\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\\\x07\x13\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\\\x16\x17\n\x96\x02\n\x04\x04\x02\x02\x01\
    \x12\x03`\x02)\x1a\x88\x02\x20A\x20list\x20of\x20DescriptorStatus\x20mes\
    sages\x20which\x20matches\x20the\x20length\x20of\x20the\x20descriptor\
    \x20list\x20passed\n\x20in\x20the\x20RateLimitRequest.\x20This\x20can\
    \x20be\x20used\x20by\x20the\x20caller\x20to\x20determine\x20which\x20ind\
    ividual\n\x20descriptors\x20failed\x20and/or\x20what\x20the\x20currently\
    \x20configured\x20limits\x20are\x20for\x20all\x20of\x20them.\n\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x06\
    \x12\x03`\x0b\x1b\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03`\x1c$\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03`'(b\x06proto3\
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
