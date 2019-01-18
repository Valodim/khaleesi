use std::env;
use std::{fs, io};
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

use backup::backup;
use input;
use khline::KhLine;
use utils::fileutil;
use KhResult;

pub fn do_edit(_args: &[String]) -> KhResult<()> {
  let khline = input::default_input_single()?;

  let tempfile = copy_to_tempfile(&khline.path).map_err(|err| format!("{}", err))?;
  loop {
    edit_file(tempfile.path())?;
    let temp_cal = KhLine::new(tempfile.path(), None);
    if let Some(errors) = temp_cal.to_cal()?.check_for_errors() {
      if !ask_continue_editing(&errors) {
        break;
      }
    } else {
      let backup_path = backup(&khline).unwrap();
      info!("Backup written to {}", backup_path.display());
      fs::copy(tempfile.path(), &khline.path).unwrap();
      info!("Successfully edited file {}", khline.path.display());
      break;
    }
  }
  Ok(())
}

fn copy_to_tempfile(path: &Path) -> io::Result<tempfile::NamedTempFile> {
  let tempfile = NamedTempFile::new()?;
  fs::copy(path, tempfile.path())?;
  Ok(tempfile)
}

fn edit_file(path: &Path) -> Result<(), String> {
  let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

  if let Err(error) = Command::new(&editor)
    .arg(path.as_os_str())
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status() {
      return Err(format!("{} command failed to start, error: {}", editor, error));
    };

  Ok(())
}

fn ask_continue_editing(error: &Vec<String>) -> bool {
  println!("Calendar contains errors:\n{}", error.join("\n"));
  println!("Continue editing? y/n:");

  let stdin = std::io::stdin();
  let stdinlock = stdin.lock();
  match fileutil::read_single_char(stdinlock).unwrap() {
    'y' => true,
    _ => false
  }
}
