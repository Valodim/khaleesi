use input;
use khline::KhLine;
use utils::fileutil;
use utils::misc;

use KhResult;

pub fn do_copy(_args: &[&str]) -> KhResult<()> {
  let khline = input::default_input_khline()?;

  let uid = &misc::make_new_uid();
  copy_internal(&khline, uid).map(|_| ())
}

fn copy_internal(khline: &KhLine, uid: &str) -> KhResult<KhLine> {
  let cal = khline.to_cal()?;
  let new_cal = cal.with_uid(uid)?.with_dtstamp_now();

  fileutil::write_cal(&new_cal)?;

  info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display());

  Ok(KhLine::from(&new_cal))
}


#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn copy_test() {
    let testdir = prepare_testdir("testdir");
    let khline_from_file = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    let uid = "my_new_uid";
    copy_internal(&khline_from_file, uid).unwrap();

    testdir.child(".khaleesi/cal/".to_string() + uid + ".ics").assert(predicate::path::exists());
  }

}
