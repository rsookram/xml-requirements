mod config;
mod error;
mod requirements;
mod violation;

use error::Error;
use requirements::Requirements;
use roxmltree::Document;
use roxmltree::Node;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use violation::Violation;

#[derive(StructOpt, Debug)]
#[structopt(name = "xml-requirements", author, about)]
struct Opt {
    /// Path to toml configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,

    /// Path of XML files to check
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    match run(&opt) {
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

fn run(opt: &Opt) -> Result<bool, Error> {
    let conf_str = fs::read_to_string(&opt.config)
        .map_err(|err| Error::ReadConfig(opt.config.clone(), err.to_string()))?;
    let config = config::from_str(&conf_str)
        .map_err(|err| Error::ParseConfig(opt.config.clone(), err.to_string()))?;

    let mut meets_requirements = true;
    for path in &opt.files {
        let content = fs::read_to_string(&path)
            .map_err(|err| Error::ReadXML(path.clone(), err.to_string()))?;

        let doc = Document::parse(&content)
            .map_err(|err| Error::ParseXML(path.clone(), err.to_string()))?;

        let requirements = requirements::resolve(&config, &doc);

        doc.descendants()
            .flat_map(|n| find_violations(path, &n, &requirements))
            .for_each(|violation| {
                meets_requirements = false;

                println!("{}", violation)
            });
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
