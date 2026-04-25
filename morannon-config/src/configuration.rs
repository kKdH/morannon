use crate::daemon::DaemonConfiguration;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Configuration {
    pub daemon: DaemonConfiguration,
}
