use chrono::{Datelike, TimeZone, Local, Date};
use itertools::Itertools;
use yansi::{Style};

use cursorfile;
use icalwrap::*;
use input;
use utils::fileutil as utils;
use config::{Config,CalendarConfig};
use khline::KhLine;
use KhResult;

pub fn show_events(config: &Config, _args: &[&str]) -> KhResult<()> {
  let mut lines = input::default_input_multiple()?;

  let cursor = cursorfile::read_cursorfile().ok();
  show_events_cursor(config, &mut lines, cursor.as_ref());

  Ok(())
}

pub fn show_events_cursor(
  config: &Config,
  lines: &mut Iterator<Item = String>,
  cursor: Option<&KhLine>,
) {
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut not_over_yet: Vec<(usize, &IcalVCalendar, IcalVEvent, Option<&CalendarConfig>)> = Vec::new();
  let mut cals_iter = cals
    .iter()
    .enumerate()
    .map(|(i, cal)| {
      let event = cal.get_principal_event();
      let config = cal.get_calendar_name().and_then(|name| config.get_config_for_calendar(&name));
      (i, cal, event, config)
    })
    .peekable();

  let start_day = match cals_iter.peek() {
    Some((_, _, event, _)) => {
      event
        .get_dtstart()
        .unwrap_or_else(|| Local.timestamp(0, 0))
        .date()
    }
    None => return,
  };

  let mut cur_day = start_day.pred();
  let mut last_printed_day = start_day.pred();
  while cals_iter.peek().is_some() || !not_over_yet.is_empty() {
    cur_day = cur_day.succ();

    maybe_print_date_line_header(&config, cur_day, start_day, &mut last_printed_day);

    not_over_yet.retain( |(index, _, event, cal_config)| {
      let is_cursor = cursor.map(|c| c.matches(&event)).unwrap_or(false);
      maybe_print_date_line(&config, cur_day, start_day, &mut last_printed_day);
      print_event_line(*cal_config, *index, &event, cur_day, is_cursor);
      event.continues_after(cur_day)
    });

    let relevant_events = cals_iter.peeking_take_while(|(_,_,event,_)| event.starts_on(cur_day));
    for (i, cal, event, cal_config) in relevant_events {
      let is_cursor = cursor.map(|c| c.matches(&event)).unwrap_or(false);
      maybe_print_date_line(&config, cur_day, start_day, &mut last_printed_day);
      print_event_line(cal_config, i, &event, cur_day, is_cursor);
      if event.continues_after(cur_day) {
        not_over_yet.push((i, cal, event, cal_config));
      }
    }
  }
}

fn maybe_print_week_separator(config: &Config, date: Date<Local>, start_date: Date<Local>, last_printed_date: Date<Local>) {
  if !config.agenda.print_week_separator {
    return;
  }
  if date != start_date && last_printed_date.iso_week() < date.iso_week() {
    khprintln!();
  }
}

fn maybe_print_date_line_header(config: &Config, date: Date<Local>, start_date: Date<Local>, last_printed_date: &mut Date<Local>) {
  if !config.agenda.print_empty_days {
    return;
  }
  maybe_print_date_line(config, date, start_date, last_printed_date);
}

fn maybe_print_date_line(config: &Config, date: Date<Local>, start_date: Date<Local>, last_printed_date: &mut Date<Local>) {
  if date <= *last_printed_date {
    return;
  }
  maybe_print_week_separator(config, date, start_date, *last_printed_date);
  print_date_line(date);
  *last_printed_date = date;
}

fn print_date_line(date: Date<Local>) {
  let style_heading = Style::new().bold();
  khprintln!("{}, {}", style_heading.paint(date.format("%Y-%m-%d")), date.format("%A"));
}

fn print_event_line(
  config: Option<&CalendarConfig>,
  index: usize,
  event: &IcalVEvent,
  date: Date<Local>,
  is_cursor: bool
) {
  match event_line(config, &event, date, is_cursor) {
    Ok(line) => khprintln!("{:4}  {}", index, line),
    Err(error) => warn!("{} in {}", error, event.get_uid())
  }
}

