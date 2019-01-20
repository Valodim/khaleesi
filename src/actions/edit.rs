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

pub fn do_edit(_args: &[&str]) -> KhResult<()> {
  let khline = input::default_input_khline()?;
  edit_internal(&khline)
}

pub fn edit_internal(khline: &KhLine) -> KhResult<()> {
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

  match fileutil::read_single_char_from_stdin().unwrap() {
    'y' => true,
    _ => false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use icalwrap::IcalComponent;

  #[test]
  fn edit_test() {
    let _testdir = prepare_testdir("testdir");

    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    assert!(edit_internal(&khline).is_ok());
    let event = khline.to_event().unwrap();

    let dtstamp_prop = ical::icalproperty_kind_ICAL_DTSTAMP_PROPERTY;
    assert_eq!("20130101T010203Z", event.get_property(dtstamp_prop).unwrap().get_value());

    let last_modified_kind = ical::icalproperty_kind_ICAL_LASTMODIFIED_PROPERTY;
    assert_eq!("20130101T010203Z", event.get_property(last_modified_kind).unwrap().get_value());
  }
}
