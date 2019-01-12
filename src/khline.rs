use std::fmt;
use std::path::PathBuf;
use chrono::prelude::*;

use icalwrap::IcalVEvent;

pub struct KhLine {
  path: PathBuf,
  time: DateTime<Local>,
}

impl KhLine {
  pub fn from(event: &IcalVEvent) -> Option<KhLine> {
    let time = event.get_dtstart()?;
    let path = event.get_parent()?.get_path()?.to_path_buf();

    Some(KhLine{ path, time })
  }

  pub fn to_string(&self) -> String {
    format!("{}", self)
  }
}

impl fmt::Display for KhLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let time_string = format!("{:010}", self.time.timestamp());
      let path_string = self.path.to_string_lossy();
      write!(f, "{} {}", time_string, path_string)
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  use testdata;
  use icalwrap::IcalVCalendar;

  #[test]
  fn get_khaleesi_line_test() {
    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, Some(&path)).unwrap();
    let khline = KhLine::from(&cal.get_principal_event());
    assert_eq!(String::from("1182988800 test/path"), khline.unwrap().to_string());
  }
}
