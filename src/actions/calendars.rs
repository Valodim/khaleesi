use calendars;

pub fn action_list_calendars(_args: &[String]) -> Result<(), String> {
  for calendar in calendars::calendar_list() {
    khprintln!("{}", calendar);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils;

  #[test]
  fn test() {
    let _testdir = testutils::prepare_testdir("testdir_two_cals");

    action_list_calendars(&[]).unwrap();

    assert_eq!("second\nsecond/second_sub\nfirst\n", testutils::test_stdout_clear());
  }
}
