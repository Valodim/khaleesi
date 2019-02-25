use crate::calendars;
use crate::KhResult;
use crate::cli::{Get, GetArgs};

pub fn action_get(args: &Get) -> KhResult<()> {
  match args.query {
    GetArgs::calendars => action_get_calendars(),
  }
}

pub fn action_get_calendars() -> KhResult<()> {
  for calendar in calendars::calendar_list() {
    khprintln!("{}", calendar);
  }

  Ok(())
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::testutils;
  use crate::utils::stdioutils;

  #[test]
  fn test_get_calendars() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");

    let args = Get { query: GetArgs::calendars };
    action_get(&args).unwrap();

    assert_eq!("first\nsecond\nsecond/second_sub\n", stdioutils::test_stdout_clear());
  }
}
