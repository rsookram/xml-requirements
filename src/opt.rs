use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "xml-requirements", author, about)]
pub struct Opt {
    /// Path to TOML configuration file
    #[structopt(short, long, parse(from_os_str))]
    pub config: PathBuf,

    /// Path of XML files to check
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
