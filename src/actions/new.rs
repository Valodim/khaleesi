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
  from: DateTime<Local>,
  to: DateTime<Local>,
  summary: String,
  location: String
}

impl EventProperties {
  fn parse_from_args(args: &[&str]) -> KhResult<EventProperties> {
    if args.len() < 3 {
      Err("new calendar from to summary location")?
    }
    let calendar = EventProperties::parse_calendar(args[0])?;
    let from = EventProperties::parse_from(args[1])?;
    let to = EventProperties::parse_to(args[2])?;
    let summary = EventProperties::parse_summary(args[3])?;
    let location = EventProperties::parse_location(args[4])?;
    Ok(EventProperties{ calendar, from, to, summary, location })
  }

  fn parse_from(arg: &str) -> KhResult<DateTime<Local>> {
    if arg.is_empty() {
      Err("no start date/time given")?
    };
    Ok(dateutil::datetime_from_str(arg)?)
  }

  fn parse_to(arg: &str) -> KhResult<DateTime<Local>> {
    if arg.is_empty() {
      Err("no end date/time given")?
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
      .with_dtstart(&ep.from.into())
      .with_dtend(&ep.to.into())
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
  use assert_fs::prelude::*;
  use chrono::{TimeZone,Local};
  use predicates::prelude::*;

  use super::*;
  use testutils;
  use testdata;

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
    let location = EventProperties::parse_location("room 101").unwrap();
    assert_eq!("room 101", location);
  }

  #[test]
  fn test_parse_location_neg() {
    let location = EventProperties::parse_location("");
    assert!(location.is_err());
  }

  #[test]
  fn test_parse_summary() {
    let summary = EventProperties::parse_summary("first").unwrap();
    assert_eq!("first", summary);
  }

  #[test]
  fn test_parse_summary_neg() {
    let summary = EventProperties::parse_summary("");
    assert!(summary.is_err());
  }

  #[test]
  fn test_parse_from() {
    testdata::setup();
    let from = EventProperties::parse_from("2017-07-14T17:45").unwrap();
    let expected = Local.ymd(2017, 7, 14).and_hms(17, 45, 0);
    assert_eq!(expected, from);
  }

  #[test]
  fn test_parse_from_neg() {
    let from = EventProperties::parse_from("blödsinn");
    assert!(from.is_err());
    let from = EventProperties::parse_from("");
    assert!(from.is_err());
  }

  #[test]
  fn test_parse_to() {
    testdata::setup();
    let to = EventProperties::parse_to("2017-07-14T17:45").unwrap();
    let expected = Local.ymd(2017, 7, 14).and_hms(17, 45, 0);
    assert_eq!(expected, to);
  }

  #[test]
  fn test_parse_to_neg() {
    let to = EventProperties::parse_to("quatsch");
    assert!(to.is_err());
    let to = EventProperties::parse_to("");
    assert!(to.is_err());
  }

  #[test]
  fn test_parse_from_args() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");
    let args = &["second", "2017-11-03T12:30", "2017-11-07T11:11", "summary text", "location text"];
    let ep = EventProperties::parse_from_args(args).unwrap();
    assert_eq!("second".to_string(), ep.calendar);
    assert_eq!("summary text".to_string(), ep.summary);
    assert_eq!("location text".to_string(), ep.location);
  }

  #[test]
  fn test_parse_from_args_neg() {
    let args = &["1", "2", "3", "4"];
    let ep = EventProperties::parse_from_args(args);
    assert!(ep.is_err());
  }

  #[test]
  fn test_with_eventprops() {
    testdata::setup();

    let calendar = "foo".to_string();
    let from = Local.ymd(2015, 04, 17).and_hms(8, 17, 3);
    let to = Local.ymd(2015, 05, 17).and_hms(8, 17, 3);
    let summary = "summary";
    let location = "home";
    let ep = EventProperties { calendar, from, to, summary: summary.to_string(), location: location.to_string() };

    let _testdir = testutils::prepare_testdir("testdir");
    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    let cal = khline.to_cal().unwrap()
      .with_eventprops(&ep);

    let event = cal.get_principal_event();
    assert_eq!(from, Into::<DateTime<Local>>::into(event.get_dtstart().unwrap()));
    assert_eq!(to, Into::<DateTime<Local>>::into(event.get_dtend().unwrap()));
    assert_eq!(summary, event.get_summary().unwrap());
    assert_eq!(location, event.get_location().unwrap());
  }

  #[test]
  fn test_do_new() {
    let testdir = testutils::prepare_testdir("testdir_two_cals");

    let args = &["second", "2017-11-03T12:30", "2017-11-07T11:11", "summary text", "location text"];

    let result = do_new(args);
    assert!(result.is_ok());

    let expected = indoc!("
      BEGIN:VCALENDAR
      VERSION:2.0
      PRODID:-//khaleesi //EN
      BEGIN:VEVENT
      SUMMARY:summary text
      LOCATION:location text
      DTSTART;TZID=/freeassociation.sourceforge.net/Europe/Berlin:
       20171103T123000
      DTEND;TZID=/freeassociation.sourceforge.net/Europe/Berlin:
       20171107T111100
      DTSTAMP:20130101T010203Z
      UID:11111111-2222-3333-4444-444444444444@khaleesi
      LAST-MODIFIED:20130101T010203Z
      END:VEVENT
      END:VCALENDAR
    ").replace("\n", "\r\n");
    let predicate = predicate::str::similar(expected);
    testdir.child(".khaleesi/cal/second/11111111-2222-3333-4444-444444444444@khaleesi.ics").assert(predicate);

    let cursor_expected = "1509708600 second/11111111-2222-3333-4444-444444444444@khaleesi.ics";
    testdir.child(".khaleesi/cursor").assert(cursor_expected);
  }
}
