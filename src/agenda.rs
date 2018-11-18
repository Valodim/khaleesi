use utils;
use ::icalwrap::*;
use yansi::Style;
use chrono::NaiveTime;

pub fn show_events(lines: &mut Iterator<Item = String>) {
  let style_heading = Style::new().bold();
  let cals = utils::read_calendars_from_files(lines);

  let mut cur_day = cals[0].get_first_event().get_dtstart().date();
  println!("{}", style_heading.paint(cur_day));

  for (i, cal) in cals.iter().enumerate() {
    if cal.get_first_event().get_dtstart().date() != cur_day {
      cur_day = cal.get_first_event().get_dtstart().date();
      println!("{}, {}", style_heading.paint(cur_day.format("%Y-%m-%d")), cur_day.format("%A"));
    }
    println!("{:4}  {}", i, event_line(&cal.get_first_event()));
  }
}

pub fn event_line(event: &IcalVEvent) -> String {
  let mut time_sep = " ";
  let start = if event.get_dtstart().time() == NaiveTime::from_hms(0, 0, 0) {
    " ".repeat(5)
  } else {
    time_sep = "-";
    format!("{}", event.get_dtstart().format("%H:%M"))
  };

  let end = if event.get_dtend().time() == NaiveTime::from_hms(0, 0, 0) {
    " ".repeat(5)
  } else {
    time_sep = "-";
    format!("{}", event.get_dtend().format("%H:%M"))
  };

  let summary = event.get_summary().unwrap_or(String::from("?"));

  format!("{:5}{}{:5}  {}", start, time_sep, end, summary)
}
