use defaults;
use icalwrap::IcalVCalendar;
use khline::KhLine;
use utils::{misc,fileutil,dateutil};
use chrono::{DateTime,Local};

use KhResult;
use cursorfile;
use calendars;

struct EventProperties {
  calendar: String,
  start: DateTime<Local>,
  end: DateTime<Local>,
  summary: String,
  location: String
}

impl EventProperties {
  fn parse_from_args(args: &[&str]) -> KhResult<EventProperties> {
    if args.len() < 3 {
      Err("new calendar from to summary location")?
    }
    let calendar = EventProperties::parse_calendar(args[0])?;
    let start = EventProperties::parse_start(args[1])?;
    let end = EventProperties::parse_end(args[2])?;
    let summary = EventProperties::parse_summary(args[3])?;
    let location = EventProperties::parse_location(args[4])?;
    Ok(EventProperties{ calendar, start, end, summary, location })
  }

  fn parse_start(arg: &str) -> KhResult<DateTime<Local>> {
    if arg.is_empty() {
      Err("no start time given")?
    };
    Ok(dateutil::datetime_from_str(arg)?)
  }

  fn parse_end(arg: &str) -> KhResult<DateTime<Local>> {
    if arg.is_empty() {
      Err("no end time given")?
    };
    Ok(dateutil::datetime_from_str(arg)?)
  }

  fn parse_location(arg: &str) -> KhResult<String> {
    if arg.is_empty() {
      Err("no location given")?
    };
    Ok(arg.to_string())
  }

  fn parse_summary(arg: &str) -> KhResult<String> {
    if arg.is_empty() {
      Err("no summary given")?
    };
    Ok(arg.to_string())
  }

  fn parse_calendar(arg: &str) -> KhResult<String> {
    if arg.is_empty() {
      Err("no calendar given")?
    };
    let cal = arg.to_string();
    if !calendars::calendar_list().contains(&cal) {
      Err("calendar does not exist")?
    }
    Ok(cal)
  }
}

pub fn do_new(args: &[&str]) -> KhResult<()> {
  let uid = misc::make_new_uid();
  let ep = EventProperties::parse_from_args(args)?;

  let mut path = defaults::get_caldir();
  path.push(&ep.calendar);
  path.push(&(uid.clone() + ".ics"));

  let new_cal = IcalVCalendar::from_str(TEMPLATE_EVENT, Some(&path))?
    .with_uid(&uid)?
    .with_dtstamp_now()
    .with_last_modified_now()
    .with_eventprops(&ep);

  let khline = KhLine::from(&new_cal);

  fileutil::write_cal(&new_cal)?;

  cursorfile::write_cursorfile(&khline.to_string())?;
  khprintln!("{}", khline);

  Ok(())
}

impl IcalVCalendar {
  fn with_eventprops(self, ep: &EventProperties) -> Self {
    self
      .with_dtstart(&ep.start)
      .with_dtend(&ep.end)
      .with_summary(&ep.summary)
      .with_location(&ep.location)
  }
}

static TEMPLATE_EVENT: &str = indoc!("
  BEGIN:VCALENDAR
  VERSION:2.0
  PRODID:-//khaleesi //EN
  BEGIN:VEVENT
  SUMMARY:<<EDIT ME>>
  LOCATION:<<EDIT ME>>
  DTSTART;VALUE=DATE-TIME:20181026T133000
  DTEND;VALUE=DATE-TIME:20181026T160000
  DTSTAMP;VALUE=DATE-TIME:20181022T145405Z
  UID:foo
  END:VEVENT
  END:VCALENDAR
");

#[cfg(test)]
mod tests {
  use super::*;
  use testutils;
  use chrono::{TimeZone,Local};

  #[test]
  fn test_parse_calendar() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");
    let calendar = EventProperties::parse_calendar("first").unwrap();
    assert_eq!("first", calendar);
  }

  #[test]
  fn test_parse_calendar_neg() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");
    let calendar = EventProperties::parse_calendar("");
    assert!(calendar.is_err());
    let calendar = EventProperties::parse_calendar("foo");
    assert!(calendar.is_err());
  }

  #[test]
  fn test_parse_location() {
    let location = EventProperties::parse_summary("room 101").unwrap();
    assert_eq!("room 101", location);
  }

  #[test]
  fn test_parse_location_neg() {
    let location = EventProperties::parse_calendar("");
    assert!(location.is_err());
  }

  #[test]
  fn test_parse_summary() {
    let summary = EventProperties::parse_summary("first").unwrap();
    assert_eq!("first", summary);
  }

  #[test]
  fn test_parse_summary_neg() {
    let summary = EventProperties::parse_calendar("");
    assert!(summary.is_err());
  }

  #[test]
  fn test_parse_start() {
    let start = EventProperties::parse_start("2017-07-14T17:45").unwrap();
    let expected = Local.ymd(2017, 7, 14).and_hms(17, 45, 0);
    assert_eq!(expected, start);
  }

  #[test]
  fn test_parse_start_neg() {
    let start = EventProperties::parse_start("45");
    assert!(start.is_err());
  }

  #[test]
  fn test_parse_end() {
    let end = EventProperties::parse_end("2017-07-14T17:45").unwrap();
    let expected = Local.ymd(2017, 7, 14).and_hms(17, 45, 0);
    assert_eq!(expected, end);
  }

  #[test]
  fn test_parse_end_neg() {
    let end = EventProperties::parse_end("45");
    assert!(end.is_err());
  }

  #[test]
  fn test_with_eventprops() {
    let calendar = "foo".to_string();
    let start = Local.ymd(2015, 04, 17).and_hms(8, 17, 3);
    let end = Local.ymd(2015, 05, 17).and_hms(8, 17, 3);
    let summary = "summary";
    let location = "home";
    let ep = EventProperties { calendar, start, end, summary: summary.to_string(), location: location.to_string() };

    let _testdir = testutils::prepare_testdir("testdir");
    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    let cal = khline.to_cal().unwrap()
      .with_eventprops(&ep);

    let event = cal.get_principal_event();
    assert_eq!(start, event.get_dtstart().unwrap());
    assert_eq!(end, event.get_dtend().unwrap());
    assert_eq!(summary, event.get_summary().unwrap());
    assert_eq!(location, event.get_location().unwrap());
  }
}
