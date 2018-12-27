use utils;

pub fn sort_filenames_by_dtstart (files: &mut Iterator<Item = String>) {
  let mut cals = utils::read_calendars_from_files(files).unwrap();
  cals.sort_unstable_by_key(|cal| cal.get_principal_event().get_dtstart());
  for cal in cals {
    if let Some(line) = cal.get_principal_event().get_khaleesi_line() {
      println!("{}", line);
    }
  }
}
