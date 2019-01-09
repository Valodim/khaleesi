use chrono::{DateTime, Datelike, TimeZone, Local, Date};
use itertools::Itertools;
use yansi::{Style};

use icalwrap::*;
use utils;
use config::{Config,CalendarConfig};

pub fn show_events(config: &Config, lines: &mut Iterator<Item = String>) {
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut not_over_yet: Vec<(usize, &IcalVCalendar, IcalVEvent, Option<&CalendarConfig>)> = Vec::new();
  let mut cals_iter = cals.iter()
    .enumerate()
    .map(|(i, cal)| (i, cal, cal.get_principal_event(), config.get_config_for_calendar(&cal)))
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
      maybe_print_date_line(&config, cur_day, start_day, &mut last_printed_day);
      print_event_line(*cal_config, *index, &event, cur_day);
      event.continues_after(cur_day)
    });

    let relevant_events = cals_iter.peeking_take_while(|(_,_,event,_)| event.starts_on(cur_day));
    for (i, cal, event, cal_config) in relevant_events {
      maybe_print_date_line(&config, cur_day, start_day, &mut last_printed_day);
      print_event_line(cal_config, i, &event, cur_day);
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
    println!();
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
  println!("{}, {}", style_heading.paint(date.format("%Y-%m-%d")), date.format("%A"));
}

fn print_event_line(config: Option<&CalendarConfig>, index: usize, event: &IcalVEvent, date: Date<Local>) {
  match event_line(config, &event, date) {
    Ok(line) => println!("{:4}  {}", index, line),
    Err(error) => warn!("{} in {}", error, event.get_uid())
  }
}

pub fn event_line(config: Option<&CalendarConfig>, event: &IcalVEvent, cur_day: Date<Local>) -> Result<String, String> {
  if !event.relevant_on(cur_day) {
    return Err(format!("event is not relevant for {:?}", cur_day));
  }

  if event.is_allday() {
    let mut summary = event.get_summary().ok_or("Invalid SUMMARY")?;
    if let Some(config) = config {
      let calendar_style = config.get_style_for_calendar();
      summary = calendar_style.paint(summary).to_string();
    }
    Ok(format!("             {}", summary))
  } else {
    let mut time_sep = " ";
    let dtstart = event.get_dtstart_for_event_line().ok_or("Invalid DTSTART")?;
    let start_string = if dtstart.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtstart.format("%H:%M"))
    };

    let dtend = event.get_dtend_for_event_line().ok_or("Invalid DTEND")?;
    let end_string = if dtend.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtend.format("%H:%M"))
    };

    let mut summary = event.get_summary().ok_or("Invalid SUMMARY")?;

    if let Some(config) = config {
      let calendar_style = config.get_style_for_calendar();
      summary = calendar_style.paint(summary).to_string();
    }

    Ok(format!("{:5}{}{:5}  {}", start_string, time_sep, end_string, summary))
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

  fn get_dtend_for_event_line(&self)  -> Option<DateTime<Local>> {
    if cfg!(test) {
      Some(self.get_dtend()?.date().and_hms(22, 29, 0))
    } else {
      self.get_dtend()
    }
  }

  fn get_dtstart_for_event_line(&self)  -> Option<DateTime<Local>> {
    if cfg!(test) {
      Some(self.get_dtstart()?.date().and_hms(7, 29, 0))
    } else {
      self.get_dtstart()
    }
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
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
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
  fn test_event_line_simple() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(1997, 3, 24);
    let event_line = event_line(None, &event, date).unwrap();
    assert_eq!("07:29-22:29  Calendaring Interoperability Planning Meeting".to_string(), event_line)
  }

  #[test]
  fn test_event_line_multiday() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();
    let date = Local.ymd(2007, 6, 28);
    let event_line = event_line(None, &event, date).unwrap();
    assert_eq!("             Festival International de Jazz de Montreal".to_string(), event_line)
  }
}
