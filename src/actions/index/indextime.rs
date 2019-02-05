use std::fs;
use std::io::{Read,Write};
use chrono::prelude::*;

use crate::defaults::*;

pub fn write_index_time(index_time: &DateTime<Utc>) {
  let mut timefile = fs::File::create(get_indextimefile()).unwrap();
  timefile.write_all(format!("{}\n", index_time.timestamp()).as_bytes()).unwrap();
}

pub fn get_index_time() -> Option<DateTime<Utc>> {
  let mut timefile = fs::File::open(get_indextimefile()).ok()?;
  let mut timestamp_str = String::new();
  timefile.read_to_string(&mut timestamp_str).ok()?;
  let timestamp = timestamp_str.trim().parse::<i64>().ok()?;
  Some(Utc.timestamp(timestamp, 0))
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::testutils;
  use assert_fs::prelude::*;

  #[test]
  fn test_write_read() {
    let testdir = testutils::prepare_testdir("testdir");

    let timestamp = Utc.ymd(1990,01,01).and_hms(1, 1, 0);
    write_index_time(&timestamp);
    testdir.child(".khaleesi/index-time").assert("631155660\n");

    let indextime = get_index_time();
    assert_eq!(Some(timestamp), indextime);
  }
}
