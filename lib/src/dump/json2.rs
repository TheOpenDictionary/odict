use std::{error::Error, io::Write};

use serde::{
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};

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

impl<T: ?Sized> Serialize for JSONSerializer<T>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(JSONSerializer(serializer))
    }
}

impl<S> Serializer for JSONSerializer<S>
where
    S: Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = JSONSerializer<S::SerializeSeq>;
    type SerializeTuple = JSONSerializer<S::SerializeTuple>;
    type SerializeTupleStruct = JSONSerializer<S::SerializeTupleStruct>;
    type SerializeTupleVariant = JSONSerializer<S::SerializeTupleVariant>;
    type SerializeMap = JSONSerializer<S::SerializeMap>;
    type SerializeStruct = JSONSerializer<S::SerializeStruct>;
    type SerializeStructVariant = JSONSerializer<S::SerializeStructVariant>;

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

    fn serialize_unit(self) -> Result<S::Ok, S::Error> {
        self.0.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<S::Ok, S::Error> {
        self.0.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
    {
        self.0
            .serialize_newtype_struct(name, &JSONSerializer(value))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
    {
        self.0
            .serialize_newtype_variant(name, variant_index, variant, &JSONSerializer(value))
    }

    fn serialize_none(self) -> Result<S::Ok, Self::Error> {
        self.0.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<S::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.0.serialize_some(&JSONSerializer(value))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.serialize_seq(len).map(JSONSerializer)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.serialize_tuple(len).map(JSONSerializer)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0.serialize_tuple_struct(name, len).map(JSONSerializer)
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
            .map(JSONSerializer)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.serialize_map(len).map(JSONSerializer)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.serialize_struct(name, len).map(JSONSerializer)
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
            .map(JSONSerializer)
    }
}

impl<S> SerializeSeq for JSONSerializer<S>
where
    S: SerializeSeq,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_element(&JSONSerializer(value))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

impl<S> SerializeTuple for JSONSerializer<S>
where
    S: SerializeTuple,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_element(&JSONSerializer(value))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

impl<S> SerializeTupleStruct for JSONSerializer<S>
where
    S: SerializeTupleStruct,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_field(&JSONSerializer(value))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

impl<S> SerializeTupleVariant for JSONSerializer<S>
where
    S: SerializeTupleVariant,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_field(&JSONSerializer(value))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

impl<S> SerializeMap for JSONSerializer<S>
where
    S: SerializeMap,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_key(&JSONSerializer(key))
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_value(&JSONSerializer(value))
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), S::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        self.0.serialize_entry(key, &JSONSerializer(value))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

fn static_str(s: String) -> &'static str {
    let b = Box::new(s);
    Box::leak(b)
}

impl<S> SerializeStruct for JSONSerializer<S>
where
    S: SerializeStruct,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, name: &'static str, field: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        let v: &'static str = static_str(name.replace("@", ""));
        self.0.serialize_field(v, &JSONSerializer(field))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
    }
}

impl<S> SerializeStructVariant for JSONSerializer<S>
where
    S: SerializeStructVariant,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T: ?Sized>(&mut self, name: &'static str, field: &T) -> Result<(), S::Error>
    where
        T: Serialize,
    {
        self.0.serialize_field(name, &JSONSerializer(field))
    }

    fn end(self) -> Result<S::Ok, S::Error> {
        self.0.end()
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
