use std::path::{Path};

use ::icalwrap::*;

pub fn prettyprint_file(filepath: &Path) {
  match ::utils::read_file_to_string(filepath) {
    Ok(content) => {
      let comp = Icalcomponent::from_str(&content);
      prettyprint_comp(&comp);
    },
    Err(error) => print!("{}", error)
  }
}

fn prettyprint_comp(comp: &Icalcomponent) {
}


