use utils;

pub fn sort_filenames_by_dtstart (files: &mut Iterator<Item = String>) {
  let mut cals = utils::read_calendars_from_files(files);
  cals.sort_unstable_by_key(|cal| cal.get_first_event().get_dtstart());
  for cal in cals {
    println!("{}", cal.get_path_as_string());
  }
}
