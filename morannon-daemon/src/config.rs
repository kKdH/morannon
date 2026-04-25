use std::path::PathBuf;

use directories::ProjectDirs;
use morannon_config::Configuration;

pub fn load(path: Option<PathBuf>) -> Result<(Configuration, PathBuf), LoadConfigurationError> {
    let path = path
        .or_else(default_config_path)
        .ok_or_else(|| LoadConfigurationError::ConfigurationNotFound(PathBuf::from("<unknown>")))?;
    let contents = std::fs::read_to_string(&path)
        .map_err(|_| LoadConfigurationError::ConfigurationNotFound(path.clone()))?;
    toml::from_str(&contents)
        .map(|config| (config, path))
        .map_err(|cause| LoadConfigurationError::ConfigurationInvalid(cause.to_string()))
}

#[derive(Debug)]
pub enum LoadConfigurationError {
    ConfigurationNotFound(PathBuf),
    ConfigurationInvalid(String),
}

impl std::fmt::Display for LoadConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadConfigurationError::ConfigurationNotFound(path) => {
                write!(f, "Configuration file not found: {}", path.display())
            }
            LoadConfigurationError::ConfigurationInvalid(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            },
        }
    }
}

impl std::error::Error for LoadConfigurationError {}

fn default_config_path() -> Option<PathBuf> {
    let proj = ProjectDirs::from("", "", "morannon")?;
    Some(proj.config_dir().join("morannon.toml"))
}
