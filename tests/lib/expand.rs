#![feature(prelude_import)]
#![allow(special_module_name)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod lib {
    mod address {
        pub struct Address {
            city: String,
            street: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Address {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Address",
                    "city",
                    &self.city,
                    "street",
                    &&self.street,
                )
            }
        }
        const _: () = {
            #[automatically_derived]
            #[allow(unused_braces)]
            impl schemars::JsonSchema for Address {
                fn schema_name() -> schemars::_private::alloc::borrow::Cow<
                    'static,
                    str,
                > {
                    schemars::_private::alloc::borrow::Cow::Borrowed("Address")
                }
                fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                    schemars::_private::alloc::borrow::Cow::Borrowed(
                        "main::lib::address::Address",
                    )
                }
                fn json_schema(
                    generator: &mut schemars::SchemaGenerator,
                ) -> schemars::Schema {
                    {
                        let mut schema = ::schemars::Schema::try_from(
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("type").into(),
                                            ::serde_json::to_value(&"object").unwrap(),
                                        );
                                    object
                                }),
                            )
                            .unwrap();
                        {
                            schemars::_private::insert_object_property(
                                &mut schema,
                                "city",
                                if generator.contract().is_serialize() {
                                    false
                                } else {
                                    false
                                        || (!false
                                            && <String as schemars::JsonSchema>::_schemars_private_is_option())
                                },
                                { generator.subschema_for::<String>() },
                            );
                        }
                        {
                            schemars::_private::insert_object_property(
                                &mut schema,
                                "street",
                                if generator.contract().is_serialize() {
                                    false
                                } else {
                                    false
                                        || (!false
                                            && <String as schemars::JsonSchema>::_schemars_private_is_option())
                                },
                                { generator.subschema_for::<String>() },
                            );
                        }
                        schema
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Address {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Address",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "city",
                        &self.city,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "street",
                        &self.street,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Address {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "city" => _serde::__private::Ok(__Field::__field0),
                                "street" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"city" => _serde::__private::Ok(__Field::__field0),
                                b"street" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Address>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Address;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Address",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Address with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Address with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Address {
                                city: __field0,
                                street: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("city"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("street"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("city")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("street")?
                                }
                            };
                            _serde::__private::Ok(Address {
                                city: __field0,
                                street: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["city", "street"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Address",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Address>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    mod client {
        use super::*;
    }
    mod parse_parameters {
        use std::path;
        pub fn parse_parameters(
            path_str: &str,
        ) -> Vec<openapiv3::ReferenceOr<openapiv3::Parameter>> {
            let mut paras = Vec::new();
            let path = path::Path::new(path_str);
            path.components()
                .for_each(|x| {
                    if let std::path::Component::Normal(s) = x {
                        let s = s.to_str().unwrap();
                        if s.starts_with("{") && s.ends_with("}") {
                            let name = &s[1..s.len() - 1];
                            let para = openapiv3::Parameter::Path {
                                parameter_data: openapiv3::ParameterData {
                                    name: name.to_string(),
                                    format: openapiv3::ParameterSchemaOrContent::Schema(
                                        openapiv3::ReferenceOr::Item(openapiv3::Schema {
                                            schema_kind: openapiv3::SchemaKind::Type(
                                                openapiv3::Type::String(openapiv3::StringType {
                                                    ..Default::default()
                                                }),
                                            ),
                                            schema_data: openapiv3::SchemaData {
                                                ..Default::default()
                                            },
                                        }),
                                    ),
                                    required: true,
                                    example: None,
                                    extensions: Default::default(),
                                    examples: Default::default(),
                                    explode: None,
                                    description: None,
                                    deprecated: None,
                                },
                                style: openapiv3::PathStyle::Simple,
                            };
                            paras.push(openapiv3::ReferenceOr::Item(para));
                        }
                    }
                });
            paras
        }
    }
    mod user {
        use super::*;
        pub struct User {
            id: i32,
            name: String,
            address: Address,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for User {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "User",
                    "id",
                    &self.id,
                    "name",
                    &self.name,
                    "address",
                    &&self.address,
                )
            }
        }
        const _: () = {
            #[automatically_derived]
            #[allow(unused_braces)]
            impl schemars::JsonSchema for User {
                fn schema_name() -> schemars::_private::alloc::borrow::Cow<
                    'static,
                    str,
                > {
                    schemars::_private::alloc::borrow::Cow::Borrowed("User")
                }
                fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {
                    schemars::_private::alloc::borrow::Cow::Borrowed(
                        "main::lib::user::User",
                    )
                }
                fn json_schema(
                    generator: &mut schemars::SchemaGenerator,
                ) -> schemars::Schema {
                    {
                        let mut schema = ::schemars::Schema::try_from(
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("type").into(),
                                            ::serde_json::to_value(&"object").unwrap(),
                                        );
                                    object
                                }),
                            )
                            .unwrap();
                        {
                            schemars::_private::insert_object_property(
                                &mut schema,
                                "id",
                                if generator.contract().is_serialize() {
                                    false
                                } else {
                                    false
                                        || (!false
                                            && <i32 as schemars::JsonSchema>::_schemars_private_is_option())
                                },
                                { generator.subschema_for::<i32>() },
                            );
                        }
                        {
                            schemars::_private::insert_object_property(
                                &mut schema,
                                "name",
                                if generator.contract().is_serialize() {
                                    false
                                } else {
                                    false
                                        || (!false
                                            && <String as schemars::JsonSchema>::_schemars_private_is_option())
                                },
                                { generator.subschema_for::<String>() },
                            );
                        }
                        {
                            schemars::_private::insert_object_property(
                                &mut schema,
                                "address",
                                if generator.contract().is_serialize() {
                                    false
                                } else {
                                    false
                                        || (!false
                                            && <Address as schemars::JsonSchema>::_schemars_private_is_option())
                                },
                                { generator.subschema_for::<Address>() },
                            );
                        }
                        schema
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for User {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "User",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "address",
                        &self.address,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for User {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "address" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"address" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<User>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = User;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct User",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct User with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct User with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Address,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct User with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(User {
                                id: __field0,
                                name: __field1,
                                address: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Address> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "address",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Address>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("address")?
                                }
                            };
                            _serde::__private::Ok(User {
                                id: __field0,
                                name: __field1,
                                address: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "name", "address"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "User",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<User>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub use address::*;
    pub use client::*;
    pub use parse_parameters::*;
    pub use user::*;
}
pub use lib::*;
use reqwest::blocking;
pub struct Client {
    client: blocking::Client,
    addr: String,
}
impl Client {
    pub fn new(client: blocking::Client, addr: &str) -> Self {
        Self {
            client,
            addr: ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0}/api/v1", addr));
                res
            }),
        }
    }
    pub fn post_users(&self, body: &User) -> reqwest::Result<blocking::Response> {
        let res = self
            .client
            .post(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("{0}/users", self.addr));
                    res
                }),
            )
            .json(body)
            .send()?;
        if let Err(err) = res.error_for_status_ref() {
            {
                ::std::io::_eprint(format_args!("{0}\n", res.text().unwrap()));
            };
            return Err(err);
        }
        Ok(res)
    }
    pub fn get_users_id(&self, id: &str) -> reqwest::Result<User> {
        let res = self
            .client
            .get(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/users/{1}", self.addr, id),
                    );
                    res
                }),
            )
            .send()?;
        if let Err(err) = res.error_for_status_ref() {
            {
                ::std::io::_eprint(format_args!("{0}\n", res.text().unwrap()));
            };
            return Err(err);
        }
        res.json()
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main"]
pub const main: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/main.rs",
        start_line: 15usize,
        start_col: 4usize,
        end_line: 15usize,
        end_col: 8usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(main())),
};
#[allow(dead_code)]
fn main() {
    let api = Client::new(blocking::Client::new(), "");
    let res = api.get_users_id("1").unwrap();
    let res = api.post_users(&res).unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
