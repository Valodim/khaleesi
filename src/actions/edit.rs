use edit;
use input;
use khline::KhLine;
use utils::{fileutil,stdioutils};
use KhResult;

pub fn do_edit(_args: &[&str]) -> KhResult<()> {
  let khline = input::default_input_khline()?;
  edit::edit(&khline)
}

