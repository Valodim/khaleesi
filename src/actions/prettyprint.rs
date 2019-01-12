use icalwrap::{IcalComponent,IcalProperty};
use utils::fileutil;

pub fn prettyprint(lines: &mut Iterator<Item = String>) {
  let cals = fileutil::read_calendars_from_files(lines).unwrap();
  for cal in cals {
    let event = cal.get_principal_event();
    prettyprint_comp(&event, cal.get_path_as_string());
  }
}

pub fn prettyprint_comp(cal: &IcalComponent, path: Option<String>) {
  let properties = cal.get_properties_all();
  if let Some(path) = path {
    debug!("path: {}", path);
  }
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

