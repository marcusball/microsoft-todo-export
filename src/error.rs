use std::convert::From;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    IOError(::std::io::Error),

    ReqwestError(::reqwest::Error),
}

impl ::std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IOError(e) => Some(e),
            Self::ReqwestError(e) => Some(e),
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        Error::IOError(e)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(e: ::reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}


