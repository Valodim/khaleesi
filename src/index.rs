extern crate chrono;
extern crate libc;

use chrono::{Datelike, Duration};
use std::collections::HashMap;
use std::fs;
use ::icalwrap::*;
use ::utils;

fn get_buckets(comp: &mut Icalcomponent) -> Vec<String> {
  let mut buckets: Vec<String> = comp
    .map(|x| {
      let mut start_date = x.get_dtstart();
      let end_date = x.get_dtend();
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
      buckets
    }).flatten()
    .collect();
  buckets.sort();
  buckets.dedup();
  buckets
}

pub fn index_dir(dir: &str) {
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        // Here, `entry` is a `DirEntry`.
        if ! entry.path().is_file() {
          continue
        }
        if entry
          .path()
          .extension()
          .map_or(false, |extension| extension == "ics")
        {
          if let Ok(contents) = utils::read_file_to_string(&entry.path()) {
            let mut comp = Icalcomponent::from_str(&contents); //
            let comp_buckets = get_buckets(&mut comp);
            for bucketid in comp_buckets {
              buckets
                .entry(bucketid)
                .and_modify(|items| items.push(comp.get_uid()))
                .or_insert(::utils::vec_from_string(comp.get_uid()));
            }
          }
        }
      }
    }
  }
  info!("{} buckets", buckets.len());
  for (key, val) in buckets.iter() {
    if let Err(error) = utils::write_file(key, val.join("\n")) {
        error!("{}", error);
    }
  }
}
