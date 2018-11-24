use utils;
use icalwrap::*;
use std::path::Path;

pub fn do_unroll(filepath: &Path) {
  let cal = utils::read_calendar_from_path(filepath).unwrap();   
  for event in cal.events_iter() {
    if event.has_recur() {
      let recurs = event.get_recurs();
      for datetime in recurs {
        println!("{} {}", datetime.timestamp(), cal.get_path_as_string());
      }
    }
  }
}
