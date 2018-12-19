use chrono::*;
use icalwrap::IcalVCalendar;
use utils;

struct ListFilters {
  from: Option<Date<Local>>,
  to: Option<Date<Local>>,
  num: Option<usize>,
}

impl ListFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut fromarg: Option<Date<Local>> = None;
    let mut toarg: Option<Date<Local>> = None;

    if args.len() < 1 {
      return Err("select [from|to parameter]+".to_string())
    }

    if args.len() == 1 {
      if let Ok(num) = args[0].parse::<usize>() {
        return Ok(ListFilters {from: fromarg, to: toarg, num: Some(num)} );
      } else { 
        return Err("select [from|to parameter]+".to_string())
      }
    }

    for chunk in args.chunks(2) {
      if chunk.len() == 2 {
        let mut datearg = match utils::date_from_str(&chunk[1]) {
          Ok(datearg) => datearg,
            Err(error) => {
              return Err(format!("{}", error))
            }
        };

        match chunk[0].as_str() {
          "from" => fromarg = Some(datearg),
          "to"   => toarg = Some(datearg),
          _      => return Err("Incorrect!".to_string())
        }
      } else {
        return Err("Syntax error!".to_string());
      }
    }
    Ok(ListFilters {from: fromarg, to: toarg, num: None})
  }

  pub fn predicate_line_is_from(&self) -> impl Fn(&IcalVCalendar) -> bool + '_ {
    move |cal| {
      match &self.from {
        Some(from) => {
          let event = cal.get_principal_event();
          let pred_dtstart = event.get_dtstart().map_or(false, |dtstart| from <= &dtstart.date() );
          let pred_dtend = event.get_dtend().map_or(false, |dtend| from <= &dtend.date());
          pred_dtstart || pred_dtend
        }
        None => true
      }
    }
  }

  pub fn predicate_line_is_to(&self) -> impl Fn(&IcalVCalendar) -> bool + '_ {
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
}

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {

  let filters = ListFilters::parse_from_args(args).unwrap();

  if let Some(num) = filters.num {
    println!("{}", filenames.nth(num).expect("No such element in sequence"));
    return;
  }

  let mut cals = utils::read_calendars_from_files(filenames).unwrap();

  cals = cals.into_iter()
    .filter( filters.predicate_line_is_from() )
    .filter( filters.predicate_line_is_to() )
    .collect();

  for cal in cals {
    if let Some(line) = cal.get_principal_event().index_line() {
      println!("{}", line);
    }
  }
}