pub fn event_line(
  config: Option<&CalendarConfig>,
  event: &IcalVEvent,
  cur_day: Date<Local>,
  is_cursor: bool
) -> Result<String, String> {
  if !event.relevant_on(cur_day) {
    return Err(format!("event is not relevant for {:?}", cur_day));
  }

  let mut summary = event.get_summary().ok_or("Invalid SUMMARY")?;
  if let Some(config) = config {
    let calendar_style = config.get_style_for_calendar();
    summary = calendar_style.paint(summary).to_string();
  }

  let cursor_icon = if is_cursor { ">" } else { "" };

  if event.is_allday() {
    Ok(format!("{:3}             {}", cursor_icon, summary))
  } else {
    let mut time_sep = " ";
    let dtstart = event.get_dtstart().ok_or("Invalid DTSTART")?;
    let start_string = if dtstart.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtstart.format("%H:%M"))
    };

    let dtend = event.get_dtend().ok_or("Invalid DTEND")?;
    let end_string = if dtend.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtend.format("%H:%M"))
    };

    Ok(format!("{:3}{:5}{}{:5}  {}", cursor_icon, start_string, time_sep, end_string, summary))
  }
}

impl IcalVEvent {
  fn starts_on(&self, date: Date<Local>) -> bool {
    self.get_dtstart().unwrap().date() == date
  }

  fn relevant_on(&self, date: Date<Local>) -> bool {
    self.get_dtstart().map(|dtstart| dtstart.date() <= date).unwrap_or(false) &&
    self.get_last_relevant_date().map(|enddate| enddate >= date).unwrap_or(false)
  }

  fn continues_after(&self, date: Date<Local>) -> bool {
    self.get_last_relevant_date()
      .map(|enddate| enddate > date)
      .unwrap_or(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use testdata;
  use testutils::*;
  use config::Config;

  use chrono::{Local, TimeZone};

  #[test]
  fn test_starts_on() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();

    let first_day = Local.ymd(2007, 6, 28);
    assert!(event.starts_on(first_day));

    let last_day = Local.ymd(2007, 7, 7);
    assert!(!event.starts_on(last_day));
  }

  #[test]
  fn test_continues_after_allday() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, None).unwrap();
    let event = cal.get_principal_event();
    let first_day = Local.ymd(2007, 6, 28);
    assert!(event.continues_after(first_day));
    let last_day = Local.ymd(2007, 7, 8);
    assert!(!event.continues_after(last_day));
  }

  #[test]
  fn test_continues_after_simple() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(1997, 3, 24);
    assert!(!event.continues_after(date));
  }

  #[test]
  fn test_event_line_negative() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(1998, 1, 1);
    let event_line = event_line(None, &event, date, false);
    assert!(event_line.is_err())
  }

  #[test]
  fn test_event_line_simple() {
    testdata::setup();
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(1997, 3, 24);
    let event_line = event_line(None, &event, date, false).unwrap();
    assert_eq!("   12:30-21:00  Calendaring Interoperability Planning Meeting".to_string(), event_line)
  }

  #[test]
  fn test_event_line_cursor() {
    testdata::setup();
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(1997, 3, 24);
    let event_line = event_line(None, &event, date, true).unwrap();
    assert_eq!(">  12:30-21:00  Calendaring Interoperability Planning Meeting".to_string(), event_line)
  }

  #[test]
  fn test_event_line_multiday() {
    testdata::setup();
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();
    let begin = Local.ymd(2007, 6, 28);
    let middle = Local.ymd(2007, 6, 30);
    let end = Local.ymd(2007, 7, 9);
    let event_line_begin = event_line(None, &event, begin, false).unwrap();
    let event_line_middle = event_line(None, &event, middle, false).unwrap();
    let event_line_end = event_line(None, &event, end, false).unwrap();
    assert_eq!("   13:29-       Festival International de Jazz de Montreal".to_string(), event_line_begin);
    assert_eq!("                Festival International de Jazz de Montreal".to_string(), event_line_middle);
    assert_eq!("        -07:29  Festival International de Jazz de Montreal".to_string(), event_line_end);
  }

  #[test]
  fn test_event_line_multiday_allday() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(2007, 6, 28);
    let event_line = event_line(None, &event, date, false).unwrap();
    assert_eq!("                Festival International de Jazz de Montreal".to_string(), event_line)
  }

  #[test]
  fn test_stdout_simple() {
    testdata::setup();
    let _testdir = prepare_testdir("testdir_with_seq");

    show_events(&Config::read_config(), &[]).unwrap();

    let stdout = test_stdout_clear();
    let expected = indoc!("
      2018-12-13, Thursday
         0     22:30-       shows up on two days
      2018-12-14, Friday
         0                  shows up on two days
      2018-12-15, Saturday
         0                  shows up on two days
      2018-12-16, Sunday
         0                  shows up on two days
      2018-12-17, Monday
         0          -18:30  shows up on two days
   ");
    assert_eq!(expected, stdout);
  }
}
