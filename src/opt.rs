use pico_args::Arguments;
use std::convert::From;
use std::convert::Infallible;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
pub struct Opt {
    /// Path to TOML configuration file
    pub config: PathBuf,

    /// Path of XML files to check
    pub files: Vec<PathBuf>,
}

impl Opt {
    /// Gets [Opt] from the command line arguments. Prints the error message
    /// and quits the program in case of failure.
    pub fn from_args() -> Self {
        let mut args = Arguments::from_env();

        if args.contains(["-h", "--help"]) {
            print_help();
            process::exit(0);
        }

        if args.contains(["-V", "--version"]) {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }

        Self::parse(args).unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        })
    }

    fn parse(mut args: Arguments) -> Result<Self, Error> {
        let result = Self {
            config: Self::parse_config(&mut args)?,
            files: args.free_os()?.iter().map(|s| s.into()).collect(),
        };

        Ok(result)
    }

    fn parse_config(args: &mut Arguments) -> Result<PathBuf, Error> {
        let into_path_buf = |s: &OsStr| Ok::<_, Infallible>(s.into());

        if args.contains("--config") && args.contains("-c") {
            return Err(Error::DuplicateArg("--config".to_string()));
        }

        let long = args.opt_value_from_os_str("--config", into_path_buf)?;

        match long {
            Some(p) => Ok(p),
            None => Ok(args.value_from_os_str("-c", into_path_buf)?),
        }
    }
}

#[derive(Debug)]
enum Error {
    DuplicateArg(String),
    Parse(pico_args::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DuplicateArg(arg) => writeln!(
                f,
                "The argument '{}' was provided more than once, but cannot be used multiple times",
                arg
            ),
            Error::Parse(err) => writeln!(f, "{}", err),
        }
    }
}

impl From<pico_args::Error> for Error {
    fn from(err: pico_args::Error) -> Self {
        Error::Parse(err)
    }
}

fn print_help() {
    println!(
        r#"{name} {version}
Command-line tool to lint XML files based on the supplied configuration

USAGE:
    {name} --config <config> [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    Path to TOML configuration file

ARGS:
    <FILE>...    Path of XML files to check"#,
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    );
}
