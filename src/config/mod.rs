use anyhow::{anyhow, format_err};
use getset::{CopyGetters, Getters};
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::{fs, io};

const DEFAULT_CONFIG_PATHS: &[&str] = &[
    ".cirrus/config.toml",
    #[cfg(unix)]
    "/etc/cirrus/config.toml",
];

#[derive(Getters, CopyGetters, Debug, Deserialize)]
#[getset(get = "pub")]
pub struct Config {
    database: Database,
}

#[derive(Getters, CopyGetters, Debug, Deserialize)]
#[getset(get = "pub")]
pub struct Database {
    url: String,
}

impl Config {
    fn read_from_path_or_default_paths(path: Option<&Path>) -> Option<io::Result<String>> {
        match path {
            None => DEFAULT_CONFIG_PATHS
                .iter()
                .find_map(|&path| match fs::read_to_string(path) {
                    Err(err) if err.kind() == ErrorKind::NotFound => None,
                    other => Some(other),
                }),
            Some(path) => Some(fs::read_to_string(path)),
        }
    }

    /// Parses the configuration file at `path`.
    ///
    /// If `path` is `None`, searches for the configuration file at the following default locations,
    /// parsing the first existing file:
    /// - `.cirrus/config.toml`
    /// - (unix only) `/etc/cirrus/config.toml`
    ///
    /// # Errors
    ///
    /// Returns an error if
    /// - the configuration file could not be found,
    /// - an I/O error occurs, or
    /// - the configuration file is invalid.
    pub fn parse(path: Option<&Path>) -> anyhow::Result<Config> {
        let file_contents = match Self::read_from_path_or_default_paths(path) {
            None => return Err(anyhow!("The configuration file could not be found.")),
            Some(Err(err)) => return Err(anyhow!("Unable to read configuration file: {err}")),
            Some(Ok(file)) => file,
        };

        match toml::from_str(&file_contents) {
            Err(err) => Err(anyhow!("Invalid configuration: {err}")),
            Ok(config) => Ok(config),
        }
    }

    /// Creates the default configuration file at `path`.
    ///
    /// `overwrite` specifies what to do if a file at `path` already exists:
    /// - If it is `None`, return an error.
    /// - If it is `Some(false)`, do not overwrite the already existing file.
    /// - If it is `Some(true)`, overwrite the already existing file.
    ///
    /// # Errors
    ///
    /// Returns an error if
    /// - `overwrite == None` and a file at `path` already exists, or
    /// - an I/O error occurs.
    pub fn create(path: &Path, overwrite: Option<bool>) -> anyhow::Result<()> {
        let result = OpenOptions::new()
            .write(true)
            .create(true)
            .create_new(!overwrite.unwrap_or(true))
            .open(path)
            .and_then(|mut file| file.write_all(include_bytes!("default_config.toml")));

        match result {
            Err(err) if err.kind() == ErrorKind::AlreadyExists && overwrite == Some(false) => {
                Ok(())
            }
            Err(err) => Err(format_err!(
                "Unable to create config file at {}: {}",
                path.display(),
                err
            )),
            Ok(()) => Ok(()),
        }
    }
}
