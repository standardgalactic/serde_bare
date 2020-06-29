#![forbid(unsafe_code)]
//! # serde_bare
//!
//! An implementation of the BARE (https://git.sr.ht/~sircmpwn/bare) encoding format draft.
//!
//! ## `u8`/`u16`/`u32`/`u64`, `i8`/`i16`/`i32`/`i64`, `f32`/`f64`, `bool`, `string`, `data`, `optional`, `[]type`, `map`, and `struct`
//!
//! Mapped exactly.
//!
//! ## `u128`, `i128`
//!
//! Encoded in the same fashion as the other integers, but the type is data<16>.
//!
//! ## `[length]type`, `data<length>`
//! Can be used as a `[T; N]` where Serde has an implementation for it, or manually with deserialize_tuple.
//!
//! ## `(type | type | ...)`
//! Cannot be derived as an enum directly, but can be a derived `{ u8, T }` or a custom Deserialize implementation on an enum.
//!
//! ## Enum representation
//!
//! Rust enums are serialized as `{ u32, fields }` by default.  
//! For enums without fields, this can be derived differently with `serde_repr`.  
//! For enums with fields, this can be overridden with `{ uN, struct }` representation or with a custom Deserialize implementation.  

pub mod de;
pub mod error;
pub mod ser;

#[doc(inline)]
pub use de::{from_reader, from_slice, Deserializer};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_vec, to_writer, Serializer};
