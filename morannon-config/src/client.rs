use crate::model::ModelId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ClientId(String);

impl From<&str> for ClientId {
    fn from(value: &str) -> Self {
        ClientId(String::from(value))
    }
}

impl From<String> for ClientId {
    fn from(value: String) -> Self {
        ClientId(value)
    }
}

impl From<ClientId> for String {
    fn from(value: ClientId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientConfiguration {
    pub key: String,
    pub models: Vec<ModelId>,
}
