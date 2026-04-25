use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use crate::client::{ClientConfiguration, ClientId};
use crate::model::{ModelConfiguration, ModelId};
use crate::provider::{ProviderConfiguration, ProviderId};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DaemonConfiguration {
    #[serde(default = "default_bind_address")]
    pub bind_address: IpAddr,
    #[serde(default = "default_bind_port")]
    pub bind_port: u16,
    #[serde(default)]
    pub clients: HashMap<ClientId, ClientConfiguration>,
    #[serde(default)]
    pub models: HashMap<ModelId, ModelConfiguration>,
    #[serde(default)]
    pub providers: HashMap<ProviderId, ProviderConfiguration>,
}

fn default_bind_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

fn default_bind_port() -> u16 {
    3019
}
