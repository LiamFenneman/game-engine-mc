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
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("io error: {0}")]
    Io(std::io::Error),
    #[error("ron error: {0}")]
    Ron(ron::de::SpannedError),
}

impl From<std::io::Error> for DataError {
    fn from(e: std::io::Error) -> Self {
        return Self::Io(e);
    }
}

impl From<ron::de::SpannedError> for DataError {
    fn from(e: ron::de::SpannedError) -> Self {
        return Self::Ron(e);
    }
}
