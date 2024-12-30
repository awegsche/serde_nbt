use crate::error::{Error, Result};
use crate::nbt::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rnbt::{write_nbt, NbtField, NbtList, NbtValue};
use serde::{ser, Serialize};
use std::io::{Read, Write};

// ---- public methods -----------------------------------------------------------------------------
pub fn to_writer<W: std::io::Write, T: Serialize>(
    writer: &mut W,
    value: &T,
    name: String,
) -> Result<()> {
    let mut serializer = Serializer::new(name, writer);

    value.serialize(&mut serializer)?;

    Ok(())
}

// ---- Serializer struct --------------------------------------------------------------------------
pub struct Serializer<'a, W: std::io::Write> {
    name: String,
    writer: &'a mut W,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(name: String, writer: &'a mut W) -> Self {
        Self { name, writer }
    }
}

// ---- Impls --------------------------------------------------------------------------------------
impl<'a, W: Write> ser::Serializer for &'a mut Serializer<'a, W> {
    type Ok = ();

    type Error = crate::error::Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_BYTE)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_u8(if v { 1 } else { 0 })?;

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_SHORT)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i16::<BigEndian>(v as i16)?;

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_SHORT)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i16::<BigEndian>(v)?;

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_INT)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i32::<BigEndian>(v)?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_LONG)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i64::<BigEndian>(v)?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_BYTE)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_u8(v)?;

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_INT)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i32::<BigEndian>(v as i32)?;

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_LONG)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i64::<BigEndian>(v as i64)?;

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_LONG)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i64::<BigEndian>(i64::try_from(v)?)?;

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_FLOAT)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_f32::<BigEndian>(v)?;

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_DOUBLE)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_f64::<BigEndian>(v)?;

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_STRING)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i16::<BigEndian>(v.len() as i16)?;
        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.writer.write_u8(TAG_BYTE_ARRAY)?;
        write_name(&mut self.writer, &self.name)?;
        self.writer.write_i32::<BigEndian>(v.len() as i32)?;
        self.writer.write_all(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.writer.write_u8(TAG_COMPOUND)?;
        write_name(&mut self.writer, &self.name)?;

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeMap for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeStruct for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer::new(key.to_owned(), self.writer);
        value.serialize(&mut serializer)
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.writer.write_u8(TAG_END)?;
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::prelude::v1::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

// ---- Helper functions ---------------------------------------------------------------------------
pub fn write_name<W: Write>(w: &mut W, name: &str) -> std::io::Result<()> {
    w.write_u16::<BigEndian>(name.len() as u16)?;
    w.write_all(name.as_bytes())
}

