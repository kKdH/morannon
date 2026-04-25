use crate::provider::ProviderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ModelId(String);

impl From<&str> for ModelId {
    fn from(value: &str) -> Self {
        ModelId(String::from(value))
    }
}

impl From<String> for ModelId {
    fn from(value: String) -> Self {
        ModelId(value)
    }
}

impl From<ModelId> for String {
    fn from(value: ModelId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ModelConfiguration {
    pub provider: ProviderId,
    pub upstream_name: String,
    pub downstream_name: String,
}
