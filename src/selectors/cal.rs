use super::*;

use icalwrap::IcalVEvent;

pub struct CalendarFilter {
  cal_name: String
}

impl CalendarFilter {
  pub fn new(cal_name: &str) -> CalendarFilter {
    CalendarFilter { cal_name: cal_name.to_owned() }
  }
}

impl SelectFilter for CalendarFilter {
  fn includes(&self, event: &IcalVEvent) -> bool {
    event.get_parent()
      .and_then(|cal| cal.get_path())
      .and_then(|path| path.parent())
      .map(|path| path.ends_with(&self.cal_name))
      .unwrap_or(false)
  }
}

