use crate::util;
use eyre::{eyre, Context, Result};
use fs_err::create_dir_all;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use tracing::debug;

const DEFAULT_DB_NAME: &str = "find_videos.sqlite";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub db_name: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub db_path: String,
}

impl Settings {
    fn save_to_config_dir(content: &str) -> Result<()> {
        let config_dir = util::config_dir();
        let config_dir = config_dir.as_path();
        let path = config_dir.join("config.toml");
        let mut file = fs_err::File::create(path).wrap_err("could not create config file.")?;
        file.write_all(content.as_bytes())
            .wrap_err("could not write default config file")?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        let config_dir = util::config_dir();
        create_dir_all(&config_dir)
            .wrap_err_with(|| format!("could not create dir {config_dir:?}"))?;

        let mut config_file = if let Ok(p) = std::env::var("FINDV_CONFIG_DIR") {
            PathBuf::from(p)
        } else {
            let mut config_file = PathBuf::new();
            config_file.push(&config_dir);
            config_file
        };

        config_file.push("config.toml");

        let db_path = &config_dir.join(DEFAULT_DB_NAME);
        let mut not_exist_config = false;

        let mut config_builder = config::Config::builder()
            .set_default("db_name", DEFAULT_DB_NAME)?
            .add_source(
                config::Environment::with_prefix("findv")
                    .prefix_separator("_")
                    .separator("__"),
            );

        config_builder = if config_file.exists() {
            config_builder.add_source(config::File::new(
                config_file.to_str().unwrap(),
                config::FileFormat::Toml,
            ))
        } else {
            // create file
            not_exist_config = true;
            config_builder
        };

        let config = config_builder.build()?;
        let mut settings: Settings = config
            .try_deserialize()
            .map_err(|e| eyre!("failed to deserialize: {}", e))?;

        // settings.db_path = db_path.into_os_string().into_string().unwrap();
        settings.db_path = db_path.display().to_string();

        if not_exist_config {
            debug!("to generate config");
            Settings::save_to_config_dir(&toml::to_string(&settings)?)?;
        }
        Ok(settings)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::log::log_init;

    #[test]
    fn test_settings_new() {
        log_init();
        let settings = Settings::new().expect("new error.");
        debug!("settings:{settings:#?}");
    }
}
