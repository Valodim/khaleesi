use crate::calendars;
use crate::KhResult;

pub fn action_get(args: &[&str]) -> KhResult<()> {
  if args.is_empty() {
    Err("get calendars")?;
  }
  match args[0] {
    "calendars" => action_get_calendars(),
    _ => Err("Unknown get parameter!")?
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

    action_get(&["calendars"]).unwrap();

    assert_eq!("first\nsecond\nsecond/second_sub\n", stdioutils::test_stdout_clear());
  }
}
