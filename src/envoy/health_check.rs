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
pub struct HealthCheck {
    // message fields
    pub timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub interval_jitter: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub unhealthy_threshold: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub healthy_threshold: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub alt_port: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub reuse_connection: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // message oneof groups
    health_checker: ::std::option::Option<HealthCheck_oneof_health_checker>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheck {}

#[derive(Clone,PartialEq)]
pub enum HealthCheck_oneof_health_checker {
    http_health_check(HealthCheck_HttpHealthCheck),
    tcp_health_check(HealthCheck_TcpHealthCheck),
    redis_health_check(HealthCheck_RedisHealthCheck),
}

impl HealthCheck {
    pub fn new() -> HealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck,
        };
        unsafe {
            instance.get(HealthCheck::new)
        }
    }

    // .google.protobuf.Duration timeout = 1;

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

    // .google.protobuf.Duration interval = 2;

    pub fn clear_interval(&mut self) {
        self.interval.clear();
    }

    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.interval.is_none() {
            self.interval.set_default();
        }
        self.interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.interval.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_interval_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.interval
    }

    fn mut_interval_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.interval
    }

    // .google.protobuf.Duration interval_jitter = 3;

    pub fn clear_interval_jitter(&mut self) {
        self.interval_jitter.clear();
    }

    pub fn has_interval_jitter(&self) -> bool {
        self.interval_jitter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_jitter(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.interval_jitter = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interval_jitter(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.interval_jitter.is_none() {
            self.interval_jitter.set_default();
        }
        self.interval_jitter.as_mut().unwrap()
    }

    // Take field
    pub fn take_interval_jitter(&mut self) -> ::protobuf::well_known_types::Duration {
        self.interval_jitter.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    pub fn get_interval_jitter(&self) -> &::protobuf::well_known_types::Duration {
        self.interval_jitter.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Duration::default_instance())
    }

    fn get_interval_jitter_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &self.interval_jitter
    }

    fn mut_interval_jitter_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration> {
        &mut self.interval_jitter
    }

    // .google.protobuf.UInt32Value unhealthy_threshold = 4;

    pub fn clear_unhealthy_threshold(&mut self) {
        self.unhealthy_threshold.clear();
    }

    pub fn has_unhealthy_threshold(&self) -> bool {
        self.unhealthy_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unhealthy_threshold(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.unhealthy_threshold = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unhealthy_threshold(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.unhealthy_threshold.is_none() {
            self.unhealthy_threshold.set_default();
        }
        self.unhealthy_threshold.as_mut().unwrap()
    }

    // Take field
    pub fn take_unhealthy_threshold(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.unhealthy_threshold.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_unhealthy_threshold(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.unhealthy_threshold.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_unhealthy_threshold_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.unhealthy_threshold
    }

    fn mut_unhealthy_threshold_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.unhealthy_threshold
    }

    // .google.protobuf.UInt32Value healthy_threshold = 5;

    pub fn clear_healthy_threshold(&mut self) {
        self.healthy_threshold.clear();
    }

    pub fn has_healthy_threshold(&self) -> bool {
        self.healthy_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_healthy_threshold(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.healthy_threshold = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_healthy_threshold(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.healthy_threshold.is_none() {
            self.healthy_threshold.set_default();
        }
        self.healthy_threshold.as_mut().unwrap()
    }

    // Take field
    pub fn take_healthy_threshold(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.healthy_threshold.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_healthy_threshold(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.healthy_threshold.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_healthy_threshold_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.healthy_threshold
    }

    fn mut_healthy_threshold_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.healthy_threshold
    }

    // .google.protobuf.UInt32Value alt_port = 6;

    pub fn clear_alt_port(&mut self) {
        self.alt_port.clear();
    }

    pub fn has_alt_port(&self) -> bool {
        self.alt_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alt_port(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.alt_port = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alt_port(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.alt_port.is_none() {
            self.alt_port.set_default();
        }
        self.alt_port.as_mut().unwrap()
    }

    // Take field
    pub fn take_alt_port(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.alt_port.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    pub fn get_alt_port(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.alt_port.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::default_instance())
    }

    fn get_alt_port_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &self.alt_port
    }

    fn mut_alt_port_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value> {
        &mut self.alt_port
    }

    // .google.protobuf.BoolValue reuse_connection = 7;

    pub fn clear_reuse_connection(&mut self) {
        self.reuse_connection.clear();
    }

    pub fn has_reuse_connection(&self) -> bool {
        self.reuse_connection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reuse_connection(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.reuse_connection = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reuse_connection(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.reuse_connection.is_none() {
            self.reuse_connection.set_default();
        }
        self.reuse_connection.as_mut().unwrap()
    }

    // Take field
    pub fn take_reuse_connection(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.reuse_connection.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }

    pub fn get_reuse_connection(&self) -> &::protobuf::well_known_types::BoolValue {
        self.reuse_connection.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::default_instance())
    }

    fn get_reuse_connection_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &self.reuse_connection
    }

    fn mut_reuse_connection_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue> {
        &mut self.reuse_connection
    }

    // .envoy.api.v2.HealthCheck.HttpHealthCheck http_health_check = 8;

    pub fn clear_http_health_check(&mut self) {
        self.health_checker = ::std::option::Option::None;
    }

    pub fn has_http_health_check(&self) -> bool {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_http_health_check(&mut self, v: HealthCheck_HttpHealthCheck) {
        self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(v))
    }

    // Mutable pointer to the field.
    pub fn mut_http_health_check(&mut self) -> &mut HealthCheck_HttpHealthCheck {
        if let ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(_)) = self.health_checker {
        } else {
            self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(HealthCheck_HttpHealthCheck::new()));
        }
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_http_health_check(&mut self) -> HealthCheck_HttpHealthCheck {
        if self.has_http_health_check() {
            match self.health_checker.take() {
                ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(v)) => v,
                _ => panic!(),
            }
        } else {
            HealthCheck_HttpHealthCheck::new()
        }
    }

    pub fn get_http_health_check(&self) -> &HealthCheck_HttpHealthCheck {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(ref v)) => v,
            _ => HealthCheck_HttpHealthCheck::default_instance(),
        }
    }

    // .envoy.api.v2.HealthCheck.TcpHealthCheck tcp_health_check = 9;

    pub fn clear_tcp_health_check(&mut self) {
        self.health_checker = ::std::option::Option::None;
    }

    pub fn has_tcp_health_check(&self) -> bool {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tcp_health_check(&mut self, v: HealthCheck_TcpHealthCheck) {
        self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tcp_health_check(&mut self) -> &mut HealthCheck_TcpHealthCheck {
        if let ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(_)) = self.health_checker {
        } else {
            self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(HealthCheck_TcpHealthCheck::new()));
        }
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tcp_health_check(&mut self) -> HealthCheck_TcpHealthCheck {
        if self.has_tcp_health_check() {
            match self.health_checker.take() {
                ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(v)) => v,
                _ => panic!(),
            }
        } else {
            HealthCheck_TcpHealthCheck::new()
        }
    }

    pub fn get_tcp_health_check(&self) -> &HealthCheck_TcpHealthCheck {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(ref v)) => v,
            _ => HealthCheck_TcpHealthCheck::default_instance(),
        }
    }

    // .envoy.api.v2.HealthCheck.RedisHealthCheck redis_health_check = 10;

    pub fn clear_redis_health_check(&mut self) {
        self.health_checker = ::std::option::Option::None;
    }

    pub fn has_redis_health_check(&self) -> bool {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_redis_health_check(&mut self, v: HealthCheck_RedisHealthCheck) {
        self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(v))
    }

    // Mutable pointer to the field.
    pub fn mut_redis_health_check(&mut self) -> &mut HealthCheck_RedisHealthCheck {
        if let ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(_)) = self.health_checker {
        } else {
            self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(HealthCheck_RedisHealthCheck::new()));
        }
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_redis_health_check(&mut self) -> HealthCheck_RedisHealthCheck {
        if self.has_redis_health_check() {
            match self.health_checker.take() {
                ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(v)) => v,
                _ => panic!(),
            }
        } else {
            HealthCheck_RedisHealthCheck::new()
        }
    }

    pub fn get_redis_health_check(&self) -> &HealthCheck_RedisHealthCheck {
        match self.health_checker {
            ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(ref v)) => v,
            _ => HealthCheck_RedisHealthCheck::default_instance(),
        }
    }
}

