use crate::error::Error;
use serde::de;
use std::{
    i16, i32, i64, i8,
    io::{Cursor, Read},
    str, u16, u32, u64, u8,
};

pub struct Deserializer<R> {
    reader: R,
}

impl<R> Deserializer<R> {
    pub fn new(reader: R) -> Self {
        Deserializer { reader }
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: Read,
{
    type Error = Error;

    /// Returns Error::AnyUnsupported.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::AnyUnsupported)
    }

    /// BARE type: bool
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match <u8 as de::Deserialize>::deserialize(self)? {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => Err(Error::InvalidBool),
        }
    }

    /// BARE type: i8
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_i8(i8::from_le_bytes(buf))
    }

    /// BARE type: i16
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    /// BARE type: i32
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    /// BARE type: i64
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_i64(i64::from_le_bytes(buf))
    }

    serde::serde_if_integer128! {
        /// Returns Error::I128Unsupported.
        fn deserialize_i128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            Err(Error::I128Unsupported)
        }
    }

    /// BARE type: u8
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_u8(u8::from_le_bytes(buf))
    }

    /// BARE type: u16
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_u16(u16::from_le_bytes(buf))
    }

    /// BARE type: u32
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_u32(u32::from_le_bytes(buf))
    }

    /// BARE type: u64
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_u64(u64::from_le_bytes(buf))
    }

    serde::serde_if_integer128! {
        /// Returns Error::U128Unsupported.
        fn deserialize_u128<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            Err(Error::U128Unsupported)
        }
    }

    /// BARE type: f32
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    /// BARE type: f64
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    /// BARE type: u32
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_u32(visitor)
    }

    /// BARE type: string
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf) as usize
        };
        let mut buf = Vec::with_capacity(length);
        buf.resize(length, 0);
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        let utf8 = str::from_utf8(&buf).map_err(|_| Error::InvalidUtf8)?;
        visitor.visit_str(utf8)
    }

    /// BARE type: string
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf) as usize
        };
        let mut buf = Vec::with_capacity(length);
        buf.resize(length, 0);
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        let utf8 = String::from_utf8(buf).map_err(|_| Error::InvalidUtf8)?;
        visitor.visit_string(utf8)
    }

    /// BARE type: data
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf) as usize
        };
        let mut buf = Vec::with_capacity(length);
        buf.resize(length, 0);
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_bytes(&buf)
    }

    /// BARE type: data
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf) as usize
        };
        let mut buf = Vec::with_capacity(length);
        buf.resize(length, 0);
        self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
        visitor.visit_byte_buf(buf)
    }

    /// BARE type: optional<T>
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let is_present = {
            let mut buf = [0u8; 1];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            match buf[0] {
                0 => false,
                1 => true,
                _ => return Err(Error::InvalidBool),
            }
        };
        if !is_present {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    /// Unserialized type.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// Unserialized type.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// Unserialized type.
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    /// BARE type: []T
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf)
        };
        struct Seq<'a, R>(&'a mut Deserializer<R>, u32);
        impl<'de, 'a, R> de::SeqAccess<'de> for Seq<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                if self.1 == 0 {
                    Ok(None)
                } else {
                    self.1 -= 1;
                    Ok(Some(seed.deserialize(&mut *self.0)?))
                }
            }
        }
        visitor.visit_seq(Seq::<'a, R>(self, length))
    }

    /// BARE type: \[len\]T
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Seq<'a, R>(&'a mut Deserializer<R>);
        impl<'de, 'a, R> de::SeqAccess<'de> for Seq<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                Ok(Some(seed.deserialize(&mut *self.0)?))
            }
        }
        visitor.visit_seq(Seq::<'a, R>(self))
    }

    /// BARE type: struct
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Seq<'a, R>(&'a mut Deserializer<R>);
        impl<'de, 'a, R> de::SeqAccess<'de> for Seq<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                Ok(Some(seed.deserialize(&mut *self.0)?))
            }
        }
        visitor.visit_seq(Seq::<'a, R>(self))
    }

    /// BARE type: map\[T\]U
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let length = {
            let mut buf = [0u8; 4];
            self.reader.read_exact(&mut buf).map_err(|e| Error::Io(e))?;
            u32::from_le_bytes(buf)
        };

        struct Map<'a, R>(&'a mut Deserializer<R>, u32);
        impl<'de, 'a, R> de::MapAccess<'de> for Map<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
            where
                K: de::DeserializeSeed<'de>,
            {
                if self.1 == 0 {
                    Ok(None)
                } else {
                    Ok(Some(seed.deserialize(&mut *self.0)?))
                }
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
            where
                V: de::DeserializeSeed<'de>,
            {
                self.1 -= 1;
                Ok(seed.deserialize(&mut *self.0)?)
            }
        }
        visitor.visit_map(Map::<'a, R>(self, length))
    }

    /// BARE type: struct
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Seq<'a, R>(&'a mut Deserializer<R>);
        impl<'de, 'a, R> de::SeqAccess<'de> for Seq<'a, R>
        where
            R: Read,
        {
            type Error = Error;
            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                Ok(Some(seed.deserialize(&mut *self.0)?))
            }
        }
        visitor.visit_seq(Seq::<'a, R>(self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Enum<'a, R>(&'a mut Deserializer<R>);
        impl<'de, 'a, R> de::EnumAccess<'de> for Enum<'a, R>
        where
            R: Read,
        {
            type Error = Error;
            type Variant = Variant<'a, R>;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where
                V: de::DeserializeSeed<'de>,
            {
                let val = seed.deserialize(&mut *self.0)?;
                Ok((val, Variant(self.0)))
            }
        }
        struct Variant<'a, R>(&'a mut Deserializer<R>);
        impl<'de, 'a, R> de::VariantAccess<'de> for Variant<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            fn unit_variant(self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                seed.deserialize(self.0)
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                de::Deserializer::deserialize_tuple(self.0, len, visitor)
            }

            fn struct_variant<V>(
                self,
                fields: &'static [&'static str],
                visitor: V,
            ) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                de::Deserializer::deserialize_struct(self.0, "", fields, visitor)
            }
        }
        visitor.visit_enum(Enum::<'a, R>(self))
    }

    /// Returns Error::IdentifierUnsupported.
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::IdentifierUnsupported)
    }

    /// Returns Error::AnyUnsupported.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::AnyUnsupported)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

pub fn from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: Read,
    T: de::DeserializeOwned,
{
    T::deserialize(&mut Deserializer { reader })
}

pub fn from_slice<T>(slice: &[u8]) -> Result<T, Error>
where
    T: de::DeserializeOwned,
{
    T::deserialize(&mut Deserializer {
        reader: Cursor::new(slice),
    })
}
