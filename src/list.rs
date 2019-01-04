use selectors::SelectFilters;
use utils;

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {
  let filters = match SelectFilters::parse_from_args_with_range(args) {
    Err(error) => { println!("{}", error); return; },
    Ok(parsed_filters) => parsed_filters,
  };

  let cals = utils::read_calendars_from_files(filenames).unwrap();

  let events = cals.into_iter()
    .map(|cal| cal.get_principal_event())
    .enumerate()
    .filter(|(index, event)| filters.is_selected_index(*index, event));

  for (_, event) in events {
    if let Some(line) = event.get_khaleesi_line() {
      println!("{}", line);
    }
  }
}

