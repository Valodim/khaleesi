use utils::fileutil;
use defaults;

pub fn calendar_list() -> impl Iterator<Item = String> {
  let caldir = defaults::get_caldir();
  let calendar_paths = fileutil::dir_iter(&caldir);
  calendar_paths
    .map(move |path| {
      path
        .strip_prefix(&caldir)
        .map(|suffix| suffix.to_string_lossy().into_owned())
    })
    .flatten()
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils;

  #[test]
  fn test() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");

    let cals = calendar_list().collect::<Vec<String>>();

    assert_eq!(vec!("second", "second/second_sub", "first"), cals);
  }
}
