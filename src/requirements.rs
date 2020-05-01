use crate::config::Attribute;
use crate::config::Config;
use roxmltree::Document;
use roxmltree::ExpandedName;
use std::collections::BTreeMap;

pub type Requirements<'a> = BTreeMap<&'a str, Vec<ResolvedName<'a>>>;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Rule;

    #[test]
    fn empty_config() -> Result<(), roxmltree::Error> {
        let config = BTreeMap::new();

        let doc = Document::parse(
            r#"
            <LinearLayout
                xmlns:android="http://schemas.android.com/apk/res/android"
                android:layout_width="match_parent"
                android:layout_height="match_parent" />
            "#,
        )?;

        let reqs = resolve(&config, &doc);

        assert!(reqs.is_empty());

        Ok(())
    }

    #[test]
    fn resolve_name_with_ns() -> Result<(), Box<dyn std::error::Error>> {
        let config = vec![(
            "EditText".to_string(),
            Rule {
                required: vec!["android:hint".parse()?],
            },
        )]
        .into_iter()
        .collect();

        let doc = Document::parse(
            r#"
            <LinearLayout
                xmlns:android="http://schemas.android.com/apk/res/android"
                android:layout_width="match_parent"
                android:layout_height="match_parent" />
            "#,
        )?;

        let reqs = resolve(&config, &doc);

        assert_eq!(1, reqs.keys().len());
        assert_eq!(
            vec![ResolvedName {
                raw: "android:hint".to_string(),
                expanded: ("http://schemas.android.com/apk/res/android", "hint").into()
            }],
            reqs["EditText"]
        );

        Ok(())
    }

    #[test]
    fn resolve_name_without_ns() -> Result<(), Box<dyn std::error::Error>> {
        let config = vec![(
            "Button".to_string(),
            Rule {
                required: vec!["style".parse()?],
            },
        )]
        .into_iter()
        .collect();

        let doc = Document::parse(
            r#"
            <LinearLayout
                xmlns:android="http://schemas.android.com/apk/res/android"
                android:layout_width="match_parent"
                android:layout_height="match_parent" />
            "#,
        )?;

        let reqs = resolve(&config, &doc);

        assert_eq!(1, reqs.keys().len());
        assert_eq!(
            vec![ResolvedName {
                raw: "style".to_string(),
                expanded: ("style").into()
            }],
            reqs["Button"]
        );

        Ok(())
    }
}
