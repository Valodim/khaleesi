use std::collections::HashMap;
use toml;
use yansi::{self,Style,Color};

use defaults;
use icalwrap::IcalVCalendar;
use utils::fileutil as utils;

#[derive(Deserialize,Debug,PartialEq)]
#[serde(default)]
pub struct Config {
  pub calendars: HashMap<String,CalendarConfig>,
  pub agenda: AgendaConfig
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

impl Config {
  pub fn get_config_for_calendar(&self,  event: &IcalVCalendar) -> Option<&CalendarConfig> {
    let calendar_name = &event.get_calendar_name()?;
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
    let mut style = Style::new();
    if let Some(color) = self.color {
      style = style.fg(Color::Fixed(color));
    }
    style
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
      calendars: HashMap::new(),
      agenda: AgendaConfig::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use testutils;

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

    let expected = Config {
      calendars: hashmap!{"sample".to_string() => CalendarConfig { color: Some(81) }},
      agenda: AgendaConfig {
        print_week_separator: true,
        print_empty_days: false
      }
    };
    assert_eq!(expected, config);
  }
}
