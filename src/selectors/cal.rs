use super::*;

//use crate::icalwrap::IcalVEvent;
use crate::khevent::KhEvent;

pub struct CalendarFilter {
  cal_names: Vec<String>
}

impl SelectFilter for CalendarFilter {
  fn add_term(&mut self, it: &mut dyn Iterator<Item = &&str>) {
    let term = it.next().unwrap();
    self.cal_names.push(term.to_lowercase());
  }

  fn is_not_empty(&self) -> bool {
    !self.cal_names.is_empty()
  }

  fn includes(&self, event: &KhEvent) -> bool {
    event.get_path()
      .and_then(|path| path.parent())
      .map(|path| path.to_string_lossy().to_lowercase())
      .map(|path| self.cal_names.iter().any(|cal| path.contains(cal)) )
      .unwrap_or(false)
  }
}

impl Default for CalendarFilter {
  fn default() -> CalendarFilter {
    CalendarFilter { cal_names: Vec::new() }
  }
}

#[cfg(test)]
mod tests {
  use super::test::test_filter_event;
  use crate::testdata;
  use std::path::PathBuf;

  #[test]
  fn test_cal_first() {
    let path1 = PathBuf::from("test/cal1/event1.ics");
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(&path1), &["cal", "cal1", "cal", "cal2"]);
    assert!(filtered);
  }

  #[test]
  fn test_cal_second() {
    let path2 = PathBuf::from("test/cal2/event2.ics");
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(&path2), &["cal", "cal1", "cal", "cal2"]);
    assert!(filtered);
  }

  #[test]
  fn test_cal_negative() {
    let path3 = PathBuf::from("test/cal3/event3.ics");
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, Some(&path3), &["cal", "cal1", "cal", "cal2"]);
    assert!(!filtered);
  }
}
