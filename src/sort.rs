use utils;
use icalwrap::IcalComponent;

pub fn sort_filenames_by_dtstart (files: &mut Iterator<Item = String>) {
  let mut comps = utils::read_calendars_from_files(files);
  comps.sort_unstable_by_key(|comp| comp.get_first_event().get_dtstart());
  for comp in comps {
    println!("{}", comp.get_path_as_string());
  }
}
