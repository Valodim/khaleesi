extern crate chrono;
extern crate libc;

use chrono::{Datelike, Duration, NaiveTime};
use icalwrap::*;
use std::collections::HashMap;
use utils;
use std::path::{Path,PathBuf};
use std::fs;
use defaults::*;

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

pub fn index_dir(dir: &Path) {
  use std::time::Instant;

  info!("Recursively indexing '.ics' files in directory: {}", dir.to_string_lossy());
  if !dir.exists() {
    error!("Directory doesn't exist: {}", dir.to_string_lossy());
    return;
  }

  let now = Instant::now();

  let ics_files = get_ics_files(dir);
  let buckets = read_buckets(ics_files);

  if check_index_dir() {
    write_index(buckets);
  }

  info!("Index written in {}ms", utils::format_duration(&now.elapsed()));
}

fn get_ics_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
  utils::file_iter(dir)
    .filter( |path| path.extension().map_or(false, |extension| extension == "ics"))
}

fn read_buckets(ics_files: impl Iterator<Item = PathBuf>) -> HashMap<String, Vec<String>> {
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  let mut total_files = 0;
  for file in ics_files {
    match utils::read_file_to_string(&file) {
      Ok(content) => {
        total_files += 1;
        match IcalVCalendar::from_str(&content, Some(file)) {
          Ok(mut cal) => add_buckets_for_calendar(&mut buckets, &mut cal),
          Err(error) => error!("{}", error)
        }
      }
      Err(error) => error!("{}", error),
    }
  }

  info!("Loaded {} files into {} buckets", total_files, buckets.len());
  buckets
}

fn write_index(buckets: HashMap<String, Vec<String>>) {
  for (key, val) in buckets.iter() {
    let bucketfile = get_indexfile(key);
    debug!("Writing bucket: {}", key);
    if let Err(error) = utils::write_file(&bucketfile, val.join("\n")) {
      error!("{}", error);
      return;
    }
  }
}

fn check_index_dir() -> bool {
  let indexdir = get_indexdir();
  if !indexdir.exists() {
    info!("Creating index directory: {}", indexdir.to_string_lossy());
    if let Err(error) = fs::create_dir(&indexdir) {
      error!("{}", error);
      return false;
    }
  }
  info!("Using index directory: {}", indexdir.to_string_lossy());
  true
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
