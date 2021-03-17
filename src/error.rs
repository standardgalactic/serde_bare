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
    InvalidChar,

    SequenceLengthRequired,
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
            Error::InvalidUtf8 => formatter.write_str("invalid utf-8 in string"),
            Error::InvalidChar => formatter.write_str("invalid unicode codepoint in char"),
            Error::SequenceLengthRequired => formatter.write_str("sequence length required"),
            Error::MapLengthRequired => formatter.write_str("map length required"),
        }
    }
}

impl std::error::Error for Error {}
