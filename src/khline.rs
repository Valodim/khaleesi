use std::fmt;
use std::str::FromStr;
use std::path::{PathBuf,Path};
use chrono::prelude::*;

use icalwrap::{IcalVCalendar,IcalVEvent};
use utils::{fileutil,dateutil};
use defaults;

#[derive(PartialEq,Eq)]
pub struct KhLine {
  pub path: PathBuf,
  time: Option<DateTime<Local>>,
}

impl KhLine {
  fn new(path: &Path, time: Option<DateTime<Local>>) -> Self {
    let path = if path.is_relative() {
      let mut fullpath = defaults::get_caldir();
      fullpath.push(path);
      fullpath
    } else {
      path.to_path_buf()
    };
    Self { path, time }
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

  pub fn matches(&self, event: &IcalVEvent) -> bool {
    self == &KhLine::from(event)
  }

  fn get_normalized_path(&self) -> &Path {
    self.path
      .strip_prefix(defaults::get_caldir())
      .unwrap_or(&self.path)
  }
}

impl From<&IcalVEvent> for KhLine {
  fn from(event: &IcalVEvent) -> Self {
    let path = event.get_parent().unwrap().get_path().unwrap().to_path_buf();
    let time = event.get_dtstart();

    KhLine{ path, time }
  }
}

impl From<&IcalVCalendar> for KhLine {
  fn from(cal: &IcalVCalendar) -> Self {
    KhLine::from(&cal.get_principal_event())
  }
}

impl fmt::Display for KhLine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let path_string = self.get_normalized_path().to_string_lossy();
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
      Ok(Self::new(&path, Some(time)))
    } else {
      let path = PathBuf::from(parts[0]);
      Ok(Self::new(&path, None))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use assert_fs::prelude::*;

  use testdata;
  use testutils::*;
  use icalwrap::IcalVCalendar;

  #[test]
  fn test_parse_absolute() {
    let khline_str = "1182988800 /x/y/z.ics";

    let khline = khline_str.parse::<KhLine>().unwrap();

    assert_eq!(PathBuf::from("/x/y/z.ics"), khline.path);
    assert_eq!(1182988800, khline.time.unwrap().timestamp());
    assert_eq!(khline_str, khline.to_string());
  }

  #[test]
  fn test_parse_absolute_no_timestamp() {
    let khline_str = "/x/y/z.ics";

    let khline = khline_str.parse::<KhLine>().unwrap();

    assert_eq!(PathBuf::from("/x/y/z.ics"), khline.path);
    assert_eq!(None, khline.time);
    assert_eq!(khline_str, khline.to_string());
  }

  #[test]
  fn test_parse_relative_no_timestamp() {
    let testdir = prepare_testdir_empty();
    let khline_str = "x/y.ics";

    let khline = khline_str.parse::<KhLine>().unwrap();

    assert_eq!(testdir.child(".khaleesi/cal/x/y.ics").path(), khline.path);
    assert_eq!(None, khline.time);
    assert_eq!(khline_str, khline.to_string());
  }

  #[test]
  fn test_parse_relative_timestamp() {
    let testdir = prepare_testdir_empty();
    let khline_str = "1182988800 x/y.ics";

    let khline = khline_str.parse::<KhLine>().unwrap();

    assert_eq!(testdir.child(".khaleesi/cal/x/y.ics").path(), khline.path);
    assert_eq!(1182988800, khline.time.unwrap().timestamp());
    assert_eq!(khline_str, khline.to_string());
  }

  #[test]
  fn test_khline_from_calendar() {
    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, Some(&path)).unwrap();

    let khline = KhLine::from(&cal.get_principal_event());

    assert_eq!(String::from("1182988800 test/path"), khline.to_string());
  }

  #[test]
  fn test_khline_from_event() {
    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, Some(&path)).unwrap();

    let khline = KhLine::from(&cal);

    assert_eq!(String::from("1182988800 test/path"), khline.to_string());
  }

  #[test]
  fn test_matches() {
    let path = PathBuf::from("test/path");
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, Some(&path)).unwrap();

    let khline = KhLine::from(&cal);

    assert!(khline.matches(&cal.get_principal_event()));
  }

  #[test]
  fn test_to_event_timestamp() {
    let testdir = prepare_testdir("testdir");

    let khline = "12345 twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();
    let event = khline.to_event().unwrap();

    assert_eq!(
      testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").path(),
      event.get_parent().unwrap().get_path().unwrap()
    );
    assert_eq!(12345, event.get_dtstart().unwrap().timestamp());
  }

  #[test]
  fn test_to_event_no_timestamp() {
    let testdir = prepare_testdir("testdir");

    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();
    let event = khline.to_event().unwrap();

    assert_eq!(
      testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").path(),
      event.get_parent().unwrap().get_path().unwrap()
    );
    assert_eq!(
      Utc.ymd(2018, 12, 13).and_hms(22, 30, 00),
      event.get_dtstart().unwrap().with_timezone(&Utc)
    );
  }
}
