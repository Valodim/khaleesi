use utils;
use icalwrap::*;
use yansi::{Style};
use config::{self,Config,CalendarConfig};
use chrono::{Datelike, TimeZone, Local, Date};
use itertools::Itertools;

pub fn show_events(config: Config, lines: &mut Iterator<Item = String>) {
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut not_over_yet: Vec<(usize, &IcalVCalendar, IcalVEvent, Option<&CalendarConfig>)> = Vec::new();
  let mut cals_iter = cals.iter()
    .enumerate()
    .map(|(i, cal)| (i, cal, cal.get_principal_event(), cal.get_config(&config)))
    .peekable();

  let start_day = match cals_iter.peek() {
    Some((_, _, event, _)) => {
      event
        .get_dtstart()
        .unwrap_or(Local.timestamp(0, 0))
        .date()
    }
    None => return,
  };

  let mut cur_day = start_day.pred();
  let mut last_printed_day = start_day.pred();
  while !cals_iter.peek().is_none() || !not_over_yet.is_empty() {
    cur_day = cur_day.succ();

    maybe_print_date_line_header(&config, &cur_day, &start_day, &mut last_printed_day);

    not_over_yet.retain( |(index, _, event, cal_config)| {
      maybe_print_date_line(&config, &cur_day, &start_day, &mut last_printed_day);
      print_event_line(*cal_config, &index, &event, &cur_day);
      event.continues_after(&cur_day)
    });

    let relevant_events = cals_iter.peeking_take_while(|(_,_,event,_)| event.relevant_on(&cur_day));
    for (i, cal, event, cal_config) in relevant_events {
      maybe_print_date_line(&config, &cur_day, &start_day, &mut last_printed_day);
      print_event_line(cal_config, &i, &event, &cur_day);
      if event.continues_after(&cur_day) {
        not_over_yet.push((i, cal, event, cal_config));
      }
    }
  }
}

fn maybe_print_week_separator(config: &Config, date: &Date<Local>, start_date: &Date<Local>, last_printed_date: &Date<Local>) {
  if !config.agenda.print_week_separator {
    return;
  }
  if date != start_date && last_printed_date.iso_week() < date.iso_week() {
    println!();
  }
}

fn maybe_print_date_line_header(config: &Config, date: &Date<Local>, start_date: &Date<Local>, last_printed_date: &mut Date<Local>) {
  if !config.agenda.print_empty_days {
    return;
  }
  maybe_print_date_line(config, date, start_date, last_printed_date);
}

fn maybe_print_date_line(config: &Config, date: &Date<Local>, start_date: &Date<Local>, last_printed_date: &mut Date<Local>) {
  if date <= last_printed_date {
    return;
  }
  maybe_print_week_separator(config, date, start_date, last_printed_date);
  print_date_line(date);
  *last_printed_date = *date;
}

fn print_date_line(date: &Date<Local>) {
  let style_heading = Style::new().bold();
  println!("{}, {}", style_heading.paint(date.format("%Y-%m-%d")), date.format("%A"));
}

fn print_event_line(config: Option<&CalendarConfig>, index: &usize, event: &IcalVEvent, date: &Date<Local>) {
  match event_line(config, &event, &date) {
    Ok(line) => println!("{:4}  {}", index, line),
    Err(error) => warn!("{} in {}", error, event.get_uid())
  }
}

pub fn event_line(config: Option<&CalendarConfig>, event: &IcalVEvent, cur_day: &Date<Local>) -> Result<String, String> {
  if event.is_allday() {
    let mut summary = event.get_summary().ok_or("Invalid SUMMARY")?;
    if let Some(config) = config {
      let calendar_style = config::get_style_for_calendar(config);
      summary = calendar_style.paint(summary).to_string();
    }
    Ok(format!("             {}", summary))
  } else {
    let mut time_sep = " ";
    let dtstart = event.get_dtstart().ok_or("Invalid DTSTART")?.with_timezone(&Local);
    let start_string = if &dtstart.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtstart.format("%H:%M"))
    };

    let dtend = event.get_dtend().ok_or("Invalid DTEND")?.with_timezone(&Local);
    let end_string = if &dtend.date() != cur_day {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtend.format("%H:%M"))
    };

    let mut summary = event.get_summary().ok_or("Invalid SUMMARY")?;

    if let Some(config) = config {
      let calendar_style = config::get_style_for_calendar(config);
      summary = calendar_style.paint(summary).to_string();
    }

    Ok(format!("{:5}{}{:5}  {}", start_string, time_sep, end_string, summary))
  }
}

impl IcalVCalendar {
  fn get_config<'a>(&self, config: &'a Config) -> Option<&'a CalendarConfig> {
    config::get_config_for_calendar(config, self)
  }
}

impl IcalVEvent {
  fn relevant_on(&self, date: &Date<Local>) -> bool {
    if let Some(start) = self.get_dtstart() {
      start.date() == *date
    } else {
        warn!("Invalid DTSTART in {}", self.get_uid());
        false
    }
  }

  fn continues_after(&self, date: &Date<Local>) -> bool {
    match self.is_allday() {
      true => self.get_dtend().map( |dtend| dtend.date().pred() > *date),
      false => self.get_dtend().map( |dtend| dtend.date() > *date)
    }.unwrap_or(false)
  }
}
