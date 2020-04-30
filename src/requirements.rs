use crate::config::Attribute;
use crate::config::Config;
use roxmltree::Document;
use roxmltree::ExpandedName;
use std::collections::BTreeMap;

pub type Requirements<'a> = BTreeMap<&'a str, Vec<ResolvedName<'a>>>;

pub struct ResolvedName<'a> {
    pub raw: String,
    pub expanded: ExpandedName<'a>,
}

pub fn resolve<'a>(config: &'a Config, doc: &'a Document) -> Requirements<'a> {
    config
        .iter()
        .map(|(tag, rule)| {
            let names: Vec<_> = rule
                .required
                .iter()
                .map(|attr| resolve_attr(attr, doc))
                .collect();

            (tag.as_str(), names)
        })
        .collect()
}

fn resolve_attr<'a>(attr: &'a Attribute, doc: &'a Document) -> ResolvedName<'a> {
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
