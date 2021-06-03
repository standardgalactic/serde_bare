/// A facade around the various collections and primitives needed
/// to support "std" and "no_std + alloc" targets.

// std::boxed
pub mod boxed {
    #[cfg(feature = "std")]
    pub use std::boxed::Box;
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use alloc::boxed::Box;
}

// std::error::Error trait
pub mod error {
    #[cfg(feature = "std")]
    pub use std::error::Error;
    #[cfg(not(feature = "std"))]
    pub trait Error: core::fmt::Debug + core::fmt::Display {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}

// std::io
#[cfg(feature = "std")]
pub use std::io;
#[cfg(not(feature = "std"))]
pub use core2::io;

// std::string
pub mod string {
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString};
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use alloc::string::{String, ToString};
}

// std::vec
pub mod vec {
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use alloc::vec::Vec;
}
