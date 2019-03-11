use tempfile::NamedTempFile;

use crate::backup::backup;
use crate::edit;
use crate::input;
use crate::khline::KhLine;
use crate::utils::fileutil;
use crate::KhResult;

pub fn do_edit() -> KhResult<()> {
  let khline = input::default_input_khline()?;
  edit(&khline)
}

fn edit(khline: &KhLine) -> KhResult<()> {
  let tempfile = NamedTempFile::new()?;
  let calendar = khline.to_cal()?;

  fileutil::write_file(tempfile.path(), &calendar.to_string())?;
  edit::edit_loop(&tempfile.path())?;

  let backup_path = backup(&khline).unwrap();
  info!("Backup written to {}", backup_path.display());

  let edited_cal = KhLine::new(tempfile.path(), None).to_cal()?.with_dtstamp_now().with_last_modified_now();
  fileutil::write_file(&khline.path, &edited_cal.to_string())?;
  info!("Successfully edited file {}", khline.path.display());

  Ok(())
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::testutils::prepare_testdir;

  #[test]
  fn edit_test() {
    let _testdir = prepare_testdir("testdir");

    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    assert!(edit(&khline).is_ok());
    let event = khline.to_event().unwrap();

    assert_eq!("20130101T010203Z", event.get_dtstamp().unwrap());
    assert_eq!("20130101T010203Z", event.get_last_modified().unwrap());
  }
}
