use std::collections::HashMap;
use utils;
use defaults;
use toml;
use icalwrap::IcalVCalendar;
use yansi::{self,Style,Color};

#[derive(Deserialize)]
pub struct Config {
  calendars: HashMap<String,CalendarConfig>
}

#[derive(Deserialize)]
pub struct CalendarConfig {
  color: Option<u8>
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
    Err(_) => default_config()
  }
}

fn default_config() -> Config {
  Config {
    calendars: HashMap::new()
  }
}
