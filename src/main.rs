mod config;

use config::Attribute;
use roxmltree::Document;
use roxmltree::ExpandedName;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

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

    let raw_requirements: Vec<(&str, &Attribute)> = config
        .required
        .iter()
        .flat_map(|(tag, req)| req.attributes.iter().map(move |v| (tag, v)))
        .map(|(tag, attr)| (tag.as_str(), attr))
        .collect();

    let mut meets_requirements = true;
    for path in opt.files {
        let content = fs::read_to_string(&path).unwrap();

        let doc = Document::parse(&content).unwrap();

        let namespaces: BTreeMap<_, _> = doc
            .root_element()
            .namespaces()
            .iter()
            .filter_map(|ns| ns.name().map(|name| (name, ns.uri())))
            .collect();

        let mut requirements = BTreeMap::new();
        raw_requirements
            .iter()
            .map(|(tag, attr)| (*tag, resolve(attr, &namespaces)))
            .for_each(|(tag, attr)| {
                let attrs = requirements.entry(tag).or_insert_with(|| vec![]);
                attrs.push(attr);
            });

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
            .for_each(|(n, attr)| {
                meets_requirements = false;

                println!(
                    "{}:{} {} missing {} attribute",
                    path.to_str().unwrap(),
                    doc.text_pos_at(n.range().start),
                    n.tag_name().name(),
                    attr.name()
                )
            });
    }

    if !meets_requirements {
        std::process::exit(1);
    }
}

fn resolve<'a>(attr: &'a Attribute, namespaces: &'a BTreeMap<&str, &str>) -> ExpandedName<'a> {
    let ns = attr.ns.as_ref().and_then(|ns| {
        let resolved = namespaces.get(ns.as_str()).copied();

        resolved.or(Some(&ns))
    });

    let name = attr.name.as_str();
    match ns {
        Some(ns) => ExpandedName::from((ns, name)),
        None => ExpandedName::from(name),
    }
}
