use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use chrono::prelude::*;

use icalwrap::{IcalVCalendar,IcalVEvent};
use utils::{fileutil,dateutil};

pub struct KhLine {
  path: PathBuf,
  time: Option<DateTime<Local>>,
}

impl KhLine {
  pub fn from(event: &IcalVEvent) -> Option<KhLine> {
    let path = event.get_parent()?.get_path()?.to_path_buf();
    let time = event.get_dtstart();

    Some(KhLine{ path, time })
  }

  pub fn to_string(&self) -> String {
    format!("{}", self)
  }

  pub fn to_cal(&self) -> Result<IcalVCalendar, String> {
    let mut calendar = fileutil::read_calendar_from_path(&self.path)?;
    if let Some(time) = self.time {
      calendar = calendar.with_internal_timestamp(time);
    }
    Ok(calendar)
  }

  pub fn to_event(&self) -> Result<IcalVEvent, String> {
    self.to_cal().map(|cal| cal.get_principal_event())
  }
}

impl fmt::Display for KhLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let path_string = self.path.to_string_lossy();
    match self.time {
      Some(time) => {
        let time_string = format!("{:010}", time.timestamp());
        write!(f, "{} {}", time_string, path_string)
      }
      None => write!(f, "{}", path_string)
    }
  }
}

impl FromStr for KhLine {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.splitn(2, ' ').collect();
    if let Some(time) = dateutil::datetime_from_timestamp(parts[0]) {
      let path = PathBuf::from(parts[1]);
      Ok(Self{ path, time: Some(time) })
    } else {
      let path = PathBuf::from(parts[0]);
      Ok(Self{ path, time: None })
    }
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
