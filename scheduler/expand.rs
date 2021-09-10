#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use tracing::warn;
mod client {
    pub type Pid = u64;
    pub struct ClientToken {
        pub pid: Pid,
        pub name: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ClientToken {
        #[inline]
        fn clone(&self) -> ClientToken {
            match *self {
                ClientToken {
                    pid: ref __self_0_0,
                    name: ref __self_0_1,
                } => ClientToken {
                    pid: ::core::clone::Clone::clone(&(*__self_0_0)),
                    name: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ClientToken {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ClientToken {
                    pid: ref __self_0_0,
                    name: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ClientToken");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "pid",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ClientToken {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ClientToken",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pid",
                    &self.pid,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for ClientToken {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "pid" => _serde::__private::Ok(__Field::__field0),
                            "name" => _serde::__private::Ok(__Field::__field1),
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
                            b"pid" => _serde::__private::Ok(__Field::__field0),
                            b"name" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ClientToken>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ClientToken;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct ClientToken")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Pid>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ClientToken with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ClientToken with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ClientToken {
                            pid: __field0,
                            name: __field1,
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
                        let mut __field0: _serde::__private::Option<Pid> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "pid",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Pid>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("pid") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ClientToken {
                            pid: __field0,
                            name: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["pid", "name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ClientToken",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ClientToken>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for ClientToken {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                ClientToken {
                    pid: ref __self_0_0,
                    name: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for ClientToken {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ClientToken {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Pid>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ClientToken {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ClientToken {
        #[inline]
        fn eq(&self, other: &ClientToken) -> bool {
            match *other {
                ClientToken {
                    pid: ref __self_1_0,
                    name: ref __self_1_1,
                } => match *self {
                    ClientToken {
                        pid: ref __self_0_0,
                        name: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ClientToken) -> bool {
            match *other {
                ClientToken {
                    pid: ref __self_1_0,
                    name: ref __self_1_1,
                } => match *self {
                    ClientToken {
                        pid: ref __self_0_0,
                        name: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl Default for ClientToken {
        fn default() -> Self {
            let pid = palaver::thread::gettid();
            ClientToken {
                pid,
                name: String::new(),
            }
        }
    }
}
mod config {
    use config::{Config, ConfigError, File};
    use serde::{Deserialize, Serialize};
    use std::path::Path;
    use crate::{DeviceId, TaskType};
    /// Define the interval in milliseconds
    /// after which the maintenance thread
    /// will perform a maintenance cycle
    const MAINTENANCE_INTERVAL: u64 = 2000;
    /// define the time in seconds after which
    /// the maintenance thread will close the
    /// scheduler if it has been inactive in the
    /// sense that there are neither pending jobs
    /// nor requests from clients
    const SHUTDOWN_TIMEOUT: u64 = 300;
    const MIN_WAIT_TIME: u64 = 120;
    const DEFAULT_DEADLINE: u64 = 1500;
    const WINNING_POST_DEADLINE: u64 = 15;
    const WINDOW_POST_DEADLINE: u64 = 900;
    const SERVER_ADDRESS: &str = "127.0.0.1:5000";
    pub struct Task {
        #[serde(deserialize_with = "TaskType::deserialize_with")]
        pub task_type: TaskType,
        pub devices: Vec<DeviceId>,
        pub timeout: u64,
        pub deadline: u64,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Task {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Task {
                    task_type: ref __self_0_0,
                    devices: ref __self_0_1,
                    timeout: ref __self_0_2,
                    deadline: ref __self_0_3,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Task");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "task_type",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "devices",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "timeout",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "deadline",
                        &&(*__self_0_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Task {
        #[inline]
        fn clone(&self) -> Task {
            match *self {
                Task {
                    task_type: ref __self_0_0,
                    devices: ref __self_0_1,
                    timeout: ref __self_0_2,
                    deadline: ref __self_0_3,
                } => Task {
                    task_type: ::core::clone::Clone::clone(&(*__self_0_0)),
                    devices: ::core::clone::Clone::clone(&(*__self_0_1)),
                    timeout: ::core::clone::Clone::clone(&(*__self_0_2)),
                    deadline: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Task {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Task {
        #[inline]
        fn eq(&self, other: &Task) -> bool {
            match *other {
                Task {
                    task_type: ref __self_1_0,
                    devices: ref __self_1_1,
                    timeout: ref __self_1_2,
                    deadline: ref __self_1_3,
                } => match *self {
                    Task {
                        task_type: ref __self_0_0,
                        devices: ref __self_0_1,
                        timeout: ref __self_0_2,
                        deadline: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Task) -> bool {
            match *other {
                Task {
                    task_type: ref __self_1_0,
                    devices: ref __self_1_1,
                    timeout: ref __self_1_2,
                    deadline: ref __self_1_3,
                } => match *self {
                    Task {
                        task_type: ref __self_0_0,
                        devices: ref __self_0_1,
                        timeout: ref __self_0_2,
                        deadline: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Task {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
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
                            "task_type" => _serde::__private::Ok(__Field::__field0),
                            "devices" => _serde::__private::Ok(__Field::__field1),
                            "timeout" => _serde::__private::Ok(__Field::__field2),
                            "deadline" => _serde::__private::Ok(__Field::__field3),
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
                            b"task_type" => _serde::__private::Ok(__Field::__field0),
                            b"devices" => _serde::__private::Ok(__Field::__field1),
                            b"timeout" => _serde::__private::Ok(__Field::__field2),
                            b"deadline" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Task>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Task;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Task")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: TaskType,
                                phantom: _serde::__private::PhantomData<Task>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: match TaskType::deserialize_with(__deserializer) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Task with 4 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<DeviceId>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Task with 4 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Task with 4 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Task with 4 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Task {
                            task_type: __field0,
                            devices: __field1,
                            timeout: __field2,
                            deadline: __field3,
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
                        let mut __field0: _serde::__private::Option<TaskType> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<DeviceId>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u64> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "task_type",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: TaskType,
                                            phantom: _serde::__private::PhantomData<Task>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: match TaskType::deserialize_with(
                                                        __deserializer,
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    },
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "devices",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<DeviceId>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timeout",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "deadline",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("task_type"),
                                )
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("devices") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("timeout") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("deadline") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Task {
                            task_type: __field0,
                            devices: __field1,
                            timeout: __field2,
                            deadline: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["task_type", "devices", "timeout", "deadline"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Task",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Task>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Task {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Task",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "task_type",
                    &self.task_type,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "devices",
                    &self.devices,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "timeout",
                    &self.timeout,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "deadline",
                    &self.deadline,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Task {
        pub fn task_type(&self) -> TaskType {
            self.task_type
        }
        pub fn devices(&self) -> Vec<DeviceId> {
            self.devices.clone()
        }
    }
    pub struct Service {
        pub address: String,
        /// interval in milliseconds. if present in the configuration file, creates a thread that performs some maintenance
        /// operations such as removing tasks that no longer exist in the system or automatic shutdown
        /// if there are not more tasks or requests.
        pub maintenance_interval: Option<u64>,
        /// Time in seconds until the service should close itself if there are not more clients or
        /// requests. This is done only if the [Service::maintenance_interval] setting is set in the
        /// configuration.
        pub shutdown_timeout: Option<u64>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Service {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Service {
                    address: ref __self_0_0,
                    maintenance_interval: ref __self_0_1,
                    shutdown_timeout: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Service");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "address",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "maintenance_interval",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "shutdown_timeout",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Service {
        #[inline]
        fn clone(&self) -> Service {
            match *self {
                Service {
                    address: ref __self_0_0,
                    maintenance_interval: ref __self_0_1,
                    shutdown_timeout: ref __self_0_2,
                } => Service {
                    address: ::core::clone::Clone::clone(&(*__self_0_0)),
                    maintenance_interval: ::core::clone::Clone::clone(&(*__self_0_1)),
                    shutdown_timeout: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Service {
        #[inline]
        fn default() -> Service {
            Service {
                address: ::core::default::Default::default(),
                maintenance_interval: ::core::default::Default::default(),
                shutdown_timeout: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Service {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Service {
        #[inline]
        fn eq(&self, other: &Service) -> bool {
            match *other {
                Service {
                    address: ref __self_1_0,
                    maintenance_interval: ref __self_1_1,
                    shutdown_timeout: ref __self_1_2,
                } => match *self {
                    Service {
                        address: ref __self_0_0,
                        maintenance_interval: ref __self_0_1,
                        shutdown_timeout: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Service) -> bool {
            match *other {
                Service {
                    address: ref __self_1_0,
                    maintenance_interval: ref __self_1_1,
                    shutdown_timeout: ref __self_1_2,
                } => match *self {
                    Service {
                        address: ref __self_0_0,
                        maintenance_interval: ref __self_0_1,
                        shutdown_timeout: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Service {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "address" => _serde::__private::Ok(__Field::__field0),
                            "maintenance_interval" => _serde::__private::Ok(__Field::__field1),
                            "shutdown_timeout" => _serde::__private::Ok(__Field::__field2),
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
                            b"address" => _serde::__private::Ok(__Field::__field0),
                            b"maintenance_interval" => _serde::__private::Ok(__Field::__field1),
                            b"shutdown_timeout" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Service>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Service;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Service")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Service with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Option<u64>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Service with 3 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<Option<u64>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Service with 3 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Service {
                            address: __field0,
                            maintenance_interval: __field1,
                            shutdown_timeout: __field2,
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
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<u64>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<u64>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "maintenance_interval",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<u64>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "shutdown_timeout",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<u64>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("address") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("maintenance_interval") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("shutdown_timeout") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Service {
                            address: __field0,
                            maintenance_interval: __field1,
                            shutdown_timeout: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["address", "maintenance_interval", "shutdown_timeout"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Service",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Service>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Service {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Service",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "maintenance_interval",
                    &self.maintenance_interval,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "shutdown_timeout",
                    &self.shutdown_timeout,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct TimeSettings {
        /// time in seconds after which a task is considered stalled
        pub min_wait_time: u64,
        /// time in seconds after which a task that is stalling would be removed
        /// this setting just remove the job from the scheduler internal state,
        /// there is not any warranty on the state of the resources the task was using.
        /// this is undefined behavior and is not enable by default.
        pub max_wait_time: Option<u64>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TimeSettings {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TimeSettings {
                    min_wait_time: ref __self_0_0,
                    max_wait_time: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TimeSettings");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "min_wait_time",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "max_wait_time",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TimeSettings {
        #[inline]
        fn clone(&self) -> TimeSettings {
            match *self {
                TimeSettings {
                    min_wait_time: ref __self_0_0,
                    max_wait_time: ref __self_0_1,
                } => TimeSettings {
                    min_wait_time: ::core::clone::Clone::clone(&(*__self_0_0)),
                    max_wait_time: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for TimeSettings {
        #[inline]
        fn default() -> TimeSettings {
            TimeSettings {
                min_wait_time: ::core::default::Default::default(),
                max_wait_time: ::core::default::Default::default(),
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for TimeSettings {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TimeSettings {
        #[inline]
        fn eq(&self, other: &TimeSettings) -> bool {
            match *other {
                TimeSettings {
                    min_wait_time: ref __self_1_0,
                    max_wait_time: ref __self_1_1,
                } => match *self {
                    TimeSettings {
                        min_wait_time: ref __self_0_0,
                        max_wait_time: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TimeSettings) -> bool {
            match *other {
                TimeSettings {
                    min_wait_time: ref __self_1_0,
                    max_wait_time: ref __self_1_1,
                } => match *self {
                    TimeSettings {
                        min_wait_time: ref __self_0_0,
                        max_wait_time: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TimeSettings {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "min_wait_time" => _serde::__private::Ok(__Field::__field0),
                            "max_wait_time" => _serde::__private::Ok(__Field::__field1),
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
                            b"min_wait_time" => _serde::__private::Ok(__Field::__field0),
                            b"max_wait_time" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TimeSettings>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TimeSettings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct TimeSettings")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TimeSettings with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Option<u64>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TimeSettings with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(TimeSettings {
                            min_wait_time: __field0,
                            max_wait_time: __field1,
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
                        let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<u64>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "min_wait_time",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "max_wait_time",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<u64>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("min_wait_time") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("max_wait_time") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(TimeSettings {
                            min_wait_time: __field0,
                            max_wait_time: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["min_wait_time", "max_wait_time"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TimeSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TimeSettings>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TimeSettings {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TimeSettings",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "min_wait_time",
                    &self.min_wait_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "max_wait_time",
                    &self.max_wait_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Settings {
        pub tasks_settings: Vec<Task>,
        pub service: Service,
        pub time_settings: TimeSettings,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Settings {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Settings {
                    tasks_settings: ref __self_0_0,
                    service: ref __self_0_1,
                    time_settings: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Settings");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tasks_settings",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "service",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "time_settings",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Settings {
        #[inline]
        fn clone(&self) -> Settings {
            match *self {
                Settings {
                    tasks_settings: ref __self_0_0,
                    service: ref __self_0_1,
                    time_settings: ref __self_0_2,
                } => Settings {
                    tasks_settings: ::core::clone::Clone::clone(&(*__self_0_0)),
                    service: ::core::clone::Clone::clone(&(*__self_0_1)),
                    time_settings: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Settings {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Settings {
        #[inline]
        fn eq(&self, other: &Settings) -> bool {
            match *other {
                Settings {
                    tasks_settings: ref __self_1_0,
                    service: ref __self_1_1,
                    time_settings: ref __self_1_2,
                } => match *self {
                    Settings {
                        tasks_settings: ref __self_0_0,
                        service: ref __self_0_1,
                        time_settings: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Settings) -> bool {
            match *other {
                Settings {
                    tasks_settings: ref __self_1_0,
                    service: ref __self_1_1,
                    time_settings: ref __self_1_2,
                } => match *self {
                    Settings {
                        tasks_settings: ref __self_0_0,
                        service: ref __self_0_1,
                        time_settings: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Settings {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "tasks_settings" => _serde::__private::Ok(__Field::__field0),
                            "service" => _serde::__private::Ok(__Field::__field1),
                            "time_settings" => _serde::__private::Ok(__Field::__field2),
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
                            b"tasks_settings" => _serde::__private::Ok(__Field::__field0),
                            b"service" => _serde::__private::Ok(__Field::__field1),
                            b"time_settings" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Settings>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Settings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Settings")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<Task>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Settings with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Service>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Settings with 3 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<TimeSettings>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Settings with 3 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Settings {
                            tasks_settings: __field0,
                            service: __field1,
                            time_settings: __field2,
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
                        let mut __field0: _serde::__private::Option<Vec<Task>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Service> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<TimeSettings> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "tasks_settings",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Task>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "service",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Service>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "time_settings",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<TimeSettings>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("tasks_settings") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("service") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("time_settings") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Settings {
                            tasks_settings: __field0,
                            service: __field1,
                            time_settings: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["tasks_settings", "service", "time_settings"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Settings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Settings>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Settings {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Settings",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "tasks_settings",
                    &self.tasks_settings,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "service",
                    &self.service,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time_settings",
                    &self.time_settings,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Default for Settings {
        fn default() -> Self {
            let service = Service {
                address: SERVER_ADDRESS.to_string(),
                maintenance_interval: Some(MAINTENANCE_INTERVAL),
                shutdown_timeout: Some(SHUTDOWN_TIMEOUT),
            };
            let time_settings = TimeSettings {
                min_wait_time: MIN_WAIT_TIME,
                max_wait_time: None,
            };
            let all_devices = crate::list_devices()
                .gpu_devices()
                .iter()
                .map(|d| d.device_id())
                .collect::<Vec<_>>();
            let tasks_settings = (0..3)
                .map(|i| {
                    let (task_type, deadline) = match i {
                        1 => (TaskType::WinningPost, WINNING_POST_DEADLINE),
                        2 => (TaskType::WindowPost, WINDOW_POST_DEADLINE),
                        _ => (TaskType::MerkleTree, DEFAULT_DEADLINE),
                    };
                    let mut task = Task {
                        task_type,
                        devices: all_devices.clone(),
                        timeout: deadline,
                        deadline,
                    };
                    if task.task_type == TaskType::WinningPost && false {
                        task.devices = [all_devices[2].clone()].to_vec();
                    }
                    task
                })
                .collect::<Vec<_>>();
            Settings {
                tasks_settings,
                service,
                time_settings,
            }
        }
    }
    impl Settings {
        pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, config::ConfigError> {
            if path.as_ref().exists() {
                let mut s = Config::new();
                s.merge(File::with_name(path.as_ref().to_str().ok_or_else(
                    || ConfigError::Message("Invalid config path".to_string()),
                )?))?;
                s.try_into()
            } else {
                let s = Self::default();
                let toml = toml::to_string(&s).map_err(|e| {
                    ConfigError::Message({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Error generating toml file: "],
                            &match (&e.to_string(),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })
                })?;
                std::fs::write(&path, &toml).map_err(|e| {
                    ConfigError::Message({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["Can not create default configuration file "],
                            &match (&e.to_string(),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    })
                })?;
                Ok(s)
            }
        }
    }
}
mod db {
    use crate::{Error, Result};
    use serde::{de::DeserializeOwned, Serialize};
    use sled::{Config, Db};
    use std::iter::DoubleEndedIterator;
    use std::iter::Iterator;
    use std::ops::RangeBounds;
    use std::path::Path;
    pub struct Database {
        db: Db,
    }
    impl Database {
        pub fn open<P: AsRef<Path>>(path: P, temporary: bool) -> Result<Self> {
            if !path.as_ref().exists() {
                std::fs::create_dir_all(&path).map_err(|e| {
                    Error::Database({
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["cannot create database in ", " err: "],
                            &match (&path.as_ref(), &e.to_string()) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    })
                })?;
            }
            let config = Config::default()
                .path(path)
                .temporary(temporary)
                .print_profile_on_drop(true)
                .flush_every_ms(Some(1000));
            let db = config.open().map_err(Error::from)?;
            {
                if match ::tracing::Level::DEBUG {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target("scheduler::db")
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    logger.log(
                                        &log::Record::builder()
                                            .file(Some("scheduler/src/db.rs"))
                                            .module_path(Some("scheduler::db"))
                                            .line(Some(30u32))
                                            .metadata(log_meta)
                                            .args({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display};
                                                ::core::fmt::Arguments::new_v1(
                                                    &["", " "],
                                                    &match (&::core::fmt::Arguments::new_v1(
                                                        &["db size ", " - was recovered "],
                                                        &match (
                                                            &db.size_on_disk(),
                                                            &db.was_recovered(),
                                                        ) {
                                                            (arg0, arg1) => [
                                                                ::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Debug::fmt,
                                                                ),
                                                                ::core::fmt::ArgumentV1::new(
                                                                    arg1,
                                                                    ::core::fmt::Debug::fmt,
                                                                ),
                                                            ],
                                                        },
                                                    ),)
                                                    {
                                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                                            arg0,
                                                            ::core::fmt::Display::fmt,
                                                        )],
                                                    },
                                                )
                                            })
                                            .build(),
                                    );
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
                if ::tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::DEBUG <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event scheduler/src/db.rs:30",
                                "scheduler::db",
                                ::tracing::Level::DEBUG,
                                Some("scheduler/src/db.rs"),
                                Some(30u32),
                                Some("scheduler::db"),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        MacroCallsite::new(&META)
                    };
                    let interest = CALLSITE.interest();
                    if !interest.is_never() && CALLSITE.is_enabled(interest) {
                        let meta = CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields().value_set(&[(
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&::core::fmt::Arguments::new_v1(
                                    &["db size ", " - was recovered "],
                                    &match (&db.size_on_disk(), &db.was_recovered()) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ) as &Value),
                            )])
                        });
                    }
                }
            };
            Ok(Self { db })
        }
        pub fn insert<K, V>(&self, key: K, value: V) -> Result<()>
        where
            K: Serialize + DeserializeOwned,
            V: Serialize + DeserializeOwned,
        {
            let key = bincode::serialize(&key).map_err(|e| Error::Other(e.to_string()))?;
            let value_bytes =
                bincode::serialize(&value).map_err(|e| Error::Other(e.to_string()))?;
            let _ = self.db.insert(key, value_bytes)?;
            Ok(())
        }
        pub fn remove<K, V>(&self, key: K) -> Result<Option<V>>
        where
            K: Serialize,
            V: DeserializeOwned,
        {
            let key = bincode::serialize(&key).map_err(|e| Error::Other(e.to_string()))?;
            self.db
                .remove(key)
                .map(|res| {
                    res.and_then(|o| {
                        let value: V = bincode::deserialize(o.as_ref()).ok()?;
                        Some(value)
                    })
                })
                .map_err(Error::from)
        }
        pub fn _flush(&self) -> Result<usize> {
            self.db.flush().map_err(Error::from)
        }
        fn range<K, R, V>(
            &self,
            range: R,
        ) -> impl DoubleEndedIterator<Item = Result<Result<(K, V)>>> + Send + Sync
        where
            K: DeserializeOwned,
            V: DeserializeOwned,
            R: RangeBounds<Vec<u8>>,
        {
            self.db.range(range).map(|res| {
                res.map_err(Error::from).map(|(k, v)| {
                    let value =
                        bincode::deserialize(&v).map_err(|e| Error::Other(e.to_string()))?;
                    let key = bincode::deserialize(&k).map_err(|e| Error::Other(e.to_string()))?;
                    Ok((key, value))
                })
            })
        }
        pub fn iter<K, V>(
            &self,
        ) -> impl DoubleEndedIterator<Item = Result<Result<(K, V)>>> + Send + Sync
        where
            K: DeserializeOwned,
            V: DeserializeOwned,
        {
            self.range::<K, _, V>(..)
        }
    }
    impl From<sled::Error> for Error {
        fn from(e: sled::Error) -> Self {
            Self::Database(e.to_string())
        }
    }
}
mod device {
    mod device_id {
        use rust_gpu_tools::opencl::UniqueId;
        use serde::{de::Deserializer, Serializer};
        use serde::{Deserialize, Serialize};
        use std::convert::TryFrom;
        use std::hash::{Hash, Hasher};
        use std::ops::Deref;
        pub struct DeviceId(pub UniqueId);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DeviceId {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DeviceId(ref __self_0_0) => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_tuple(f, "DeviceId");
                        let _ =
                            ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                        ::core::fmt::DebugTuple::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for DeviceId {
            #[inline]
            fn clone(&self) -> DeviceId {
                match *self {
                    DeviceId(ref __self_0_0) => {
                        DeviceId(::core::clone::Clone::clone(&(*__self_0_0)))
                    }
                }
            }
        }
        impl PartialEq for DeviceId {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl Eq for DeviceId {}
        impl Hash for DeviceId {
            fn hash<H: Hasher>(&self, state: &mut H) {
                match self.0 {
                    UniqueId::PciId(id) => id.hash(state),
                    UniqueId::Uuid(uuid) => uuid.hash(state),
                }
            }
        }
        impl Deref for DeviceId {
            type Target = UniqueId;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl TryFrom<&str> for DeviceId {
            type Error = rust_gpu_tools::opencl::GPUError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(Self(UniqueId::try_from(value)?))
            }
        }
        impl DeviceId {
            pub fn serialize_impl<S>(v: &Self, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match v.0 {
                    UniqueId::PciId(id) => s.serialize_str(id.to_string().as_str()),
                    UniqueId::Uuid(uuid) => s.serialize_str(uuid.to_string().as_str()),
                }
            }
            pub fn deserialize_impl<'de, D>(de: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v = String::deserialize(de)?;
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["DATA ", "\n"],
                        &match (&v,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ));
                };
                let inner = UniqueId::try_from(v.as_str())
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["UNIQUE ", "\n"],
                        &match (&inner,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ));
                };
                Ok(DeviceId(inner))
            }
        }
        impl Serialize for DeviceId {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                Self::serialize_impl(self, serializer)
            }
        }
        impl<'de> Deserialize<'de> for DeviceId {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                DeviceId::deserialize_impl(deserializer)
            }
        }
    }
    pub use device_id::DeviceId;
    use rust_gpu_tools::opencl::Device as ClDevice;
    #[cfg(not(dummy_devices))]
    pub struct Device {
        dev: ClDevice,
        pub id: DeviceId,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Device {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Device {
                    dev: ref __self_0_0,
                    id: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Device");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "dev",
                        &&(*__self_0_0),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_1));
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Device {
        #[inline]
        fn clone(&self) -> Device {
            match *self {
                Device {
                    dev: ref __self_0_0,
                    id: ref __self_0_1,
                } => Device {
                    dev: ::core::clone::Clone::clone(&(*__self_0_0)),
                    id: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[cfg(not(dummy_devices))]
    impl Device {
        pub fn device_id(&self) -> DeviceId {
            self.id.clone()
        }
        pub fn name(&self) -> String {
            self.dev.name()
        }
        pub fn memory(&self) -> u64 {
            self.dev.memory()
        }
        pub fn get_inner(&self) -> ClDevice {
            self.dev.clone()
        }
    }
    #[repr(C)]
    pub struct Devices {
        gpu_devices: Vec<Device>,
        num_cpus: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Devices {
        #[inline]
        fn clone(&self) -> Devices {
            match *self {
                Devices {
                    gpu_devices: ref __self_0_0,
                    num_cpus: ref __self_0_1,
                } => Devices {
                    gpu_devices: ::core::clone::Clone::clone(&(*__self_0_0)),
                    num_cpus: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Devices {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Devices {
                    gpu_devices: ref __self_0_0,
                    num_cpus: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Devices");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "gpu_devices",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_cpus",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Devices {
        pub fn gpu_devices(&self) -> &[Device] {
            self.gpu_devices.as_ref()
        }
    }
    /// Returns all the devices on the system
    ///
    /// It includes the GPUs and the number of logical CPUs
    #[cfg(not(dummy_devices))]
    pub fn list_devices() -> Devices {
        let gpu_devices = {
            ClDevice::all()
                .into_iter()
                .map(|dev| {
                    let unique_id = dev.unique_id();
                    (DeviceId(unique_id), dev)
                })
                .map(|(id, dev)| Device {
                    dev: dev.clone(),
                    id,
                })
                .collect::<Vec<Device>>()
        };
        let num_cpus = num_cpus::get();
        Devices {
            gpu_devices,
            num_cpus,
        }
    }
}
mod error {
    use crate::Pid;
    pub enum Error {
        #[error("Invalid address format")]
        InvalidAddress,
        #[error("Connection error `{0}`")]
        ConnectionError(String),
        #[error("Error: `{0}`")]
        Other(String),
        #[error("Resource requirements list is empty")]
        ResourceReqEmpty,
        #[error("Can not read/write scheduler state - try later")]
        RwError,
        #[error("Error creating solver")]
        NoSolver,
        #[error("Error reading configuration file: `{0}`")]
        InvalidConfig(String),
        #[error("Unknown client")]
        UnknownClient,
        #[error("Solver error: `{0}`")]
        SolverOther(String),
        #[error("Job:`{0}` is not stalling")]
        JobNotStalling(Pid),
        #[error("Database error: `{0}`")]
        Database(String),
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for Error {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(
                unused_variables,
                deprecated,
                clippy::nonstandard_macro_braces,
                clippy::used_underscore_binding
            )]
            match self {
                Error::InvalidAddress {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Invalid address format"],
                    &match () {
                        () => [],
                    },
                )),
                Error::ConnectionError(_0) => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Connection error `", "`"],
                        &match (&_0.as_display(),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Error::Other(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error: `", "`"],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                Error::ResourceReqEmpty {} => {
                    __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                        &["Resource requirements list is empty"],
                        &match () {
                            () => [],
                        },
                    ))
                }
                Error::RwError {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Can not read/write scheduler state - try later"],
                    &match () {
                        () => [],
                    },
                )),
                Error::NoSolver {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error creating solver"],
                    &match () {
                        () => [],
                    },
                )),
                Error::InvalidConfig(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Error reading configuration file: `", "`"],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                Error::UnknownClient {} => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Unknown client"],
                    &match () {
                        () => [],
                    },
                )),
                Error::SolverOther(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Solver error: `", "`"],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                Error::JobNotStalling(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Job:`", "` is not stalling"],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                Error::Database(_0) => __formatter.write_fmt(::core::fmt::Arguments::new_v1(
                    &["Database error: `", "`"],
                    &match (&_0.as_display(),) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Error::InvalidAddress,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "InvalidAddress");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::ConnectionError(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ConnectionError");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::Other(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Other");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::ResourceReqEmpty,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ResourceReqEmpty");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::RwError,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "RwError");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::NoSolver,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "NoSolver");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::InvalidConfig(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "InvalidConfig");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::UnknownClient,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "UnknownClient");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::SolverOther(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "SolverOther");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::JobNotStalling(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "JobNotStalling");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Error::Database(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Database");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Error::InvalidAddress => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Error",
                        0u32,
                        "InvalidAddress",
                    ),
                    Error::ConnectionError(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Error",
                            1u32,
                            "ConnectionError",
                            __field0,
                        )
                    }
                    Error::Other(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Error",
                        2u32,
                        "Other",
                        __field0,
                    ),
                    Error::ResourceReqEmpty => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Error",
                        3u32,
                        "ResourceReqEmpty",
                    ),
                    Error::RwError => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Error",
                        4u32,
                        "RwError",
                    ),
                    Error::NoSolver => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Error",
                        5u32,
                        "NoSolver",
                    ),
                    Error::InvalidConfig(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Error",
                            6u32,
                            "InvalidConfig",
                            __field0,
                        )
                    }
                    Error::UnknownClient => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Error",
                        7u32,
                        "UnknownClient",
                    ),
                    Error::SolverOther(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Error",
                            8u32,
                            "SolverOther",
                            __field0,
                        )
                    }
                    Error::JobNotStalling(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Error",
                            9u32,
                            "JobNotStalling",
                            __field0,
                        )
                    }
                    Error::Database(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Error",
                        10u32,
                        "Database",
                        __field0,
                    ),
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
        impl<'de> _serde::Deserialize<'de> for Error {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 11",
                            )),
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
                            "InvalidAddress" => _serde::__private::Ok(__Field::__field0),
                            "ConnectionError" => _serde::__private::Ok(__Field::__field1),
                            "Other" => _serde::__private::Ok(__Field::__field2),
                            "ResourceReqEmpty" => _serde::__private::Ok(__Field::__field3),
                            "RwError" => _serde::__private::Ok(__Field::__field4),
                            "NoSolver" => _serde::__private::Ok(__Field::__field5),
                            "InvalidConfig" => _serde::__private::Ok(__Field::__field6),
                            "UnknownClient" => _serde::__private::Ok(__Field::__field7),
                            "SolverOther" => _serde::__private::Ok(__Field::__field8),
                            "JobNotStalling" => _serde::__private::Ok(__Field::__field9),
                            "Database" => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"InvalidAddress" => _serde::__private::Ok(__Field::__field0),
                            b"ConnectionError" => _serde::__private::Ok(__Field::__field1),
                            b"Other" => _serde::__private::Ok(__Field::__field2),
                            b"ResourceReqEmpty" => _serde::__private::Ok(__Field::__field3),
                            b"RwError" => _serde::__private::Ok(__Field::__field4),
                            b"NoSolver" => _serde::__private::Ok(__Field::__field5),
                            b"InvalidConfig" => _serde::__private::Ok(__Field::__field6),
                            b"UnknownClient" => _serde::__private::Ok(__Field::__field7),
                            b"SolverOther" => _serde::__private::Ok(__Field::__field8),
                            b"JobNotStalling" => _serde::__private::Ok(__Field::__field9),
                            b"Database" => _serde::__private::Ok(__Field::__field10),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Error>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Error;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum Error")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Error::InvalidAddress)
                            }
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                Error::ConnectionError,
                            ),
                            (__Field::__field2, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                Error::Other,
                            ),
                            (__Field::__field3, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Error::ResourceReqEmpty)
                            }
                            (__Field::__field4, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Error::RwError)
                            }
                            (__Field::__field5, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Error::NoSolver)
                            }
                            (__Field::__field6, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                Error::InvalidConfig,
                            ),
                            (__Field::__field7, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(Error::UnknownClient)
                            }
                            (__Field::__field8, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                Error::SolverOther,
                            ),
                            (__Field::__field9, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Pid>(__variant),
                                Error::JobNotStalling,
                            ),
                            (__Field::__field10, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<String>(__variant),
                                Error::Database,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "InvalidAddress",
                    "ConnectionError",
                    "Other",
                    "ResourceReqEmpty",
                    "RwError",
                    "NoSolver",
                    "InvalidConfig",
                    "UnknownClient",
                    "SolverOther",
                    "JobNotStalling",
                    "Database",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Error",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Error>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
}
mod handler {
    use crate::requests::SchedulerRequest;
    pub trait Handler: Send + Sync + 'static {
        fn process_request(&self, request: SchedulerRequest);
        fn maintenance(&self) -> bool {
            false
        }
    }
}
mod monitor {
    use serde::{Deserialize, Serialize};
    use crate::{Deadline, DeviceId, Pid, ResourceAlloc, TaskType};
    pub struct MonitorInfo {
        pub task_states: Vec<Task>,
        pub resources: Vec<GpuResource>,
        pub job_plan: Vec<Pid>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for MonitorInfo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MonitorInfo {
                    task_states: ref __self_0_0,
                    resources: ref __self_0_1,
                    job_plan: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "MonitorInfo");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "task_states",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "resources",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "job_plan",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for MonitorInfo {
        #[inline]
        fn default() -> MonitorInfo {
            MonitorInfo {
                task_states: ::core::default::Default::default(),
                resources: ::core::default::Default::default(),
                job_plan: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MonitorInfo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "MonitorInfo",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "task_states",
                    &self.task_states,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "resources",
                    &self.resources,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "job_plan",
                    &self.job_plan,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for MonitorInfo {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "task_states" => _serde::__private::Ok(__Field::__field0),
                            "resources" => _serde::__private::Ok(__Field::__field1),
                            "job_plan" => _serde::__private::Ok(__Field::__field2),
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
                            b"task_states" => _serde::__private::Ok(__Field::__field0),
                            b"resources" => _serde::__private::Ok(__Field::__field1),
                            b"job_plan" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MonitorInfo>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MonitorInfo;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct MonitorInfo")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<Task>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct MonitorInfo with 3 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<GpuResource>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct MonitorInfo with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Vec<Pid>>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct MonitorInfo with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(MonitorInfo {
                            task_states: __field0,
                            resources: __field1,
                            job_plan: __field2,
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
                        let mut __field0: _serde::__private::Option<Vec<Task>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<GpuResource>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<Pid>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "task_states",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Task>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resources",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<GpuResource>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "job_plan",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Pid>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("task_states") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("resources") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("job_plan") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(MonitorInfo {
                            task_states: __field0,
                            resources: __field1,
                            job_plan: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["task_states", "resources", "job_plan"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MonitorInfo",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MonitorInfo>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralPartialEq for MonitorInfo {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for MonitorInfo {
        #[inline]
        fn eq(&self, other: &MonitorInfo) -> bool {
            match *other {
                MonitorInfo {
                    task_states: ref __self_1_0,
                    resources: ref __self_1_1,
                    job_plan: ref __self_1_2,
                } => match *self {
                    MonitorInfo {
                        task_states: ref __self_0_0,
                        resources: ref __self_0_1,
                        job_plan: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &MonitorInfo) -> bool {
            match *other {
                MonitorInfo {
                    task_states: ref __self_1_0,
                    resources: ref __self_1_1,
                    job_plan: ref __self_1_2,
                } => match *self {
                    MonitorInfo {
                        task_states: ref __self_0_0,
                        resources: ref __self_0_1,
                        job_plan: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for MonitorInfo {
        #[inline]
        fn clone(&self) -> MonitorInfo {
            match *self {
                MonitorInfo {
                    task_states: ref __self_0_0,
                    resources: ref __self_0_1,
                    job_plan: ref __self_0_2,
                } => MonitorInfo {
                    task_states: ::core::clone::Clone::clone(&(*__self_0_0)),
                    resources: ::core::clone::Clone::clone(&(*__self_0_1)),
                    job_plan: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    pub struct Task {
        pub id: Pid,
        pub alloc: ResourceAlloc,
        pub task_type: Option<TaskType>,
        pub deadline: Option<Deadline>,
        pub last_seen: u64,
        pub stalled: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Task {
        #[inline]
        fn clone(&self) -> Task {
            match *self {
                Task {
                    id: ref __self_0_0,
                    alloc: ref __self_0_1,
                    task_type: ref __self_0_2,
                    deadline: ref __self_0_3,
                    last_seen: ref __self_0_4,
                    stalled: ref __self_0_5,
                } => Task {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    alloc: ::core::clone::Clone::clone(&(*__self_0_1)),
                    task_type: ::core::clone::Clone::clone(&(*__self_0_2)),
                    deadline: ::core::clone::Clone::clone(&(*__self_0_3)),
                    last_seen: ::core::clone::Clone::clone(&(*__self_0_4)),
                    stalled: ::core::clone::Clone::clone(&(*__self_0_5)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Task {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Task {
                    id: ref __self_0_0,
                    alloc: ref __self_0_1,
                    task_type: ref __self_0_2,
                    deadline: ref __self_0_3,
                    last_seen: ref __self_0_4,
                    stalled: ref __self_0_5,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Task");
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "alloc",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "task_type",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "deadline",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "last_seen",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "stalled",
                        &&(*__self_0_5),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Task {
        #[inline]
        fn default() -> Task {
            Task {
                id: ::core::default::Default::default(),
                alloc: ::core::default::Default::default(),
                task_type: ::core::default::Default::default(),
                deadline: ::core::default::Default::default(),
                last_seen: ::core::default::Default::default(),
                stalled: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Task {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Task",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "alloc",
                    &self.alloc,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "task_type",
                    &self.task_type,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "deadline",
                    &self.deadline,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "last_seen",
                    &self.last_seen,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "stalled",
                    &self.stalled,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for Task {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
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
                            "alloc" => _serde::__private::Ok(__Field::__field1),
                            "task_type" => _serde::__private::Ok(__Field::__field2),
                            "deadline" => _serde::__private::Ok(__Field::__field3),
                            "last_seen" => _serde::__private::Ok(__Field::__field4),
                            "stalled" => _serde::__private::Ok(__Field::__field5),
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
                            b"alloc" => _serde::__private::Ok(__Field::__field1),
                            b"task_type" => _serde::__private::Ok(__Field::__field2),
                            b"deadline" => _serde::__private::Ok(__Field::__field3),
                            b"last_seen" => _serde::__private::Ok(__Field::__field4),
                            b"stalled" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Task>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Task;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Task")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Pid>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Task with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            ResourceAlloc,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Task with 6 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<TaskType>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Task with 6 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<Deadline>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Task with 6 elements",
                                ));
                            }
                        };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Task with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Task with 6 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Task {
                            id: __field0,
                            alloc: __field1,
                            task_type: __field2,
                            deadline: __field3,
                            last_seen: __field4,
                            stalled: __field5,
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
                        let mut __field0: _serde::__private::Option<Pid> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ResourceAlloc> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<TaskType>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<Deadline>> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Pid>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "alloc",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ResourceAlloc>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "task_type",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<TaskType>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "deadline",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Deadline>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "stalled",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("alloc") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("task_type") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("deadline") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("last_seen") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("stalled") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Task {
                            id: __field0,
                            alloc: __field1,
                            task_type: __field2,
                            deadline: __field3,
                            last_seen: __field4,
                            stalled: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "alloc",
                    "task_type",
                    "deadline",
                    "last_seen",
                    "stalled",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Task",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Task>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralPartialEq for Task {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Task {
        #[inline]
        fn eq(&self, other: &Task) -> bool {
            match *other {
                Task {
                    id: ref __self_1_0,
                    alloc: ref __self_1_1,
                    task_type: ref __self_1_2,
                    deadline: ref __self_1_3,
                    last_seen: ref __self_1_4,
                    stalled: ref __self_1_5,
                } => match *self {
                    Task {
                        id: ref __self_0_0,
                        alloc: ref __self_0_1,
                        task_type: ref __self_0_2,
                        deadline: ref __self_0_3,
                        last_seen: ref __self_0_4,
                        stalled: ref __self_0_5,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Task) -> bool {
            match *other {
                Task {
                    id: ref __self_1_0,
                    alloc: ref __self_1_1,
                    task_type: ref __self_1_2,
                    deadline: ref __self_1_3,
                    last_seen: ref __self_1_4,
                    stalled: ref __self_1_5,
                } => match *self {
                    Task {
                        id: ref __self_0_0,
                        alloc: ref __self_0_1,
                        task_type: ref __self_0_2,
                        deadline: ref __self_0_3,
                        last_seen: ref __self_0_4,
                        stalled: ref __self_0_5,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                    }
                },
            }
        }
    }
    pub struct GpuResource {
        pub name: String,
        pub device_id: DeviceId,
        pub memory: u64,
        pub mem_usage: u64,
        pub is_busy: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for GpuResource {
        #[inline]
        fn clone(&self) -> GpuResource {
            match *self {
                GpuResource {
                    name: ref __self_0_0,
                    device_id: ref __self_0_1,
                    memory: ref __self_0_2,
                    mem_usage: ref __self_0_3,
                    is_busy: ref __self_0_4,
                } => GpuResource {
                    name: ::core::clone::Clone::clone(&(*__self_0_0)),
                    device_id: ::core::clone::Clone::clone(&(*__self_0_1)),
                    memory: ::core::clone::Clone::clone(&(*__self_0_2)),
                    mem_usage: ::core::clone::Clone::clone(&(*__self_0_3)),
                    is_busy: ::core::clone::Clone::clone(&(*__self_0_4)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for GpuResource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                GpuResource {
                    name: ref __self_0_0,
                    device_id: ref __self_0_1,
                    memory: ref __self_0_2,
                    mem_usage: ref __self_0_3,
                    is_busy: ref __self_0_4,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "GpuResource");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "device_id",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "memory",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mem_usage",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_busy",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for GpuResource {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GpuResource",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "device_id",
                    &self.device_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "memory",
                    &self.memory,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mem_usage",
                    &self.mem_usage,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "is_busy",
                    &self.is_busy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for GpuResource {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
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
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "device_id" => _serde::__private::Ok(__Field::__field1),
                            "memory" => _serde::__private::Ok(__Field::__field2),
                            "mem_usage" => _serde::__private::Ok(__Field::__field3),
                            "is_busy" => _serde::__private::Ok(__Field::__field4),
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
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"device_id" => _serde::__private::Ok(__Field::__field1),
                            b"memory" => _serde::__private::Ok(__Field::__field2),
                            b"mem_usage" => _serde::__private::Ok(__Field::__field3),
                            b"is_busy" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GpuResource>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GpuResource;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GpuResource")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GpuResource with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<DeviceId>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GpuResource with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GpuResource with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct GpuResource with 5 elements",
                                        ),
                                    );
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct GpuResource with 5 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(GpuResource {
                            name: __field0,
                            device_id: __field1,
                            memory: __field2,
                            mem_usage: __field3,
                            is_busy: __field4,
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
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<DeviceId> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "device_id",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<DeviceId>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "memory",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mem_usage",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "is_busy",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("name") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("device_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("memory") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("mem_usage") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("is_busy") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GpuResource {
                            name: __field0,
                            device_id: __field1,
                            memory: __field2,
                            mem_usage: __field3,
                            is_busy: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["name", "device_id", "memory", "mem_usage", "is_busy"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GpuResource",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GpuResource>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralPartialEq for GpuResource {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for GpuResource {
        #[inline]
        fn eq(&self, other: &GpuResource) -> bool {
            match *other {
                GpuResource {
                    name: ref __self_1_0,
                    device_id: ref __self_1_1,
                    memory: ref __self_1_2,
                    mem_usage: ref __self_1_3,
                    is_busy: ref __self_1_4,
                } => match *self {
                    GpuResource {
                        name: ref __self_0_0,
                        device_id: ref __self_0_1,
                        memory: ref __self_0_2,
                        mem_usage: ref __self_0_3,
                        is_busy: ref __self_0_4,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &GpuResource) -> bool {
            match *other {
                GpuResource {
                    name: ref __self_1_0,
                    device_id: ref __self_1_1,
                    memory: ref __self_1_2,
                    mem_usage: ref __self_1_3,
                    is_busy: ref __self_1_4,
                } => match *self {
                    GpuResource {
                        name: ref __self_0_0,
                        device_id: ref __self_0_1,
                        memory: ref __self_0_2,
                        mem_usage: ref __self_0_3,
                        is_busy: ref __self_0_4,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                    }
                },
            }
        }
    }
}
mod requests {
    use futures::channel::oneshot;
    use serde::{Deserialize, Serialize};
    use crate::monitor::MonitorInfo;
    use crate::{ClientToken, DeviceId, Pid, ResourceAlloc, Result, TaskRequirements};
    pub enum RequestMethod {
        Schedule(ClientToken, TaskRequirements, String),
        ListAllocations,
        WaitPreemptive(ClientToken),
        Release(ClientToken),
        ReleasePreemptive(ClientToken),
        Abort(Vec<Pid>),
        RemoveStalled(Vec<Pid>),
        CheckService,
        Monitoring,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RequestMethod {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    RequestMethod::Schedule(ref __field0, ref __field1, ref __field2) => {
                        let mut __serde_state = match _serde::Serializer::serialize_tuple_variant(
                            __serializer,
                            "RequestMethod",
                            0u32,
                            "Schedule",
                            0 + 1 + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(
                            &mut __serde_state,
                            __field0,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(
                            &mut __serde_state,
                            __field1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(
                            &mut __serde_state,
                            __field2,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeTupleVariant::end(__serde_state)
                    }
                    RequestMethod::ListAllocations => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "RequestMethod",
                        1u32,
                        "ListAllocations",
                    ),
                    RequestMethod::WaitPreemptive(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "RequestMethod",
                            2u32,
                            "WaitPreemptive",
                            __field0,
                        )
                    }
                    RequestMethod::Release(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "RequestMethod",
                            3u32,
                            "Release",
                            __field0,
                        )
                    }
                    RequestMethod::ReleasePreemptive(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "RequestMethod",
                            4u32,
                            "ReleasePreemptive",
                            __field0,
                        )
                    }
                    RequestMethod::Abort(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "RequestMethod",
                            5u32,
                            "Abort",
                            __field0,
                        )
                    }
                    RequestMethod::RemoveStalled(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "RequestMethod",
                            6u32,
                            "RemoveStalled",
                            __field0,
                        )
                    }
                    RequestMethod::CheckService => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "RequestMethod",
                        7u32,
                        "CheckService",
                    ),
                    RequestMethod::Monitoring => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "RequestMethod",
                        8u32,
                        "Monitoring",
                    ),
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
        impl<'de> _serde::Deserialize<'de> for RequestMethod {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 9",
                            )),
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
                            "Schedule" => _serde::__private::Ok(__Field::__field0),
                            "ListAllocations" => _serde::__private::Ok(__Field::__field1),
                            "WaitPreemptive" => _serde::__private::Ok(__Field::__field2),
                            "Release" => _serde::__private::Ok(__Field::__field3),
                            "ReleasePreemptive" => _serde::__private::Ok(__Field::__field4),
                            "Abort" => _serde::__private::Ok(__Field::__field5),
                            "RemoveStalled" => _serde::__private::Ok(__Field::__field6),
                            "CheckService" => _serde::__private::Ok(__Field::__field7),
                            "Monitoring" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"Schedule" => _serde::__private::Ok(__Field::__field0),
                            b"ListAllocations" => _serde::__private::Ok(__Field::__field1),
                            b"WaitPreemptive" => _serde::__private::Ok(__Field::__field2),
                            b"Release" => _serde::__private::Ok(__Field::__field3),
                            b"ReleasePreemptive" => _serde::__private::Ok(__Field::__field4),
                            b"Abort" => _serde::__private::Ok(__Field::__field5),
                            b"RemoveStalled" => _serde::__private::Ok(__Field::__field6),
                            b"CheckService" => _serde::__private::Ok(__Field::__field7),
                            b"Monitoring" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RequestMethod>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RequestMethod;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum RequestMethod")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                struct __Visitor<'de> {
                                    marker: _serde::__private::PhantomData<RequestMethod>,
                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                }
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestMethod;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private::Formatter,
                                    ) -> _serde::__private::fmt::Result
                                    {
                                        _serde::__private::Formatter::write_str(
                                            __formatter,
                                            "tuple variant RequestMethod::Schedule",
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
                                        let __field0 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                ClientToken,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "tuple variant RequestMethod::Schedule with 3 elements")) ;
                                                }
                                            };
                                        let __field1 =
                                            match match _serde::de::SeqAccess::next_element::<
                                                TaskRequirements,
                                            >(
                                                &mut __seq
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "tuple variant RequestMethod::Schedule with 3 elements")) ;
                                                }
                                            };
                                        let __field2 =
                                            match match _serde::de::SeqAccess::next_element::<String>(
                                                &mut __seq,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            } {
                                                _serde::__private::Some(__value) => __value,
                                                _serde::__private::None => {
                                                    return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (2usize , & "tuple variant RequestMethod::Schedule with 3 elements")) ;
                                                }
                                            };
                                        _serde::__private::Ok(RequestMethod::Schedule(
                                            __field0, __field1, __field2,
                                        ))
                                    }
                                }
                                _serde::de::VariantAccess::tuple_variant(
                                    __variant,
                                    3usize,
                                    __Visitor {
                                        marker: _serde::__private::PhantomData::<RequestMethod>,
                                        lifetime: _serde::__private::PhantomData,
                                    },
                                )
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RequestMethod::ListAllocations)
                            }
                            (__Field::__field2, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ClientToken>(
                                    __variant,
                                ),
                                RequestMethod::WaitPreemptive,
                            ),
                            (__Field::__field3, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ClientToken>(
                                    __variant,
                                ),
                                RequestMethod::Release,
                            ),
                            (__Field::__field4, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ClientToken>(
                                    __variant,
                                ),
                                RequestMethod::ReleasePreemptive,
                            ),
                            (__Field::__field5, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Vec<Pid>>(__variant),
                                RequestMethod::Abort,
                            ),
                            (__Field::__field6, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Vec<Pid>>(__variant),
                                RequestMethod::RemoveStalled,
                            ),
                            (__Field::__field7, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RequestMethod::CheckService)
                            }
                            (__Field::__field8, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(RequestMethod::Monitoring)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "Schedule",
                    "ListAllocations",
                    "WaitPreemptive",
                    "Release",
                    "ReleasePreemptive",
                    "Abort",
                    "RemoveStalled",
                    "CheckService",
                    "Monitoring",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "RequestMethod",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RequestMethod>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum PreemptionResponse {
        Execute,
        Wait,
        Abort,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PreemptionResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PreemptionResponse::Execute => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PreemptionResponse",
                        0u32,
                        "Execute",
                    ),
                    PreemptionResponse::Wait => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PreemptionResponse",
                        1u32,
                        "Wait",
                    ),
                    PreemptionResponse::Abort => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PreemptionResponse",
                        2u32,
                        "Abort",
                    ),
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
        impl<'de> _serde::Deserialize<'de> for PreemptionResponse {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
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
                            "Execute" => _serde::__private::Ok(__Field::__field0),
                            "Wait" => _serde::__private::Ok(__Field::__field1),
                            "Abort" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"Execute" => _serde::__private::Ok(__Field::__field0),
                            b"Wait" => _serde::__private::Ok(__Field::__field1),
                            b"Abort" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PreemptionResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PreemptionResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum PreemptionResponse",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(PreemptionResponse::Execute)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(PreemptionResponse::Wait)
                            }
                            (__Field::__field2, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(PreemptionResponse::Abort)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Execute", "Wait", "Abort"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "PreemptionResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PreemptionResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralEq for PreemptionResponse {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for PreemptionResponse {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl ::core::marker::StructuralPartialEq for PreemptionResponse {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for PreemptionResponse {
        #[inline]
        fn eq(&self, other: &PreemptionResponse) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for PreemptionResponse {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&PreemptionResponse::Execute,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Execute");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&PreemptionResponse::Wait,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Wait");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&PreemptionResponse::Abort,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Abort");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    pub enum SchedulerResponse {
        Schedule(Result<Option<ResourceAlloc>>),
        SchedulerWaitPreemptive(Result<PreemptionResponse>),
        ListAllocations(Result<Vec<(DeviceId, u64)>>),
        Release,
        ReleasePreemptive,
        Abort(Result<()>),
        RemoveStalled(Result<()>),
        CheckService(Pid),
        Monitoring(std::result::Result<MonitorInfo, String>),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SchedulerResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    SchedulerResponse::Schedule(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            0u32,
                            "Schedule",
                            __field0,
                        )
                    }
                    SchedulerResponse::SchedulerWaitPreemptive(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            1u32,
                            "SchedulerWaitPreemptive",
                            __field0,
                        )
                    }
                    SchedulerResponse::ListAllocations(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            2u32,
                            "ListAllocations",
                            __field0,
                        )
                    }
                    SchedulerResponse::Release => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "SchedulerResponse",
                        3u32,
                        "Release",
                    ),
                    SchedulerResponse::ReleasePreemptive => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "SchedulerResponse",
                            4u32,
                            "ReleasePreemptive",
                        )
                    }
                    SchedulerResponse::Abort(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            5u32,
                            "Abort",
                            __field0,
                        )
                    }
                    SchedulerResponse::RemoveStalled(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            6u32,
                            "RemoveStalled",
                            __field0,
                        )
                    }
                    SchedulerResponse::CheckService(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            7u32,
                            "CheckService",
                            __field0,
                        )
                    }
                    SchedulerResponse::Monitoring(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "SchedulerResponse",
                            8u32,
                            "Monitoring",
                            __field0,
                        )
                    }
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
        impl<'de> _serde::Deserialize<'de> for SchedulerResponse {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 9",
                            )),
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
                            "Schedule" => _serde::__private::Ok(__Field::__field0),
                            "SchedulerWaitPreemptive" => _serde::__private::Ok(__Field::__field1),
                            "ListAllocations" => _serde::__private::Ok(__Field::__field2),
                            "Release" => _serde::__private::Ok(__Field::__field3),
                            "ReleasePreemptive" => _serde::__private::Ok(__Field::__field4),
                            "Abort" => _serde::__private::Ok(__Field::__field5),
                            "RemoveStalled" => _serde::__private::Ok(__Field::__field6),
                            "CheckService" => _serde::__private::Ok(__Field::__field7),
                            "Monitoring" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"Schedule" => _serde::__private::Ok(__Field::__field0),
                            b"SchedulerWaitPreemptive" => _serde::__private::Ok(__Field::__field1),
                            b"ListAllocations" => _serde::__private::Ok(__Field::__field2),
                            b"Release" => _serde::__private::Ok(__Field::__field3),
                            b"ReleasePreemptive" => _serde::__private::Ok(__Field::__field4),
                            b"Abort" => _serde::__private::Ok(__Field::__field5),
                            b"RemoveStalled" => _serde::__private::Ok(__Field::__field6),
                            b"CheckService" => _serde::__private::Ok(__Field::__field7),
                            b"Monitoring" => _serde::__private::Ok(__Field::__field8),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SchedulerResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SchedulerResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum SchedulerResponse",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<Option<ResourceAlloc>>,
                                >(__variant),
                                SchedulerResponse::Schedule,
                            ),
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<PreemptionResponse>,
                                >(__variant),
                                SchedulerResponse::SchedulerWaitPreemptive,
                            ),
                            (__Field::__field2, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<Vec<(DeviceId, u64)>>,
                                >(__variant),
                                SchedulerResponse::ListAllocations,
                            ),
                            (__Field::__field3, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(SchedulerResponse::Release)
                            }
                            (__Field::__field4, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(SchedulerResponse::ReleasePreemptive)
                            }
                            (__Field::__field5, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Result<()>>(__variant),
                                SchedulerResponse::Abort,
                            ),
                            (__Field::__field6, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Result<()>>(__variant),
                                SchedulerResponse::RemoveStalled,
                            ),
                            (__Field::__field7, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Pid>(__variant),
                                SchedulerResponse::CheckService,
                            ),
                            (__Field::__field8, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    std::result::Result<MonitorInfo, String>,
                                >(__variant),
                                SchedulerResponse::Monitoring,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &[
                    "Schedule",
                    "SchedulerWaitPreemptive",
                    "ListAllocations",
                    "Release",
                    "ReleasePreemptive",
                    "Abort",
                    "RemoveStalled",
                    "CheckService",
                    "Monitoring",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SchedulerResponse",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SchedulerResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct SchedulerRequest {
        pub sender: oneshot::Sender<SchedulerResponse>,
        pub method: RequestMethod,
    }
}
mod resource {
    use crate::DeviceId;
    use serde::{Deserialize, Serialize};
    pub enum ResourceType {
        Cpu,
        Gpu(ResourceMemory),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ResourceType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ResourceType {
        #[inline]
        fn clone(&self) -> ResourceType {
            {
                let _: ::core::clone::AssertParamIsClone<ResourceMemory>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ResourceType::Cpu,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Cpu");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ResourceType::Gpu(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Gpu");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ResourceType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ResourceType {
        #[inline]
        fn eq(&self, other: &ResourceType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ResourceType::Gpu(ref __self_0), &ResourceType::Gpu(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &ResourceType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ResourceType::Gpu(ref __self_0), &ResourceType::Gpu(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for ResourceType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ResourceType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<ResourceMemory>;
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ResourceType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ResourceType::Cpu => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ResourceType",
                        0u32,
                        "Cpu",
                    ),
                    ResourceType::Gpu(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "ResourceType",
                            1u32,
                            "Gpu",
                            __field0,
                        )
                    }
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
        impl<'de> _serde::Deserialize<'de> for ResourceType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
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
                            "Cpu" => _serde::__private::Ok(__Field::__field0),
                            "Gpu" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"Cpu" => _serde::__private::Ok(__Field::__field0),
                            b"Gpu" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ResourceType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResourceType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum ResourceType")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ResourceType::Cpu)
                            }
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ResourceMemory>(
                                    __variant,
                                ),
                                ResourceType::Gpu,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Cpu", "Gpu"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ResourceType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ResourceType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum ResourceMemory {
        All,
        Mem(u64),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ResourceMemory {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ResourceMemory {
        #[inline]
        fn clone(&self) -> ResourceMemory {
            {
                let _: ::core::clone::AssertParamIsClone<u64>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceMemory {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&ResourceMemory::All,) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "All");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&ResourceMemory::Mem(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Mem");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ResourceMemory {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ResourceMemory {
        #[inline]
        fn eq(&self, other: &ResourceMemory) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &ResourceMemory::Mem(ref __self_0),
                            &ResourceMemory::Mem(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &ResourceMemory) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &ResourceMemory::Mem(ref __self_0),
                            &ResourceMemory::Mem(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else {
                    true
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for ResourceMemory {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ResourceMemory {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<u64>;
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ResourceMemory {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ResourceMemory::All => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "ResourceMemory",
                        0u32,
                        "All",
                    ),
                    ResourceMemory::Mem(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "ResourceMemory",
                            1u32,
                            "Mem",
                            __field0,
                        )
                    }
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
        impl<'de> _serde::Deserialize<'de> for ResourceMemory {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
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
                            "All" => _serde::__private::Ok(__Field::__field0),
                            "Mem" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"All" => _serde::__private::Ok(__Field::__field0),
                            b"Mem" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ResourceMemory>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResourceMemory;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum ResourceMemory")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(ResourceMemory::All)
                            }
                            (__Field::__field1, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<u64>(__variant),
                                ResourceMemory::Mem,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["All", "Mem"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ResourceMemory",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ResourceMemory>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct ResourceReq {
        pub resource: ResourceType,
        pub quantity: usize,
        pub preemptible: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for ResourceReq {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ResourceReq {
        #[inline]
        fn clone(&self) -> ResourceReq {
            {
                let _: ::core::clone::AssertParamIsClone<ResourceType>;
                let _: ::core::clone::AssertParamIsClone<usize>;
                let _: ::core::clone::AssertParamIsClone<bool>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceReq {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResourceReq {
                    resource: ref __self_0_0,
                    quantity: ref __self_0_1,
                    preemptible: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ResourceReq");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "resource",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "quantity",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "preemptible",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ResourceReq {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ResourceReq {
        #[inline]
        fn eq(&self, other: &ResourceReq) -> bool {
            match *other {
                ResourceReq {
                    resource: ref __self_1_0,
                    quantity: ref __self_1_1,
                    preemptible: ref __self_1_2,
                } => match *self {
                    ResourceReq {
                        resource: ref __self_0_0,
                        quantity: ref __self_0_1,
                        preemptible: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ResourceReq) -> bool {
            match *other {
                ResourceReq {
                    resource: ref __self_1_0,
                    quantity: ref __self_1_1,
                    preemptible: ref __self_1_2,
                } => match *self {
                    ResourceReq {
                        resource: ref __self_0_0,
                        quantity: ref __self_0_1,
                        preemptible: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for ResourceReq {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for ResourceReq {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<ResourceType>;
                let _: ::core::cmp::AssertParamIsEq<usize>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ResourceReq {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ResourceReq",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "resource",
                    &self.resource,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "quantity",
                    &self.quantity,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "preemptible",
                    &self.preemptible,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for ResourceReq {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "resource" => _serde::__private::Ok(__Field::__field0),
                            "quantity" => _serde::__private::Ok(__Field::__field1),
                            "preemptible" => _serde::__private::Ok(__Field::__field2),
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
                            b"resource" => _serde::__private::Ok(__Field::__field0),
                            b"quantity" => _serde::__private::Ok(__Field::__field1),
                            b"preemptible" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ResourceReq>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResourceReq;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct ResourceReq")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<ResourceType>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ResourceReq with 3 elements",
                                ));
                            }
                        };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ResourceReq with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ResourceReq with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ResourceReq {
                            resource: __field0,
                            quantity: __field1,
                            preemptible: __field2,
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
                        let mut __field0: _serde::__private::Option<ResourceType> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<usize> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<bool> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "resource",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ResourceType>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "quantity",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<usize>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "preemptible",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<bool>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("resource") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("quantity") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("preemptible") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ResourceReq {
                            resource: __field0,
                            quantity: __field1,
                            preemptible: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["resource", "quantity", "preemptible"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ResourceReq",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ResourceReq>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct ResourceAlloc {
        pub requirement: ResourceReq,
        pub devices: Vec<DeviceId>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ResourceAlloc {
        #[inline]
        fn clone(&self) -> ResourceAlloc {
            match *self {
                ResourceAlloc {
                    requirement: ref __self_0_0,
                    devices: ref __self_0_1,
                } => ResourceAlloc {
                    requirement: ::core::clone::Clone::clone(&(*__self_0_0)),
                    devices: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceAlloc {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResourceAlloc {
                    requirement: ref __self_0_0,
                    devices: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ResourceAlloc");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "requirement",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "devices",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for ResourceAlloc {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for ResourceAlloc {
        #[inline]
        fn eq(&self, other: &ResourceAlloc) -> bool {
            match *other {
                ResourceAlloc {
                    requirement: ref __self_1_0,
                    devices: ref __self_1_1,
                } => match *self {
                    ResourceAlloc {
                        requirement: ref __self_0_0,
                        devices: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ResourceAlloc) -> bool {
            match *other {
                ResourceAlloc {
                    requirement: ref __self_1_0,
                    devices: ref __self_1_1,
                } => match *self {
                    ResourceAlloc {
                        requirement: ref __self_0_0,
                        devices: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ResourceAlloc {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "ResourceAlloc",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "requirement",
                    &self.requirement,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "devices",
                    &self.devices,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for ResourceAlloc {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "requirement" => _serde::__private::Ok(__Field::__field0),
                            "devices" => _serde::__private::Ok(__Field::__field1),
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
                            b"requirement" => _serde::__private::Ok(__Field::__field0),
                            b"devices" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ResourceAlloc>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResourceAlloc;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct ResourceAlloc")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<ResourceReq>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ResourceAlloc with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<DeviceId>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ResourceAlloc with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(ResourceAlloc {
                            requirement: __field0,
                            devices: __field1,
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
                        let mut __field0: _serde::__private::Option<ResourceReq> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<DeviceId>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "requirement",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ResourceReq>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "devices",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<DeviceId>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("requirement") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("devices") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ResourceAlloc {
                            requirement: __field0,
                            devices: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["requirement", "devices"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ResourceAlloc",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ResourceAlloc>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for ResourceAlloc {
        fn default() -> Self {
            Self {
                requirement: ResourceReq {
                    resource: ResourceType::Cpu,
                    quantity: 0,
                    preemptible: false,
                },
                devices: ::alloc::vec::Vec::new(),
            }
        }
    }
}
mod scheduler {
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use sysinfo::{System, SystemExt};
    use chrono::Utc;
    use crossbeam::channel::Sender;
    use parking_lot::{Mutex, RwLock};
    use std::time::Instant;
    use tracing::{debug, error, instrument, warn};
    use crate::config::{Settings, Task};
    use crate::db::Database;
    use crate::handler::Handler;
    use crate::monitor::{GpuResource, MonitorInfo, Task as MonitorTask};
    use crate::requests::{SchedulerRequest, SchedulerResponse};
    use crate::solver::{ResourceState, Resources, TaskState};
    use crate::solvers::create_solver;
    use crate::{
        ClientToken, DeviceId, Devices, Pid, PreemptionResponse, RequestMethod, ResourceAlloc,
        ResourceType, TaskRequirements, TaskType,
    };
    use crate::{Error, Result};
    pub fn match_task_devices(
        task_type: Option<TaskType>,
        scheduler_settings: &[Task],
    ) -> Option<Vec<DeviceId>> {
        let this_task = task_type?;
        for task in scheduler_settings {
            let devices = task.devices();
            if task.task_type() == this_task && !devices.is_empty() {
                return Some(devices);
            }
        }
        None
    }
    /// compute whether a task is considered stalled
    ///
    /// using the value of [Settings::min_wait_time] seconds before now
    ///
    /// if [Settings::max_wait_time] is set, this function will check if the
    /// stalled task should be removed regardless its parent process remains
    /// active in the system.
    pub fn task_is_stalled(
        last_seen: u64,
        _task_type: Option<TaskType>,
        scheduler_settings: &Settings,
    ) -> (bool, bool) {
        let min_wait_time = scheduler_settings.time_settings.min_wait_time;
        let max_wait_time = scheduler_settings.time_settings.max_wait_time;
        let now = Utc::now().timestamp() as u64;
        let is_stalled = now - min_wait_time > last_seen;
        let must_be_removed = max_wait_time
            .map(|max| now - max > last_seen)
            .unwrap_or(false);
        (is_stalled, must_be_removed)
    }
    pub struct Scheduler {
        tasks_state: RwLock<HashMap<Pid, TaskState>>,
        jobs_queue: RwLock<VecDeque<Pid>>,
        db: Database,
        devices: RwLock<Resources>,
        settings: Settings,
        system: Mutex<System>,
        pid: Pid,
        shutdown_tracker: RwLock<Instant>,
        shutdown_tx: Option<Sender<()>>,
    }
    impl Scheduler {
        pub fn new(
            settings: Settings,
            devices: Devices,
            shutdown_tx: Option<Sender<()>>,
            db: Database,
        ) -> Result<Self> {
            let mut devices = devices
                .gpu_devices()
                .iter()
                .map(|dev| {
                    (
                        dev.device_id(),
                        ResourceState {
                            dev: dev.clone(),
                            mem_usage: 0,
                            current_task: None,
                        },
                    )
                })
                .collect::<HashMap<DeviceId, ResourceState>>();
            let system = Mutex::new(System::new());
            let shutdown_tracker = RwLock::new(Instant::now());
            let pid = palaver::thread::gettid();
            let mut tasks_state = HashMap::new();
            let mut jobs_queue = VecDeque::new();
            for res in db.iter::<Pid, TaskState>() {
                let (key, value) = res??;
                value.allocation.devices.iter().for_each(|id| {
                    let _ = devices
                        .get_mut(id)
                        .map(|dev| dev.update_memory_usage(&value.allocation.requirement.resource));
                });
                tasks_state.insert(key, value);
            }
            if !tasks_state.is_empty() {
                let mut solver = create_solver(None);
                jobs_queue = solver.solve_job_schedule(&tasks_state, &settings)?;
            }
            let devices = RwLock::new(Resources(devices));
            Ok(Self {
                tasks_state: RwLock::new(tasks_state),
                jobs_queue: RwLock::new(jobs_queue),
                devices,
                settings,
                db,
                system,
                pid,
                shutdown_tracker,
                shutdown_tx,
            })
        }
        pub fn schedule(
            &self,
            client: ClientToken,
            requirements: TaskRequirements,
            job_context: String,
        ) -> Result<Option<ResourceAlloc>> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "schedule",
                            "scheduler::scheduler",
                            tracing::Level::INFO,
                            Some("scheduler/src/scheduler.rs"),
                            Some(145u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "job_context"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&job_context) as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&job_context) as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                self.log_stalled_jobs();
                if requirements.req.is_empty() {
                    {
                        if match ::tracing::Level::ERROR {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (157u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Schedule request with empty parameters"] , & match () { () => [] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::ERROR <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:157",
                                        "scheduler::scheduler",
                                        ::tracing::Level::ERROR,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(157u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["Schedule request with empty parameters"],
                                            &match () {
                                                () => [],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    return Err(Error::ResourceReqEmpty);
                }
                if self.tasks_state.read().contains_key(&client.pid) {
                    {
                        if match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (164u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Ignoring request - A client with the same id: " , " is already in the queue "] , & match (& client . pid ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:164",
                                        "scheduler::scheduler",
                                        ::tracing::Level::WARN,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(164u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &[
                                                "Ignoring request - A client with the same id: ",
                                                " is already in the queue ",
                                            ],
                                            &match (&client.pid,) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                )],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    return Ok(None);
                }
                let restrictions =
                    match_task_devices(requirements.task_type, &self.settings.tasks_settings);
                let resources = self.devices.read();
                if !resources.has_min_available_memory(&requirements) {
                    return Ok(None);
                }
                let mut solver = create_solver(None);
                let alloc = match solver.allocate_task(
                    &resources,
                    &requirements,
                    &restrictions,
                    &*self.tasks_state.read(),
                ) {
                    Some(res) => res,
                    _ => return Ok(None),
                };
                drop(resources);
                let time: u64 = Utc::now().timestamp() as u64;
                let task_state = TaskState {
                    requirements,
                    allocation: alloc.clone(),
                    last_seen: AtomicU64::new(time),
                    aborted: AtomicBool::new(false),
                    creation_time: time,
                    context: job_context,
                };
                let state_clone = task_state.clone();
                self.tasks_state.write().insert(client.pid, state_clone);
                let new_plan = match solver
                    .solve_job_schedule(&self.tasks_state.read(), &self.settings)
                {
                    Ok(plan) => {
                        {
                            if match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::DEBUG {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target("scheduler::scheduler")
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (211u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["scheduler job_plan "] , & match (& plan ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Debug :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            if ::tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                            {
                                use ::tracing::__macro_support::*;
                                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                    use ::tracing::__macro_support::MacroCallsite;
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event scheduler/src/scheduler.rs:211",
                                            "scheduler::scheduler",
                                            ::tracing::Level::DEBUG,
                                            Some("scheduler/src/scheduler.rs"),
                                            Some(211u32),
                                            Some("scheduler::scheduler"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    MacroCallsite::new(&META)
                                };
                                let interest = CALLSITE.interest();
                                if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = meta.fields().iter();
                                        meta.fields().value_set(&[(
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&::core::fmt::Arguments::new_v1(
                                                &["scheduler job_plan "],
                                                &match (&plan,) {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Debug::fmt,
                                                    )],
                                                },
                                            )
                                                as &Value),
                                        )])
                                    });
                                }
                            }
                        };
                        plan
                    }
                    Err(e) => {
                        {
                            if match ::tracing::Level::ERROR {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target("scheduler::scheduler")
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (215u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Solver error: "] , & match (& e . to_string () ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            if ::tracing::Level::ERROR <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                            {
                                use ::tracing::__macro_support::*;
                                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                    use ::tracing::__macro_support::MacroCallsite;
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event scheduler/src/scheduler.rs:215",
                                            "scheduler::scheduler",
                                            ::tracing::Level::ERROR,
                                            Some("scheduler/src/scheduler.rs"),
                                            Some(215u32),
                                            Some("scheduler::scheduler"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    MacroCallsite::new(&META)
                                };
                                let interest = CALLSITE.interest();
                                if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                    let meta = CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = meta.fields().iter();
                                        meta.fields().value_set(&[(
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&::core::fmt::Arguments::new_v1(
                                                &["Solver error: "],
                                                &match (&e.to_string(),) {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    )],
                                                },
                                            )
                                                as &Value),
                                        )])
                                    });
                                }
                            }
                        };
                        self.tasks_state.write().remove(&client.pid);
                        return Err(Error::SolverOther(e.to_string()));
                    }
                };
                self.db.insert(client.pid, task_state)?;
                let mut resources = self.devices.write();
                alloc.devices.iter().for_each(|id| {
                    let _ = resources
                        .0
                        .get_mut(id)
                        .map(|dev| dev.update_memory_usage(&alloc.requirement.resource));
                });
                *self.jobs_queue.write() = new_plan;
                *self.shutdown_tracker.write() = Instant::now();
                Ok(Some(alloc))
            }
        }
        fn wait_for_busy_resources(&self, client: &ClientToken) -> Result<bool> {
            let state = self.tasks_state.read();
            let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
            let resources = self.devices.read();
            Ok(resources.has_busy_resources(current_task.allocation.devices.as_slice()))
        }
        fn update_last_seen(&self, client: &ClientToken) -> Result<()> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "update_last_seen",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(248u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client.pid as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client.pid as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                let state = self.tasks_state.read();
                let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
                current_task
                    .last_seen
                    .store(Utc::now().timestamp() as u64, Ordering::Relaxed);
                Ok(())
            }
        }
        fn set_resource_as_busy(&self, client: ClientToken) {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "set_resource_as_busy",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(260u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client.pid as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client.pid as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                let state = self.tasks_state.read();
                if let Some(current_task) = state.get(&client.pid) {
                    self.devices
                        .write()
                        .set_busy_resources(&current_task.allocation.devices, client.pid);
                }
            }
        }
        fn check_priority_queue(&self, client: Pid) -> Result<bool> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "check_priority_queue",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(273u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                let queue = self.jobs_queue.read();
                {
                    if match ::tracing::Level::DEBUG {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::DEBUG {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target("scheduler::scheduler")
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        logger.log(
                                            &log::Record::builder()
                                                .file(Some("scheduler/src/scheduler.rs"))
                                                .module_path(Some("scheduler::scheduler"))
                                                .line(Some(276u32))
                                                .metadata(log_meta)
                                                .args({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display};
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["", " "],
                                                        &match (&::core::fmt::Arguments::new_v1(
                                                            &["current job_plan "],
                                                            &match (&*queue,) {
                                                                (arg0,) => {
                                                                    [::core::fmt::ArgumentV1::new(
                                                                        arg0,
                                                                        ::core::fmt::Debug::fmt,
                                                                    )]
                                                                }
                                                            },
                                                        ),)
                                                        {
                                                            (arg0,) => {
                                                                [::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Display::fmt,
                                                                )]
                                                            }
                                                        },
                                                    )
                                                })
                                                .build(),
                                        );
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    if ::tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                    {
                        use ::tracing::__macro_support::*;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event scheduler/src/scheduler.rs:276",
                                    "scheduler::scheduler",
                                    ::tracing::Level::DEBUG,
                                    Some("scheduler/src/scheduler.rs"),
                                    Some(276u32),
                                    Some("scheduler::scheduler"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let interest = CALLSITE.interest();
                        if !interest.is_never() && CALLSITE.is_enabled(interest) {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[(
                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                    Some(&::core::fmt::Arguments::new_v1(
                                        &["current job_plan "],
                                        &match (&*queue,) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Debug::fmt,
                                            )],
                                        },
                                    ) as &Value),
                                )])
                            });
                        }
                    }
                };
                if let Some(job) = queue.front() {
                    if *job == client {
                        Ok(true)
                    } else {
                        let state = self.tasks_state.read();
                        let current_task = state.get(&client).ok_or(Error::UnknownClient)?;
                        let sub_queue = queue
                            .iter()
                            .filter(|id| {
                                if let Some(next_task) = state.get(id) {
                                    current_task.allocation.devices.iter().any(|dev_id| {
                                        next_task.allocation.devices.iter().any(|id| dev_id == id)
                                    })
                                } else {
                                    false
                                }
                            })
                            .collect::<Vec<_>>();
                        if !sub_queue.is_empty() {
                            Ok(client == *sub_queue[0])
                        } else {
                            Err(Error::UnknownClient)
                        }
                    }
                } else {
                    {
                        if match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger.log(
                                                &log::Record::builder()
                                                    .file(Some("scheduler/src/scheduler.rs"))
                                                    .module_path(Some("scheduler::scheduler"))
                                                    .line(Some(307u32))
                                                    .metadata(log_meta)
                                                    .args({
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display};
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["", " "],
                                                            &match (
                                                                &::core::fmt::Arguments::new_v1(
                                                                    &["Queue empty!"],
                                                                    &match () {
                                                                        () => [],
                                                                    },
                                                                ),
                                                            ) {
                                                                (arg0,) => {
                                                                    [::core::fmt::ArgumentV1::new(
                                                                        arg0,
                                                                        ::core::fmt::Display::fmt,
                                                                    )]
                                                                }
                                                            },
                                                        )
                                                    })
                                                    .build(),
                                            );
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:307",
                                        "scheduler::scheduler",
                                        ::tracing::Level::WARN,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(307u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["Queue empty!"],
                                            &match () {
                                                () => [],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    Err(Error::UnknownClient)
                }
            }
        }
        fn abort_client(&self, client: &ClientToken) -> Result<bool> {
            let state = self.tasks_state.read();
            let current_task = state.get(&client.pid).ok_or(Error::UnknownClient)?;
            Ok(current_task.aborted.load(Ordering::Relaxed))
        }
        pub fn wait_preemptive(&self, client: ClientToken) -> Result<PreemptionResponse> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wait_preemptive",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(319u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client.pid as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client.pid as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                if self.abort_client(&client)? {
                    return Ok(PreemptionResponse::Abort);
                }
                self.update_last_seen(&client)?;
                self.log_stalled_jobs();
                if self.wait_for_busy_resources(&client)? {
                    return Ok(PreemptionResponse::Wait);
                }
                if self.check_priority_queue(client.pid)? {
                    self.set_resource_as_busy(client);
                    Ok(PreemptionResponse::Execute)
                } else {
                    Ok(PreemptionResponse::Wait)
                }
            }
        }
        fn list_allocations(&self) -> SchedulerResponse {
            let alloc = self
                .devices
                .read()
                .0
                .iter()
                .filter_map(|(i, device)| {
                    if device.mem_usage() > 0 {
                        Some((i.clone(), device.available_memory()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(DeviceId, u64)>>();
            SchedulerResponse::ListAllocations(Ok(alloc))
        }
        pub fn release_preemptive(&self, client: ClientToken) {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "release_preemptive",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(359u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client.pid as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client.pid as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                let state = self.tasks_state.read();
                if let Some(current_task) = state.get(&client.pid) {
                    self.devices
                        .write()
                        .unset_busy_resources(&current_task.allocation.devices, client.pid);
                    {
                        if match ::tracing::Level::DEBUG {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::DEBUG {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (366u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["marking resource(s) as free "] , & match (& current_task . allocation . devices ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Debug :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:366",
                                        "scheduler::scheduler",
                                        ::tracing::Level::DEBUG,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(366u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["marking resource(s) as free "],
                                            &match (&current_task.allocation.devices,) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Debug::fmt,
                                                )],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                } else {
                    {
                        if match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (371u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Task: " , " is not in the queue - ignoring"] , & match (& client . pid ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:371",
                                        "scheduler::scheduler",
                                        ::tracing::Level::WARN,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(371u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["Task: ", " is not in the queue - ignoring"],
                                            &match (&client.pid,) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                )],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                }
            }
        }
        pub fn release(&self, client: ClientToken) {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "release",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(375u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["client", "pid"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&tracing::field::debug(&client) as &Value),
                            ),
                            (
                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                Some(&client.pid as &Value),
                            ),
                        ])
                    })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = CALLSITE.metadata().fields().iter();
                                    CALLSITE.metadata().fields().value_set(&[
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&tracing::field::debug(&client) as &Value),
                                        ),
                                        (
                                            &iter
                                                .next()
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            Some(&client.pid as &Value),
                                        ),
                                    ])
                                });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                self.remove_job(client.pid)
            }
        }
        fn abort(&self, clients: Vec<Pid>) -> Result<()> {
            for client in clients.iter() {
                let state = self.tasks_state.read();
                let current_task = state.get(client).ok_or(Error::UnknownClient)?;
                {
                    if match ::tracing::Level::WARN {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::WARN {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target("scheduler::scheduler")
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        logger.log(
                                            &log::Record::builder()
                                                .file(Some("scheduler/src/scheduler.rs"))
                                                .module_path(Some("scheduler::scheduler"))
                                                .line(Some(384u32))
                                                .metadata(log_meta)
                                                .args({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display};
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["", " "],
                                                        &match (&::core::fmt::Arguments::new_v1(
                                                            &["aborting client: ", " from: "],
                                                            &match (&client, &current_task.context)
                                                            {
                                                                (arg0, arg1) => [
                                                                    ::core::fmt::ArgumentV1::new(
                                                                        arg0,
                                                                        ::core::fmt::Display::fmt,
                                                                    ),
                                                                    ::core::fmt::ArgumentV1::new(
                                                                        arg1,
                                                                        ::core::fmt::Display::fmt,
                                                                    ),
                                                                ],
                                                            },
                                                        ),)
                                                        {
                                                            (arg0,) => {
                                                                [::core::fmt::ArgumentV1::new(
                                                                    arg0,
                                                                    ::core::fmt::Display::fmt,
                                                                )]
                                                            }
                                                        },
                                                    )
                                                })
                                                .build(),
                                        );
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::WARN
                            <= ::tracing::level_filters::LevelFilter::current()
                    {
                        use ::tracing::__macro_support::*;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event scheduler/src/scheduler.rs:384",
                                    "scheduler::scheduler",
                                    ::tracing::Level::WARN,
                                    Some("scheduler/src/scheduler.rs"),
                                    Some(384u32),
                                    Some("scheduler::scheduler"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let interest = CALLSITE.interest();
                        if !interest.is_never() && CALLSITE.is_enabled(interest) {
                            let meta = CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields().value_set(&[(
                                    &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                    Some(&::core::fmt::Arguments::new_v1(
                                        &["aborting client: ", " from: "],
                                        &match (&client, &current_task.context) {
                                            (arg0, arg1) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        },
                                    ) as &Value),
                                )])
                            });
                        }
                    }
                };
                current_task.aborted.store(true, Ordering::Relaxed);
                self.db.insert(*client, current_task.clone())?;
            }
            Ok(())
        }
        fn check_process_exist(&self, pid: Pid) -> bool {
            let mut s = self.system.lock();
            s.refresh_process(pid as _)
        }
        fn log_stalled_jobs(&self) {
            for (id, remove) in self.get_stalled_jobs().into_iter() {
                if !self.check_process_exist(id) || remove {
                    self.remove_job(id);
                    continue;
                }
                if let Some(task) = self.tasks_state.read().get(&id) {
                    {
                        if match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (410u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Process " , ":" , " is stalling!!"] , & match (& id , & task . context) { (arg0 , arg1) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:410",
                                        "scheduler::scheduler",
                                        ::tracing::Level::WARN,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(410u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["Process ", ":", " is stalling!!"],
                                            &match (&id, &task.context) {
                                                (arg0, arg1) => [
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg1,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                ],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                }
            }
        }
        fn remove_stalled(&self, clients: Vec<Pid>) -> Result<()> {
            let stalled = self.get_stalled_jobs();
            clients
                .into_iter()
                .filter(|to_remove| stalled.iter().any(|stalled_id| stalled_id.0 == *to_remove))
                .for_each(|to_remove| self.remove_job(to_remove));
            Ok(())
        }
        fn get_stalled_jobs(&self) -> Vec<(Pid, bool)> {
            let mut stalled = ::alloc::vec::Vec::new();
            for (job_id, task) in self.tasks_state.read().iter() {
                let (stalls, remove) = task_is_stalled(
                    task.last_seen.load(Ordering::Relaxed),
                    task.requirements.task_type,
                    &self.settings,
                );
                if stalls {
                    stalled.push((*job_id, remove));
                }
            }
            stalled
        }
        fn remove_job(&self, id: Pid) {
            self.jobs_queue.write().retain(|pid| *pid != id);
            if let Some(current_task) = self.tasks_state.write().remove(&id) {
                let mut devices = self.devices.write();
                devices.unset_busy_resources(&current_task.allocation.devices, id);
                if let ResourceType::Gpu(ref m) = current_task.allocation.requirement.resource {
                    devices.free_memory(m, current_task.allocation.devices.as_slice());
                }
                let _ = self.db.remove::<_, TaskState>(id);
            }
            if !self.tasks_state.read().is_empty() {
                *self.shutdown_tracker.write() = Instant::now();
            }
        }
        fn monitor(&self) -> std::result::Result<MonitorInfo, String> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "monitor",
                            "scheduler::scheduler",
                            tracing::Level::TRACE,
                            Some("scheduler/src/scheduler.rs"),
                            Some(460u32),
                            Some("scheduler::scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    if match tracing::Level::TRACE {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(&{ CALLSITE.metadata().fields().value_set(&[]) });
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_attr_guard = __tracing_attr_span.enter();
            {
                let task_states = self.tasks_state.read();
                let resources = self.devices.read();
                let task_states = task_states
                    .iter()
                    .map(|(id, state)| {
                        let last_seen = state.last_seen.load(Ordering::Relaxed);
                        MonitorTask {
                            id: *id,
                            alloc: state.allocation.clone(),
                            task_type: state.requirements.task_type,
                            deadline: state.requirements.deadline,
                            last_seen,
                            stalled: task_is_stalled(
                                last_seen,
                                state.requirements.task_type,
                                &self.settings,
                            )
                            .0,
                        }
                    })
                    .collect::<Vec<_>>();
                let resources = resources
                    .0
                    .iter()
                    .map(|(id, state)| GpuResource {
                        device_id: id.clone(),
                        name: state.dev.name(),
                        memory: state.dev.memory(),
                        mem_usage: state.mem_usage,
                        is_busy: state.is_busy(),
                    })
                    .collect::<Vec<_>>();
                Ok(MonitorInfo {
                    task_states,
                    resources,
                    job_plan: self.jobs_queue.read().iter().copied().collect::<Vec<_>>(),
                })
            }
        }
    }
    impl Handler for Scheduler {
        fn process_request(&self, request: SchedulerRequest) {
            let sender = request.sender;
            let response = match request.method {
                RequestMethod::Schedule(client, req, context) => {
                    SchedulerResponse::Schedule(self.schedule(client, req, context))
                }
                RequestMethod::ListAllocations => self.list_allocations(),
                RequestMethod::WaitPreemptive(client) => {
                    SchedulerResponse::SchedulerWaitPreemptive(self.wait_preemptive(client))
                }
                RequestMethod::Release(client) => {
                    self.release(client);
                    SchedulerResponse::Release
                }
                RequestMethod::ReleasePreemptive(client) => {
                    self.release_preemptive(client);
                    SchedulerResponse::ReleasePreemptive
                }
                RequestMethod::Abort(client_id) => SchedulerResponse::Abort(self.abort(client_id)),
                RequestMethod::RemoveStalled(client_id) => {
                    SchedulerResponse::RemoveStalled(self.remove_stalled(client_id))
                }
                RequestMethod::Monitoring => SchedulerResponse::Monitoring(self.monitor()),
                RequestMethod::CheckService => SchedulerResponse::CheckService(self.pid),
            };
            let _ = sender.send(response);
        }
        fn maintenance(&self) -> bool {
            let mut _continue = true;
            let mut to_remove = ::alloc::vec::Vec::new();
            for id in self.jobs_queue.read().iter() {
                if !self.check_process_exist(*id) {
                    {
                        if match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target("scheduler::scheduler")
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (539u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Removing job " , ". Parent process does not exist"] , & match (& id ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                        {
                            use ::tracing::__macro_support::*;
                            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                                use ::tracing::__macro_support::MacroCallsite;
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event scheduler/src/scheduler.rs:539",
                                        "scheduler::scheduler",
                                        ::tracing::Level::WARN,
                                        Some("scheduler/src/scheduler.rs"),
                                        Some(539u32),
                                        Some("scheduler::scheduler"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                MacroCallsite::new(&META)
                            };
                            let interest = CALLSITE.interest();
                            if !interest.is_never() && CALLSITE.is_enabled(interest) {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields().value_set(&[(
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&::core::fmt::Arguments::new_v1(
                                            &["Removing job ", ". Parent process does not exist"],
                                            &match (&id,) {
                                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                )],
                                            },
                                        ) as &Value),
                                    )])
                                });
                            }
                        }
                    };
                    to_remove.push(*id);
                }
            }
            for id in to_remove.into_iter() {
                self.remove_job(id);
            }
            if let Some(shutdown_timeout) = self.settings.service.shutdown_timeout {
                if self.shutdown_tracker.read().elapsed().as_secs() > shutdown_timeout {
                    let _ = self . shutdown_tx . as_ref () . map (| tx | { { if match :: tracing :: Level :: WARN { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } <= :: tracing :: log :: STATIC_MAX_LEVEL { if ! :: tracing :: dispatcher :: has_been_set () { { use :: tracing :: log ; let level = match :: tracing :: Level :: WARN { :: tracing :: Level :: ERROR => :: tracing :: log :: Level :: Error , :: tracing :: Level :: WARN => :: tracing :: log :: Level :: Warn , :: tracing :: Level :: INFO => :: tracing :: log :: Level :: Info , :: tracing :: Level :: DEBUG => :: tracing :: log :: Level :: Debug , _ => :: tracing :: log :: Level :: Trace , } ; if level <= log :: max_level () { let log_meta = log :: Metadata :: builder () . level (level) . target ("scheduler::scheduler") . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/scheduler.rs")) . module_path (Some ("scheduler::scheduler")) . line (Some (551u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Closing service after " , "s of inactivity"] , & match (& shutdown_timeout ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ; } } } } else { { } } } else { { } } ; if :: tracing :: Level :: WARN <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: WARN <= :: tracing :: level_filters :: LevelFilter :: current () { use :: tracing :: __macro_support :: * ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event scheduler/src/scheduler.rs:551" , "scheduler::scheduler" , :: tracing :: Level :: WARN , Some ("scheduler/src/scheduler.rs") , Some (551u32) , Some ("scheduler::scheduler") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let interest = CALLSITE . interest () ; if ! interest . is_never () && CALLSITE . is_enabled (interest) { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = meta . fields () . iter () ; meta . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["Closing service after " , "s of inactivity"] , & match (& shutdown_timeout ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) as & Value))]) }) ; } } } ; let _ = tx . try_send (()) ; _continue = false ; }) ;
                }
            }
            _continue
        }
    }
}
mod server {
    use std::sync::Arc;
    use futures::channel::oneshot;
    use futures::FutureExt;
    use jsonrpc_derive::rpc;
    use jsonrpc_http_server::jsonrpc_core::{BoxFuture, Result as RpcResult};
    use crate::handler::Handler;
    use crate::monitor::MonitorInfo;
    use crate::requests::{SchedulerRequest, SchedulerResponse};
    use crate::{
        ClientToken, DeviceId, Pid, PreemptionResponse, RequestMethod, ResourceAlloc,
        TaskRequirements,
    };
    use crate::Result;
    use tracing::warn;
    pub type AsyncRpcResult<T> = BoxFuture<RpcResult<Result<T>>>;
    mod rpc_impl_RpcMethods {
        use jsonrpc_core as _jsonrpc_core;
        use super::*;
        /// The generated server module.
        pub mod gen_server {
            use self::_jsonrpc_core::futures as _futures;
            use super::*;
            pub trait RpcMethods: Sized + Send + Sync + 'static {
                fn wait_allocation(
                    &self,
                    params: (ClientToken, TaskRequirements, String),
                ) -> AsyncRpcResult<Option<ResourceAlloc>>;
                fn wait_preemptive(&self, task: ClientToken) -> AsyncRpcResult<PreemptionResponse>;
                fn list_allocations(&self) -> AsyncRpcResult<Vec<(DeviceId, u64)>>;
                fn health_check(&self) -> BoxFuture<RpcResult<Pid>>;
                fn release(&self, client: ClientToken) -> AsyncRpcResult<()>;
                fn release_preemptive(&self, client: ClientToken) -> AsyncRpcResult<()>;
                fn abort(&self, client: Vec<Pid>) -> AsyncRpcResult<()>;
                fn remove_stalled(&self, client: Vec<Pid>) -> AsyncRpcResult<()>;
                fn monitoring(
                    &self,
                ) -> BoxFuture<RpcResult<std::result::Result<MonitorInfo, String>>>;
                /// Create an `IoDelegate`, wiring rpc calls to the trait methods.
                fn to_delegate<M: _jsonrpc_core::Metadata>(
                    self,
                ) -> _jsonrpc_core::IoDelegate<Self, M> {
                    let mut del = _jsonrpc_core::IoDelegate::new(self.into());
                    del.add_method("wait_allocation", move |base, params| {
                        let method = &(Self::wait_allocation
                            as fn(
                                &Self,
                                (ClientToken, TaskRequirements, String),
                            )
                                -> AsyncRpcResult<Option<ResourceAlloc>>);
                        let params = params.parse::<((ClientToken, TaskRequirements, String),)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("wait_preemptive", move |base, params| {
                        let method = &(Self::wait_preemptive
                            as fn(&Self, ClientToken) -> AsyncRpcResult<PreemptionResponse>);
                        let params = params.parse::<(ClientToken,)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("list_allocations", move |base, params| {
                        let method = &(Self::list_allocations
                            as fn(&Self) -> AsyncRpcResult<Vec<(DeviceId, u64)>>);
                        let params = params.expect_no_params();
                        match params {
                            Ok(()) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("service_status", move |base, params| {
                        let method =
                            &(Self::health_check as fn(&Self) -> BoxFuture<RpcResult<Pid>>);
                        let params = params.expect_no_params();
                        match params {
                            Ok(()) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("release", move |base, params| {
                        let method =
                            &(Self::release as fn(&Self, ClientToken) -> AsyncRpcResult<()>);
                        let params = params.parse::<(ClientToken,)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("release_preemptive", move |base, params| {
                        let method = &(Self::release_preemptive
                            as fn(&Self, ClientToken) -> AsyncRpcResult<()>);
                        let params = params.parse::<(ClientToken,)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("abort", move |base, params| {
                        let method = &(Self::abort as fn(&Self, Vec<Pid>) -> AsyncRpcResult<()>);
                        let params = params.parse::<(Vec<Pid>,)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("remove_stalled", move |base, params| {
                        let method =
                            &(Self::remove_stalled as fn(&Self, Vec<Pid>) -> AsyncRpcResult<()>);
                        let params = params.parse::<(Vec<Pid>,)>();
                        match params {
                            Ok((a,)) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base, a))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del.add_method("monitoring", move |base, params| {
                        let method = &(Self::monitoring
                            as fn(
                                &Self,
                            ) -> BoxFuture<
                                RpcResult<std::result::Result<MonitorInfo, String>>,
                            >);
                        let params = params.expect_no_params();
                        match params {
                            Ok(()) => {
                                use self::_futures::{FutureExt, TryFutureExt};
                                let fut =
                                    self::_jsonrpc_core::WrapFuture::into_future((method)(base))
                                        .map_ok(|value| {
                                            _jsonrpc_core::to_value(value)
                                                .expect("Expected always-serializable type; qed")
                                        })
                                        .map_err(Into::into as fn(_) -> _jsonrpc_core::Error);
                                _futures::future::Either::Left(fut)
                            }
                            Err(e) => {
                                _futures::future::Either::Right(_futures::future::ready(Err(e)))
                            }
                        }
                    });
                    del
                }
            }
        }
    }
    pub use self::rpc_impl_RpcMethods::gen_server::RpcMethods;
    pub struct Server<H: Handler>(Arc<H>);
    impl<H> Server<H>
    where
        H: Handler,
    {
        pub fn new(handler: H) -> Self {
            let handler = Arc::new(handler);
            Self(handler)
        }
        pub fn start_maintenance_thread(&self, tick_interval: u64) {
            use crossbeam::channel::{select, tick};
            use std::time::Duration;
            let handler = self.0.clone();
            let ticker = tick(Duration::from_millis(tick_interval));
            std::thread::spawn(move || loop {
                {
                    match ticker {
                        ref _r => {
                            let _r: &::crossbeam_channel::Receiver<_> = _r;
                            let _res = _r.recv();
                            let _ = _res;
                            {
                                {
                                    if !handler.maintenance() {
                                        {
                                            if match ::tracing::Level::WARN {
                                                ::tracing::Level::ERROR => {
                                                    ::tracing::log::Level::Error
                                                }
                                                ::tracing::Level::WARN => {
                                                    ::tracing::log::Level::Warn
                                                }
                                                ::tracing::Level::INFO => {
                                                    ::tracing::log::Level::Info
                                                }
                                                ::tracing::Level::DEBUG => {
                                                    ::tracing::log::Level::Debug
                                                }
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::WARN {
                                                            ::tracing::Level::ERROR => {
                                                                ::tracing::log::Level::Error
                                                            }
                                                            ::tracing::Level::WARN => {
                                                                ::tracing::log::Level::Warn
                                                            }
                                                            ::tracing::Level::INFO => {
                                                                ::tracing::log::Level::Info
                                                            }
                                                            ::tracing::Level::DEBUG => {
                                                                ::tracing::log::Level::Debug
                                                            }
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target("scheduler::server")
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                logger . log (& log :: Record :: builder () . file (Some ("scheduler/src/server.rs")) . module_path (Some ("scheduler::server")) . line (Some (74u32)) . metadata (log_meta) . args ({ # [allow (unused_imports)] use :: tracing :: field :: { debug , display } ; :: core :: fmt :: Arguments :: new_v1 (& ["" , " "] , & match (& :: core :: fmt :: Arguments :: new_v1 (& ["Closing maintenance thread"] , & match () { () => [] , }) ,) { (arg0 ,) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Display :: fmt)] , }) }) . build ()) ;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                            if :: tracing :: Level :: WARN <= :: tracing :: level_filters :: STATIC_MAX_LEVEL && :: tracing :: Level :: WARN <= :: tracing :: level_filters :: LevelFilter :: current () { use :: tracing :: __macro_support :: * ; static CALLSITE : :: tracing :: __macro_support :: MacroCallsite = { use :: tracing :: __macro_support :: MacroCallsite ; static META : :: tracing :: Metadata < 'static > = { :: tracing_core :: metadata :: Metadata :: new ("event scheduler/src/server.rs:74" , "scheduler::server" , :: tracing :: Level :: WARN , Some ("scheduler/src/server.rs") , Some (74u32) , Some ("scheduler::server") , :: tracing_core :: field :: FieldSet :: new (& ["message"] , :: tracing_core :: callsite :: Identifier (& CALLSITE)) , :: tracing :: metadata :: Kind :: EVENT) } ; MacroCallsite :: new (& META) } ; let interest = CALLSITE . interest () ; if ! interest . is_never () && CALLSITE . is_enabled (interest) { let meta = CALLSITE . metadata () ; :: tracing :: Event :: dispatch (meta , & { # [allow (unused_imports)] use :: tracing :: field :: { debug , display , Value } ; let mut iter = meta . fields () . iter () ; meta . fields () . value_set (& [(& iter . next () . expect ("FieldSet corrupted (this is a bug)") , Some (& :: core :: fmt :: Arguments :: new_v1 (& ["Closing maintenance thread"] , & match () { () => [] , }) as & Value))]) }) ; } }
                                        };
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    impl<H: Handler> RpcMethods for Server<H> {
        fn wait_allocation(
            &self,
            params: (ClientToken, TaskRequirements, String),
        ) -> AsyncRpcResult<Option<ResourceAlloc>> {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["WAIT_ALLOCATION ************************\n"],
                    &match () {
                        () => [],
                    },
                ));
            };
            let method = RequestMethod::Schedule(params.0, params.1, params.2);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::Schedule(res)) => Ok(res),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn wait_preemptive(&self, client: ClientToken) -> AsyncRpcResult<PreemptionResponse> {
            let method = RequestMethod::WaitPreemptive(client);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::SchedulerWaitPreemptive(res)) => Ok(res),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn list_allocations(&self) -> AsyncRpcResult<Vec<(DeviceId, u64)>> {
            let method = RequestMethod::ListAllocations;
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::ListAllocations(res)) => Ok(res),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn release(&self, client: ClientToken) -> AsyncRpcResult<()> {
            let method = RequestMethod::Release(client);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::Release) => Ok(Ok(())),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn release_preemptive(&self, client: ClientToken) -> AsyncRpcResult<()> {
            let method = RequestMethod::ReleasePreemptive(client);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::ReleasePreemptive) => Ok(Ok(())),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn abort(&self, client: Vec<Pid>) -> AsyncRpcResult<()> {
            let method = RequestMethod::Abort(client);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::Abort(res)) => Ok(res),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn remove_stalled(&self, client: Vec<Pid>) -> AsyncRpcResult<()> {
            let method = RequestMethod::RemoveStalled(client);
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::RemoveStalled(res)) => Ok(res),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn monitoring(&self) -> BoxFuture<RpcResult<std::result::Result<MonitorInfo, String>>> {
            let method = RequestMethod::Monitoring;
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::Monitoring(info)) => Ok(info),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
        fn health_check(&self) -> BoxFuture<RpcResult<u64>> {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["STATUS ************************\n"],
                    &match () {
                        () => [],
                    },
                ));
            };
            let method = RequestMethod::CheckService;
            let (sender, receiver) = oneshot::channel();
            let request = SchedulerRequest { sender, method };
            self.0.process_request(request);
            Box::pin(receiver.map(|e| match e {
                Ok(SchedulerResponse::CheckService(pid)) => Ok(pid),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }))
        }
    }
}
mod solver {
    use std::collections::{HashMap, VecDeque};
    use serde::{de::Deserializer, Serializer};
    use serde::{Deserialize, Serialize};
    use crate::config::Settings;
    use crate::Error;
    use crate::{
        Device, DeviceId, Pid, ResourceAlloc, ResourceMemory, ResourceReq, ResourceType,
        TaskRequirements,
    };
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    /// Wrapper that add additional information regarding to the Resource
    /// memory and usage.
    pub struct ResourceState {
        /// Index that points to the Device.
        pub dev: Device,
        /// Current memory in use
        pub mem_usage: u64,
        /// The task that is using this resource
        /// None means the resource is free
        pub current_task: Option<Pid>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ResourceState {
        #[inline]
        fn clone(&self) -> ResourceState {
            match *self {
                ResourceState {
                    dev: ref __self_0_0,
                    mem_usage: ref __self_0_1,
                    current_task: ref __self_0_2,
                } => ResourceState {
                    dev: ::core::clone::Clone::clone(&(*__self_0_0)),
                    mem_usage: ::core::clone::Clone::clone(&(*__self_0_1)),
                    current_task: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceState {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResourceState {
                    dev: ref __self_0_0,
                    mem_usage: ref __self_0_1,
                    current_task: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ResourceState");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "dev",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mem_usage",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "current_task",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ResourceState {
        pub fn available_memory(&self) -> u64 {
            self.dev.memory() - self.mem_usage
        }
        pub fn update_memory_usage(&mut self, resource_type: &ResourceType) {
            if let ResourceType::Gpu(mem) = resource_type {
                match mem {
                    ResourceMemory::All => self.mem_usage = self.dev.memory(),
                    ResourceMemory::Mem(value) => self.mem_usage += value,
                }
            }
        }
        pub fn mem_usage(&self) -> u64 {
            self.mem_usage
        }
        pub fn free_memory(&mut self, mem: &ResourceMemory) {
            match mem {
                ResourceMemory::All => self.mem_usage = 0,
                ResourceMemory::Mem(value) => {
                    self.mem_usage -= value;
                }
            }
        }
        pub fn set_as_busy(&mut self, task: Pid) {
            if true {
                if !self.current_task.is_none() {
                    {
                        :: std :: rt :: begin_panic ("Resource already in used -> multiple process trying to use it at the same time")
                    }
                };
            };
            self.current_task.replace(task);
        }
        pub fn set_as_free(&mut self, task: Pid) {
            if Some(task) == self.current_task {
                self.current_task.take();
            }
        }
        pub fn current_task(&self) -> Option<Pid> {
            self.current_task
        }
        pub fn is_busy(&self) -> bool {
            self.current_task.is_some()
        }
    }
    pub struct Resources(pub HashMap<DeviceId, ResourceState>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Resources {
        #[inline]
        fn clone(&self) -> Resources {
            match *self {
                Resources(ref __self_0_0) => Resources(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Resources {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Resources(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Resources");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Resources {
        pub fn available_memory(&self) -> u64 {
            self.0.iter().map(|(_, dev)| dev.available_memory()).sum()
        }
        pub fn get_devices_with_requirements<'r>(
            &'r self,
            requirements: &'r ResourceReq,
        ) -> impl Iterator<Item = DeviceId> + 'r {
            self.0
                .iter()
                .filter_map(move |(sel, dev)| {
                    if let ResourceType::Gpu(mem) = &requirements.resource {
                        match mem {
                            ResourceMemory::All => Some(sel),
                            ResourceMemory::Mem(val) => {
                                if dev.available_memory() >= *val {
                                    Some(sel)
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                })
                .cloned()
        }
        ///Indicates if these resources can accomodate at least 1 of the resource requests
        /// for the given task
        pub fn has_min_available_memory(&self, requirements: &TaskRequirements) -> bool {
            for req in &requirements.req {
                let n_res_with_memory = self.get_devices_with_requirements(req).count();
                if n_res_with_memory >= req.quantity {
                    return true;
                }
            }
            false
        }
        pub fn free_memory(&mut self, mem: &ResourceMemory, devices: &[DeviceId]) {
            for id in devices {
                let _ = self.0.get_mut(id).map(|dev| dev.free_memory(mem));
            }
        }
        pub fn has_busy_resources(&self, devices: &[DeviceId]) -> bool {
            devices
                .iter()
                .any(|id| self.0.get(id).map(|dev| dev.is_busy()).unwrap_or(false))
        }
        pub fn set_busy_resources(&mut self, devices: &[DeviceId], task: Pid) {
            devices.iter().for_each(|id| {
                let _ = self.0.get_mut(id).map(|dev| dev.set_as_busy(task));
            });
        }
        pub fn unset_busy_resources(&mut self, devices: &[DeviceId], task: Pid) {
            devices.iter().for_each(|id| {
                let _ = self.0.get_mut(id).map(|dev| dev.set_as_free(task));
            });
        }
    }
    fn serialize_atomic_u64<S>(v: &AtomicU64, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u64(v.load(Ordering::Relaxed))
    }
    fn deserialize_atomic_u64<'de, D>(de: D) -> Result<AtomicU64, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u64::deserialize(de) {
            Ok(value) => Ok(AtomicU64::new(value)),
            Err(_) => Err(serde::de::Error::custom(
                "error trying to deserialize u64 for task last_seen timestamp",
            )),
        }
    }
    fn serialize_atomic_bool<S>(v: &AtomicBool, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_bool(v.load(Ordering::Relaxed))
    }
    fn deserialize_atomic_bool<'de, D>(de: D) -> Result<AtomicBool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match bool::deserialize(de) {
            Ok(value) => Ok(AtomicBool::new(value)),
            Err(_) => Err(serde::de::Error::custom(
                "error trying to deserialize boolean for task abort flag",
            )),
        }
    }
    pub struct TaskState {
        pub requirements: TaskRequirements,
        pub allocation: ResourceAlloc,
        #[serde(
            deserialize_with = "deserialize_atomic_u64",
            serialize_with = "serialize_atomic_u64"
        )]
        pub last_seen: AtomicU64,
        #[serde(
            deserialize_with = "deserialize_atomic_bool",
            serialize_with = "serialize_atomic_bool"
        )]
        pub aborted: AtomicBool,
        pub creation_time: u64,
        pub context: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TaskState {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TaskState {
                    requirements: ref __self_0_0,
                    allocation: ref __self_0_1,
                    last_seen: ref __self_0_2,
                    aborted: ref __self_0_3,
                    creation_time: ref __self_0_4,
                    context: ref __self_0_5,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TaskState");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "requirements",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "allocation",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "last_seen",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "aborted",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "creation_time",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "context",
                        &&(*__self_0_5),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TaskState {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
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
                            "requirements" => _serde::__private::Ok(__Field::__field0),
                            "allocation" => _serde::__private::Ok(__Field::__field1),
                            "last_seen" => _serde::__private::Ok(__Field::__field2),
                            "aborted" => _serde::__private::Ok(__Field::__field3),
                            "creation_time" => _serde::__private::Ok(__Field::__field4),
                            "context" => _serde::__private::Ok(__Field::__field5),
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
                            b"requirements" => _serde::__private::Ok(__Field::__field0),
                            b"allocation" => _serde::__private::Ok(__Field::__field1),
                            b"last_seen" => _serde::__private::Ok(__Field::__field2),
                            b"aborted" => _serde::__private::Ok(__Field::__field3),
                            b"creation_time" => _serde::__private::Ok(__Field::__field4),
                            b"context" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TaskState>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TaskState;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct TaskState")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            TaskRequirements,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct TaskState with 6 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            ResourceAlloc,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TaskState with 6 elements",
                                ));
                            }
                        };
                        let __field2 = match {
                            struct __DeserializeWith<'de> {
                                value: AtomicU64,
                                phantom: _serde::__private::PhantomData<TaskState>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: match deserialize_atomic_u64(__deserializer) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct TaskState with 6 elements",
                                ));
                            }
                        };
                        let __field3 = match {
                            struct __DeserializeWith<'de> {
                                value: AtomicBool,
                                phantom: _serde::__private::PhantomData<TaskState>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::__private::Ok(__DeserializeWith {
                                        value: match deserialize_atomic_bool(__deserializer) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                        phantom: _serde::__private::PhantomData,
                                        lifetime: _serde::__private::PhantomData,
                                    })
                                }
                            }
                            _serde::__private::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct TaskState with 6 elements",
                                ));
                            }
                        };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct TaskState with 6 elements",
                                        ),
                                    );
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct TaskState with 6 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(TaskState {
                            requirements: __field0,
                            allocation: __field1,
                            last_seen: __field2,
                            aborted: __field3,
                            creation_time: __field4,
                            context: __field5,
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
                        let mut __field0: _serde::__private::Option<TaskRequirements> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<ResourceAlloc> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<AtomicU64> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<AtomicBool> =
                            _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "requirements",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<TaskRequirements>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "allocation",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<ResourceAlloc>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "last_seen",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: AtomicU64,
                                            phantom: _serde::__private::PhantomData<TaskState>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: match deserialize_atomic_u64(
                                                        __deserializer,
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    },
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "aborted",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some({
                                        struct __DeserializeWith<'de> {
                                            value: AtomicBool,
                                            phantom: _serde::__private::PhantomData<TaskState>,
                                            lifetime: _serde::__private::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private::Ok(__DeserializeWith {
                                                    value: match deserialize_atomic_bool(
                                                        __deserializer,
                                                    ) {
                                                        _serde::__private::Ok(__val) => __val,
                                                        _serde::__private::Err(__err) => {
                                                            return _serde::__private::Err(__err);
                                                        }
                                                    },
                                                    phantom: _serde::__private::PhantomData,
                                                    lifetime: _serde::__private::PhantomData,
                                                })
                                            }
                                        }
                                        match _serde::de::MapAccess::next_value::<
                                            __DeserializeWith<'de>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__wrapper) => __wrapper.value,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    });
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "creation_time",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "context",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<String>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("requirements") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("allocation") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("last_seen"),
                                )
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("aborted"),
                                )
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("creation_time") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("context") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(TaskState {
                            requirements: __field0,
                            allocation: __field1,
                            last_seen: __field2,
                            aborted: __field3,
                            creation_time: __field4,
                            context: __field5,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "requirements",
                    "allocation",
                    "last_seen",
                    "aborted",
                    "creation_time",
                    "context",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TaskState",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TaskState>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TaskState {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TaskState",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "requirements",
                    &self.requirements,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "allocation",
                    &self.allocation,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "last_seen",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a AtomicU64,),
                            phantom: _serde::__private::PhantomData<TaskState>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                serialize_atomic_u64(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.last_seen,),
                            phantom: _serde::__private::PhantomData::<TaskState>,
                        }
                    },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "aborted",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a AtomicBool,),
                            phantom: _serde::__private::PhantomData<TaskState>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                serialize_atomic_bool(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.aborted,),
                            phantom: _serde::__private::PhantomData::<TaskState>,
                        }
                    },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "creation_time",
                    &self.creation_time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "context",
                    &self.context,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Clone for TaskState {
        fn clone(&self) -> Self {
            Self {
                requirements: self.requirements.clone(),
                allocation: self.allocation.clone(),
                last_seen: AtomicU64::new(self.last_seen.load(Ordering::Relaxed)),
                aborted: AtomicBool::new(self.aborted.load(Ordering::Relaxed)),
                creation_time: self.creation_time,
                context: self.context.clone(),
            }
        }
    }
    impl TaskState {
        pub fn end_timestamp(&self) -> i64 {
            self.requirements
                .deadline
                .map_or(i64::MAX, |d| d.end_timestamp_secs())
        }
    }
    pub trait Solver {
        fn solve_job_schedule(
            &mut self,
            current_state: &HashMap<Pid, TaskState>,
            scheduler_settings: &Settings,
        ) -> Result<VecDeque<Pid>, Error>;
        fn allocate_task(
            &mut self,
            resources: &Resources,
            requirements: &TaskRequirements,
            restrictions: &Option<Vec<DeviceId>>,
            task_state: &HashMap<Pid, TaskState>,
        ) -> Option<ResourceAlloc>;
    }
}
mod solvers {
    mod greedy {
        use std::collections::{HashMap, VecDeque};
        use crate::{
            config::Settings,
            solver::{Resources, Solver, TaskState},
            Result,
        };
        use crate::{DeviceId, Pid, ResourceAlloc, TaskRequirements};
        use priority_queue::PriorityQueue;
        use std::cmp::Reverse;
        pub struct GreedySolver;
        fn get_by_resource_load(
            resources: &Resources,
            tasks_state: &HashMap<Pid, TaskState>,
        ) -> Vec<DeviceId> {
            let mut map = HashMap::new();
            resources.0.iter().for_each(|(id, _)| {
                map.insert(id, 0usize);
            });
            for (id, counter) in map.iter_mut() {
                if tasks_state
                    .iter()
                    .any(|(_, state)| state.allocation.devices.iter().any(|dev| dev == *id))
                {
                    *counter += 1;
                }
            }
            let mut resource_load_queue = PriorityQueue::new();
            map.into_iter().for_each(|(key, val)| {
                resource_load_queue.push(key, Reverse(val));
            });
            resource_load_queue
                .into_sorted_iter()
                .map(|(i, _)| i.clone())
                .collect::<Vec<_>>()
        }
        impl Solver for GreedySolver {
            fn allocate_task(
                &mut self,
                resources: &Resources,
                requirements: &TaskRequirements,
                restrictions: &Option<Vec<DeviceId>>,
                tasks_state: &HashMap<Pid, TaskState>,
            ) -> Option<ResourceAlloc> {
                let device_restrictions = restrictions
                    .clone()
                    .unwrap_or_else(|| resources.0.keys().cloned().collect::<Vec<DeviceId>>());
                let mut options = ::alloc::vec::Vec::new();
                for req in requirements.req.iter() {
                    let mut quantity = req.quantity;
                    if quantity > device_restrictions.len() {
                        quantity = device_restrictions.len();
                    }
                    let mut optional_resources = resources
                        .get_devices_with_requirements(req)
                        .filter(|b| device_restrictions.iter().any(|x| x == b))
                        .collect::<Vec<DeviceId>>();
                    if optional_resources.len() >= quantity {
                        if resources.0.len() > 1 {
                            let ordered = get_by_resource_load(resources, tasks_state);
                            let filtered = ordered
                                .iter()
                                .filter(|id| {
                                    optional_resources.iter().any(|optional| optional == *id)
                                })
                                .take(quantity)
                                .cloned()
                                .collect::<Vec<_>>();
                            options.push((filtered, req));
                        } else {
                            optional_resources.truncate(quantity);
                            options.push((optional_resources, req));
                        }
                    }
                }
                if !options.is_empty() {
                    let requirement = *options[0].1;
                    let devices = options[0].0.clone();
                    return Some(ResourceAlloc {
                        requirement,
                        devices,
                    });
                }
                None
            }
            fn solve_job_schedule(
                &mut self,
                current_state: &HashMap<Pid, TaskState>,
                settings: &Settings,
            ) -> Result<VecDeque<Pid>> {
                let mut priority_queue = PriorityQueue::new();
                for (job_id, state) in current_state.iter() {
                    let deadline = settings
                        .tasks_settings
                        .iter()
                        .find(|task| Some(task.task_type) == state.requirements.task_type)
                        .map(|task| task.deadline)
                        .unwrap_or_else(|| {
                            state.requirements.deadline.map_or(u64::MAX, |d| {
                                d.as_duration().map(|d| d.as_secs()).unwrap_or(u64::MAX)
                            })
                        });
                    let conditions = (Reverse(deadline), Reverse(state.creation_time));
                    priority_queue.push(job_id, conditions);
                }
                Ok(priority_queue
                    .into_sorted_iter()
                    .map(|(i, _)| *i)
                    .collect::<VecDeque<Pid>>())
            }
        }
    }
    use crate::config::Settings;
    use crate::solver::Solver;
    pub use greedy::GreedySolver;
    pub(crate) fn create_solver(_config: Option<&Settings>) -> Box<dyn Solver> {
        Box::new(GreedySolver)
    }
}
mod task {
    use std::time::Duration;
    use chrono::{offset::Utc, DateTime};
    use serde::{Deserialize, Serialize};
    use super::ResourceReq;
    /// Helper type that indicates if a task should be executed again
    pub enum TaskResult {
        Continue,
        Done,
    }
    impl ::core::marker::StructuralPartialEq for TaskResult {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TaskResult {
        #[inline]
        fn eq(&self, other: &TaskResult) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for TaskResult {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for TaskResult {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    #[non_exhaustive]
    pub enum TaskType {
        MerkleTree,
        WinningPost,
        WindowPost,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TaskType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&TaskType::MerkleTree,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "MerkleTree");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&TaskType::WinningPost,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "WinningPost");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&TaskType::WindowPost,) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "WindowPost");
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for TaskType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TaskType {
        #[inline]
        fn clone(&self) -> TaskType {
            {
                *self
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TaskType {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    TaskType::MerkleTree => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TaskType",
                        0u32,
                        "MerkleTree",
                    ),
                    TaskType::WinningPost => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TaskType",
                        1u32,
                        "WinningPost",
                    ),
                    TaskType::WindowPost => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "TaskType",
                        2u32,
                        "WindowPost",
                    ),
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
        impl<'de> _serde::Deserialize<'de> for TaskType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
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
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
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
                            "MerkleTree" => _serde::__private::Ok(__Field::__field0),
                            "WinningPost" => _serde::__private::Ok(__Field::__field1),
                            "WindowPost" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
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
                            b"MerkleTree" => _serde::__private::Ok(__Field::__field0),
                            b"WinningPost" => _serde::__private::Ok(__Field::__field1),
                            b"WindowPost" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TaskType>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TaskType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum TaskType")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(TaskType::MerkleTree)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(TaskType::WinningPost)
                            }
                            (__Field::__field2, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                                _serde::__private::Ok(TaskType::WindowPost)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] =
                    &["MerkleTree", "WinningPost", "WindowPost"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TaskType",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TaskType>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralPartialEq for TaskType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for TaskType {
        #[inline]
        fn eq(&self, other: &TaskType) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for TaskType {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for TaskType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl TaskType {
        pub fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let mut s = String::deserialize(de)?;
            s.make_ascii_lowercase();
            match s.as_ref() {
                "merkletree" => Ok(TaskType::MerkleTree),
                "winningpost" => Ok(TaskType::WinningPost),
                "windowpost" => Ok(TaskType::WindowPost),
                _ => Err(serde::de::Error::custom(
                    "Trying to deserialize an unsupported task type",
                )),
            }
        }
    }
    impl TaskResult {
        pub fn is_continue(&self) -> bool {
            match self {
                Self::Continue => true,
                _ => false,
            }
        }
    }
    /// Deadline struct to configure when the task should be started and finished
    pub struct Deadline {
        pub start: DateTime<Utc>,
        pub end: DateTime<Utc>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Deadline {
        #[inline]
        fn clone(&self) -> Deadline {
            {
                let _: ::core::clone::AssertParamIsClone<DateTime<Utc>>;
                let _: ::core::clone::AssertParamIsClone<DateTime<Utc>>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Deadline {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Deadline {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Deadline {
                    start: ref __self_0_0,
                    end: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Deadline");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "start",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "end",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Deadline {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Deadline",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "start",
                    &self.start,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "end",
                    &self.end,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for Deadline {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "start" => _serde::__private::Ok(__Field::__field0),
                            "end" => _serde::__private::Ok(__Field::__field1),
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
                            b"start" => _serde::__private::Ok(__Field::__field0),
                            b"end" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Deadline>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Deadline;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Deadline")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Deadline with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            DateTime<Utc>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Deadline with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(Deadline {
                            start: __field0,
                            end: __field1,
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
                        let mut __field0: _serde::__private::Option<DateTime<Utc>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<DateTime<Utc>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "start",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "end",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<DateTime<Utc>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("start") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("end") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(Deadline {
                            start: __field0,
                            end: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["start", "end"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Deadline",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Deadline>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl ::core::marker::StructuralPartialEq for Deadline {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Deadline {
        #[inline]
        fn eq(&self, other: &Deadline) -> bool {
            match *other {
                Deadline {
                    start: ref __self_1_0,
                    end: ref __self_1_1,
                } => match *self {
                    Deadline {
                        start: ref __self_0_0,
                        end: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Deadline) -> bool {
            match *other {
                Deadline {
                    start: ref __self_1_0,
                    end: ref __self_1_1,
                } => match *self {
                    Deadline {
                        start: ref __self_0_0,
                        end: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl Deadline {
        pub fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> Self {
            Self { start, end: finish }
        }
        pub fn from_secs(start: u64, end: u64) -> Self {
            let start = chrono::Utc::now() + chrono::Duration::seconds(start as _);
            let end = start + chrono::Duration::seconds(end as _);
            Self::new(start, end)
        }
        pub fn default_now() -> Self {
            let start = chrono::Utc::now();
            Self::new(start, start)
        }
        pub fn start_timestamp_secs(&self) -> i64 {
            self.start.timestamp()
        }
        pub fn end_timestamp_secs(&self) -> i64 {
            self.end.timestamp()
        }
        pub fn as_duration(&self) -> Option<Duration> {
            let start = self.start_timestamp_secs();
            let end = self.end_timestamp_secs();
            end.checked_sub(start)
                .map(|duration_secs| Duration::from_secs(duration_secs as u64))
        }
    }
    /// Contains all the timing descriptions for
    /// a task. These parameters will be used by the scheduler solve for
    /// scheduling the task in the right time window and resource
    pub struct TaskEstimations {
        pub time_per_iter: Duration,
        pub num_of_iter: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TaskEstimations {
        #[inline]
        fn clone(&self) -> TaskEstimations {
            match *self {
                TaskEstimations {
                    time_per_iter: ref __self_0_0,
                    num_of_iter: ref __self_0_1,
                } => TaskEstimations {
                    time_per_iter: ::core::clone::Clone::clone(&(*__self_0_0)),
                    num_of_iter: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TaskEstimations {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TaskEstimations {
                    time_per_iter: ref __self_0_0,
                    num_of_iter: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TaskEstimations");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "time_per_iter",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "num_of_iter",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TaskEstimations {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TaskEstimations",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time_per_iter",
                    &self.time_per_iter,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "num_of_iter",
                    &self.num_of_iter,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for TaskEstimations {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            "time_per_iter" => _serde::__private::Ok(__Field::__field0),
                            "num_of_iter" => _serde::__private::Ok(__Field::__field1),
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
                            b"time_per_iter" => _serde::__private::Ok(__Field::__field0),
                            b"num_of_iter" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TaskEstimations>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TaskEstimations;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TaskEstimations",
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
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Duration>(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct TaskEstimations with 2 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct TaskEstimations with 2 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(TaskEstimations {
                            time_per_iter: __field0,
                            num_of_iter: __field1,
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
                        let mut __field0: _serde::__private::Option<Duration> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<usize> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "time_per_iter",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Duration>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "num_of_iter",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<usize>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("time_per_iter") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("num_of_iter") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(TaskEstimations {
                            time_per_iter: __field0,
                            num_of_iter: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["time_per_iter", "num_of_iter"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TaskEstimations",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TaskEstimations>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct TaskReqBuilder {
        req: Vec<ResourceReq>,
        deadline: Option<Deadline>,
        task_estimations: Option<TaskEstimations>,
        task_type: Option<TaskType>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for TaskReqBuilder {
        #[inline]
        fn default() -> TaskReqBuilder {
            TaskReqBuilder {
                req: ::core::default::Default::default(),
                deadline: ::core::default::Default::default(),
                task_estimations: ::core::default::Default::default(),
                task_type: ::core::default::Default::default(),
            }
        }
    }
    impl TaskReqBuilder {
        pub fn new() -> Self {
            Self {
                req: ::alloc::vec::Vec::new(),
                ..Default::default()
            }
        }
        pub fn resource_req(mut self, req: ResourceReq) -> Self {
            self.req.push(req);
            self
        }
        pub fn with_deadline(mut self, deadline: Option<Deadline>) -> Self {
            self.deadline = deadline;
            self
        }
        pub fn with_time_estimations(
            mut self,
            time_per_iter: Duration,
            num_of_iter: usize,
        ) -> Self {
            self.task_estimations.replace(TaskEstimations {
                time_per_iter,
                num_of_iter,
            });
            self
        }
        pub fn with_task_type(mut self, task: TaskType) -> Self {
            self.task_type = Some(task);
            self
        }
        pub fn build(self) -> TaskRequirements {
            TaskRequirements {
                req: self.req,
                deadline: self.deadline,
                estimations: self.task_estimations,
                task_type: self.task_type,
            }
        }
    }
    /// Contains all the requirements and timing description for
    /// a task. This parameter will be used by the scheduler solve for
    /// scheduling the task in the right time window and resource
    pub struct TaskRequirements {
        pub req: Vec<ResourceReq>,
        pub deadline: Option<Deadline>,
        pub estimations: Option<TaskEstimations>,
        pub task_type: Option<TaskType>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TaskRequirements {
        #[inline]
        fn clone(&self) -> TaskRequirements {
            match *self {
                TaskRequirements {
                    req: ref __self_0_0,
                    deadline: ref __self_0_1,
                    estimations: ref __self_0_2,
                    task_type: ref __self_0_3,
                } => TaskRequirements {
                    req: ::core::clone::Clone::clone(&(*__self_0_0)),
                    deadline: ::core::clone::Clone::clone(&(*__self_0_1)),
                    estimations: ::core::clone::Clone::clone(&(*__self_0_2)),
                    task_type: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TaskRequirements {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TaskRequirements {
                    req: ref __self_0_0,
                    deadline: ref __self_0_1,
                    estimations: ref __self_0_2,
                    task_type: ref __self_0_3,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TaskRequirements");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "req",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "deadline",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "estimations",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "task_type",
                        &&(*__self_0_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TaskRequirements {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "TaskRequirements",
                    false as usize + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "req",
                    &self.req,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "deadline",
                    &self.deadline,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "estimations",
                    &self.estimations,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "task_type",
                    &self.task_type,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
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
        impl<'de> _serde::Deserialize<'de> for TaskRequirements {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
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
                            3u64 => _serde::__private::Ok(__Field::__field3),
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
                            "req" => _serde::__private::Ok(__Field::__field0),
                            "deadline" => _serde::__private::Ok(__Field::__field1),
                            "estimations" => _serde::__private::Ok(__Field::__field2),
                            "task_type" => _serde::__private::Ok(__Field::__field3),
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
                            b"req" => _serde::__private::Ok(__Field::__field0),
                            b"deadline" => _serde::__private::Ok(__Field::__field1),
                            b"estimations" => _serde::__private::Ok(__Field::__field2),
                            b"task_type" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TaskRequirements>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TaskRequirements;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TaskRequirements",
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
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Vec<ResourceReq>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct TaskRequirements with 4 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<Deadline>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TaskRequirements with 4 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<TaskEstimations>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct TaskRequirements with 4 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<TaskType>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct TaskRequirements with 4 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(TaskRequirements {
                            req: __field0,
                            deadline: __field1,
                            estimations: __field2,
                            task_type: __field3,
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
                        let mut __field0: _serde::__private::Option<Vec<ResourceReq>> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<Deadline>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<TaskEstimations>> =
                            _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<TaskType>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "req",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<ResourceReq>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "deadline",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<Deadline>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "estimations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<TaskEstimations>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "task_type",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Option<TaskType>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("req") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("deadline") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("estimations") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("task_type") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(TaskRequirements {
                            req: __field0,
                            deadline: __field1,
                            estimations: __field2,
                            task_type: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["req", "deadline", "estimations", "task_type"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TaskRequirements",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TaskRequirements>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub fn dummy_task_requirements() -> TaskRequirements {
        use super::{ResourceMemory, ResourceType};
        let start = chrono::Utc::now();
        let end = start + chrono::Duration::seconds(30);
        let deadline = Deadline::new(start, end);
        TaskReqBuilder::new()
            .resource_req(ResourceReq {
                resource: ResourceType::Gpu(ResourceMemory::All),
                quantity: 1,
                preemptible: true,
            })
            .with_time_estimations(Duration::from_millis(500), 1)
            .with_deadline(Some(deadline))
            .build()
    }
}
pub use crate::config::Settings;
pub use crate::scheduler::Scheduler;
pub use client::{ClientToken, Pid};
pub use device::*;
pub use error::Error;
pub use handler::Handler;
pub use monitor::*;
pub use requests::{PreemptionResponse, RequestMethod};
pub use resource::*;
pub use server::RpcMethods;
pub use server::Server;
pub use solver::{ResourceState, Solver, TaskState};
use std::net::SocketAddr;
pub use task::*;
use crate::db::Database;
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::CloseHandle;
use jsonrpc_http_server::ServerBuilder;
use std::path::Path;
use crossbeam::channel::bounded;
pub type Result<T> = std::result::Result<T, Error>;
/// Starts a json-rpc server listening to *addr*
pub fn run_scheduler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<()> {
    {}
    let __tracing_attr_span = {
        use ::tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
            use ::tracing::__macro_support::MacroCallsite;
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "run_scheduler",
                    "scheduler",
                    tracing::Level::DEBUG,
                    Some("scheduler/src/lib.rs"),
                    Some(44u32),
                    Some("scheduler"),
                    ::tracing_core::field::FieldSet::new(
                        &[],
                        ::tracing_core::callsite::Identifier(&CALLSITE),
                    ),
                    ::tracing::metadata::Kind::SPAN,
                )
            };
            MacroCallsite::new(&META)
        };
        let mut interest = ::tracing::subscriber::Interest::never();
        if tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && tracing::Level::DEBUG <= ::tracing::level_filters::LevelFilter::current()
            && {
                interest = CALLSITE.interest();
                !interest.is_never()
            }
            && CALLSITE.is_enabled(interest)
        {
            let meta = CALLSITE.metadata();
            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
        } else {
            let span = CALLSITE.disabled_span();
            if match tracing::Level::DEBUG {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            } <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        span.record_all(&{ CALLSITE.metadata().fields().value_set(&[]) });
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
            span
        }
    };
    let __tracing_attr_guard = __tracing_attr_span.enter();
    {
        let maintenance_interval = settings.service.maintenance_interval;
        let (shutdown_tx, shutdown_rx) = bounded(0);
        let db = Database::open(database_path, false)?;
        let handler = scheduler::Scheduler::new(settings.clone(), devices, Some(shutdown_tx), db)?;
        let server = Server::new(handler);
        if let Some(tick) = maintenance_interval {
            server.start_maintenance_thread(tick);
        }
        let close_handle = spawn_service(server, settings)?;
        let _ = shutdown_rx.recv().unwrap();
        close_handle.close();
        {
            if match ::tracing::Level::WARN {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            } <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        use ::tracing::log;
                        let level = match ::tracing::Level::WARN {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        };
                        if level <= log::max_level() {
                            let log_meta = log::Metadata::builder()
                                .level(level)
                                .target("scheduler")
                                .build();
                            let logger = log::logger();
                            if logger.enabled(&log_meta) {
                                logger.log(
                                    &log::Record::builder()
                                        .file(Some("scheduler/src/lib.rs"))
                                        .module_path(Some("scheduler"))
                                        .line(Some(63u32))
                                        .metadata(log_meta)
                                        .args({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display};
                                            ::core::fmt::Arguments::new_v1(
                                                &["", " "],
                                                &match (&::core::fmt::Arguments::new_v1(
                                                    &["Service closed"],
                                                    &match () {
                                                        () => [],
                                                    },
                                                ),)
                                                {
                                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    )],
                                                },
                                            )
                                        })
                                        .build(),
                                );
                            }
                        }
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
            if ::tracing::Level::WARN <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::WARN <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event scheduler/src/lib.rs:63",
                            "scheduler",
                            ::tracing::Level::WARN,
                            Some("scheduler/src/lib.rs"),
                            Some(63u32),
                            Some("scheduler"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let interest = CALLSITE.interest();
                if !interest.is_never() && CALLSITE.is_enabled(interest) {
                    let meta = CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &{
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = meta.fields().iter();
                        meta.fields().value_set(&[(
                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                            Some(&::core::fmt::Arguments::new_v1(
                                &["Service closed"],
                                &match () {
                                    () => [],
                                },
                            ) as &Value),
                        )])
                    });
                }
            }
        };
        Ok(())
    }
}
pub fn spawn_scheduler_with_handler<P: AsRef<Path>>(
    settings: Settings,
    database_path: P,
    devices: Devices,
) -> Result<CloseHandle> {
    {}
    let __tracing_attr_span = {
        use ::tracing::__macro_support::Callsite as _;
        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
            use ::tracing::__macro_support::MacroCallsite;
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "spawn_scheduler_with_handler",
                    "scheduler",
                    tracing::Level::DEBUG,
                    Some("scheduler/src/lib.rs"),
                    Some(67u32),
                    Some("scheduler"),
                    ::tracing_core::field::FieldSet::new(
                        &[],
                        ::tracing_core::callsite::Identifier(&CALLSITE),
                    ),
                    ::tracing::metadata::Kind::SPAN,
                )
            };
            MacroCallsite::new(&META)
        };
        let mut interest = ::tracing::subscriber::Interest::never();
        if tracing::Level::DEBUG <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && tracing::Level::DEBUG <= ::tracing::level_filters::LevelFilter::current()
            && {
                interest = CALLSITE.interest();
                !interest.is_never()
            }
            && CALLSITE.is_enabled(interest)
        {
            let meta = CALLSITE.metadata();
            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
        } else {
            let span = CALLSITE.disabled_span();
            if match tracing::Level::DEBUG {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            } <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        span.record_all(&{ CALLSITE.metadata().fields().value_set(&[]) });
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
            span
        }
    };
    let __tracing_attr_guard = __tracing_attr_span.enter();
    {
        let db = Database::open(database_path, true)?;
        let handler = scheduler::Scheduler::new(settings.clone(), devices, None, db)?;
        let server = Server::new(handler);
        spawn_service(server, settings)
    }
}
fn spawn_service<H: Handler>(server: Server<H>, settings: Settings) -> Result<CloseHandle> {
    let address: SocketAddr = settings
        .service
        .address
        .parse()
        .map_err(|_| Error::InvalidAddress)?;
    let mut io = IoHandler::new();
    io.extend_with(server.to_delegate());
    let server = ServerBuilder::new(io)
        .threads(num_cpus::get())
        .start_http(&address)
        .map_err(|e| Error::ConnectionError(e.to_string()))?;
    let close_handle = server.close_handle();
    std::thread::spawn(move || {
        server.wait();
    });
    Ok(close_handle)
}
