use crate::error::{Error, Result};
use rnbt::{NbtField, NbtList, NbtValue, write_nbt};
use serde::{Serialize, ser};

pub struct Serializer {
    output: NbtField,
}

pub fn to_writer<W: std::io::Write, T: Serialize>(
    writer: &mut W,
    value: &T,
    name: String,
) -> Result<()> {
    let mut serializer = Serializer::new(name);

    value.serialize(&mut serializer)?;

    write_nbt(writer, &serializer.output)?;

    Ok(())
}

impl Serializer {
    pub fn new(name: String) -> Self {
        Self {
            output: NbtField {
                name,
                value: NbtValue::End,
            },
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
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
        self.output.value = NbtValue::Byte(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.output.value = NbtValue::Short(v as i16);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.output.value = NbtValue::Short(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.output.value = NbtValue::Int(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        self.output.value = NbtValue::Long(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.output.value = NbtValue::Byte(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.output.value = NbtValue::Int(v as i32);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.output.value = NbtValue::Long(v as i64);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        self.output.value = NbtValue::Long(i64::try_from(v).unwrap());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.output.value = NbtValue::Float(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        self.output.value = NbtValue::Double(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.output.value = NbtValue::String(v.to_string());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.output.value = NbtValue::String(v.to_string());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.output.value = NbtValue::ByteArray(v.to_vec());
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
        //self.output.name = name.to_string();
        self.output.value = NbtValue::Compound(vec![]);
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

fn push_value_to_list(list: &mut NbtList, value: NbtValue) -> Result<()> {
    match value {
        NbtValue::Byte(b) => {
            if let NbtList::Byte(l) = list {
                l.push(b);
                return Ok(());
            }
        }
        NbtValue::Boolean(b) => {
            if let NbtList::Boolean(l) = list {
                l.push(b);
                return Ok(());
            }
        }
        NbtValue::Short(s) => {
            if let NbtList::Short(l) = list {
                l.push(s);
                return Ok(());
            }
        }
        NbtValue::Int(i) => {
            if let NbtList::Int(l) = list {
                l.push(i);
                return Ok(());
            }
        }
        NbtValue::Long(i) => {
            if let NbtList::Long(l) = list {
                l.push(i);
                return Ok(());
            }
        }
        NbtValue::Float(f) => {
            if let NbtList::Float(l) = list {
                l.push(f);
                return Ok(());
            }
        }
        NbtValue::Double(d) => {
            if let NbtList::Double(l) = list {
                l.push(d);
                return Ok(());
            }
        }
        NbtValue::String(s) => todo!(),
        NbtValue::List(_) => todo!(),
        NbtValue::Compound(_) => todo!(),
        NbtValue::ByteArray(_) => todo!(),
        NbtValue::IntArray(_) => todo!(),
        NbtValue::LongArray(_) => todo!(),
        NbtValue::End => todo!(),
    }
    return Err(Error::IncompatibleListType);
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer {
            output: NbtField {
                name: String::new(),
                value: NbtValue::End,
            },
        };
        value.serialize(&mut serializer)?;

        if let NbtValue::End = serializer.output.value {
            match serializer.output.value {
                NbtValue::Byte(b) => self.output.value = NbtValue::List(NbtList::Byte(vec![b])),
                NbtValue::Boolean(b) => {
                    self.output.value = NbtValue::List(NbtList::Boolean(vec![b]))
                }
                NbtValue::Short(s) => self.output.value = NbtValue::List(NbtList::Short(vec![s])),
                NbtValue::Int(i) => self.output.value = NbtValue::List(NbtList::Int(vec![i])),
                NbtValue::Long(i) => self.output.value = NbtValue::List(NbtList::Long(vec![i])),
                NbtValue::Float(f) => self.output.value = NbtValue::List(NbtList::Float(vec![f])),
                NbtValue::Double(d) => self.output.value = NbtValue::List(NbtList::Double(vec![d])),
                _ => return Err(Error::UnknownListType),
            }
        }
        if let NbtValue::List(l) = &mut self.output.value {
            push_value_to_list(l, serializer.output.value)
        } else {
            Err(Error::NotWritingToList)
        }
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
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

impl<'a> ser::SerializeMap for &'a mut Serializer {
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

impl<'a> ser::SerializeStruct for &'a mut Serializer {
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
        let mut serializer = Serializer::new(key.to_owned());
        value.serialize(&mut serializer)?;

        if let NbtValue::Compound(c) = &mut self.output.value {
            c.push(serializer.output);
            Ok(())
        } else {
            Err(Error::NotWritingToCompound)
        }
    }

    fn end(self) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
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

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
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

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
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

#[cfg(test)]
mod tests {
    use rnbt::read_nbt;

    use super::*;

    #[test]
    fn try_simple_struct() {
        #[derive(serde::Serialize)]
        struct TestStruct {
            a: i32,
            b: String,
        }

        let mut output = Vec::new();
        to_writer(
            &mut output,
            &TestStruct {
                a: 1,
                b: "hello".to_string(),
            },
            "test".to_string(),
        )
        .unwrap();

        println!("{:?}", output);


        let mut cursor = std::io::Cursor::new(output.clone());
        let output_nbt = read_nbt(&mut cursor).unwrap();
        println!("{:?}", output_nbt);

        let mut expected = Vec::new();
        write_nbt(&mut expected, &NbtField {
            name: "test".to_string(),
            value: NbtValue::Compound(vec![
                NbtField {
                    name: "a".to_string(),
                    value: NbtValue::Int(1),
                },
                NbtField {
                    name: "b".to_string(),
                    value: NbtValue::String("hello".to_string()),
                },
            ]),
        })
        .unwrap();

        assert_eq!(expected, output);
    }
}
