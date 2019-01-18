use selectors::SelectFilters;
use utils::fileutil;
use khline::KhLine;
use input;
use KhResult;

pub fn list_by_args(args: &[String]) -> KhResult<()> {
  let mut filenames = input::default_input_multiple()?;
  let cals = fileutil::read_calendars_from_files(&mut filenames)?;

  let filters = SelectFilters::parse_from_args_with_range(args)?;

  let events = cals.into_iter()
    .map(|cal| cal.get_principal_event())
    .enumerate()
    .filter(|(index, event)| filters.is_selected_index(*index, event));

  for (_, event) in events {
    println!("{}", KhLine::from(&event));
  }

  Ok(())
}

