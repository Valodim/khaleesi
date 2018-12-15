use chrono::*;
use icalwrap::IcalVCalendar;
use utils;

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {
  let mut fromarg: Option<Date<Local>> = None;
  let mut toarg: Option<Date<Local>> = None;

  if args.len() == 1 {
    if let Ok(num) = args[0].parse::<usize>() {
      println!("{}", filenames.nth(num).expect("No such element in sequence"));
      return
    }
  }
  if args.len() < 2 {
    info!("select [from|to parameter]+");
    return
  }

  for chunk in args.chunks(2) {
    if chunk.len() == 2 {
      let mut datearg = match utils::date_from_str(&chunk[1]) {
        Ok(datearg) => datearg,
        Err(error) => {
          info!("{}", error);
          return
        }
      };

      match chunk[0].as_str() {
        "from" => fromarg = Some(datearg),
        "to" => toarg = Some(datearg),
        _ => { info!("Incorrect!"); return }
      }
    } else {
      info!("Syntax error!");
    }
  }

  let mut cals = utils::read_calendars_from_files(filenames).unwrap();

  if let Some(fromarg) = fromarg {
    cals = filter_date_from(cals, fromarg);
  }

  if let Some(toarg) = toarg {
    cals = filter_date_to(cals, toarg);
  }

  for cal in cals {
    println!("{}", cal.get_path_as_string());
  }
}

fn filter_date_from(cals: Vec<IcalVCalendar>, from: Date<Local>) -> Vec<IcalVCalendar> {
  cals.into_iter().filter(|cal| cal.get_first_event().get_dtstart().unwrap().date() >= from).collect()
}

fn filter_date_to(cals: Vec<IcalVCalendar>, to: Date<Local>) -> Vec<IcalVCalendar> {
  cals.into_iter().filter(|cal| cal.get_first_event().get_dtstart().unwrap().date() <= to).collect()
}
