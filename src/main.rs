use roxmltree::Document;
use roxmltree::ExpandedName;
use std::collections::BTreeMap;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("missing required path to XML file");

    let content = std::fs::read_to_string(&path).unwrap();

    let doc = Document::parse(&content).unwrap();

    let namespaces: BTreeMap<_, _> = doc
        .root_element()
        .namespaces()
        .iter()
        .filter_map(|ns| ns.name().map(|name| (name, ns.uri())))
        .collect();

    let requirements: BTreeMap<_, _> = vec![("LinearLayout", "android:orientation")]
        .into_iter()
        .map(|(tag, attr)| {
            let mut parts = attr.rsplitn(2, ':');
            let name = parts.next().unwrap();
            let ns = parts.next().and_then(|ns| {
                let resolved = namespaces.get(ns).copied();

                resolved.or(Some(ns))
            });

            (tag, (ns, name))
        })
        .collect();

    let mut meets_requirements = true;
    doc.descendants()
        .filter_map(|n| {
            if let Some((ns, name)) = requirements.get(n.tag_name().name()) {
                let ex_name = match ns {
                    Some(ns) => ExpandedName::from((*ns, *name)),
                    None => ExpandedName::from(*name),
                };

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
                "{}:{} {:?} missing {} attribute",
                path,
                doc.text_pos_at(n.range().start),
                n.tag_name(),
                attr.name()
            )
        });

    if !meets_requirements {
        std::process::exit(1);
    }
}
