use std::path::Path;

use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub log_level: String,
    pub log_file: String,
    pub theme: String,
    pub icon_theme: String,
    pub font: String,
}

fn default_config() -> AppConfig {
    AppConfig {
        log_level: "Info".to_string(),
        log_file: "/var/log/nyow-greeter.log".to_string(),
        theme: "Adwaita-dark".to_string(),
        icon_theme: "Papirus-Dark".to_string(),
        font: "Lato 11".to_string(),
    }
}

pub fn load_config(config_path: &str) -> AppConfig {
    let mut builder = Config::builder();

    let defaults = default_config();
    builder = builder
        .set_default("log_level", defaults.log_level)
        .unwrap()
        .set_default("log_file", defaults.log_file)
        .unwrap()
        .set_default("theme", defaults.theme)
        .unwrap()
        .set_default("icon_theme", defaults.icon_theme)
        .unwrap()
        .set_default("font", defaults.font)
        .unwrap();

    if Path::new(config_path).exists() {
        builder = builder.add_source(File::with_name(config_path));
    }

    let settings = builder.build().unwrap();

    settings
        .try_deserialize()
        .expect("Failed to load configuration")
}
