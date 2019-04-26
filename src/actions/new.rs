use crate::calendars;
use crate::cursorfile;
use crate::defaults;
use crate::icalwrap::{IcalTime, IcalTimeZone, IcalVCalendar};
use crate::khline::KhLine;
use crate::utils::{fileutil, misc};
use crate::KhResult;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct NewArgs {
  /// the calendar
  #[structopt(name = "calendar")]
  pub calendar: String,
  /// from
  #[structopt(name = "from")]
  pub from: String,
  /// to
  #[structopt(name = "to")]
  pub to: String,
  /// summary
  #[structopt(name = "summary")]
  pub summary: String,
  /// location
  #[structopt(name = "location")]
  pub location: String,
}

struct EventProperties {
  calendar: String,
  from: IcalTime,
  to: IcalTime,
  summary: String,
  location: String,
}

impl EventProperties {
  fn parse_from_args(args: &NewArgs) -> KhResult<EventProperties> {
    let calendar = EventProperties::parse_calendar(&args.calendar)?;
    let from = EventProperties::parse_from(&args.from)?;
    let to = EventProperties::parse_to(&args.to)?;
    let summary = EventProperties::parse_summary(&args.summary)?;
    let location = EventProperties::parse_location(&args.location)?;
    Ok(EventProperties {
      calendar,
      from,
      to,
      summary,
      location,
    })
  }

  fn parse_from(arg: &str) -> KhResult<IcalTime> {
    if arg.is_empty() {
      Err("no start date/time given")?
    };
    let time = arg.parse::<IcalTime>()?;
    Ok(time)
  }

  fn parse_to(arg: &str) -> KhResult<IcalTime> {
    if arg.is_empty() {
      Err("no end date/time given")?
    };
    let time = arg.parse::<IcalTime>()?;
    Ok(time)
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

pub fn do_new(args: &NewArgs) -> KhResult<()> {
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
      .with_dtstart(&ep.from)
      .with_dtend(&ep.to)
      .with_summary(&ep.summary)
      .with_location(&ep.location)
  }
}

static TEMPLATE_EVENT: &str = indoc!(
  "
  BEGIN:VCALENDAR
  VERSION:2.0
  PRODID:-//khaleesi //EN
  BEGIN:VTIMEZONE
  TZID:Europe/Berlin
  BEGIN:STANDARD
  DTSTART:19711025T030000
  TZOFFSETFROM:+0200
  TZOFFSETTO:+0100
  RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10
  END:STANDARD
  BEGIN:DAYLIGHT
  DTSTART:19710329T020000
  TZOFFSETFROM:+0100
  TZOFFSETTO:+0200
  RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=3
  END:DAYLIGHT
  END:VTIMEZONE
  BEGIN:VEVENT
  SUMMARY:<<EDIT ME>>
  LOCATION:<<EDIT ME>>
  DTSTART;TZID=Europe/Berlin;VALUE=DATE-TIME:20181026T133000
  DTEND;TZID=Europe/Berlin;VALUE=DATE-TIME:20181026T160000
  DTSTAMP;VALUE=DATE-TIME:20181022T145405Z
  UID:foo
  END:VEVENT
  END:VCALENDAR
"
);

#[cfg(test)]
mod integration {
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  use super::*;
  use crate::testdata;
  use crate::testutils;

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
    let from = EventProperties::parse_from("2017-07-14T17:45:00").unwrap();
    let expected = IcalTime::floating_ymd(2017, 7, 14).and_hms(17, 45, 0);
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
    let to = EventProperties::parse_to("2017-07-14T17:45:00").unwrap();
    let expected = IcalTime::floating_ymd(2017, 7, 14).and_hms(17, 45, 0);
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
    let args = NewArgs {
      calendar: "second".to_string(),
      from: "2017-11-03T12:30:00".to_string(),
      to: "2017-11-07T11:11:00".to_string(),
      summary: "summary text".to_string(),
      location: "location text".to_string(),
    };
    let ep = EventProperties::parse_from_args(&args).unwrap();
    assert_eq!("second".to_string(), ep.calendar);
    assert_eq!("summary text".to_string(), ep.summary);
    assert_eq!("location text".to_string(), ep.location);
  }

  //#[test]
  //fn test_parse_from_args_neg() {
  //  let args = &["1", "2", "3", "4"];
  //  let ep = EventProperties::parse_from_args(args);
  //  assert!(ep.is_err());
  //}

  #[test]
  fn test_with_eventprops() {
    testdata::setup();

    let calendar = "foo".to_string();
    let from = IcalTime::floating_ymd(2015, 04, 17).and_hms(8, 17, 3);
    let to = IcalTime::floating_ymd(2015, 05, 17).and_hms(8, 17, 3);
    let summary = "summary";
    let location = "home";
    let ep = EventProperties {
      calendar,
      from: from.clone(),
      to: to.clone(),
      summary: summary.to_string(),
      location: location.to_string(),
    };

    let _testdir = testutils::prepare_testdir("testdir");
    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    let cal = khline.to_cal().unwrap().with_eventprops(&ep);

    let event = cal.get_principal_khevent();
    assert_eq!(Some(from), event.get_start());
    assert_eq!(Some(to), event.get_end());
    assert_eq!(summary, event.get_summary().unwrap());
    assert_eq!(location, event.get_location().unwrap());
  }

  #[test]
  fn test_do_new() {
    testdata::setup();
    let testdir = testutils::prepare_testdir("testdir_two_cals");

    let args = NewArgs {
      calendar: "second".to_string(),
      from: "2017-11-03T12:30:00".to_string(),
      to: "2017-11-07T11:11:00".to_string(),
      summary: "summary text".to_string(),
      location: "location text".to_string(),
    };

    let result = do_new(&args);
    assert!(result.is_ok());

    let expected = indoc!(
      "
      BEGIN:VCALENDAR
      VERSION:2.0
      PRODID:-//khaleesi //EN
      BEGIN:VTIMEZONE
      TZID:Europe/Berlin
      BEGIN:STANDARD
      DTSTART:19711025T030000
      TZOFFSETFROM:+0200
      TZOFFSETTO:+0100
      RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10
      END:STANDARD
      BEGIN:DAYLIGHT
      DTSTART:19710329T020000
      TZOFFSETFROM:+0100
      TZOFFSETTO:+0200
      RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=3
      END:DAYLIGHT
      END:VTIMEZONE
      BEGIN:VEVENT
      SUMMARY:summary text
      LOCATION:location text
      DTSTART;TZID=Europe/Berlin:20171103T123000
      DTEND;TZID=Europe/Berlin:20171107T111100
      DTSTAMP:20130101T010203Z
      UID:11111111-2222-3333-4444-444444444444@khaleesi
      LAST-MODIFIED:20130101T010203Z
      END:VEVENT
      END:VCALENDAR
    "
    )
    .replace("\n", "\r\n");
    let predicate = predicate::str::similar(expected);
    testdir
      .child(".khaleesi/cal/second/11111111-2222-3333-4444-444444444444@khaleesi.ics")
      .assert(predicate);

    let cursor_expected = "1509708600 second/11111111-2222-3333-4444-444444444444@khaleesi.ics";
    testdir.child(".khaleesi/cursor").assert(cursor_expected);
  }
}
