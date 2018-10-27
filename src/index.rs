extern crate chrono;
extern crate libc;

use chrono::{Datelike, Duration, NaiveTime};
use icalwrap::*;
use std::collections::HashMap;
use utils;
use std::path::Path;

fn get_buckets(comp: &mut Icalcomponent) -> Vec<String> {
  let mut buckets: Vec<String> = comp.into_iter().map(|x| {
      let mut start_date = x.get_dtstart();
      let mut end_date = x.get_dtend();
      //info!("start: {}", start_date);
      //info!("end: {}", end_date);

      // end-dtimes are non-inclusive 
      // so in case of date-only events, the last day of the event is dtend-1 
      if end_date.time() == NaiveTime::from_hms(0, 0, 0) {
        end_date = end_date.checked_sub_signed(Duration::days(1)).unwrap();
      }
      let mut buckets = Vec::new();
      while start_date.iso_week() <= end_date.iso_week() {
        let bucket = format!(
          "{}-{:02}",
          start_date.iso_week().year(),
          start_date.iso_week().week()
        );
        buckets.push(bucket);
        start_date = start_date.checked_add_signed(Duration::days(7)).unwrap();
      }
      //if buckets.len() > 1 {
      //  info!("{}: {} buckets", x.get_uid(), buckets.len());
      //}
      buckets
    }).flatten()
    .collect();
  buckets.sort();
  buckets.dedup();
  buckets
}

fn add_buckets_for_component(buckets: &mut HashMap<String, Vec<String>>, comp: &mut Icalcomponent) {
  let comp_buckets = get_buckets(comp);
  for bucketid in comp_buckets {
    buckets
      .entry(bucketid)
      .and_modify(|items| items.push(comp.get_uid()))
      .or_insert(::utils::vec_from_string(comp.get_uid()));
  }
}

pub fn index_dir(dir: &Path ) {
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  let ics_files = utils::file_iter(dir)
    .filter( |path| path.is_file() )
    .filter( |path| path.extension().map_or(false, |extension| extension == "ics"));
  
  for file in ics_files {
    match utils::read_file_to_string(&file) {
      Ok(content) => {
        match Icalcomponent::from_str(&content, Some(file)) {
          Ok(mut comp) => add_buckets_for_component(&mut buckets, &mut comp),
          Err(error) => error!("{}", error)
        }
      }
      Err(error) => error!("{}", error),
    }
  }

  info!("{} buckets", buckets.len());
  for (key, val) in buckets.iter() {
    if let Err(error) = utils::write_file(key, val.join("\n")) {
      error!("{}", error);
    }
  }
}

#[test]
fn buckets_multi_day_allday() {
  use testdata;
  let mut comp = Icalcomponent::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
  let comp_buckets = get_buckets(&mut comp);
  assert_eq!(comp_buckets, ["2007-26", "2007-27"]);
}
