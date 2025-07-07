use std::path::Path;

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use log::debug;
use serde::Deserialize;

use crate::{cli::Cli, diagnostics::Level, error::Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub level: Level,
    #[serde(default)]
    pub clippy_args: Vec<String>,
}

impl Config {
    pub fn new(cli: &Cli, workspace_root: &Path) -> Result<Self> {
        let config_path = cli.config_path.clone().unwrap_or(workspace_root.join(".clipped.toml"));
        debug!("attempting to load config from `{}`", config_path.display());

        let config = Figment::from(Toml::file(config_path))
            .merge(Env::prefixed("CLIPPED_"))
            .merge(Serialized::defaults(cli))
            .extract()
            .map_err(Box::new)?;
        debug!("config loaded: {config:?}");

        Ok(config)
    }
}
