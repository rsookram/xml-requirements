use roxmltree::Document;
use roxmltree::ExpandedName;
use std::collections::BTreeMap;

struct Attribute<'a>((Option<&'a str>, &'a str));

fn main() {
    // TODO: Use structopt
    let mut args = std::env::args();
    args.next();

    let paths = args;

    // TODO: Parse from config, use something like TOML
    let raw_requirements: Vec<(&str, Attribute)> = vec![("LinearLayout", "android:orientation")]
        .iter()
        .map(|(tag, attr)| {
            let mut parts = attr.rsplitn(2, ':');
            let name = parts.next().unwrap();
            let ns = parts.next();

            (*tag, Attribute((ns, name)))
        })
        .collect();

    let mut meets_requirements = true;
    for path in paths {
        let content = std::fs::read_to_string(&path).unwrap();

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
                    path,
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
