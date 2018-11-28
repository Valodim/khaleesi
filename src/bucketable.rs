use icalwrap::IcalVEvent;
use chrono::{Datelike, NaiveDateTime};
use chrono::{Duration, NaiveTime};
use std::collections::HashMap;

pub trait Bucketable {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String>;

  fn get_bucket_for_datetime(date: &NaiveDateTime) -> String {
    let bucket = format!(
        "{}-{:02}",
        date.iso_week().year(),
        date.iso_week().week()
        );
    bucket
  }
  
  fn buckets_for_interval(mut start: NaiveDateTime, mut end: NaiveDateTime) -> Vec<String> {
    // end-dtimes are non-inclusive
    // so in case of date-only events, the last day of the event is dtend-1
    if end.time() == NaiveTime::from_hms(0, 0, 0) {
      end = end.checked_sub_signed(Duration::days(1)).unwrap();
    }

    let mut buckets = Vec::new();

    while start.iso_week() <= end.iso_week() {
      let bucket = Self::get_bucket_for_datetime(&start);
      buckets.push(bucket);
      start = start.checked_add_signed(Duration::days(7)).unwrap();
    }
    buckets
  }
}

impl<'a> Bucketable for IcalVEvent<'a> {
  fn get_buckets(&self) -> Result<HashMap<String, Vec<String>>, String> {
    let result:  HashMap<String, Vec<String>>;

    let start_date = self.get_dtstart().ok_or(format!("Invalid DTSTART in {}", self.get_uid()))?;
    let end_date = self.get_dtend().ok_or(format!("Invalid DTEND in {}", self.get_uid()))?;

    let mut buckets = Self::buckets_for_interval(start_date, end_date);
    for b in buckets {
      result.insert(b, event.index_string())  
    }

    if self.has_recur() {
      let duration = end_date - start_date;
      let mut recur_buckets: Vec<String> = self.get_recurs()
        .iter()
        .map(|dt| Self::buckets_for_interval(dt.naive_local(), dt.naive_local() + duration ))
        .flatten()
        .collect();
      buckets.append(&mut recur_buckets);
    }
    Ok(buckets)
  }
}

