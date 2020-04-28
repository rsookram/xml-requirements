use std::fmt;
use std::path::PathBuf;

type Cause = String;

#[derive(Debug)]
pub enum Error {
    ReadConfig(PathBuf, Cause),
    ParseConfig(PathBuf, Cause),
    ReadXML(PathBuf, Cause),
    ParseXML(PathBuf, Cause),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadConfig(path, cause) => write!(
                f,
                "Error: Failed to read {}: {}",
                path.to_string_lossy(),
                cause
            ),
            Error::ParseConfig(path, cause) => write!(
                f,
                "Error: Failed to parse config {}: {}",
                path.to_string_lossy(),
                cause
            ),
            Error::ReadXML(path, cause) => write!(
                f,
                "Error: Failed to read XML file {}: {}",
                path.to_string_lossy(),
                cause
            ),
            Error::ParseXML(path, cause) => write!(
                f,
                "Error: Failed to parse XML file {}: {}",
                path.to_string_lossy(),
                cause
            ),
        }
    }
}
