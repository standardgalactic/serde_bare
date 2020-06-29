use crate::error::Error;
use serde::{ser, Serialize};
use std::{convert::TryInto, io::Write};

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Serializer { writer }
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    /// BARE type: bool
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(v as u8)
    }

    /// BARE type: i8
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: i16
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: i32
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: i64
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    serde::serde_if_integer128! {
        /// BARE type: data<16>
        fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
            self.writer
                .write_all(&v.to_le_bytes())
                .map_err(Error::Io)?;
            Ok(())
        }
    }

    /// BARE type: u8
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: u16
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: u32
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: u64
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    serde::serde_if_integer128! {
        /// BARE type: data<16>
        fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
            self.writer
                .write_all(&v.to_le_bytes())
                .map_err(Error::Io)?;
            Ok(())
        }
    }

    /// BARE type: f32
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: f64
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: u32
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    /// BARE type: string
    /// Restriction: len < u32::MAX, or Error::LengthOverflow is returned.
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthOverflow)?;
        self.serialize_u32(len)?;
        self.writer.write_all(v.as_bytes()).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: data
    /// Restriction: len < u32::MAX, or Error::LengthOverflow is returned.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let len: u32 = v.len().try_into().map_err(|_| Error::LengthOverflow)?;
        self.serialize_u32(len)?;
        self.writer.write_all(v).map_err(Error::Io)?;
        Ok(())
    }

    /// BARE type: optional<T>
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(0)
    }

    /// BARE type: optional<T>
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_u8(1)?;
        value.serialize(self)
    }

    /// No-op.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    /// No-op.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    /// BARE type: u32
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(variant_index)
    }

    /// BARE type: T
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    /// BARE type: { u32, struct }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }

    /// BARE type if len is None: []type
    /// BARE type if len is Some: \[len\]type
    /// Error::LengthOverflow if len > u32::MAX
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.serialize_u32(len.try_into().map_err(|_| Error::LengthOverflow)?)?;
        }
        Ok(self)
    }

    /// BARE type: \[len\]type
    /// Error::LengthOverflow if len > u32::MAX
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    /// BARE type: struct
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    /// BARE type: { u32, struct }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    /// BARE type: map\[T\]U
    /// Error::MapLengthRequired if len is None
    /// Error::LengthOverflow if len > u32::MAX
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.serialize_u32(
            len.ok_or(Error::MapLengthRequired)?
                .try_into()
                .map_err(|_| Error::LengthOverflow)?,
        )?;
        Ok(self)
    }

    /// BARE type: struct
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    /// BARE type: { u32, struct }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    /// Returns false.
    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut vec = Vec::new();
    let mut serializer = Serializer { writer: &mut vec };
    value.serialize(&mut serializer)?;
    Ok(vec)
}

pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<(), Error>
where
    W: Write,
    T: Serialize,
{
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    Ok(())
}
