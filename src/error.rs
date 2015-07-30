use toml;

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum YAPCCError {
    IOError(io::Error),
    DecodeError(toml::DecodeError),
    GenericError
}

impl Display for YAPCCError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            YAPCCError::IOError(ref e) => write!(f, "{}", e),
            YAPCCError::DecodeError(ref e) => write!(f, "{}", e),
            YAPCCError::GenericError => write!(f, "Generic error")
        }
    }
}

pub type YAPCCResult<T> = Result<T, YAPCCError>;

macro_rules! from_impls {
    ($($err:ty => $name:ident),+) => {$(
        impl From<$err> for YAPCCError {
            fn from(e: $err) -> YAPCCError {
                YAPCCError::$name(e)
            }
        }
    )*}
}

from_impls! {
    io::Error => IOError,
    toml::DecodeError => DecodeError
}
