use icalwrap::{IcalComponent,IcalProperty};
use utils::fileutil;

pub fn prettyprint(lines: &mut Iterator<Item = String>) {
  let cals = fileutil::read_calendars_from_files(lines).unwrap();
  for cal in cals {
    prettyprint_comp(&cal);
  }
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

