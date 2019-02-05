use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::khline::KhLine;
use crate::KhResult;
use crate::utils::stdioutils;

pub fn edit_loop(path: &Path) -> KhResult<()> {
  loop {
    edit_file(path)?;
    let edited_cal = KhLine::new(path, None).to_cal()?;
    if let Some(errors) = edited_cal.check_for_errors() {
      if !ask_continue_editing(&errors) {
        return Err("editing aborted by user")?;
      }
    } else {
      return Ok(());
    }
  }
}

fn edit_file(path: &Path) -> KhResult<()> {
  if cfg!(test) { return Ok(()) };

  let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

  Command::new(&editor)
    .arg(path.as_os_str())
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status()?;

  Ok(())
}

fn ask_continue_editing(error: &[String]) -> bool {
  println!("Calendar contains errors:\n{}", error.join("\n"));
  println!("Continue editing? y/n:");

  match stdioutils::read_single_char_from_stdin().unwrap() {
    'y' => true,
    _ => false
  }
}

