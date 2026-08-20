use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct Item {
    pub icon: String,
    pub command: String,
}

pub fn parse_config(location: &PathBuf) -> Result<toml::Table> {
    let contents = fs::read_to_string(location).context("failed to read config")?;

    toml::from_str(&contents).context("failed to parse config")
}
