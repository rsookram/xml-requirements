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

    doc.descendants()
        .filter(|n| {
            if let Some((ns, name)) = requirements.get(n.tag_name().name()) {
                let ex_name = match ns {
                    Some(ns) => ExpandedName::from((*ns, *name)),
                    None => ExpandedName::from(*name),
                };

                !n.has_attribute(ex_name)
            } else {
                false
            }
        })
        .for_each(|n| {
            println!(
                "{}:{} {:?} missing ...", // TODO: Include missing attribute name
                path,
                doc.text_pos_at(n.range().start),
                n.tag_name()
            )
        });
}
