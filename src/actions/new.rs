use defaults;
use icalwrap::IcalVCalendar;
use khline::KhLine;
use utils::fileutil;
use utils::misc;

use KhResult;

pub fn do_new(_args: &[&str]) -> KhResult<()> {
  let uid = misc::make_new_uid();
  let path = defaults::get_datafile(&(uid.clone() + ".ics"));

  let new_cal = IcalVCalendar::from_str(TEMPLATE_EVENT, Some(&path))?.with_uid(&uid)?;
  let new_cal = new_cal.with_dtstamp_now();

  fileutil::write_cal(&new_cal)?;

  let khline = KhLine::from(&new_cal);
  println!("{}", khline);

  Ok(())
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
