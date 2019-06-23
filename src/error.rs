use std::convert::TryInto;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    JWTError(jwt::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError(err) => err.fmt(f),
            Error::JWTError(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IOError(err) => err.source(),
            Error::JWTError(err) => err.source(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<jwt::Error> for Error {
    fn from(err: jwt::Error) -> Error {
        Error::JWTError(err)
    }
}

impl TryInto<io::Error> for Error {
    type Error = ();
    fn try_into(self) -> Result<io::Error, Self::Error> {
        match self {
            Error::IOError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<jwt::Error> for Error {
    type Error = ();
    fn try_into(self) -> Result<jwt::Error, Self::Error> {
        match self {
            Error::JWTError(err) => Ok(err),
            _ => Err(()),
        }
    }
}