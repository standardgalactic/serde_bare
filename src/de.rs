use crate::{error::Error, Uint};
use serde::de;
use std::{
    convert::TryInto,
    i16, i32, i64, i8,
    io::{Cursor, Read},
    str, u16, u32, u64, u8,
};

/// Try and return a Vec<u8> of `len` bytes from a Reader
#[inline]
fn read_bytes<R: Read>(reader: R, len: usize) -> Result<Vec<u8>, std::io::Error> {
    // Allocate at most 4096 bytes to start with. Growing a Vec is fairly efficient once you get out
    // of the region of the first few hundred bytes.
    let capacity = len.min(4096);
    let mut buffer = Vec::with_capacity(capacity);
    let read = reader.take(len as u64).read_to_end(&mut buffer)?;
    if read < len {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Unexpected EOF reading number of bytes expected in field prefix",
        ))
    } else {
        Ok(buffer)
    }
}

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
            _ => visitor.visit_bool(true),
        }
    }

    /// BARE type: i8
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_i8(i8::from_le_bytes(buf))
    }

    /// BARE type: i16
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    /// BARE type: i32
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    /// BARE type: i64
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_i64(i64::from_le_bytes(buf))
    }

    serde::serde_if_integer128! {
        /// BARE type: data<16>
        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            let mut buf = [0u8; 16];
            self.reader.read_exact(&mut buf).map_err(Error::Io)?;
            visitor.visit_i128(i128::from_le_bytes(buf))
        }
    }

    /// BARE type: u8
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_u8(u8::from_le_bytes(buf))
    }

    /// BARE type: u16
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_u16(u16::from_le_bytes(buf))
    }

    /// BARE type: u32
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_u32(u32::from_le_bytes(buf))
    }

    /// BARE type: u64
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_u64(u64::from_le_bytes(buf))
    }

    serde::serde_if_integer128! {
        /// BARE type: data<16>
        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            let mut buf = [0u8; 16];
            self.reader.read_exact(&mut buf).map_err(Error::Io)?;
            visitor.visit_u128(u128::from_le_bytes(buf))
        }
    }

    /// BARE type: f32
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    /// BARE type: f64
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    /// BARE type: u32
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let codepoint = <u32 as de::Deserialize>::deserialize(self)?;
        visitor.visit_char(codepoint.try_into().map_err(|_| Error::InvalidChar)?)
    }

    /// BARE type: string
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;
        let length = length as usize;
        let buf = read_bytes(&mut self.reader, length).map_err(Error::Io)?;
        let utf8 = str::from_utf8(&buf).map_err(|_| Error::InvalidUtf8)?;
        visitor.visit_str(utf8)
    }

    /// BARE type: string
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;
        let length = length as usize;
        let buf = read_bytes(&mut self.reader, length).map_err(Error::Io)?;
        let utf8 = String::from_utf8(buf).map_err(|_| Error::InvalidUtf8)?;
        visitor.visit_string(utf8)
    }

    /// BARE type: data
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;
        let length = length as usize;
        let buf = read_bytes(&mut self.reader, length).map_err(Error::Io)?;
        visitor.visit_bytes(&buf)
    }

    /// BARE type: data
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;
        let length = length as usize;
        let buf = read_bytes(&mut self.reader, length).map_err(Error::Io)?;
        visitor.visit_byte_buf(buf)
    }

    /// BARE type: optional<type>
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if <bool as de::Deserialize>::deserialize(&mut *self)? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    /// BARE type: void
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    /// BARE type: void
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

    /// BARE type: void
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
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;

        struct Seq<'a, R>(&'a mut Deserializer<R>, u64);

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
    /// Deserializing fewer elements than `len` is allowed.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Seq<'a, R>(&'a mut Deserializer<R>, usize);

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

        visitor.visit_seq(Seq::<'a, R>(self, len))
    }

    /// BARE type: struct
    /// `name` is ignored.
    /// Deserializing fewer elements than `len` is allowed.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Seq<'a, R>(&'a mut Deserializer<R>, usize);

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

        visitor.visit_seq(Seq::<'a, R>(self, len))
    }

    /// BARE type: map\[T\]U
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(length) = <Uint as de::Deserialize>::deserialize(&mut *self)?;

        struct Map<'a, R>(&'a mut Deserializer<R>, u64);

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
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
            where
                V: de::DeserializeSeed<'de>,
            {
                let val = seed.deserialize(&mut *self.0)?;
                Ok((val, self))
            }
        }

        impl<'de, 'a, R> de::VariantAccess<'de> for Enum<'a, R>
        where
            R: Read,
        {
            type Error = Error;

            /// Unserialized type.
            fn unit_variant(self) -> Result<(), Self::Error> {
                Ok(())
            }

            /// Bare type: T
            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
            where
                T: de::DeserializeSeed<'de>,
            {
                seed.deserialize(self.0)
            }

            /// Bare type: struct
            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                de::Deserializer::deserialize_tuple(self.0, len, visitor)
            }

            /// Bare type: struct
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

    /// Deserialize the enum discriminant as a BARE Uint
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let Uint(id) = <Uint as de::Deserialize>::deserialize(&mut *self)?;
        let variant: u32 = id.try_into().map_err(|_| {
            Error::Message("Enum identifiers larger than u32 are not supported".to_string())
        })?;
        visitor.visit_u32(variant)
    }

    /// Returns Error::AnyUnsupported.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::AnyUnsupported)
    }

    /// Returns false.
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(false, from_slice(&[0]).unwrap());
        assert_eq!(true, from_slice(&[1]).unwrap());
        assert_eq!(true, from_slice(&[2]).unwrap());
    }

    #[test]
    fn test_signed() {
        assert_eq!(1i8, from_slice(&[1]).unwrap());
        assert_eq!(513i16, from_slice(&[1, 2]).unwrap());
        assert_eq!(67305985i32, from_slice(&[1, 2, 3, 4]).unwrap());
        assert_eq!(
            578437695752307201i64,
            from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]).unwrap()
        );
        serde::serde_if_integer128! {
            assert_eq!(
                21345817372864405881847059188222722561i128,
                from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap()
            );
        }
    }

    #[test]
    fn test_unsigned() {
        assert_eq!(1u8, from_slice(&[1]).unwrap());
        assert_eq!(513u16, from_slice(&[1, 2]).unwrap());
        assert_eq!(67305985u32, from_slice(&[1, 2, 3, 4]).unwrap());
        assert_eq!(
            578437695752307201u64,
            from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]).unwrap()
        );
        serde::serde_if_integer128! {
            assert_eq!(
                21345817372864405881847059188222722561u128,
                from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).unwrap()
            );
        }
    }

    #[test]
    fn test_float() {
        assert_eq!(1.0f32, from_slice(&1.0f32.to_le_bytes()).unwrap());
        assert!(from_slice::<f32>(&f32::NAN.to_le_bytes()).unwrap().is_nan());
        assert_eq!(
            f32::INFINITY,
            from_slice(&f32::INFINITY.to_le_bytes()).unwrap()
        );
        assert_eq!(
            f32::NEG_INFINITY,
            from_slice(&f32::NEG_INFINITY.to_le_bytes()).unwrap()
        );
        assert_eq!(1.0f64, from_slice(&1.0f64.to_le_bytes()).unwrap());
        assert!(from_slice::<f64>(&f64::NAN.to_le_bytes()).unwrap().is_nan());
        assert_eq!(
            f64::INFINITY,
            from_slice(&f64::INFINITY.to_le_bytes()).unwrap()
        );
        assert_eq!(
            f64::NEG_INFINITY,
            from_slice(&f64::NEG_INFINITY.to_le_bytes()).unwrap()
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            "hello",
            from_slice::<String>(&[5, b'h', b'e', b'l', b'l', b'o']).unwrap()
        )
    }

    #[test]
    fn test_data() {
        assert_eq!(
            &[1u8, 2, 3, 4, 5][..],
            &*from_slice::<Vec<u8>>(&[5, 1, 2, 3, 4, 5]).unwrap()
        )
    }

    #[test]
    fn test_optional() {
        assert_eq!(None, from_slice::<Option<u32>>(&[0]).unwrap());
        assert_eq!(
            Some(67305985u32),
            from_slice::<Option<u32>>(&[1, 1, 2, 3, 4]).unwrap()
        );
    }

    #[test]
    fn test_slice() {
        assert_eq!(
            &[0u8; 4][..],
            &*from_slice::<Box<[u8]>>(&[4, 0, 0, 0, 0]).unwrap()
        );
    }
}
