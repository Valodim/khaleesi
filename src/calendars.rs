use utils::fileutil;
use defaults;

pub fn calendar_list() -> Vec<String> {
  let caldir = defaults::get_caldir();
  let calendar_paths = fileutil::dir_iter(&caldir);
  let mut calendars: Vec<String> = calendar_paths
    .map(move |path| {
      path
        .strip_prefix(&caldir)
        .map(|suffix| suffix.to_string_lossy().into_owned())
    })
    .flatten()
    .collect();
  calendars.sort();
  calendars
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils;

  #[test]
  fn test() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");

    let cals = calendar_list();

    assert_eq!(vec!("first", "second", "second/second_sub"), cals);
  }
}
