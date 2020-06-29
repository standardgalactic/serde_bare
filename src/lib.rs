#![forbid(unsafe_code)]
//! # serde_bare
//! An implementation of the BARE (https://git.sr.ht/~sircmpwn/bare) encoding format draft.
//!
//! ## `u8`/`u16`/`u32`/`u64`, `i8`/`i16`/`i32`/`i64`, `f32`/`f64`, `bool`, `string`, `data`, `optional<type>`, `[]type`, `map`, and `struct`
//! Mapped exactly.
//!
//! ## `u128`, `i128`
//! Encoded in the same fashion as the other integers, but the type is data<16>.
//!
//! ## `uint`, `int`, `enum`
//! [Uint] and [Int] types wrap a u64/i64 for these types. Uint can be used for `enum`.
//!
//! ## `[length]type`, `data<length>`
//! `[T; N]`.
//!
//! ## `(type | type | ...)`
//! Rust enums, with or without fields are represented as tagged unions in BARE.
//! If the enum has no fields, it can be represented as an integer with `serde_repr`.

pub mod de;
pub mod error;
pub mod ser;

#[doc(inline)]
pub use de::{from_reader, from_slice, Deserializer};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_vec, to_writer, Serializer};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Uint(pub u64);

impl Default for Uint {
    fn default() -> Uint {
        Uint(0)
    }
}

impl serde::ser::Serialize for Uint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeTuple;

        let Uint(mut x) = *self;
        let mut buf = [0u8; 10];
        let mut i = 0usize;
        while x >= 0x80 {
            buf[i] = (x as u8) | 0x80;
            x >>= 7;
            i += 1;
        }
        buf[i] = x as u8;
        i += 1;

        let mut s = serializer.serialize_tuple(usize::MAX)?;
        for j in 0..i {
            s.serialize_element(&buf[j])?;
        }
        s.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for Uint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use std::fmt;

        struct UintVisitor;
        impl<'de> serde::de::Visitor<'de> for UintVisitor {
            type Value = Uint;

            fn expecting(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
                unreachable!()
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut x = 0u64;
                let mut s = 0usize;
                for i in 0.. {
                    // SeqAccess::next_element should never return None
                    let b = seq.next_element::<u8>()?.unwrap();
                    if b < 0x80 {
                        if i > 9 || i == 9 && b > 1 {
                            todo!("error out");
                        }
                        return Ok(Uint(x | (b as u64) << s));
                    }
                    x |= ((b & 0x7f) as u64) << s;
                    s += 7;
                }
                unreachable!()
            }
        }
        deserializer.deserialize_tuple(usize::MAX, UintVisitor)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Int(pub i64);

impl Default for Int {
    fn default() -> Int {
        Int(0)
    }
}

impl serde::ser::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let Int(x) = *self;
        let mut ux = (x as u64) << 1;
        if x < 0 {
            ux = !ux;
        }
        Uint(ux).serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let Uint(ux) = <Uint as serde::de::Deserialize>::deserialize(deserializer)?;
        let mut x = (ux >> 1) as i64;
        if ux & 1 != 0 {
            x = !x;
        }
        Ok(Int(x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int() {
        const CASES: &'static [(i64, &'static [u8])] = &[
            (0, &[0]),
            (1, &[2]),
            (i64::MIN, &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]),
            (i64::MAX, &[254, 255, 255, 255, 255, 255, 255, 255, 255, 1]),
        ];
        for &(n, bytes) in CASES {
            println!("testing {}", n);
            let int = Int(n);
            let got_bytes = to_vec(&int).unwrap();
            assert_eq!(got_bytes, bytes);
            let got_int = from_slice::<Int>(&got_bytes).unwrap();
            assert_eq!(got_int, int);
        }
    }

    #[test]
    fn test_uint() {
        const CASES: &'static [(u64, &'static [u8])] = &[
            (0, &[0]),
            (1, &[1]),
            (275, &[147, 2]),
            (
                18446744073709551615,
                &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1],
            ),
        ];
        for &(n, bytes) in CASES {
            println!("testing {}", n);
            let int = Uint(n);
            let got_bytes = to_vec(&int).unwrap();
            assert_eq!(got_bytes, bytes);
            let got_int = from_slice::<Uint>(&got_bytes).unwrap();
            assert_eq!(got_int, int);
        }
    }
}
