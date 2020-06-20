#![forbid(unsafe_code)]
//!```text
//! serde_bare
//!
//! An incomplete implementation of the BARE (https://git.sr.ht/~sircmpwn/bare) encoding format.
//!
//! Missing types:
//!     - f16 (in BARE, not in Serde's data model)
//!     - i128 (in Serde's data model, not in BARE)
//!     - u128 (in Serde's data model, not in BARE)
//!     - tagged unions (in BARE, not immediately clear how to integrate with Serde's data model)
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
