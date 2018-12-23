use chrono::*;
use icalwrap::IcalVCalendar;
use std::path::{Path,PathBuf};
use utils;

pub fn do_show(filenames: &mut Iterator<Item = String>, args: &[String]) {
  info!("do_show");

  for line in filenames {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let path = match utils::datetime_from_timestamp(parts[0]) {
      Some(_) => Path::new(parts[1]),
      None => Path::new(parts[0]),
    };
    let output = utils::read_file_to_string(path).unwrap();
    println!("{}", output);
  }
  
}
  //let mut cals = utils::read_calendars_from_files(filenames).unwrap();

  //cals = cals.into_iter()
    //.filter( filters.predicate_is_from() )
    //.filter( filters.predicate_is_to() )
    //.filter( filters.predicate_is_in_calendar() )
    //.collect();

  //for cal in cals {
    //if let Some(line) = cal.get_principal_event().index_line() {
      //println!("{}", line);
    //}
  //}

