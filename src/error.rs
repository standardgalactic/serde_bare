use serde::{de, ser};
use std::{
    fmt::{self, Display},
    io,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),

    AnyUnsupported,
    I128Unsupported,
    U128Unsupported,
    IdentifierUnsupported,

    InvalidUtf8,

    LengthOverflow,
    MapLengthRequired,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Io(e) => e.fmt(formatter),
            Error::AnyUnsupported => formatter.write_str("BARE does not support any"),
            Error::U128Unsupported => formatter.write_str("Bare does not support u128"),
            Error::I128Unsupported => formatter.write_str("Bare does not support i128"),
            Error::IdentifierUnsupported => formatter.write_str("BARE does not support identifier"),
            Error::InvalidUtf8 => formatter.write_str("invalid utf-8 in string"),
            Error::LengthOverflow => formatter.write_str("length above u32::MAX"),
            Error::MapLengthRequired => formatter.write_str("map length required"),
        }
    }
}

impl std::error::Error for Error {}
