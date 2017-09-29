
#[derive(Deserialize)]
pub struct Config {
  consul: Consul
}

#[derive(Deserialize)]
pub struct Consul {
    scheme: String,
    address: String,
    datacenter: String,
    username: String,
    password: String,
    token: String,
}

use toml;
use std::fs::File;
use std::io;

pub enum ConfigError {
  IoError(io::Error),
  TomlError(toml::de::Error),
}
impl From<io::Error> for ConfigError {
  fn from(error: io::Error) -> Self {
    ConfigError::IoError(error)
  }
}
impl From<toml::de::Error> for ConfigError {
  fn from(error: toml::de::Error) -> Self {
    ConfigError::TomlError(error)
  }
}

/**
 * Given a string path, attempt to load the file.
 *
 * TODO: This function should do some sanitization on the input path,
 * which will eventually be supplied by a user or defaulted. Eitherway,
 * validation is needed here such that things dont get too crazy or unsafe.
 */
pub fn load(path: String) -> Result<Config, ConfigError> {
  let a = read_file(path);
  let b: Result<Config,ConfigError> = a.and_then(|c| toml::from_str(&c)
    .map_err(|e| ConfigError::from(e)) );
  b
}

use std::io::prelude::*;

fn read_file(path: String) -> Result<String, ConfigError> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  let res = file.read_to_string(&mut contents);

  match res {
    Ok(_)  => Ok(contents),
    Err(e) => Err(ConfigError::from(e)),
  }
}