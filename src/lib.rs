#![forbid(unsafe_code)]
//!```text
//! serde_bare
//!
//! An implementation of the BARE (https://git.sr.ht/~sircmpwn/bare) encoding format draft.
//!```

pub mod de;
pub mod error;
pub mod ser;

#[doc(inline)]
pub use de::{from_reader, from_slice, Deserializer};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_vec, to_writer, Serializer};
