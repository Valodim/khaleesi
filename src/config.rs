use std::collections::HashMap;
use toml;
use yansi::{self,Style,Color};

use defaults;
use icalwrap::IcalVCalendar;
use utils;

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
  pub calendars: HashMap<String,CalendarConfig>,
  pub agenda: AgendaConfig
}

#[derive(Deserialize)]
#[serde(default)]
pub struct AgendaConfig {
  pub print_week_separator: bool,
  pub print_empty_days: bool,
}

#[derive(Deserialize)]
pub struct CalendarConfig {
  pub color: Option<u8>
}

pub fn get_config_for_calendar<'a>(config: &'a Config, event: &IcalVCalendar) -> Option<&'a CalendarConfig> {
  let calendar_name = &event.get_calendar_name()?;
  config.calendars.get(calendar_name)
}

pub fn get_style_for_calendar(calendar_config: &CalendarConfig) -> yansi::Style {
  let mut style = Style::new();
  if let Some(color) = calendar_config.color {
    style = style.fg(Color::Fixed(color));
  }
  style
}

pub fn read_config() -> Config {
  let config = utils::read_file_to_string(&defaults::get_configfile());
  match config {
    Ok(config) => toml::from_str(&config).unwrap(),
    Err(_) => Config::default()
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
