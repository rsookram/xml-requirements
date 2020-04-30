mod config;
mod error;
mod violation;

use config::Attribute;
use config::Config;
use error::Error;
use roxmltree::Document;
use roxmltree::ExpandedName;
use roxmltree::Node;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use violation::Violation;

#[derive(StructOpt, Debug)]
#[structopt(name = "xml-requirements")]
struct Opt {
    /// Configuration file
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,

    /// XML files to check
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

struct ResolvedName<'a> {
    raw: String,
    expanded: ExpandedName<'a>,
}

fn main() {
    let opt = Opt::from_args();

    match run(&opt) {
        Ok(meets_requirements) => {
            if !meets_requirements {
                std::process::exit(1)
            }
        }
        Err(err) => eprintln!("{}", err),
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

        let requirements = get_requirements(&config, &doc);

        doc.descendants()
            .flat_map(|n| find_violations(path, &n, &requirements))
            .for_each(|violation| {
                meets_requirements = false;

                println!("{}", violation)
            });
    }

    Ok(meets_requirements)
}

fn get_requirements<'a>(
    config: &'a Config,
    doc: &'a Document,
) -> BTreeMap<&'a str, Vec<ResolvedName<'a>>> {
    config
        .iter()
        .map(|(tag, rule)| {
            let names: Vec<_> = rule
                .required
                .iter()
                .map(|attr| resolve(attr, doc))
                .collect();

            (tag.as_str(), names)
        })
        .collect()
}

fn resolve<'a>(attr: &'a Attribute, doc: &'a Document) -> ResolvedName<'a> {
    let ns = attr
        .ns
        .as_ref()
        .and_then(|ns| doc.root_element().lookup_namespace_uri(Some(ns)));

    let name = attr.name.as_str();
    let expanded = match ns {
        Some(ns) => ExpandedName::from((ns, name)),
        None => ExpandedName::from(name),
    };

    ResolvedName {
        raw: attr.raw.to_string(),
        expanded,
    }
}

fn find_violations(
    path: &PathBuf,
    node: &Node,
    requirements: &BTreeMap<&str, Vec<ResolvedName>>,
) -> Vec<Violation> {
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
