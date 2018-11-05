use chrono::*;
use icalwrap::Icalcomponent;
use utils;

pub fn select_by_args(files: &mut Iterator<Item = String>, from: &str, to: Option<&str>) {
  let comps = utils::read_comps_from_files(files);

  let from = utils::date_from_str(from);
  let to = to.map(|to| utils::date_from_str(to));

  let mut filtered = filter_date_from(comps, from);
  if let Some(to) = to {
    filtered = filter_date_to(filtered, to);
  }

  for comp in filtered {
    println!("{}", comp.get_path_as_string());
  }
}

fn filter_date_from(comps: Vec<Icalcomponent>, from: NaiveDate) -> Vec<Icalcomponent> {
  comps.into_iter().filter(|comp| comp.get_dtstart_date() >= from).collect()
}

fn filter_date_to(comps: Vec<Icalcomponent>, to: NaiveDate) -> Vec<Icalcomponent> {
  comps.into_iter().filter(|comp| comp.get_dtstart_date() <= to).collect()
}
