#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ProviderId(String);

impl From<&str> for ProviderId {
    fn from(value: &str) -> Self {
        ProviderId(String::from(value))
    }
}

impl From<String> for ProviderId {
    fn from(value: String) -> Self {
        ProviderId(value)
    }
}

impl From<ProviderId> for String {
    fn from(value: ProviderId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProviderConfiguration {
    pub api_base: String,
    pub api_key: String,
}
