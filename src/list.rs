use chrono::*;
use icalwrap::IcalVCalendar;
use utils;

struct ListFilters {
  from: Option<Date<Local>>,
  to: Option<Date<Local>>,
  num: Option<usize>,
  calendar: Option<String>,
}

impl ListFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut from: Option<Date<Local>> = None;
    let mut to: Option<Date<Local>> = None;
    let mut calendar: Option<String> = None;

    if args.len() < 1 {
      return Err("list [num] | [from|to|cal parameter]+".to_string())
    }

    if args.len() == 1 {
      if let Ok(num) = args[0].parse::<usize>() {
        return Ok(ListFilters {from, to, num: Some(num), calendar} );
      } else {
        return Err("list [num] | [from|to|cal parameter]+".to_string())
      }
    }

    for chunk in args.chunks(2) {
      if chunk.len() == 2 {
        match chunk[0].as_str() {
          "from" => from = Some(ListFilters::parse_datearg(chunk[1].as_str())?),
          "to"   => to = Some(ListFilters::parse_datearg(chunk[1].as_str())?),
          "cal"  => calendar = Some(chunk[1].clone()) ,
          _      => return Err("Incorrect!".to_string())
        }

      } else {
        return Err("Syntax error!".to_string());
      }
    }
    Ok(ListFilters {from, to, num: None, calendar})
  }

  pub fn predicate_is_from(&self) -> impl Fn(&IcalVCalendar) -> bool + '_ {
    move |cal| {
      match &self.from {
        Some(from) => {
          let event = cal.get_principal_event();
          let pred_dtstart = event.get_dtstart().map_or(true, |dtstart| from <= &dtstart.date() );
          let pred_dtend = event.get_dtend().map_or(true, |dtend| from <= &dtend.date());
          pred_dtstart || pred_dtend
        }
        None => true
      }
    }
  }

  pub fn predicate_is_to(&self) -> impl Fn(&IcalVCalendar) -> bool + '_ {
    move |cal| {
      match &self.to {
        Some(to) => {
          let event = cal.get_principal_event();
          let pred_dtstart = event.get_dtstart().map_or(true, |dtstart| &dtstart.date() <= to);
          let pred_dtend = event.get_dtend().map_or(true, |dtend| &dtend.date() <= to);
          pred_dtstart || pred_dtend
        }
        None => true
      }
    }
  }

  pub fn predicate_is_in_calendar(&self) -> impl Fn(&IcalVCalendar) -> bool + '_ {
    move |cal| {
      match &self.calendar {
        Some(calendar) => {
          cal.get_path()
            .map_or(false,  |path| path.parent().map_or(false, |path| path.ends_with(calendar)))
        }
        None => true
      }
    }
  }

  fn parse_datearg(datearg: &str) -> Result<Date<Local>, String> {
    utils::date_from_str(datearg).map_err( |err| format!("{}", err))
  }
}

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {

  let filters = match ListFilters::parse_from_args(args) {
    Ok(filters) => filters,
    Err(err) => {
      error!("{}", err);
      return;
    }
  };

  if let Some(num) = filters.num {
    match filenames.nth(num) {
      Some(line) => println!("{}", line),
      None => error!("Element {} out of range!", num),
    }
    return;
  }

  let cals = utils::read_calendars_from_files(filenames).unwrap();

  let cals = cals.into_iter()
    .filter( filters.predicate_is_from() )
    .filter( filters.predicate_is_to() )
    .filter( filters.predicate_is_in_calendar() );

  for cal in cals {
    if let Some(line) = cal.get_principal_event().get_khaleesi_line() {
      println!("{}", line);
    }
  }
}

