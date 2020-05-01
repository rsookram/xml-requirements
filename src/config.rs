use serde::Deserialize;
use serde::Deserializer;
use std::collections::BTreeMap;
use std::str::FromStr;

pub type Config = BTreeMap<String, Rule>;

pub fn from_str(s: &str) -> Result<Config, impl std::error::Error> {
    toml::from_str(s)
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rule {
    #[serde(deserialize_with = "vec_attribute")]
    pub required: Vec<Attribute>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub ns: Option<String>,
    pub name: String,
    pub raw: String,
}

impl FromStr for Attribute {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.rsplitn(2, ':');
        let name = parts.next().expect("not possible");
        let ns = parts.next().map(std::string::ToString::to_string);

        Ok(Attribute {
            ns,
            name: name.to_string(),
            raw: s.to_string(),
        })
    }
}

fn vec_attribute<'de, D>(deserializer: D) -> Result<Vec<Attribute>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "attribute")] Attribute);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

pub fn attribute<'de, D>(deserializer: D) -> Result<Attribute, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let s = String::deserialize(deserializer)?;

    s.parse().map_err(Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_config() -> Result<(), Box<dyn std::error::Error>> {
        let config_str = "";

        let config = from_str(config_str)?;

        assert!(config.is_empty());

        Ok(())
    }

    #[test]
    fn tag_with_no_requirements() -> Result<(), Box<dyn std::error::Error>> {
        let config_str = r#"
            [LinearLayout]
            required = []
        "#;

        let config = from_str(config_str)?;

        assert_eq!(new_config(vec![("LinearLayout", vec![])]), config);

        Ok(())
    }

    #[test]
    fn tag_with_requirement() -> Result<(), Box<dyn std::error::Error>> {
        let config_str = r#"
            [LinearLayout]
            required = [ "android:orientation" ]
        "#;

        let config = from_str(config_str)?;

        assert_eq!(
            new_config(vec![("LinearLayout", vec!["android:orientation"])]),
            config
        );

        Ok(())
    }

    #[test]
    fn multiple_tags_multiple_requirements() -> Result<(), Box<dyn std::error::Error>> {
        let config_str = r#"
            [LinearLayout]
            required = [ "android:orientation", "tools:elevation" ]

            [EditText]
            required = [ "android:hint", "style" ]
        "#;

        let config = from_str(config_str)?;

        assert_eq!(
            new_config(vec![
                (
                    "LinearLayout",
                    vec!["android:orientation", "tools:elevation"]
                ),
                ("EditText", vec!["android:hint", "style"]),
            ]),
            config
        );

        Ok(())
    }

    fn new_config(tag_rules: Vec<(&str, Vec<&str>)>) -> Config {
        tag_rules
            .into_iter()
            .map(|(tag, attrs)| {
                (
                    tag.to_string(),
                    Rule {
                        required: attrs.into_iter().map(|s| s.parse().unwrap()).collect(),
                    },
                )
            })
            .collect()
    }
}
