use crate::icalwrap::{IcalComponent,IcalProperty};
use std::path::Path;
use crate::input;
use crate::KhResult;

pub fn prettyprint() -> KhResult<()> {
  let lines = input::default_input_khlines()?;
  for line in lines {
    let event = line.to_event()?;
    prettyprint_comp(&event, line.get_path());
  }
  Ok(())
}

pub fn prettyprint_comp(cal: &IcalComponent, path: &Path) {
  let properties = cal.get_properties_all();
  debug!("path: {:?}", path);
  debug!("property count: {}", properties.len());
  for property in properties {
    prettyprint_prop(&property);
  }
  println!();
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

