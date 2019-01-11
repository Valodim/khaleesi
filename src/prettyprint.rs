use std::path::{Path};

use icalwrap::{IcalComponent,IcalVCalendar,IcalProperty};
use utils::fileutil as utils;

pub fn shortprint_dir(dir: &Path) {
  for filepath in utils::file_iter(dir) {
    shortprint_file(&filepath);
  }
}

pub fn shortprint_file(filepath: &Path) {
  match utils::read_file_to_string(filepath) {
    Ok(content) => {
      let cal = IcalVCalendar::from_str(&content, None);
      let inner = cal.unwrap();
      shortprint_comp(&inner);
    },
    Err(error) => print!("{}", error)
  }
}

pub fn prettyprint_file(filepath: &Path) {
  match utils::read_file_to_string(filepath) {
    Ok(content) => {
      let cal = IcalVCalendar::from_str(&content, Some(filepath)).unwrap();
      prettyprint_comp(&cal);
    },
    Err(error) => print!("{}", error)
  }
}

pub fn shortprint_comp(cal: &IcalVCalendar) {
  let event = cal.events_iter().next().expect("No event in VCalendar!");
  let mut output = String::new();
  if let Some(date) = event.get_dtstart() {
    output.push_str(&date.format("%Y-%m-%d").to_string());
  } else {
    warn!("Invalid DTSTART in {}", event.get_uid());
    return;
  };
  if let Some(summary) = event.get_summary() {
    output.push_str(&summary);
  } else {
    warn!("Invalid SUMMARY in {}", event.get_uid());
    return;
  };
  println!("{}", output);

}

pub fn prettyprint_comp(cal: &IcalVCalendar) {
  let properties = cal.get_properties_all();
  println!("num: {}", properties.len());
  for property in properties {
    prettyprint_prop(&property);
  }
}

fn prettyprint_prop(property: &IcalProperty) {
  let name = property.get_name();
  let value = property.get_value();
  match name.as_str() {
    "DTSTART" => {
      let date = property.get_value_as_date();
      println!("start: {}", date.unwrap());
    },
    "DESCRIPTION" => println!("description: {}", value),
    _  => println!("{} - {}", name, value),
  }
}

