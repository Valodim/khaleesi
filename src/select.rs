use chrono::*;
use icalwrap::IcalVCalendar;
use utils;
use seq;

pub fn select_by_args(files: &mut Iterator<Item = String>, args: &[String]) {

  if args.len() == 1 {
    if let Ok(num) = args[0].parse::<usize>() {
      println!("{}", files.nth(num).expect("No such element in sequence"));
      return
    }
  }
  if args.len() < 2 {
    info!("select [from|to parameter]+");
    return
  }

  let mut cals = utils::read_calendars_from_files(files).unwrap();

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
        "from" => cals = filter_date_from(cals, datearg),
        "to" => cals = filter_date_to(cals, datearg),
        _ => { info!("Incorrect!"); return }
      }
    } else {
      info!("Syntax error!");
    }
  }

  for cal in cals {
    println!("{}", cal.get_path_as_string());
  }
}

fn filter_date_from(cals: Vec<IcalVCalendar>, from: NaiveDate) -> Vec<IcalVCalendar> {
  cals.into_iter().filter(|cal| cal.get_first_event().get_dtstart_date() >= from).collect()
}

fn filter_date_to(cals: Vec<IcalVCalendar>, to: NaiveDate) -> Vec<IcalVCalendar> {
  cals.into_iter().filter(|cal| cal.get_first_event().get_dtstart_date() <= to).collect()
}
