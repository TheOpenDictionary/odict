use std::{error::Error, io::Write};

use serde::{Serialize, Serializer};
use serde_json::{
    ser::{CompactFormatter, Formatter, PrettyFormatter},
    to_string_pretty,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JSONSerializer<T: ?Sized>(T);

impl<S> JSONSerializer<S>
where
    S: Serializer,
{
    pub fn new(serializer: S) -> Self {
        JSONSerializer(serializer)
    }
}

macro_rules! forward_method {
    ($name: ident (self $(, $arg: ident : $arg_type: ty)* ) -> $return_type: ty) => {
        fn $name (self $(, $arg : $arg_type)* ) -> $return_type {
            (self.0).$name( $($arg),* )
        }
    };
}

macro_rules! forward_serialize_methods {
    ( $( $name: ident $arg_type: ty),* ) => {
        $(
            forward_method!($name(self, v: $arg_type) -> Result<Self::Ok, Self::Error>);
        )*
    };
}

impl<S> Serializer for JSONSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = S::SerializeTuple;
    type SerializeTupleStruct = S::SerializeTupleStruct;
    type SerializeTupleVariant = S::SerializeTupleVariant;
    type SerializeMap = S::SerializeMap;
    type SerializeStruct = S::SerializeStruct;
    type SerializeStructVariant = S::SerializeStructVariant;

    forward_serialize_methods! {
        serialize_bool bool,
        serialize_i8 i8,
        serialize_i16 i16,
        serialize_i32 i32,
        serialize_i64 i64,
        serialize_u8 u8,
        serialize_u16 u16,
        serialize_u32 u32,
        serialize_u64 u64,
        serialize_f32 f32,
        serialize_f64 f64,
        serialize_char char,
        serialize_str &str,
        serialize_bytes &[u8],
        serialize_unit_struct &'static str
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.0
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.serialize_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0
            .serialize_struct_variant(name, variant_index, variant, len)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.serialize_tuple(len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.serialize_map(len)
    }
}

fn stringify<V>(value: &V, pretty: bool) -> Result<String, Box<dyn Error>>
where
    V: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);

    if pretty {
        // value.serialize(JSONSerializer::pretty(writer))?;
    } else {
        let mut ser = serde_json::Serializer::new(&mut writer);
        value.serialize(JSONSerializer::new(&mut ser))?;
    }

    let string = unsafe { String::from_utf8_unchecked(writer) };

    Ok(string)
}

pub fn to_json<S>(value: S) -> Result<String, Box<dyn Error>>
where
    S: Serialize,
{
    stringify(&value, false)
}

// pub fn to_json_pretty<S: ?Sized + Serialize>(value: S) -> Result<String, Box<dyn Error>> {
//     let mut writer = Vec::with_capacity(128);
//     let string = stringify(value, &JSONSerializer::pretty(writer))?;
//     Ok(string)
// }
