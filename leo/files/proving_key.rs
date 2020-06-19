//! The proving key file.

use crate::{directories::outputs::OUTPUTS_DIRECTORY_NAME, errors::ProvingKeyFileError};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static PROVING_KEY_FILE_EXTENSION: &str = ".leo.pk";

#[derive(Deserialize)]
pub struct ProvingKeyFile {
    pub package_name: String,
}

impl ProvingKeyFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the proving key from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, ProvingKeyFileError> {
        let path = self.setup_file_path(path);

        Ok(fs::read(&path).map_err(|_| ProvingKeyFileError::FileReadError(path.clone()))?)
    }

    /// Writes the given proving key to a file.
    pub fn write_to(&self, path: &PathBuf, proving_key: &[u8]) -> Result<(), ProvingKeyFileError> {
        let path = self.setup_file_path(path);

        let mut file = File::create(&path)?;
        file.write_all(proving_key)?;

        log::info!("Proving key stored to {:?}", path);

        Ok(())
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!(
                "{}{}",
                self.package_name, PROVING_KEY_FILE_EXTENSION
            )));
        }
        path
    }
}