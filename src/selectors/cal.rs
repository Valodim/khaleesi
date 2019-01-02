use super::*;

use icalwrap::IcalVEvent;

pub struct CalendarFilter {
  cal_names: Vec<String>
}

impl CalendarFilter {
  pub fn add_cal(mut self, cal_name: &str) -> CalendarFilter {
    self.cal_names.push(cal_name.to_owned());
    self
  }
}

impl SelectFilter for CalendarFilter {
  fn includes(&self, event: &IcalVEvent) -> bool {
    event.get_parent()
      .and_then(|cal| cal.get_path())
      .and_then(|path| path.parent())
      .map(|path| self.cal_names.iter().any(|cal| path.ends_with(cal)) )
      .unwrap_or(false)
  }
}

impl Default for CalendarFilter {
  fn default() -> CalendarFilter {
    CalendarFilter { cal_names: Vec::new() }
  }
}

#[cfg(test)]
use super::test::test_filter_event;
#[cfg(test)]
use testdata;
#[cfg(test)]
use std::path::PathBuf;

#[test]
fn test_cal_first() {
  let path1 = PathBuf::from("test/cal1/event1.ics");
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(path1), &["cal", "cal1", "cal", "cal2"]);
  assert!(filtered);
}

#[test]
fn test_cal_second() {
  let path2 = PathBuf::from("test/cal2/event2.ics");
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(path2), &["cal", "cal1", "cal", "cal2"]);
  assert!(filtered);
}

#[test]
fn test_cal_negative() {
  let path3 = PathBuf::from("test/cal3/event3.ics");
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(path3), &["cal", "cal1", "cal", "cal2"]);
  assert!(!filtered);
}
