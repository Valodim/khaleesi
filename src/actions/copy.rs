use input;
use khline::KhLine;
use utils::fileutil;
use utils::misc;

use KhResult;

pub fn do_copy(_args: &[&str]) -> KhResult<()> {
  let khline = input::default_input_khline()?;

  let uid = &misc::make_new_uid();
  let cal = khline.to_cal()?;
  let new_cal = cal.with_uid(uid)?.with_dtstamp_now();

  fileutil::write_cal(&new_cal)?;

  info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display());

  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use utils::stdioutils;

  #[test]
  fn copy_test() {
    let testdir = prepare_testdir("testdir");

    stdioutils::test_stdin_write("twodaysacrossbuckets.ics");

    do_copy(&[]).unwrap();

    let child = testdir.child(".khaleesi/cal/11111111-2222-3333-4444-444444444444@khaleesi.ics");
    child.assert(predicate::path::exists());
  }
}
