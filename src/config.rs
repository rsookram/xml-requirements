use serde::Deserialize;
use serde::Deserializer;
use std::collections::HashMap;

pub fn from_str(s: &str) -> Config {
    toml::from_str(s).unwrap()
}

#[derive(Deserialize)]
pub struct Config {
    pub required: HashMap<String, Requirement>,
}

#[derive(Deserialize)]
pub struct Requirement {
    #[serde(deserialize_with = "vec_attribute")]
    pub attributes: Vec<Attribute>,
}

pub struct Attribute {
    pub ns: Option<String>,
    pub name: String,
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
    let name = parts.next().unwrap();
    let ns = parts.next().map(std::string::ToString::to_string);

    Ok(Attribute {
        ns,
        name: name.to_string(),
    })
}
