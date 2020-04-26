mod config;
mod violation;

use config::Attribute;
use config::Rule;
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

    let conf_str = fs::read_to_string(&opt.config).unwrap();
    let config = config::from_str(&conf_str);

    let mut meets_requirements = true;
    for path in &opt.files {
        let content = fs::read_to_string(&path).unwrap();

        let doc = Document::parse(&content).unwrap();

        let requirements = get_requirements(&config, &doc);

        doc.descendants()
            .flat_map(|n| find_violations(path, &n, &requirements))
            .for_each(|violation| {
                meets_requirements = false;

                println!("{}", violation)
            });
    }

    if !meets_requirements {
        std::process::exit(1);
    }
}

fn get_requirements<'a>(
    config: &'a BTreeMap<String, Rule>,
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
    let n_start = node.range().start;

    requirements
        .get(tag)
        .into_iter()
        .flatten()
        .filter_map(move |name| {
            if node.has_attribute(name.expanded) {
                None
            } else {
                let pos = node.document().text_pos_at(n_start);

                Some(Violation::new(path, pos.row, pos.col, tag, &name.raw))
            }
        })
        .collect()
}
