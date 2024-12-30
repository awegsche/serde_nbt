use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use serde::{
    de::{MapAccess, SeqAccess},
    Deserialize,
};

use crate::{
    error::Result,
    nbt::{
        TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_END, TAG_FLOAT, TAG_INT, TAG_LONG,
        TAG_SHORT, TAG_STRING,
    },
    Error,
};

pub struct Deserializer<R: Read> {
    reader: R,
    last_tag: Option<u8>,
    last_name: Option<String>,
}

impl<'de, R: Read> Deserializer<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader,
            last_tag: None,
            last_name: None,
        }
    }

    fn get_last_tag(&mut self) -> std::io::Result<u8> {
        if self.last_tag.is_none() {
            self.last_tag = Some(self.reader.read_u8()?);
        }

        Ok(self.last_tag.unwrap())
    }

    /// Reads the name of the NBT tag from reader but discards it.
    fn skip_name(&mut self) -> Result<()> {
        if self.last_name.is_none() {
            read_name(&mut self.reader)?;
        }
        self.last_name = None;

        Ok(())
    }

    fn get_last_name(&mut self) -> Result<&str> {
        if self.last_name.is_none() {
            self.last_name = Some(read_name(&mut self.reader)?);
        }

        Ok(&self.last_name.as_ref().unwrap())
    }
}

pub fn from_reader<'de, R: Read, T: serde::de::Deserialize<'de>>(reader: &'de mut R) -> Result<T> {
    let mut deserializer = Deserializer::from_reader(reader);

    T::deserialize(&mut deserializer)
}

pub fn from_bytes<'de, T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    from_reader(&mut Cursor::new(bytes))
}

impl<'de, 'a, R: Read> serde::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_BYTE {
            return Err(Error::ExpectedByte);
        }

        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_bool(self.reader.read_u8()? == 1)
    }

    fn deserialize_i8<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_SHORT {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i8(i8::try_from(self.reader.read_i16::<BigEndian>()?)?)
    }

    fn deserialize_i16<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_SHORT {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i16(self.reader.read_i16::<BigEndian>()?)
    }

    fn deserialize_i32<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_INT {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i32(self.reader.read_i32::<BigEndian>()?)
    }

    fn deserialize_i64<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_LONG {
            return Err(Error::ExpectedLong);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i64(self.reader.read_i64::<BigEndian>()?)
    }

    fn deserialize_u8<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_BYTE {
            return Err(Error::ExpectedByte);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_u8(self.reader.read_u8()?)
    }

    fn deserialize_u16<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_INT {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i32(self.reader.read_i32::<BigEndian>()?)
    }

    fn deserialize_u32<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_LONG {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i64(self.reader.read_i64::<BigEndian>()?)
    }

    fn deserialize_u64<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_LONG {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_i64(i64::try_from(self.reader.read_u64::<BigEndian>()?)?)
    }

    fn deserialize_f32<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_FLOAT {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_f32(self.reader.read_f32::<BigEndian>()?)
    }

    fn deserialize_f64<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_DOUBLE {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        visitor.visit_f64(self.reader.read_f64::<BigEndian>()?)
    }

    fn deserialize_char<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("deserializing string");
        if self.get_last_tag()? != TAG_STRING {
            return Err(Error::ExpectedShort);
        }
        self.skip_name()?;
        self.last_tag = None;

        let len = self.reader.read_i16::<BigEndian>()?;
        let mut buf = vec![0; len as usize];
        self.reader.read(&mut buf)?;
        let string = String::from_utf8(buf)?;

        println!("string: {}, len: {}", string, len);

        visitor.visit_str(&string)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(mut self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if self.get_last_tag()? != TAG_BYTE_ARRAY {
            return Err(Error::ExpectedByteArray);
        }
        self.skip_name()?;
        self.last_tag = None;

        let len = self.reader.read_i16::<BigEndian>()?;
        let mut buf = vec![0; len as usize];
        self.reader.read(&mut buf)?;

        visitor.visit_bytes(&buf)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("try deserializing struct");
        if self.get_last_tag()? != TAG_COMPOUND {
            return Err(Error::ExpectedCompound);
        }
        println!("nbt tag is compound");
        self.skip_name()?;

        println!("nbt name is {}", name);
        self.last_tag = None;

        visitor.visit_map(&mut *self)

        /*
        println!("end of struct");
        let end = self.reader.read_u8()?;

        if end != TAG_END {
            return Err(Error::ExpectedEnd);
        }
        Ok(val)
            */
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        if let Some(name) = self.last_name.as_ref() {
            println!("identifier: {}", name);
            return visitor.visit_str(name);
        }

        return Err(Error::ExpectedIdentifier);
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        println!("{:?}", self.last_name);

        self.reader.read_u8()?;
        visitor.visit_unit()
    }
}

impl<'de, R: Read> SeqAccess<'de> for Deserializer<R> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::prelude::v1::Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let tag = self.get_last_tag()?;

        if tag == TAG_END {
            return Ok(None);
        }

        seed.deserialize(self).map(Some)
    }
}

impl<'de, R: Read> MapAccess<'de> for Deserializer<R> {
    type Error = Error;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> std::prelude::v1::Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let tag = self.get_last_tag()?;
        if tag == TAG_END {
            return Ok(None);
        }
        let name = self.get_last_name()?;

        println!("tag: {}, name: {}", tag, name);

        println!("type of seed: {}", std::any::type_name::<K::Value>());

        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

pub fn read_name<R: std::io::Read>(r: &mut R) -> Result<String> {
    let len = r.read_u16::<BigEndian>()?;
    let mut buf = vec![0; len as usize];
    r.read(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}
