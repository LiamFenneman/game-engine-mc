use ron::ser::PrettyConfig;
use thiserror::Error;

impl crate::ResourceManager {
    /// Loads a RON data file from disk.
    ///
    /// # Errors
    /// Errors if the file doesn't exist or the file cannot be parsed into `T`.
    pub fn load_data<T>(&self, name: &str) -> Result<T, DataError>
    where
        T: serde::de::DeserializeOwned,
    {
        let str = std::fs::read_to_string(self.data_path.join(name))?;
        return Ok(ron::from_str(&str)?);
    }

    /// Saves a RON data file to disk.
    ///
    /// # Errors
    /// Errors if the file cannot be written to disk.
    pub fn save_data<T>(&self, name: &str, data: &T) -> Result<(), DataError>
    where
        T: serde::Serialize,
    {
        let config = PrettyConfig::new();
        let contents = ron::ser::to_string_pretty(data, config)?;
        std::fs::write(self.data_path.join(name), contents)?;
        return Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ron deserialize error: {0}")]
    RonDe(#[from] ron::de::SpannedError),
    #[error("ron serialize error: {0}")]
    RonSer(#[from] ron::Error),
}
