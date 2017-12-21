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
pub struct Snowflake {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snowflake {}

impl Snowflake {
    pub fn new() -> Snowflake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snowflake {
        static mut instance: ::protobuf::lazy::Lazy<Snowflake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snowflake,
        };
        unsafe {
            instance.get(Snowflake::new)
        }
    }
}

impl ::protobuf::Message for Snowflake {
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

impl ::protobuf::MessageStatic for Snowflake {
    fn new() -> Snowflake {
        Snowflake::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snowflake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Snowflake>(
                    "Snowflake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snowflake {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snowflake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snowflake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snowflake_Key {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snowflake_Key {}

impl Snowflake_Key {
    pub fn new() -> Snowflake_Key {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snowflake_Key {
        static mut instance: ::protobuf::lazy::Lazy<Snowflake_Key> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snowflake_Key,
        };
        unsafe {
            instance.get(Snowflake_Key::new)
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

impl ::protobuf::Message for Snowflake_Key {
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
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

impl ::protobuf::MessageStatic for Snowflake_Key {
    fn new() -> Snowflake_Key {
        Snowflake_Key::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snowflake_Key>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Snowflake_Key::get_name_for_reflect,
                    Snowflake_Key::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snowflake_Key>(
                    "Snowflake_Key",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snowflake_Key {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snowflake_Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snowflake_Key {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snowflake_Value {
    // message fields
    pub value: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snowflake_Value {}

impl Snowflake_Value {
    pub fn new() -> Snowflake_Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snowflake_Value {
        static mut instance: ::protobuf::lazy::Lazy<Snowflake_Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snowflake_Value,
        };
        unsafe {
            instance.get(Snowflake_Value::new)
        }
    }

    // int64 value = 1;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &i64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl ::protobuf::Message for Snowflake_Value {
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
                    let tmp = is.read_int64()?;
                    self.value = tmp;
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
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(1, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.value != 0 {
            os.write_int64(1, self.value)?;
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

impl ::protobuf::MessageStatic for Snowflake_Value {
    fn new() -> Snowflake_Value {
        Snowflake_Value::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snowflake_Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "value",
                    Snowflake_Value::get_value_for_reflect,
                    Snowflake_Value::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snowflake_Value>(
                    "Snowflake_Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snowflake_Value {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snowflake_Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snowflake_Value {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snowflake_NullRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snowflake_NullRequest {}

impl Snowflake_NullRequest {
    pub fn new() -> Snowflake_NullRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snowflake_NullRequest {
        static mut instance: ::protobuf::lazy::Lazy<Snowflake_NullRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snowflake_NullRequest,
        };
        unsafe {
            instance.get(Snowflake_NullRequest::new)
        }
    }
}

impl ::protobuf::Message for Snowflake_NullRequest {
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

impl ::protobuf::MessageStatic for Snowflake_NullRequest {
    fn new() -> Snowflake_NullRequest {
        Snowflake_NullRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snowflake_NullRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Snowflake_NullRequest>(
                    "Snowflake_NullRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snowflake_NullRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snowflake_NullRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snowflake_NullRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Snowflake_UUID {
    // message fields
    pub uuid: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Snowflake_UUID {}

impl Snowflake_UUID {
    pub fn new() -> Snowflake_UUID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Snowflake_UUID {
        static mut instance: ::protobuf::lazy::Lazy<Snowflake_UUID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Snowflake_UUID,
        };
        unsafe {
            instance.get(Snowflake_UUID::new)
        }
    }

    // uint64 uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid = 0;
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: u64) {
        self.uuid = v;
    }

    pub fn get_uuid(&self) -> u64 {
        self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &u64 {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut u64 {
        &mut self.uuid
    }
}

impl ::protobuf::Message for Snowflake_UUID {
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
                    let tmp = is.read_uint64()?;
                    self.uuid = tmp;
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
        if self.uuid != 0 {
            my_size += ::protobuf::rt::value_size(1, self.uuid, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.uuid != 0 {
            os.write_uint64(1, self.uuid)?;
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

impl ::protobuf::MessageStatic for Snowflake_UUID {
    fn new() -> Snowflake_UUID {
        Snowflake_UUID::new()
    }

    fn descriptor_static(_: ::std::option::Option<Snowflake_UUID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uuid",
                    Snowflake_UUID::get_uuid_for_reflect,
                    Snowflake_UUID::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Snowflake_UUID>(
                    "Snowflake_UUID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Snowflake_UUID {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Snowflake_UUID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Snowflake_UUID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fsnowflake.proto\x12\x05proto\"p\n\tSnowflake\x1a\x19\n\x03Key\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x1a\x1d\n\x05Value\x12\x14\n\
    \x05value\x18\x01\x20\x01(\x03R\x05value\x1a\r\n\x0bNullRequest\x1a\x1a\
    \n\x04UUID\x12\x12\n\x04uuid\x18\x01\x20\x01(\x04R\x04uuid2\x88\x01\n\
    \x10SnowflakeService\x124\n\x04Next\x12\x14.proto.Snowflake.Key\x1a\x16.\
    proto.Snowflake.Value\x12>\n\x07GetUUID\x12\x1c.proto.Snowflake.NullRequ\
    est\x1a\x15.proto.Snowflake.UUIDJ\xe5\x04\n\x06\x12\x04\0\0\x16\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\r\n*\n\
    \x02\x06\0\x12\x04\x05\0\x08\x01\x1a\x1e\x20snowflake\x20service\x20defi\
    nition\n\n\n\n\x03\x06\0\x01\x12\x03\x05\x08\x18\n$\n\x04\x06\0\x02\0\
    \x12\x03\x06\x08:\"\x17\x20\xe4\xba\xa7\xe7\x94\x9f\xe4\xb8\x8b\xe4\xb8\
    \x80\xe4\xb8\xaa\xe5\xba\x8f\xe5\x8f\xb7\n\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\x06\x0c\x10\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x06\x11\x1e\n\
    \x0c\n\x05\x06\0\x02\0\x03\x12\x03\x06)8\n\x1d\n\x04\x06\0\x02\x01\x12\
    \x03\x07\x08D\"\x10\x20UUID\x20\xe5\x8f\x91\xe7\x94\x9f\xe5\x99\xa8\n\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x07\x0c\x13\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x07\x14)\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x074B\n\
    \n\n\x02\x04\0\x12\x04\n\0\x16\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x11\
    \n\x0c\n\x04\x04\0\x03\0\x12\x04\x0b\x08\r\t\n\x0c\n\x05\x04\0\x03\0\x01\
    \x12\x03\x0b\x10\x13\n\r\n\x06\x04\0\x03\0\x02\0\x12\x03\x0c\x10\x1e\n\
    \x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04\x0c\x10\x0b\x15\n\x0e\n\x07\x04\
    \0\x03\0\x02\0\x05\x12\x03\x0c\x10\x16\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\
    \x12\x03\x0c\x17\x1b\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x0c\x1c\
    \x1d\n\x0c\n\x04\x04\0\x03\x01\x12\x04\x0e\x08\x10\t\n\x0c\n\x05\x04\0\
    \x03\x01\x01\x12\x03\x0e\x10\x15\n\r\n\x06\x04\0\x03\x01\x02\0\x12\x03\
    \x0f\x10\x1e\n\x0f\n\x07\x04\0\x03\x01\x02\0\x04\x12\x04\x0f\x10\x0e\x17\
    \n\x0e\n\x07\x04\0\x03\x01\x02\0\x05\x12\x03\x0f\x10\x15\n\x0e\n\x07\x04\
    \0\x03\x01\x02\0\x01\x12\x03\x0f\x16\x1b\n\x0e\n\x07\x04\0\x03\x01\x02\0\
    \x03\x12\x03\x0f\x1c\x1d\n\x0c\n\x04\x04\0\x03\x02\x12\x04\x11\x08\x12\t\
    \n\x0c\n\x05\x04\0\x03\x02\x01\x12\x03\x11\x10\x1b\n\x0c\n\x04\x04\0\x03\
    \x03\x12\x04\x13\x08\x15\t\n\x0c\n\x05\x04\0\x03\x03\x01\x12\x03\x13\x10\
    \x14\n\r\n\x06\x04\0\x03\x03\x02\0\x12\x03\x14\x10\x1f\n\x0f\n\x07\x04\0\
    \x03\x03\x02\0\x04\x12\x04\x14\x10\x13\x16\n\x0e\n\x07\x04\0\x03\x03\x02\
    \0\x05\x12\x03\x14\x10\x16\n\x0e\n\x07\x04\0\x03\x03\x02\0\x01\x12\x03\
    \x14\x17\x1b\n\x0e\n\x07\x04\0\x03\x03\x02\0\x03\x12\x03\x14\x1d\x1eb\
    \x06proto3\
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
