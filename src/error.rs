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
    InvalidUtf8,
    InvalidBool,
    IdentifierUnsupported,
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
            Error::AnyUnsupported => {
                formatter.write_str("any is unsupported because BARE is not self-describing")
            }
            Error::InvalidUtf8 => formatter.write_str("invalid utf-8 in string"),
            Error::InvalidBool => formatter.write_str("invalid bool, neither 0 or 1"),
            Error::IdentifierUnsupported => formatter.write_str("identifier is not supported"),
            Error::LengthOverflow => formatter.write_str("length above u32::MAX"),
            Error::MapLengthRequired => formatter.write_str("map length required"),
        }
    }
}

impl std::error::Error for Error {}
