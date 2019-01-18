use std::env;
use std::fs;
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

  let tempfile = NamedTempFile::new()?;
  let calendar = khline.to_cal()?.with_dtstamp_now().with_last_modified_now();
  fileutil::write_file(tempfile.path(), &calendar.to_string())?;
  loop {
    edit_file(tempfile.path())?;
    let edited_cal = KhLine::new(tempfile.path(), None).to_cal()?;
    if let Some(errors) = edited_cal.check_for_errors() {
      if !ask_continue_editing(&errors) {
        break;
      }
    } else {
      let backup_path = backup(&khline).unwrap();
      info!("Backup written to {}", backup_path.display());
      fileutil::write_file(&khline.path, &edited_cal.with_dtstamp_now().with_last_modified_now().to_string())?;
      info!("Successfully edited file {}", khline.path.display());
      break;
    }
  }
  Ok(())
}

fn edit_file(path: &Path) -> Result<(), String> {
  if cfg!(test) { return Ok(()) };

  let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

  if let Err(error) = Command::new(&editor)
    .arg(path.as_os_str())
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status() {
      return Err(format!("{} command failed to start, error: {}", editor, error));
    };

  Ok(())
}

fn ask_continue_editing(error: &[String]) -> bool {
  println!("Calendar contains errors:\n{}", error.join("\n"));
  println!("Continue editing? y/n:");

  let stdin = std::io::stdin();
  let stdinlock = stdin.lock();
  match fileutil::read_single_char(stdinlock).unwrap() {
    'y' => true,
    _ => false
  }
}
