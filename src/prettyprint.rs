use std::path::{Path};

use ::icalwrap::*;

fn prettyprint_comp(comp: &Icalcomponent) {
}

pub fn prettyprint_file(filepath: &Path) {
  match ::utils::read_file_to_string(filepath) {
    Ok(content) => { 
      let comp = Icalcomponent::from_str(&content);
      prettyprint_comp(&comp);
    },
    Err(error) => print!("{}", error)
  }
}

