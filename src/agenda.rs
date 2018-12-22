use utils;
use icalwrap::*;
use yansi::{Style};
use config::{self,Config,CalendarConfig};
use chrono::{TimeZone, Local, Date};

pub fn show_events(config: Config, lines: &mut Iterator<Item = String>) {
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut not_over_yet: Vec<(usize, &IcalVCalendar)> = Vec::new();
  let mut cals_iter = cals.iter().enumerate();
  let mut cur_cal: (usize, &IcalVCalendar);

  match cals_iter.next() {
    Some(foo) => cur_cal = foo,
    None => return,
  }
  let mut cur_day = cur_cal.1
    .get_principal_event()
    .get_dtstart()
    .unwrap_or(Local.timestamp(0, 0))
    .date();

  let mut events_remaining = true;
  loop {
    print_date_line(&cur_day);

    for (index, cal) in not_over_yet.clone() {
      let cal_config = config::get_config_for_calendar(&config, &cal);
      let event = cal.get_principal_event();
      print_event_line(cal_config, &index, &event, &cur_day); 
    }
    not_over_yet.retain( |(_, cal)| {
      let event = cal.get_principal_event();
      event.continues_after(&cur_day) 
    });

    loop {
      let cal_config = config::get_config_for_calendar(&config, &cur_cal.1);
      let event = cur_cal.1.get_principal_event();
      let i = cur_cal.0;

      match event.get_dtstart() {
        None => {
          warn!("Invalid DTSTART in {}", event.get_uid());
          continue
        },
        Some(start) => {
          if start.date() != cur_day {
            break;
          }
        }
      }

      print_event_line(cal_config, &i, &event, &cur_day);
      if event.continues_after(&cur_day) {
        not_over_yet.push(cur_cal);
      }
      match cals_iter.next() {
        Some(foo) => cur_cal = foo,
        None => {
          events_remaining=false;
          break
        }
      }
    }

    cur_day = cur_day.succ();
    if not_over_yet.len() == 0 && !events_remaining { break };
  }
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
    let summary = event.get_summary().ok_or("Invalid SUMMARY")?;
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

impl IcalVEvent {
  fn continues_after(&self, date: &Date<Local>) -> bool {
    match self.is_allday() {
      true => self.get_dtend().map( |dtend| dtend.date().pred() > *date),
      false => self.get_dtend().map( |dtend| dtend.date() > *date)
    }.unwrap_or(false)
  }
}
