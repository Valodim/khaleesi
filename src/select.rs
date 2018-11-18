use chrono::*;
use icalwrap::Icalcomponent;
use utils;

pub fn select_by_args(files: &mut Iterator<Item = String>, args: &[String]) {
  let mut comps = utils::read_calendars_from_files(files);

  if args.len() < 2 {
    println!("select [from|to parameter]+");
    return
  }
  for chunk in args.chunks(2) {
    if chunk.len() == 2 {
      let datearg = utils::date_from_str(&chunk[1]);
      match chunk[0].as_str() {
        "from" => comps = filter_date_from(comps, datearg),
        "to" => comps = filter_date_to(comps, datearg),
        _ => { println!("Incorrect!"); return }
      }
    } else {
      println!("Syntax error!");
    }
  }

  for comp in comps {
    println!("{}", comp.get_path_as_string());
  }
}

fn filter_date_from(comps: Vec<Icalcomponent>, from: NaiveDate) -> Vec<Icalcomponent> {
  comps.into_iter().filter(|comp| comp.get_dtstart_date() >= from).collect()
}

fn filter_date_to(comps: Vec<Icalcomponent>, to: NaiveDate) -> Vec<Icalcomponent> {
  comps.into_iter().filter(|comp| comp.get_dtstart_date() <= to).collect()
}
