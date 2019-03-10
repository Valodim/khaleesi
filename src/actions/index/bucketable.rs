use chrono::{Local, Date, Datelike, Duration};
use std::collections::HashMap;
use std::{hash, cmp};

use crate::icalwrap::IcalVCalendar;
//use crate::icalwrap::IcalVEvent;
use crate::utils::misc;
use crate::khline::KhLine;
use crate::khevent::KhEvent;

pub trait Bucketable {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String>;

  fn buckets_for_interval(mut start: Date<Local>, end: Date<Local>) -> Vec<String> {
    let mut buckets = Vec::new();

    while start.iso_week() <= end.iso_week() {
      let bucket = misc::get_bucket_for_date(start);
      buckets.push(bucket);
      start = start.checked_add_signed(Duration::days(7)).unwrap();
    }
    buckets
  }
}

impl Bucketable for KhEvent {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String> {
    let mut result:  HashMap<String, Vec<String>> = HashMap::new();

    let start_date: Date<Local> = self.get_start().ok_or_else(|| format!("Invalid DTSTART in {}", self.get_uid()))?.into();
    //let mut end_date: Date<Local> = self.get_end().map(|date| date.into()).unwrap_or(start_date);

    let mut end_date = self.get_last_relevant_date().map(|date| date.into()).unwrap_or(start_date);
    // end-dtimes are non-inclusive
    // so in case of date-only events, the last day of the event is dtend-1
    //if self.is_allday() {
      //end_date = end_date.pred();
    //}

    let buckets = Self::buckets_for_interval(start_date, end_date);
    let khline = KhLine::from(self);
    for bucketid in buckets {
      result
        .entry(bucketid)
        .and_modify(|items| items.push(khline.to_string()))
        .or_insert_with(|| vec!(khline.to_string()));
    }

    if self.is_recur_master() {
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
      let event = KhEvent::from_event(event);
      //let recur_buckets = event.get_buckets()?;
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

impl<K, S> Merge<K> for HashMap<K, Vec<String>, S>
where K: cmp::Eq + hash::Hash,
  S: std::hash::BuildHasher
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

#[cfg(test)]
mod tests {
  use super::*;

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
    use crate::testdata;
    use std::path::PathBuf;

    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, Some(&path)).unwrap();

    let event_buckets = cal.get_principal_khevent().get_buckets().unwrap();

    assert_eq!(2, event_buckets.len());

    let mut bucket_names = event_buckets.keys().collect::<Vec<&String>>();
    bucket_names.sort_unstable();
    assert_eq!(vec!("2007-W26", "2007-W27"), bucket_names);

    let cal_buckets = cal.get_buckets().unwrap();
    assert_eq!(event_buckets, cal_buckets);
  }

  #[test]
  fn buckets_single_event() {
    use crate::testdata;
    use std::path::PathBuf;

    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, Some(&path)).unwrap();

    let comp_buckets = cal.get_buckets().unwrap();
    assert_eq!(vec!("1997-W13"), comp_buckets.keys().collect::<Vec<&String>>());
  }

  #[test]
  fn buckets_simple_recurring_event() {
    use crate::testdata;
    use std::path::PathBuf;

    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, Some(&path)).unwrap();

    //let event = cal.get_principal_khevent();
    //let event_buckets = event.get_buckets().unwrap();
    let cal_buckets = cal.get_buckets().unwrap();
    //assert_eq!(event_buckets, cal_buckets);
    //let mut cal_bucket_names = cal_buckets.keys().collect::<Vec<&String>>();
    //cal_bucket_names.sort_unstable();
    //assert_eq!(vec!("2018-W41", "2018-W42", "2018-W43", "2018-W44", "2018-W45", "2018-W46", "2018-W47", "2018-W48", "2018-W49", "2018-W50"), cal_bucket_names);
  }
}
