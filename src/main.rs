mod config;
mod violation;

use config::Attribute;
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

fn main() {
    let opt = Opt::from_args();

    let conf_str = fs::read_to_string(&opt.config).unwrap();
    let config = config::from_str(&conf_str);

    let mut meets_requirements = true;
    for path in &opt.files {
        let content = fs::read_to_string(&path).unwrap();

        let doc = Document::parse(&content).unwrap();

        let requirements: BTreeMap<_, _> = config
            .iter()
            .map(|(tag, rule)| {
                let names: Vec<_> = rule
                    .required
                    .iter()
                    .map(|attr| resolve(attr, &doc))
                    .collect();

                (tag.as_str(), names)
            })
            .collect();

        doc.descendants()
            .flat_map(|n| find_violations(path, &doc, &n, &requirements))
            .for_each(|violation| {
                meets_requirements = false;

                println!("{}", violation)
            });
    }

    if !meets_requirements {
        std::process::exit(1);
    }
}

fn find_violations(
    path: &PathBuf,
    doc: &Document,
    node: &Node,
    requirements: &BTreeMap<&str, Vec<ExpandedName>>,
) -> Vec<Violation> {
    let tag = node.tag_name().name();
    let n_start = node.range().start;

    requirements
        .get(tag)
        .into_iter()
        .flatten()
        .filter_map(move |&ex_name| {
            if node.has_attribute(ex_name) {
                None
            } else {
                let pos = doc.text_pos_at(n_start);

                Some(Violation::new(path, pos.row, pos.col, tag, ex_name.name()))
            }
        })
        .collect()
}

fn resolve<'a>(attr: &'a Attribute, doc: &'a Document) -> ExpandedName<'a> {
    let ns = attr
        .ns
        .as_ref()
        .and_then(|ns| doc.root_element().lookup_namespace_uri(Some(ns)));

    let name = attr.name.as_str();
    match ns {
        Some(ns) => ExpandedName::from((ns, name)),
        None => ExpandedName::from(name),
    }
}
