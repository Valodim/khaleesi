extern crate chrono;
extern crate libc;

use chrono::{Datelike, Duration, NaiveTime};
use icalwrap::*;
use std::collections::HashMap;
use utils;
use std::path::Path;

fn get_buckets_for_event(event: &IcalVEvent) -> Result<Vec<String>, String> {
  let mut start_date = event.get_dtstart().ok_or("Invalid DTSTART")?;
  let mut end_date = event.get_dtend().ok_or("Invalid DTEND")?;
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
  Ok(buckets)
}

fn get_buckets_for_calendar(cal: &mut IcalVCalendar) -> Vec<String> {
  let mut buckets: Vec<String> = cal.events_iter().map(|x| {
      match get_buckets_for_event(&x) {
        Ok(buckets) => buckets,
        Err(error) => {
          warn!("{}", error);
          Vec::new()
        }
      }
      }).flatten()
    .collect();
  buckets.sort_unstable();
  buckets.dedup();
  buckets
}

fn add_buckets_for_calendar(buckets: &mut HashMap<String, Vec<String>>, cal: &mut IcalVCalendar) {
  let cal_buckets = get_buckets_for_calendar(cal);
  for bucketid in cal_buckets {
    buckets
      .entry(bucketid)
      .and_modify(|items| items.push(cal.get_path_as_string()))
      .or_insert(vec!(cal.get_path_as_string()));
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
        match IcalVCalendar::from_str(&content, Some(file)) {
          Ok(mut cal) => add_buckets_for_calendar(&mut buckets, &mut cal),
          Err(error) => error!("{}", error)
        }
      }
      Err(error) => error!("{}", error),
    }
  }

  info!("{} buckets", buckets.len());
  for (key, val) in buckets.iter() {
    use defaults::INDEXDIR;
    let mut bucketpath = INDEXDIR.to_owned();
    bucketpath.push_str("/");
    bucketpath.push_str(key);
    if let Err(error) = utils::write_file(&bucketpath, val.join("\n")) {
      error!("{}", error);
    }
  }
}

#[test]
fn buckets_multi_day_allday() {
  use testdata;
  let mut cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
  let comp_buckets = get_buckets_for_calendar(&mut cal);
  assert_eq!(comp_buckets, ["2007-26", "2007-27"]);
}

#[test]
fn buckets_single_event() {
  use testdata;
  let mut cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
  let comp_buckets = get_buckets_for_calendar(&mut cal);
  assert_eq!(comp_buckets, ["1997-13"]);
}
