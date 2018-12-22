use std::collections::HashMap;
use utils;
use defaults;
use toml;

#[derive(Deserialize)]
pub struct Config {
  calendars: HashMap<String,CalendarConfig>
}

#[derive(Deserialize)]
pub struct CalendarConfig {
  color: Option<i32>
}

pub fn read_config() -> Config {
  let config = utils::read_file_to_string(&defaults::get_configfile());
  match config {
    Ok(config) => toml::from_str(&config).unwrap(),
    Err(_) => default_config()
  }
}

fn default_config() -> Config {
  Config {
    calendars: HashMap::new()
  }
}
