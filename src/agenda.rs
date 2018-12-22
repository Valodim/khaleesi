use utils;
use icalwrap::*;
use yansi::{Style,Color};
use chrono::{NaiveTime, TimeZone, Local};
use config::{self,Config,CalendarConfig};

pub fn show_events(config: Config, lines: &mut Iterator<Item = String>) {
  let style_heading = Style::new().bold();
  let cals = utils::read_calendars_from_files(lines).unwrap();

  let mut cur_day = cals[0].get_principal_event().get_dtstart()
    .unwrap_or(Local.timestamp(0, 0))
    .date();
  println!("{}, {}", style_heading.paint(cur_day.format("%Y-%m-%d")), cur_day.format("%A"));

  for (i, cal) in cals.iter().enumerate() {
    let cal_config = config::get_config_for_calendar(&config, &cal);

    if let Some(start) = cal.get_principal_event().get_dtstart() {
      if start.date() != cur_day {
        cur_day = start.date();
        println!("{}, {}", style_heading.paint(cur_day.format("%Y-%m-%d")), cur_day.format("%A"));
      }
      match event_line(cal_config, &cal.get_principal_event()) {
        Ok(line) => println!("{:4}  {}", i, line),
        Err(error) => warn!("{} in {}", error, cal.get_principal_event().get_uid())
      }
    } else {
      warn!("Invalid DTSTART in {}", cal.get_principal_event().get_uid());
    };

  }
}

pub fn event_line(config: Option<&CalendarConfig>, event: &IcalVEvent) -> Result<String, String> {
  if event.is_allday() {
    let summary = event.get_summary().ok_or("Invalid SUMMARY")?;
    Ok(format!("             {}", summary))
  } else {
    let mut time_sep = " ";
    let dtstart = event.get_dtstart().ok_or("Invalid DTSTART")?.with_timezone(&Local);
    let start_string = if dtstart.time() == NaiveTime::from_hms(0, 0, 0) {
      "".to_string()
    } else {
      time_sep = "-";
      format!("{}", dtstart.format("%H:%M"))
    };

    let dtend = event.get_dtend().ok_or("Invalid DTEND")?.with_timezone(&Local);
    let end_string = if dtend.time() == NaiveTime::from_hms(0, 0, 0) {
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