impl ::protobuf::Message for HealthCheck {
    fn is_initialized(&self) -> bool {
        for v in &self.timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interval_jitter {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unhealthy_threshold {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.healthy_threshold {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.alt_port {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reuse_connection {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(HealthCheck_oneof_health_checker::http_health_check(ref v)) = self.health_checker {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(HealthCheck_oneof_health_checker::tcp_health_check(ref v)) = self.health_checker {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(HealthCheck_oneof_health_checker::redis_health_check(ref v)) = self.health_checker {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timeout)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interval_jitter)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unhealthy_threshold)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.healthy_threshold)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alt_port)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reuse_connection)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::http_health_check(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::tcp_health_check(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.health_checker = ::std::option::Option::Some(HealthCheck_oneof_health_checker::redis_health_check(is.read_message()?));
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
        if let Some(ref v) = self.timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.interval_jitter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unhealthy_threshold.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.healthy_threshold.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.alt_port.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.reuse_connection.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.health_checker {
            match v {
                &HealthCheck_oneof_health_checker::http_health_check(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &HealthCheck_oneof_health_checker::tcp_health_check(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &HealthCheck_oneof_health_checker::redis_health_check(ref v) => {
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
        if let Some(ref v) = self.timeout.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.interval.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.interval_jitter.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unhealthy_threshold.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.healthy_threshold.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.alt_port.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.reuse_connection.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.health_checker {
            match v {
                &HealthCheck_oneof_health_checker::http_health_check(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &HealthCheck_oneof_health_checker::tcp_health_check(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &HealthCheck_oneof_health_checker::redis_health_check(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for HealthCheck {
    fn new() -> HealthCheck {
        HealthCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "timeout",
                    HealthCheck::get_timeout_for_reflect,
                    HealthCheck::mut_timeout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "interval",
                    HealthCheck::get_interval_for_reflect,
                    HealthCheck::mut_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                    "interval_jitter",
                    HealthCheck::get_interval_jitter_for_reflect,
                    HealthCheck::mut_interval_jitter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "unhealthy_threshold",
                    HealthCheck::get_unhealthy_threshold_for_reflect,
                    HealthCheck::mut_unhealthy_threshold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "healthy_threshold",
                    HealthCheck::get_healthy_threshold_for_reflect,
                    HealthCheck::mut_healthy_threshold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                    "alt_port",
                    HealthCheck::get_alt_port_for_reflect,
                    HealthCheck::mut_alt_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                    "reuse_connection",
                    HealthCheck::get_reuse_connection_for_reflect,
                    HealthCheck::mut_reuse_connection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, HealthCheck_HttpHealthCheck>(
                    "http_health_check",
                    HealthCheck::has_http_health_check,
                    HealthCheck::get_http_health_check,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, HealthCheck_TcpHealthCheck>(
                    "tcp_health_check",
                    HealthCheck::has_tcp_health_check,
                    HealthCheck::get_tcp_health_check,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, HealthCheck_RedisHealthCheck>(
                    "redis_health_check",
                    HealthCheck::has_redis_health_check,
                    HealthCheck::get_redis_health_check,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck>(
                    "HealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheck {
    fn clear(&mut self) {
        self.clear_timeout();
        self.clear_interval();
        self.clear_interval_jitter();
        self.clear_unhealthy_threshold();
        self.clear_healthy_threshold();
        self.clear_alt_port();
        self.clear_reuse_connection();
        self.clear_http_health_check();
        self.clear_tcp_health_check();
        self.clear_redis_health_check();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheck_Payload {
    // message oneof groups
    payload: ::std::option::Option<HealthCheck_Payload_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheck_Payload {}

#[derive(Clone,PartialEq)]
pub enum HealthCheck_Payload_oneof_payload {
    text(::std::string::String),
    binary(::std::vec::Vec<u8>),
}

impl HealthCheck_Payload {
    pub fn new() -> HealthCheck_Payload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck_Payload {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck_Payload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck_Payload,
        };
        unsafe {
            instance.get(HealthCheck_Payload::new)
        }
    }

    // string text = 1;

    pub fn clear_text(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_text(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(::std::string::String::new()));
        }
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        if self.has_text() {
            match self.payload.take() {
                ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_text(&self) -> &str {
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(ref v)) => v,
            _ => "",
        }
    }

    // bytes binary = 2;

    pub fn clear_binary(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_binary(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_binary(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(v))
    }

    // Mutable pointer to the field.
    pub fn mut_binary(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(::std::vec::Vec::new()));
        }
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_binary(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_binary() {
            match self.payload.take() {
                ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_binary(&self) -> &[u8] {
        match self.payload {
            ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(ref v)) => v,
            _ => &[],
        }
    }
}

impl ::protobuf::Message for HealthCheck_Payload {
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
                    self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::text(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.payload = ::std::option::Option::Some(HealthCheck_Payload_oneof_payload::binary(is.read_bytes()?));
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &HealthCheck_Payload_oneof_payload::text(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &HealthCheck_Payload_oneof_payload::binary(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &HealthCheck_Payload_oneof_payload::text(ref v) => {
                    os.write_string(1, v)?;
                },
                &HealthCheck_Payload_oneof_payload::binary(ref v) => {
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

impl ::protobuf::MessageStatic for HealthCheck_Payload {
    fn new() -> HealthCheck_Payload {
        HealthCheck_Payload::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheck_Payload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "text",
                    HealthCheck_Payload::has_text,
                    HealthCheck_Payload::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "binary",
                    HealthCheck_Payload::has_binary,
                    HealthCheck_Payload::get_binary,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck_Payload>(
                    "HealthCheck_Payload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheck_Payload {
    fn clear(&mut self) {
        self.clear_text();
        self.clear_binary();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheck_Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheck_Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheck_HttpHealthCheck {
    // message fields
    pub host: ::std::string::String,
    pub path: ::std::string::String,
    pub send: ::protobuf::SingularPtrField<HealthCheck_Payload>,
    pub receive: ::protobuf::SingularPtrField<HealthCheck_Payload>,
    pub service_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheck_HttpHealthCheck {}

impl HealthCheck_HttpHealthCheck {
    pub fn new() -> HealthCheck_HttpHealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck_HttpHealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck_HttpHealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck_HttpHealthCheck,
        };
        unsafe {
            instance.get(HealthCheck_HttpHealthCheck::new)
        }
    }

    // string host = 1;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.host, ::std::string::String::new())
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    fn get_host_for_reflect(&self) -> &::std::string::String {
        &self.host
    }

    fn mut_host_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.host
    }

    // string path = 2;

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

    // .envoy.api.v2.HealthCheck.Payload send = 3;

    pub fn clear_send(&mut self) {
        self.send.clear();
    }

    pub fn has_send(&self) -> bool {
        self.send.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send(&mut self, v: HealthCheck_Payload) {
        self.send = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send(&mut self) -> &mut HealthCheck_Payload {
        if self.send.is_none() {
            self.send.set_default();
        }
        self.send.as_mut().unwrap()
    }

    // Take field
    pub fn take_send(&mut self) -> HealthCheck_Payload {
        self.send.take().unwrap_or_else(|| HealthCheck_Payload::new())
    }

    pub fn get_send(&self) -> &HealthCheck_Payload {
        self.send.as_ref().unwrap_or_else(|| HealthCheck_Payload::default_instance())
    }

    fn get_send_for_reflect(&self) -> &::protobuf::SingularPtrField<HealthCheck_Payload> {
        &self.send
    }

    fn mut_send_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HealthCheck_Payload> {
        &mut self.send
    }

    // .envoy.api.v2.HealthCheck.Payload receive = 4;

    pub fn clear_receive(&mut self) {
        self.receive.clear();
    }

    pub fn has_receive(&self) -> bool {
        self.receive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_receive(&mut self, v: HealthCheck_Payload) {
        self.receive = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_receive(&mut self) -> &mut HealthCheck_Payload {
        if self.receive.is_none() {
            self.receive.set_default();
        }
        self.receive.as_mut().unwrap()
    }

    // Take field
    pub fn take_receive(&mut self) -> HealthCheck_Payload {
        self.receive.take().unwrap_or_else(|| HealthCheck_Payload::new())
    }

    pub fn get_receive(&self) -> &HealthCheck_Payload {
        self.receive.as_ref().unwrap_or_else(|| HealthCheck_Payload::default_instance())
    }

    fn get_receive_for_reflect(&self) -> &::protobuf::SingularPtrField<HealthCheck_Payload> {
        &self.receive
    }

    fn mut_receive_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HealthCheck_Payload> {
        &mut self.receive
    }

    // string service_name = 5;

    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_name, ::std::string::String::new())
    }

    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }

    fn get_service_name_for_reflect(&self) -> &::std::string::String {
        &self.service_name
    }

    fn mut_service_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }
}

impl ::protobuf::Message for HealthCheck_HttpHealthCheck {
    fn is_initialized(&self) -> bool {
        for v in &self.send {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.receive {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.host)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.send)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.receive)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_name)?;
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
        if !self.host.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.host);
        }
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.path);
        }
        if let Some(ref v) = self.send.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.receive.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.service_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.service_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.host.is_empty() {
            os.write_string(1, &self.host)?;
        }
        if !self.path.is_empty() {
            os.write_string(2, &self.path)?;
        }
        if let Some(ref v) = self.send.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.receive.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.service_name.is_empty() {
            os.write_string(5, &self.service_name)?;
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

impl ::protobuf::MessageStatic for HealthCheck_HttpHealthCheck {
    fn new() -> HealthCheck_HttpHealthCheck {
        HealthCheck_HttpHealthCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheck_HttpHealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host",
                    HealthCheck_HttpHealthCheck::get_host_for_reflect,
                    HealthCheck_HttpHealthCheck::mut_host_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    HealthCheck_HttpHealthCheck::get_path_for_reflect,
                    HealthCheck_HttpHealthCheck::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HealthCheck_Payload>>(
                    "send",
                    HealthCheck_HttpHealthCheck::get_send_for_reflect,
                    HealthCheck_HttpHealthCheck::mut_send_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HealthCheck_Payload>>(
                    "receive",
                    HealthCheck_HttpHealthCheck::get_receive_for_reflect,
                    HealthCheck_HttpHealthCheck::mut_receive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service_name",
                    HealthCheck_HttpHealthCheck::get_service_name_for_reflect,
                    HealthCheck_HttpHealthCheck::mut_service_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck_HttpHealthCheck>(
                    "HealthCheck_HttpHealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheck_HttpHealthCheck {
    fn clear(&mut self) {
        self.clear_host();
        self.clear_path();
        self.clear_send();
        self.clear_receive();
        self.clear_service_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheck_HttpHealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheck_HttpHealthCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheck_TcpHealthCheck {
    // message fields
    pub send: ::protobuf::SingularPtrField<HealthCheck_Payload>,
    pub receive: ::protobuf::RepeatedField<HealthCheck_Payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheck_TcpHealthCheck {}

impl HealthCheck_TcpHealthCheck {
    pub fn new() -> HealthCheck_TcpHealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck_TcpHealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck_TcpHealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck_TcpHealthCheck,
        };
        unsafe {
            instance.get(HealthCheck_TcpHealthCheck::new)
        }
    }

    // .envoy.api.v2.HealthCheck.Payload send = 1;

    pub fn clear_send(&mut self) {
        self.send.clear();
    }

    pub fn has_send(&self) -> bool {
        self.send.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send(&mut self, v: HealthCheck_Payload) {
        self.send = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send(&mut self) -> &mut HealthCheck_Payload {
        if self.send.is_none() {
            self.send.set_default();
        }
        self.send.as_mut().unwrap()
    }

    // Take field
    pub fn take_send(&mut self) -> HealthCheck_Payload {
        self.send.take().unwrap_or_else(|| HealthCheck_Payload::new())
    }

    pub fn get_send(&self) -> &HealthCheck_Payload {
        self.send.as_ref().unwrap_or_else(|| HealthCheck_Payload::default_instance())
    }

    fn get_send_for_reflect(&self) -> &::protobuf::SingularPtrField<HealthCheck_Payload> {
        &self.send
    }

    fn mut_send_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HealthCheck_Payload> {
        &mut self.send
    }

    // repeated .envoy.api.v2.HealthCheck.Payload receive = 2;

    pub fn clear_receive(&mut self) {
        self.receive.clear();
    }

    // Param is passed by value, moved
    pub fn set_receive(&mut self, v: ::protobuf::RepeatedField<HealthCheck_Payload>) {
        self.receive = v;
    }

    // Mutable pointer to the field.
    pub fn mut_receive(&mut self) -> &mut ::protobuf::RepeatedField<HealthCheck_Payload> {
        &mut self.receive
    }

    // Take field
    pub fn take_receive(&mut self) -> ::protobuf::RepeatedField<HealthCheck_Payload> {
        ::std::mem::replace(&mut self.receive, ::protobuf::RepeatedField::new())
    }

    pub fn get_receive(&self) -> &[HealthCheck_Payload] {
        &self.receive
    }

    fn get_receive_for_reflect(&self) -> &::protobuf::RepeatedField<HealthCheck_Payload> {
        &self.receive
    }

    fn mut_receive_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HealthCheck_Payload> {
        &mut self.receive
    }
}

impl ::protobuf::Message for HealthCheck_TcpHealthCheck {
    fn is_initialized(&self) -> bool {
        for v in &self.send {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.receive {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.send)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.receive)?;
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
        if let Some(ref v) = self.send.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.receive {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.send.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.receive {
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

impl ::protobuf::MessageStatic for HealthCheck_TcpHealthCheck {
    fn new() -> HealthCheck_TcpHealthCheck {
        HealthCheck_TcpHealthCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheck_TcpHealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HealthCheck_Payload>>(
                    "send",
                    HealthCheck_TcpHealthCheck::get_send_for_reflect,
                    HealthCheck_TcpHealthCheck::mut_send_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HealthCheck_Payload>>(
                    "receive",
                    HealthCheck_TcpHealthCheck::get_receive_for_reflect,
                    HealthCheck_TcpHealthCheck::mut_receive_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck_TcpHealthCheck>(
                    "HealthCheck_TcpHealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheck_TcpHealthCheck {
    fn clear(&mut self) {
        self.clear_send();
        self.clear_receive();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheck_TcpHealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheck_TcpHealthCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HealthCheck_RedisHealthCheck {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HealthCheck_RedisHealthCheck {}

impl HealthCheck_RedisHealthCheck {
    pub fn new() -> HealthCheck_RedisHealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck_RedisHealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck_RedisHealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck_RedisHealthCheck,
        };
        unsafe {
            instance.get(HealthCheck_RedisHealthCheck::new)
        }
    }
}

impl ::protobuf::Message for HealthCheck_RedisHealthCheck {
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

impl ::protobuf::MessageStatic for HealthCheck_RedisHealthCheck {
    fn new() -> HealthCheck_RedisHealthCheck {
        HealthCheck_RedisHealthCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<HealthCheck_RedisHealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck_RedisHealthCheck>(
                    "HealthCheck_RedisHealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HealthCheck_RedisHealthCheck {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HealthCheck_RedisHealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthCheck_RedisHealthCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HealthStatus {
    UNKNOWN = 0,
    HEALTHY = 1,
    UNHEALTHY = 2,
    DRAINING = 3,
    TIMEOUT = 4,
}

impl ::protobuf::ProtobufEnum for HealthStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HealthStatus> {
        match value {
            0 => ::std::option::Option::Some(HealthStatus::UNKNOWN),
            1 => ::std::option::Option::Some(HealthStatus::HEALTHY),
            2 => ::std::option::Option::Some(HealthStatus::UNHEALTHY),
            3 => ::std::option::Option::Some(HealthStatus::DRAINING),
            4 => ::std::option::Option::Some(HealthStatus::TIMEOUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HealthStatus] = &[
            HealthStatus::UNKNOWN,
            HealthStatus::HEALTHY,
            HealthStatus::UNHEALTHY,
            HealthStatus::DRAINING,
            HealthStatus::TIMEOUT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HealthStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HealthStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HealthStatus {
}

impl ::std::default::Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for HealthStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16api/health_check.proto\x12\x0cenvoy.api.v2\x1a\x1egoogle/protobuf/\
    duration.proto\x1a\x1egoogle/protobuf/wrappers.proto\"\xa8\t\n\x0bHealth\
    Check\x123\n\x07timeout\x18\x01\x20\x01(\x0b2\x19.google.protobuf.Durati\
    onR\x07timeout\x125\n\x08interval\x18\x02\x20\x01(\x0b2\x19.google.proto\
    buf.DurationR\x08interval\x12B\n\x0finterval_jitter\x18\x03\x20\x01(\x0b\
    2\x19.google.protobuf.DurationR\x0eintervalJitter\x12M\n\x13unhealthy_th\
    reshold\x18\x04\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x12unheal\
    thyThreshold\x12I\n\x11healthy_threshold\x18\x05\x20\x01(\x0b2\x1c.googl\
    e.protobuf.UInt32ValueR\x10healthyThreshold\x127\n\x08alt_port\x18\x06\
    \x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x07altPort\x12E\n\x10reu\
    se_connection\x18\x07\x20\x01(\x0b2\x1a.google.protobuf.BoolValueR\x0fre\
    useConnection\x12W\n\x11http_health_check\x18\x08\x20\x01(\x0b2).envoy.a\
    pi.v2.HealthCheck.HttpHealthCheckH\0R\x0fhttpHealthCheck\x12T\n\x10tcp_h\
    ealth_check\x18\t\x20\x01(\x0b2(.envoy.api.v2.HealthCheck.TcpHealthCheck\
    H\0R\x0etcpHealthCheck\x12Z\n\x12redis_health_check\x18\n\x20\x01(\x0b2*\
    .envoy.api.v2.HealthCheck.RedisHealthCheckH\0R\x10redisHealthCheck\x1aD\
    \n\x07Payload\x12\x14\n\x04text\x18\x01\x20\x01(\tH\0R\x04text\x12\x18\n\
    \x06binary\x18\x02\x20\x01(\x0cH\0R\x06binaryB\t\n\x07payload\x1a\xd0\
    \x01\n\x0fHttpHealthCheck\x12\x12\n\x04host\x18\x01\x20\x01(\tR\x04host\
    \x12\x12\n\x04path\x18\x02\x20\x01(\tR\x04path\x125\n\x04send\x18\x03\
    \x20\x01(\x0b2!.envoy.api.v2.HealthCheck.PayloadR\x04send\x12;\n\x07rece\
    ive\x18\x04\x20\x01(\x0b2!.envoy.api.v2.HealthCheck.PayloadR\x07receive\
    \x12!\n\x0cservice_name\x18\x05\x20\x01(\tR\x0bserviceName\x1a\x84\x01\n\
    \x0eTcpHealthCheck\x125\n\x04send\x18\x01\x20\x01(\x0b2!.envoy.api.v2.He\
    althCheck.PayloadR\x04send\x12;\n\x07receive\x18\x02\x20\x03(\x0b2!.envo\
    y.api.v2.HealthCheck.PayloadR\x07receive\x1a\x12\n\x10RedisHealthCheckB\
    \x10\n\x0ehealth_checker*R\n\x0cHealthStatus\x12\x0b\n\x07UNKNOWN\x10\0\
    \x12\x0b\n\x07HEALTHY\x10\x01\x12\r\n\tUNHEALTHY\x10\x02\x12\x0c\n\x08DR\
    AINING\x10\x03\x12\x0b\n\x07TIMEOUT\x10\x04J\xab\x20\n\x06\x12\x04\0\0W\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x14\
    \n\t\n\x02\x03\0\x12\x03\x04\x07'\n\t\n\x02\x03\x01\x12\x03\x05\x07'\n\n\
    \n\x02\x04\0\x12\x04\x07\0H\x01\n\n\n\x03\x04\0\x01\x12\x03\x07\x08\x13\
    \n\x8f\x01\n\x04\x04\0\x02\0\x12\x03\n\x02'\x1a\x81\x01\x20The\x20time\
    \x20to\x20wait\x20for\x20a\x20health\x20check\x20response.\x20If\x20the\
    \x20timeout\x20is\x20reached\x20the\n\x20health\x20check\x20attempt\x20w\
    ill\x20be\x20considered\x20a\x20failure.\n\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\n\x02\x07\x15\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\n\x02\x1a\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\n\x1b\"\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \n%&\n2\n\x04\x04\0\x02\x01\x12\x03\x0c\x02(\x1a%\x20The\x20interval\x20\
    between\x20health\x20checks.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0c\
    \x02\n'\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x0c\x02\x1a\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x0c\x1b#\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x0c&'\n\x94\x01\n\x04\x04\0\x02\x02\x12\x03\x0f\x02/\x1a\x86\x01\
    \x20An\x20optional\x20jitter\x20amount\x20in\x20millseconds.\x20If\x20sp\
    ecified,\x20during\x20every\n\x20internal\x20Envoy\x20will\x20add\x200\
    \x20to\x20interval_jitter\x20to\x20the\x20wait\x20time.\n\n\r\n\x05\x04\
    \0\x02\x02\x04\x12\x04\x0f\x02\x0c(\n\x0c\n\x05\x04\0\x02\x02\x06\x12\
    \x03\x0f\x02\x1a\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0f\x1b*\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x0f-.\n\xee\x01\n\x04\x04\0\x02\x03\x12\
    \x03\x14\x026\x1a\xe0\x01\x20The\x20number\x20of\x20unhealthy\x20health\
    \x20checks\x20required\x20before\x20a\x20host\x20is\x20marked\n\x20unhea\
    lthy.\x20Note\x20that\x20for\x20http\x20health\x20checking\x20if\x20a\
    \x20host\x20responds\x20with\x20503\n\x20this\x20threshold\x20is\x20igno\
    red\x20and\x20the\x20host\x20is\x20considered\x20unhealthy\x20immediatel\
    y.\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x14\x02\x0f/\n\x0c\n\x05\x04\0\
    \x02\x03\x06\x12\x03\x14\x02\x1d\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\
    \x14\x1e1\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x1445\n\xc3\x01\n\x04\
    \x04\0\x02\x04\x12\x03\x18\x024\x1a\xb5\x01\x20The\x20number\x20of\x20he\
    althy\x20health\x20checks\x20required\x20before\x20a\x20host\x20is\x20ma\
    rked\n\x20healthy.\x20Note\x20that\x20during\x20startup,\x20only\x20a\
    \x20single\x20successful\x20health\x20check\x20is\n\x20required\x20to\
    \x20mark\x20a\x20host\x20healthy.\n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\
    \x18\x02\x146\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03\x18\x02\x1d\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\x18\x1e/\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\x1823\n4\n\x04\x04\0\x02\x05\x12\x03\x1b\x02+\x1a'\x20Non-servi\
    ng\x20port\x20for\x20health\x20checking.\n\n\r\n\x05\x04\0\x02\x05\x04\
    \x12\x04\x1b\x02\x184\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03\x1b\x02\x1d\
    \n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x1b\x1e&\n\x0c\n\x05\x04\0\x02\
    \x05\x03\x12\x03\x1b)*\nT\n\x04\x04\0\x02\x06\x12\x03\x1d\x021\x1aG\x20R\
    euse\x20health\x20check\x20connection\x20between\x20health\x20checks.\
    \x20Default\x20is\x20true.\n\n\r\n\x05\x04\0\x02\x06\x04\x12\x04\x1d\x02\
    \x1b+\n\x0c\n\x05\x04\0\x02\x06\x06\x12\x03\x1d\x02\x1b\n\x0c\n\x05\x04\
    \0\x02\x06\x01\x12\x03\x1d\x1c,\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\
    \x1d/0\nJ\n\x04\x04\0\x03\0\x12\x04\x20\x02%\x03\x1a<\x20Describes\x20th\
    e\x20encoding\x20of\x20the\x20payload\x20bytes\x20in\x20the\x20payload\n\
    \n\x0c\n\x05\x04\0\x03\0\x01\x12\x03\x20\n\x11\n\x0e\n\x06\x04\0\x03\0\
    \x08\0\x12\x04!\x04$\x05\n\x0e\n\x07\x04\0\x03\0\x08\0\x01\x12\x03!\n\
    \x11\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03\"\x06\x16\n\x0e\n\x07\x04\0\x03\
    \0\x02\0\x05\x12\x03\"\x06\x0c\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x03\
    \"\r\x11\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\"\x14\x15\n\r\n\x06\
    \x04\0\x03\0\x02\x01\x12\x03#\x06\x17\n\x0e\n\x07\x04\0\x03\0\x02\x01\
    \x05\x12\x03#\x06\x0b\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03#\x0c\
    \x12\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03#\x15\x16\n\x0c\n\x04\
    \x04\0\x03\x01\x12\x04&\x028\x03\n\x0c\n\x05\x04\0\x03\x01\x01\x12\x03&\
    \n\x19\n\xb9\x01\n\x06\x04\0\x03\x01\x02\0\x12\x03*\x04\x14\x1a\xa9\x01\
    \x20The\x20value\x20of\x20the\x20host\x20header\x20in\x20the\x20HTTPS\
    \x20health\x20check\x20request.\x20If\x20left\n\x20empty\x20(default\x20\
    value),\x20the\x20IP\x20on\x20behalf\x20of\x20which\x20this\x20health\
    \x20check\x20is\n\x20performed\x20will\x20be\x20used.\n\n\x0f\n\x07\x04\
    \0\x03\x01\x02\0\x04\x12\x04*\x04&\x1b\n\x0e\n\x07\x04\0\x03\x01\x02\0\
    \x05\x12\x03*\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\0\x01\x12\x03*\x0b\x0f\
    \n\x0e\n\x07\x04\0\x03\x01\x02\0\x03\x12\x03*\x12\x13\n\xa4\x01\n\x06\
    \x04\0\x03\x01\x02\x01\x12\x03-\x04\x14\x1a\x94\x01\x20This\x20parameter\
    \x20is\x20required\x20if\x20the\x20type\x20is\x20http.\x20It\x20species\
    \x20the\x20HTTP\x20path\n\x20that\x20will\x20be\x20requested\x20during\
    \x20health\x20checking.\x20For\x20example\x20/healthcheck.\n\n\x0f\n\x07\
    \x04\0\x03\x01\x02\x01\x04\x12\x04-\x04*\x14\n\x0e\n\x07\x04\0\x03\x01\
    \x02\x01\x05\x12\x03-\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x01\x12\
    \x03-\x0b\x0f\n\x0e\n\x07\x04\0\x03\x01\x02\x01\x03\x12\x03-\x12\x13\n\r\
    \n\x06\x04\0\x03\x01\x02\x02\x12\x03.\x04\x15\n\x0f\n\x07\x04\0\x03\x01\
    \x02\x02\x04\x12\x04.\x04-\x14\n\x0e\n\x07\x04\0\x03\x01\x02\x02\x06\x12\
    \x03.\x04\x0b\n\x0e\n\x07\x04\0\x03\x01\x02\x02\x01\x12\x03.\x0c\x10\n\
    \x0e\n\x07\x04\0\x03\x01\x02\x02\x03\x12\x03.\x13\x14\n\r\n\x06\x04\0\
    \x03\x01\x02\x03\x12\x03/\x04\x18\n\x0f\n\x07\x04\0\x03\x01\x02\x03\x04\
    \x12\x04/\x04.\x15\n\x0e\n\x07\x04\0\x03\x01\x02\x03\x06\x12\x03/\x04\
    \x0b\n\x0e\n\x07\x04\0\x03\x01\x02\x03\x01\x12\x03/\x0c\x13\n\x0e\n\x07\
    \x04\0\x03\x01\x02\x03\x03\x12\x03/\x16\x17\n\xe4\x03\n\x06\x04\0\x03\
    \x01\x02\x04\x12\x037\x04\x1c\x1a\xd4\x03\x20The\x20Envoy\x20HTTP\x20hea\
    lth\x20checker\x20supports\x20the\x20service_name\x20option.\x20If\x20th\
    is\n\x20option\x20is\x20set,\x20the\x20health\x20checker\x20additionally\
    \x20compares\x20the\x20value\x20of\x20the\n\x20x-envoy-upstream-healthch\
    ecked-cluster\x20response\x20header\x20to\x20service_name.\x20If\n\x20th\
    e\x20values\x20do\x20not\x20match,\x20the\x20health\x20check\x20does\x20\
    not\x20pass.\x20The\x20upstream\n\x20health\x20check\x20filter\x20append\
    s\x20x-envoy-upstream-healthchecked-cluster\x20to\x20the\n\x20response\
    \x20headers.\x20The\x20appended\x20value\x20is\x20determined\x20by\x20th\
    e\x20--service-cluster\n\x20command\x20line\x20option.\n\n\x0f\n\x07\x04\
    \0\x03\x01\x02\x04\x04\x12\x047\x04/\x18\n\x0e\n\x07\x04\0\x03\x01\x02\
    \x04\x05\x12\x037\x04\n\n\x0e\n\x07\x04\0\x03\x01\x02\x04\x01\x12\x037\
    \x0b\x17\n\x0e\n\x07\x04\0\x03\x01\x02\x04\x03\x12\x037\x1a\x1b\n\x0c\n\
    \x04\x04\0\x03\x02\x12\x049\x02@\x03\n\x0c\n\x05\x04\0\x03\x02\x01\x12\
    \x039\n\x18\nB\n\x06\x04\0\x03\x02\x02\0\x12\x03;\x04\x15\x1a3\x20Empty\
    \x20payloads\x20imply\x20a\x20connect-only\x20health\x20check.\n\n\x0f\n\
    \x07\x04\0\x03\x02\x02\0\x04\x12\x04;\x049\x1a\n\x0e\n\x07\x04\0\x03\x02\
    \x02\0\x06\x12\x03;\x04\x0b\n\x0e\n\x07\x04\0\x03\x02\x02\0\x01\x12\x03;\
    \x0c\x10\n\x0e\n\x07\x04\0\x03\x02\x02\0\x03\x12\x03;\x13\x14\n\xb7\x01\
    \n\x06\x04\0\x03\x02\x02\x01\x12\x03?\x04!\x1a\xa7\x01\x20When\x20checki\
    ng\x20the\x20response,\x20\xe2\x80\x9cfuzzy\xe2\x80\x9d\x20matching\x20i\
    s\x20performed\x20such\x20that\x20each\n\x20binary\x20block\x20must\x20b\
    e\x20found,\x20and\x20in\x20the\x20order\x20specified,\x20but\x20not\n\
    \x20necessarly\x20contiguous.\n\n\x0e\n\x07\x04\0\x03\x02\x02\x01\x04\
    \x12\x03?\x04\x0c\n\x0e\n\x07\x04\0\x03\x02\x02\x01\x06\x12\x03?\r\x14\n\
    \x0e\n\x07\x04\0\x03\x02\x02\x01\x01\x12\x03?\x15\x1c\n\x0e\n\x07\x04\0\
    \x03\x02\x02\x01\x03\x12\x03?\x1f\x20\n\x0c\n\x04\x04\0\x03\x03\x12\x04A\
    \x02B\x03\n\x0c\n\x05\x04\0\x03\x03\x01\x12\x03A\n\x1a\n\x0c\n\x04\x04\0\
    \x08\0\x12\x04C\x02G\x03\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03C\x08\x16\n\
    \x0b\n\x04\x04\0\x02\x07\x12\x03D\x04*\n\x0c\n\x05\x04\0\x02\x07\x06\x12\
    \x03D\x04\x13\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03D\x14%\n\x0c\n\x05\
    \x04\0\x02\x07\x03\x12\x03D()\n\x0b\n\x04\x04\0\x02\x08\x12\x03E\x04(\n\
    \x0c\n\x05\x04\0\x02\x08\x06\x12\x03E\x04\x12\n\x0c\n\x05\x04\0\x02\x08\
    \x01\x12\x03E\x13#\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03E&'\n\x0b\n\x04\
    \x04\0\x02\t\x12\x03F\x04-\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03F\x04\x14\
    \n\x0c\n\x05\x04\0\x02\t\x01\x12\x03F\x15'\n\x0c\n\x05\x04\0\x02\t\x03\
    \x12\x03F*,\n\n\n\x02\x05\0\x12\x04J\0W\x01\n\n\n\x03\x05\0\x01\x12\x03J\
    \x05\x11\n=\n\x04\x05\0\x02\0\x12\x03L\x02\x0e\x1a0\x20UNKNOWN\x20should\
    \x20be\x20treated\x20by\x20Envoy\x20as\x20HEALTHY.\n\n\x0c\n\x05\x05\0\
    \x02\0\x01\x12\x03L\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03L\x0c\r\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03M\x02\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03M\x02\t\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03M\x0c\r\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03N\x02\x10\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03N\
    \x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03N\x0e\x0f\n\xed\x01\n\x04\
    \x05\0\x02\x03\x12\x03S\x02\x0f\x1a\xdf\x01\x20Connection\x20draining\
    \x20in\x20progress\x20-\n\x20https://aws.amazon.com/blogs/aws/elb-connec\
    tion-draining-remove-instances-from-service-with-care/\n\x20and\n\x20htt\
    ps://cloud.google.com/compute/docs/load-balancing/enabling-connection-dr\
    aining.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03S\x02\n\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03S\r\x0e\n\x89\x01\n\x04\x05\0\x02\x04\x12\x03V\x02\
    \x0e\x1a|\x20This\x20value\x20is\x20used\x20by\x20HDS\x20Remote\x20serve\
    r.\x20From\x20Envoy\xe2\x80\x99s\x20perspective\n\x20TIMEOUT\x20=\x20UNH\
    EALTHY\x20in\x20case\x20EDS\x20returns\x20HealthStatus.\n\n\x0c\n\x05\
    \x05\0\x02\x04\x01\x12\x03V\x02\t\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03V\
    \x0c\rb\x06proto3\
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
