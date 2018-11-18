use utils;
use ::icalwrap::*;
use yansi::Style;
use chrono::NaiveTime;

pub fn show_events(lines: &mut Iterator<Item = String>) {
  let style_heading = Style::new().bold();
  let comps = utils::read_calendars_from_files(lines);

  let mut cur_day = comps[0].get_dtstart().date();
  println!("{}", style_heading.paint(cur_day));

  for (i, comp) in comps.iter().enumerate() {
    if comp.get_dtstart().date() != cur_day {
      cur_day = comp.get_dtstart().date();
      println!("{}, {}", style_heading.paint(cur_day.format("%Y-%m-%d")), cur_day.format("%A"));
    }
    println!("{:4}  {}", i, event_line(comp));
  }
}

pub fn event_line(comp: &Icalcomponent) -> String {
  let mut time_sep = " ";
  let start = if comp.get_dtstart().time() == NaiveTime::from_hms(0, 0, 0) {
    " ".repeat(5)
  } else {
    time_sep = "-";
    format!("{}", comp.get_dtstart().format("%H:%M"))
  };

  let end = if comp.get_dtend().time() == NaiveTime::from_hms(0, 0, 0) {
    " ".repeat(5)
  } else {
    time_sep = "-";
    format!("{}", comp.get_dtend().format("%H:%M"))
  };

  let summary = comp.get_summary().unwrap_or(String::from("?"));

  format!("{:5}{}{:5}  {}", start, time_sep, end, summary)
}
