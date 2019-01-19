use defaults;
use icalwrap::IcalVCalendar;
use khline::KhLine;
use utils::{misc,fileutil,dateutil};
use chrono::{DateTime,Local};

use KhResult;
use cursorfile;

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
    let calendar = args[0].to_string();
    let start = dateutil::datetime_from_str(args[1])?;
    let end = dateutil::datetime_from_str(args[2])?;
    let summary = args[3].to_string();
    let location = args[4].to_string();
    Ok(EventProperties{ calendar, start, end, summary, location })
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
    .with_eventprops(ep);

  fileutil::write_cal(&new_cal)?;

  let khline = KhLine::from(&new_cal);
  cursorfile::write_cursorfile(&khline.to_string())?;
  println!("{}", khline);

  Ok(())
}

impl IcalVCalendar {
  fn with_eventprops(self, ep: EventProperties) -> Self {
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
