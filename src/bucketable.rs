use chrono::{Local, Date, Datelike, Duration};
use std::collections::HashMap;
use std::{hash, cmp};

use icalwrap::{IcalVEvent, IcalVCalendar};
use utils;

pub trait Bucketable {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String>;

  fn buckets_for_interval(mut start: Date<Local>, end: Date<Local>) -> Vec<String> {
    let mut buckets = Vec::new();

    while start.iso_week() <= end.iso_week() {
      let bucket = utils::get_bucket_for_date(start);
      buckets.push(bucket);
      start = start.checked_add_signed(Duration::days(7)).unwrap();
    }
    buckets
  }
}

impl Bucketable for IcalVEvent {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String> {
    let mut result:  HashMap<String, Vec<String>> = HashMap::new();

    let start_date = self.get_dtstart_date().ok_or(format!("Invalid DTSTART in {}", self.get_uid()))?;
    let mut end_date = self.get_dtend_date().unwrap_or(start_date);

    // end-dtimes are non-inclusive
    // so in case of date-only events, the last day of the event is dtend-1
    if self.is_allday() {
      end_date = end_date.pred();
    }

    let buckets = Self::buckets_for_interval(start_date, end_date);
    for bucketid in buckets {
      result
        .entry(bucketid)
        .and_modify(|items| items.push(self.get_khaleesi_line().unwrap()))
        .or_insert(vec!(self.get_khaleesi_line().unwrap()));
    }

    if self.has_recur() {
      for instance in self.get_recur_instances() {
        let recur_buckets = instance.get_buckets()?;
        result.merge(recur_buckets)
      }
    }

    for vec in result.values_mut() {
      vec.dedup()
    }
    Ok(result)
  }
}

impl Bucketable for IcalVCalendar {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String> {
    let mut result:  HashMap<String, Vec<String>> = HashMap::new();
    for event in self.events_iter() {
      let recur_buckets = event.get_buckets()?;
      result.merge(recur_buckets);
    }
    Ok(result)
  }
}

pub trait Merge<K>
where K: cmp::Eq + hash::Hash
{
  fn merge(&mut self, other: HashMap<K, Vec<String>>);
}

impl<K> Merge<K> for HashMap<K, Vec<String>>
where K: cmp::Eq + hash::Hash
{
  fn merge(&mut self, other: HashMap<K, Vec<String>>) {
    for (key, mut lines) in other.into_iter() {
      self
        .entry(key)
        .and_modify(|items| items.append(&mut lines))
        .or_insert(lines);
    }
  }
}

#[test]
fn buckets_multi_day_allday() {
  use testdata;
  use std::path::PathBuf;

  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, path).unwrap();

  let event_buckets = cal.get_principal_event().get_buckets().unwrap();

  assert_eq!(2, event_buckets.len());

  let mut bucket_names = event_buckets.keys().collect::<Vec<&String>>();
  bucket_names.sort_unstable();
  assert_eq!(vec!("2007-W26", "2007-W27"), bucket_names);

  let cal_buckets = cal.get_buckets().unwrap();
  assert_eq!(event_buckets, cal_buckets);
}

#[test]
fn buckets_single_event() {
  use testdata;
  use std::path::PathBuf;

  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, path).unwrap();

  let comp_buckets = cal.get_buckets().unwrap();
  assert_eq!(vec!("1997-W13"), comp_buckets.keys().collect::<Vec<&String>>());
}

#[test]
fn buckets_simple_recurring_event() {
  use testdata;
  use std::path::PathBuf;

  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, path).unwrap();

  let event = cal.get_principal_event();
  let event_buckets = event.get_buckets().unwrap();
  let cal_buckets = cal.get_buckets().unwrap();
  assert_eq!(event_buckets, cal_buckets);
  let mut cal_bucket_names = cal_buckets.keys().collect::<Vec<&String>>();
  cal_bucket_names.sort_unstable();
  assert_eq!(vec!("2018-W41", "2018-W42", "2018-W43", "2018-W44", "2018-W45", "2018-W46", "2018-W47", "2018-W48", "2018-W49", "2018-W50"), cal_bucket_names);
}
