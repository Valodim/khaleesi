use crate::selectors::SelectFilters;
use crate::input;
use crate::KhResult;

pub fn list_by_args(args: &[&str]) -> KhResult<()> {
  let lines = input::default_input_khlines()?;
  let filters = SelectFilters::parse_from_args_with_range(args)?;

  let events = lines
    .enumerate()
    .filter(|(index, khline)| {
      match khline.to_event() {
        Ok(event) => filters.is_selected_index(*index, &event),
        Err(cause) => { warn!("{}", cause); false },
      }
    });

  for (_, khline) in events {
    khprintln!("{}", khline);
  }

  Ok(())
}

