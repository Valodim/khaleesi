use crate::calendars;
use crate::KhResult;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct GetArgs {
  /// Show information about this
  #[structopt(name = "query", raw(possible_values = "&GetQueryArgs::variants()"))]
  pub query: GetQueryArgs,
}

arg_enum! {
#[derive(Debug)]
  pub enum GetQueryArgs{
    Calendars,
  }
}

pub fn action_get(args: &GetArgs) -> KhResult<()> {
  match args.query {
    GetQueryArgs::Calendars => action_get_calendars(),
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

    let args = GetArgs { query: GetQueryArgs::Calendars };
    action_get(&args).unwrap();

    assert_eq!("first\nsecond\nsecond/second_sub\n", stdioutils::test_stdout_clear());
  }
}
