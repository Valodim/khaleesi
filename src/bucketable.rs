use icalwrap::{IcalVEvent, IcalVCalendar};
use chrono::{Local, Date, Datelike, Duration};
use std::collections::HashMap;
use utils;

pub trait Bucketable {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String>;

  fn buckets_for_interval(mut start: Date<Local>, end: Date<Local>) -> Vec<String> {
    let mut buckets = Vec::new();

    while start.iso_week() <= end.iso_week() {
      let bucket = utils::get_bucket_for_date(&start);
      buckets.push(bucket);
      start = start.checked_add_signed(Duration::days(7)).unwrap();
    }
    buckets
  }
}

impl<'a> Bucketable for IcalVEvent<'a> {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String> {
    let mut result:  HashMap<String, Vec<String>> = HashMap::new();

    let start_date = self.get_dtstart_date().ok_or(format!("Invalid DTSTART in {}", self.get_uid()))?;
    let mut end_date = self.get_dtend_date().ok_or(format!("Invalid DTEND in {}", self.get_uid()))?;

    // end-dtimes are non-inclusive
    // so in case of date-only events, the last day of the event is dtend-1
    if self.is_allday() {
      end_date = end_date.pred();
    }

    let buckets = Self::buckets_for_interval(start_date, end_date);
    for bucketid in buckets {
      result
        .entry(bucketid)
        .and_modify(|items| items.push(self.index_line().unwrap()))
        .or_insert(vec!(self.index_line().unwrap()));
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
where K: std::cmp::Eq + std::hash::Hash
{
  fn merge(&mut self, other: HashMap<K, Vec<String>>);
}

impl<K> Merge<K> for HashMap<K, Vec<String>>
where K: std::cmp::Eq + std::hash::Hash
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
fn merge_test() {
  let mut map_a: HashMap<&str, Vec<String>> = HashMap::new();
  let mut map_b: HashMap<&str, Vec<String>> = HashMap::new();

  let key = "key";
  map_a.insert(&key, vec!["a".to_string(), "b".to_string()]);
  map_b.insert(&key, vec!["c".to_string(), "d".to_string()]);

  map_a.merge(map_b);
  assert_eq!(map_a.get(&key).unwrap(), &vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]);
}

#[test]
fn buckets_multi_day_allday() {
  use testdata;
  use std::path::PathBuf;

  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, path).unwrap();

  let event_buckets = cal.get_first_event().get_buckets().unwrap();

  assert_eq!(event_buckets.len(), 2);

  let mut bucket_names = event_buckets.keys().collect::<Vec<&String>>();
  bucket_names.sort_unstable();
  assert_eq!(bucket_names, ["2007-26", "2007-27"]);

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
  assert_eq!(comp_buckets.keys().collect::<Vec<&String>>(), ["1997-13"]);
}

#[test]
fn buckets_simple_recurring_event() {
  use testdata;
  use std::path::PathBuf;

  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, path).unwrap();

  let event = cal.get_first_event();
  let event_buckets = event.get_buckets().unwrap();
  let cal_buckets = cal.get_buckets().unwrap();
  assert_eq!(event_buckets, cal_buckets);
  let mut cal_bucket_names = cal_buckets.keys().collect::<Vec<&String>>();
  cal_bucket_names.sort_unstable();
  assert_eq!(cal_bucket_names,
     ["2018-41", "2018-42", "2018-43", "2018-44", "2018-45", "2018-46", "2018-47", "2018-48", "2018-49", "2018-50"]);
}
