use thiserror::Error;

impl crate::ResourceManager {
    /// Loads a TOML config file from disk.
    ///
    /// # Errors
    /// Errors if the file doesn't exist or the file cannot be parsed into `T`.
    pub fn load_config<T>(&self, name: &str) -> Result<T, ConfigError>
    where
        T: serde::de::DeserializeOwned,
    {
        let str = std::fs::read_to_string(self.config_path.join(name))?;
        return Ok(toml::from_str(&str)?);
    }

    /// Saves a TOML config file to disk.
    ///
    /// # Errors
    /// Errors if the file cannot be written to disk.
    pub fn save_config<T>(&self, name: &str, data: &T) -> Result<(), ConfigError>
    where
        T: serde::Serialize,
    {
        let contents = toml::to_string_pretty(data)?;
        std::fs::write(self.config_path.join(name), contents)?;
        return Ok(());
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml deserialize error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("toml serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),
}
