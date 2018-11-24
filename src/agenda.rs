use utils;
use icalwrap::*;
use yansi::Style;
use chrono::{NaiveDateTime, NaiveTime};

pub fn show_events(lines: &mut Iterator<Item = String>) {
  let style_heading = Style::new().bold();
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut cur_day = cals[0].get_first_event().get_dtstart()
    .unwrap_or(NaiveDateTime::from_timestamp(0, 0))
    .date();
  println!("{}", style_heading.paint(cur_day));

  for (i, cal) in cals.iter().enumerate() {

    if let Some(start) = cal.get_first_event().get_dtstart() {
      if start.date() != cur_day {
        cur_day = start.date();
        println!("{}, {}", style_heading.paint(cur_day.format("%Y-%m-%d")), cur_day.format("%A"));
      }
      match event_line(&cal.get_first_event()) {
        Ok(line) => println!("{:4}  {}", i, line),
        Err(error) => warn!("{} in {}", error, cal.get_first_event().get_uid())
      }
    } else {
      warn!("Invalid DTSTART in {}", cal.get_first_event().get_uid());
    };

  }
}

pub fn event_line(event: &IcalVEvent) -> Result<String, String> {
  let mut time_sep = " ";
  let dtstart = event.get_dtstart().ok_or("Invalid DTSTART")?;
  let start_string = if dtstart.time() == NaiveTime::from_hms(0, 0, 0) {
    "".to_string()
  } else {
    time_sep = "-";
    format!("{}", dtstart.format("%H:%M"))
  };

  let dtend = event.get_dtend().ok_or("Invalid DTEND")?;
  let end_string = if dtend.time() == NaiveTime::from_hms(0, 0, 0) {
    "".to_string()
  } else {
    time_sep = "-";
    format!("{}", dtend.format("%H:%M"))
  };

  let summary = event.get_summary().ok_or("Invalid SUMMARY")?;

  Ok(format!("{:5}{}{:5}  {}", start_string, time_sep, end_string, summary))
}
