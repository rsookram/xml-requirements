mod config;
mod violation;

use config::Attribute;
use roxmltree::Document;
use roxmltree::ExpandedName;
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
    for path in opt.files {
        let content = fs::read_to_string(&path).unwrap();

        let doc = Document::parse(&content).unwrap();

        let requirements: BTreeMap<_, _> = config
            .required
            .iter()
            .map(|(tag, req)| {
                let names: Vec<_> = req
                    .attributes
                    .iter()
                    .map(|attr| resolve(attr, &doc))
                    .collect();

                (tag.to_string(), names)
            })
            .collect();

        doc.descendants()
            .filter_map(|n| {
                if let Some(attrs) = requirements.get(n.tag_name().name()) {
                    Some((n, attrs))
                } else {
                    None
                }
            })
            .flat_map(|(n, attrs)| attrs.iter().map(move |attr| (n, attr)))
            .filter(|(n, &ex_name)| !n.has_attribute(ex_name))
            .map(|(n, attr)| Violation::new(&path, &doc, &n, attr.name()))
            .for_each(|violation| {
                meets_requirements = false;

                println!("{}", violation)
            });
    }

    if !meets_requirements {
        std::process::exit(1);
    }
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
