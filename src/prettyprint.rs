use std::path::{Path};

use ::icalwrap::*;

pub fn prettyprint_file(filepath: &Path) {
  match utils::read_file_to_string(filepath) {
    Ok(content) => {
      let comp = Icalcomponent::from_str(&content);
      let inner = comp.get_inner();
      prettyprint_comp(&inner);
    },
    Err(error) => print!("{}", error)
  }
}

fn prettyprint_comp(comp: &Icalcomponent) {
  let properties = comp.get_properties_all();
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
      println!("start: {}", date);
    },
    "DESCRIPTION" => println!("description: {}", value),
    _  => println!("{} - {}", name, value),
  }
}

