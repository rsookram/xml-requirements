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

struct Attribute<'a>((Option<&'a str>, &'a str));

fn main() {
    let opt = Opt::from_args();

    let conf_str = fs::read_to_string(&opt.config).unwrap();
    let config = conf_str.parse::<toml::Value>().unwrap();

    let raw_requirements: Vec<(&str, Attribute)> = config["required"]
        .as_table()
        .unwrap()
        .iter()
        .flat_map(|(tag, value)| {
            let values = value.as_table().unwrap();

            values["attributes"]
                .as_array()
                .unwrap()
                .iter()
                .map(move |v| (tag, v.as_str().unwrap()))
        })
        .map(|(tag, attr)| {
            let mut parts = attr.rsplitn(2, ':');
            let name = parts.next().unwrap();
            let ns = parts.next();

            (tag.as_str(), Attribute((ns, name)))
        })
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

        let requirements: BTreeMap<&str, Attribute> = raw_requirements
            .iter()
            .map(|(tag, attr)| {
                let Attribute((ns, name)) = attr;
                let ns = ns.and_then(|ns| {
                    let resolved = namespaces.get(ns).copied();

                    resolved.or(Some(ns))
                });

                (*tag, Attribute((ns, name)))
            })
            .collect();

        doc.descendants()
            .filter_map(|n| {
                if let Some(attr) = requirements.get(n.tag_name().name()) {
                    let ex_name = to_expanded_name(attr);

                    if n.has_attribute(ex_name) {
                        None
                    } else {
                        Some((n, ex_name))
                    }
                } else {
                    None
                }
            })
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

// TODO: impl From?
fn to_expanded_name<'a>(attr: &'a Attribute) -> ExpandedName<'a> {
    let Attribute((ns, name)) = attr;

    match ns {
        Some(ns) => ExpandedName::from((*ns, *name)),
        None => ExpandedName::from(*name),
    }
}
