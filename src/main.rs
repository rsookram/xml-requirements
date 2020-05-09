mod config;
mod error;
mod requirements;
mod violation;

use error::Error;
use requirements::Requirements;
use roxmltree::Document;
use roxmltree::Node;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use violation::Violation;

#[derive(StructOpt, Debug)]
#[structopt(name = "xml-requirements", author, about)]
struct Opt {
    /// Path to TOML configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,

    /// Path of XML files to check
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    match run(&opt, &mut stdout) {
        Ok(meets_requirements) => {
            if !meets_requirements {
                std::process::exit(1)
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(2);
        }
    }
}

fn run(opt: &Opt, mut w: impl Write) -> Result<bool, Error> {
    let conf_str = fs::read_to_string(&opt.config)
        .map_err(|err| Error::ReadConfig(opt.config.clone(), Box::new(err)))?;
    let config = config::from_str(&conf_str)
        .map_err(|err| Error::ParseConfig(opt.config.clone(), Box::new(err)))?;

    let mut meets_requirements = true;
    for path in &opt.files {
        let content =
            fs::read_to_string(&path).map_err(|err| Error::ReadXML(path.clone(), Box::new(err)))?;

        let doc = Document::parse(&content)
            .map_err(|err| Error::ParseXML(path.clone(), Box::new(err)))?;

        let requirements = requirements::resolve(&config, &doc);

        let result = doc
            .descendants()
            .flat_map(|n| find_violations(path, &n, &requirements))
            .try_for_each(|violation| {
                meets_requirements = false;

                writeln!(w, "{}", violation)
            });

        match result {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => {
                if !meets_requirements {
                    // Return early since nothing more can be printed and
                    // violations have already been found
                    return Ok(false);
                }
            }
            Err(e) => return Err(Error::WriteOutput(Box::new(e))),
        }
    }

    Ok(meets_requirements)
}

fn find_violations(path: &PathBuf, node: &Node, requirements: &Requirements) -> Vec<Violation> {
    let tag = node.tag_name().name();

    requirements
        .get(tag)
        .into_iter()
        .flatten()
        .filter_map(|name| {
            if node.has_attribute(name.expanded) {
                None
            } else {
                let start = node.range().start;
                let pos = node.document().text_pos_at(start);

                Some(Violation::new(path, pos.row, pos.col, tag, &name.raw))
            }
        })
        .collect()
}
