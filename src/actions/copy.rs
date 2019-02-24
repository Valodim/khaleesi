use crate::input;
use crate::utils::fileutil;
use crate::utils::misc;

use crate::KhResult;

pub fn do_copy() -> KhResult<()> {
  let khline = input::default_input_khline()?;

  let uid = &misc::make_new_uid();
  let cal = khline.to_cal()?;
  let new_cal = cal.with_uid(uid)?.with_dtstamp_now();

  fileutil::write_cal(&new_cal)?;

  info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display());

  Ok(())
}


#[cfg(test)]
mod integration {
  use super::*;

  use assert_fs::prelude::*;
  use crate::khline::KhLine;
  use crate::testutils::prepare_testdir;
  use crate::utils::stdioutils;
  use predicates::prelude::*;

  #[test]
  fn copy_test() {
    let testdir = prepare_testdir("testdir");
    stdioutils::test_stdin_write("twodaysacrossbuckets.ics");

    do_copy(&[]).unwrap();

    let child = testdir.child(".khaleesi/cal/11111111-2222-3333-4444-444444444444@khaleesi.ics");
    child.assert(predicate::path::exists());

    let khline = "11111111-2222-3333-4444-444444444444@khaleesi.ics".parse::<KhLine>().unwrap();
    assert_eq!("11111111-2222-3333-4444-444444444444@khaleesi", khline.to_event().unwrap().get_uid());
  }
}
