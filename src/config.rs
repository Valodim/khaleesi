use std::collections::HashMap;
use toml;
use yansi::{self,Style,Color};

use crate::defaults;
use crate::utils::fileutil as utils;

#[derive(Deserialize,Debug,PartialEq)]
#[serde(default)]
pub struct Config {
  pub calendars: HashMap<String,CalendarConfig>,
  pub agenda: AgendaConfig,
  pub local_tz: Option<LocalTZConfig>
}

#[derive(Deserialize,Debug,PartialEq)]
#[serde(default)]
pub struct AgendaConfig {
  pub print_week_separator: bool,
  pub print_empty_days: bool,
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct CalendarConfig {
  pub color: Option<u8>
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct LocalTZConfig {
  pub timezone: String
}

impl Config {
  pub fn get_config_for_calendar(&self,  calendar_name: &str) -> Option<&CalendarConfig> {
    self.calendars.get(calendar_name)
  }

  pub fn read_config() -> Self {
    let config = utils::read_file_to_string(&defaults::get_configfile());
    match config {
      Ok(config) => toml::from_str(&config).unwrap(),
      Err(_) => Config::default()
    }
  }
}

impl CalendarConfig {
  pub fn get_style_for_calendar(&self) -> yansi::Style {
    let mut style = Style::default();
    if let Some(color) = self.color {
      style = style.fg(Color::Fixed(color));
    }
    style
  }
}

impl LocalTZConfig {
  pub fn get_local_tz(&self) -> String {
    self.timezone.clone()
  }
}

impl Default for AgendaConfig {
  fn default() -> Self {
    AgendaConfig {
      print_week_separator: false,
      print_empty_days: true,
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Config {
      agenda: AgendaConfig::default(),
      calendars: HashMap::new(),
      local_tz: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testutils;

  #[test]
  fn test_read_config_none() {
    let _testdir = testutils::prepare_testdir("testdir");

    let config = Config::read_config();

    assert_eq!(Config::default(), config);
  }

  #[test]
  fn test_read_config() {
    let _testdir = testutils::prepare_testdir("testdir_config");

    let config = Config::read_config();
    let cal_config = config.get_config_for_calendar("sample").unwrap();

    let expected = Config {
      calendars: hashmap!{"sample".to_string() => CalendarConfig { color: Some(81) }},
      agenda: AgendaConfig {
        print_week_separator: true,
        print_empty_days: false
      },
      local_tz: None,
    };

    assert_eq!(expected, config);
    assert_eq!(expected.calendars.get("sample").unwrap(), cal_config);
  }

  #[test]
  fn test_get_style_for_calendar() {
    let config = CalendarConfig { color: Some(81) };
    let style = config.get_style_for_calendar();

    assert_eq!(Color::Fixed(81).style(), style);
  }

  #[test]
  fn test_get_local_tz() {
    let config = LocalTZConfig { timezone: "Europe/Berlin".to_string() };
    let tz = config.get_local_tz();

    assert_eq!("Europe/Berlin".to_string(), tz);
  }
}
