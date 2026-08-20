use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Item {
    pub icon: String,
    pub command: String,
}

pub fn parse_config(location: &str) -> Result<toml::Table> {
    let contents = fs::read_to_string(location)
        .with_context(|| format!("failed to read config: {location}"))?;

    toml::from_str(&contents).context("failed to parse config")
}
