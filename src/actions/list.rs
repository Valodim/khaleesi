use selectors::SelectFilters;
use utils::fileutil;
use khline::KhLine;

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {
  let filters = match SelectFilters::parse_from_args_with_range(args) {
    Err(error) => { println!("{}", error); return; },
    Ok(parsed_filters) => parsed_filters,
  };

  let cals = fileutil::read_calendars_from_files(filenames).unwrap();

  let events = cals.into_iter()
    .map(|cal| cal.get_principal_event())
    .enumerate()
    .filter(|(index, event)| filters.is_selected_index(*index, event));

  for (_, event) in events {
    if let Some(khline) = KhLine::from(&event) {
      println!("{}", khline);
    }
  }
}

