use std::fmt;
use std::path::PathBuf;

type Cause = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum Error {
    ReadConfig(PathBuf, Cause),
    ParseConfig(PathBuf, Cause),
    ReadXML(PathBuf, Cause),
    ParseXML(PathBuf, Cause),
    WriteOutput(Cause),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ReadConfig(path, cause) => {
                write_error(f, &format!("Failed to read `{}`", path.display()), cause)
            }
            Error::ParseConfig(path, cause) => write_error(
                f,
                &format!("Failed to parse config `{}`", path.display()),
                cause,
            ),
            Error::ReadXML(path, cause) => write_error(
                f,
                &format!("Failed to read XML file `{}`", path.display()),
                cause,
            ),
            Error::ParseXML(path, cause) => write_error(
                f,
                &format!("Failed to parse XML file `{}`", path.display()),
                cause,
            ),
            Error::WriteOutput(cause) => write_error(f, "Failed to write output", cause),
        }
    }
}

fn write_error(f: &mut fmt::Formatter, msg: &str, cause: &Cause) -> fmt::Result {
    write!(f, "Error: {}\n\nCause: {}", msg, cause)
}
