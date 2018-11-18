use chrono::*;
use icalwrap::IcalVCalendar;
use utils;

pub fn select_by_args(files: &mut Iterator<Item = String>, args: &[String]) {
  let mut cals = utils::read_calendars_from_files(files);

  if args.len() < 2 {
    println!("select [from|to parameter]+");
    return
  }
  for chunk in args.chunks(2) {
    if chunk.len() == 2 {
      let datearg = utils::date_from_str(&chunk[1]);
      match chunk[0].as_str() {
        "from" => cals = filter_date_from(cals, datearg),
        "to" => cals = filter_date_to(cals, datearg),
        _ => { println!("Incorrect!"); return }
      }
    } else {
      println!("Syntax error!");
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
