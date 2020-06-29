use std::convert::From;
use std::fmt;

/// An error when retrieving or parsing command line arguments
#[derive(Debug)]
pub enum Error {
    DuplicateArg(String),
    Parse(pico_args::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DuplicateArg(arg) => write!(
                f,
                "The argument '{}' was provided more than once, but cannot be used multiple times",
                arg
            ),
            Error::Parse(err) => write!(f, "{}", err),
        }
    }
}

impl From<pico_args::Error> for Error {
    fn from(err: pico_args::Error) -> Self {
        Error::Parse(err)
    }
}
