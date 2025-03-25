use anyhow::Result;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(default)]
pub struct Config {
    pub log_level: LevelFilter,
    pub logs_folder: String,
    pub stream_urls: Vec<String>,
    pub streamlink_cli: String,
}

impl Config {
    pub fn load_or_default(config_path: &Path) -> Result<Self> {
        if config_path.as_os_str().is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "empty path").into());
        }

        if !config_path.exists() {
            let default_config = Config {
                ..Default::default()
            };
            default_config.save(config_path)?;

            Ok(default_config)
        } else {
            let config = Config::load(config_path)?;
            config.save(config_path)?; // Re-save to add missing fields.

            Ok(config)
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::de::from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string(self)?;
        fs::write(path, content)?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            logs_folder: "logs".to_string(),
            stream_urls: vec![],
            streamlink_cli: r##"streamlink --force --logfile "logs\{source}_{timestamp}.log" --output "streams\{author}_{time:%Y%m%d-%H%M%S}.mp4" --progress no --twitch-disable-ads --default-stream "1080p, 720p, best" --url {url}"##.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_load_and_save() -> Result<()> {
        let path = Path::new("debug_config.toml");
        fs::remove_file(path)?;
        let mut config = Config::load_or_default(path)?;

        assert_eq!(config, Config::default());

        let new_level_filter = LevelFilter::Trace;
        config.log_level = new_level_filter;
        config.save(path)?;
        let config = Config::load_or_default(path)?;
        assert_eq!(config.log_level, new_level_filter);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_empty_config_path() {
        let empty_path = Path::new("");
        Config::load_or_default(empty_path).unwrap();
    }
}
