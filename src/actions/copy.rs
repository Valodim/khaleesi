use khline::KhLine;
use utils::fileutil;
use utils::misc;

pub fn do_copy(khline: &KhLine, _args: &[String]) {

  let uid = &misc::make_new_uid();
  copy_internal(khline, uid);
}

fn copy_internal(khline: &KhLine, uid: &str) {

  let cal = match khline.to_cal() {
    Ok(calendar) => calendar,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = match cal.with_uid(uid) {
    Ok(new_cal) => new_cal,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = new_cal.with_dtstamp_now();

  match fileutil::write_cal(&new_cal) {
    Ok(_) => info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display()),
    Err(error) => {
      error!("{}", error);
      return
    },
  }

  println!("{}", KhLine::from(&new_cal));
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
    copy_internal(&khline_from_file, uid);

    testdir.child(".khaleesi/cal/".to_string() + uid + ".ics").assert(predicate::path::exists());
  }

}
