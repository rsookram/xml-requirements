use serde::Deserialize;
use serde::Deserializer;
use std::collections::BTreeMap;

pub type Config = BTreeMap<String, Rule>;

pub fn from_str(s: &str) -> Result<Config, impl std::error::Error> {
    toml::from_str(s)
}

#[derive(Deserialize)]
pub struct Rule {
    #[serde(deserialize_with = "vec_attribute")]
    pub required: Vec<Attribute>,
}

pub struct Attribute {
    pub ns: Option<String>,
    pub name: String,
    pub raw: String,
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
    let s = String::deserialize(deserializer)?;

    let mut parts = s.rsplitn(2, ':');
    let name = parts.next().expect("not possible");
    let ns = parts.next().map(std::string::ToString::to_string);

    Ok(Attribute {
        ns,
        name: name.to_string(),
        raw: s.to_string(),
    })
}
