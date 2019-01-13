use icalwrap::IcalVEvent;
use khline::KhLine;
use defaults;
use std::fs;
use chrono::Local;
use std::path::{Path,PathBuf};
use utils::fileutil;

impl IcalVEvent {
  pub fn backup(&self) -> Result<PathBuf, String> {
    let khline = KhLine::from(self);
    let normalized_path = khline.get_normalized_path();
    
    let backupdir = defaults::get_backupdir();
    let path = backupdir.join(with_timestamp(normalized_path));
    let cal = self.get_parent().unwrap();
    let new_cal = cal.clone().with_path(&path);

    prepare_backup_dir(&backupdir);
    fileutil::write_cal(&new_cal)?;
    Ok(path)
  }
}

fn with_timestamp(path: &Path) -> PathBuf {
  let mut filename = path.file_stem().unwrap().to_owned();
  filename.push(format!("{}", Local::now().format("%FT%T")));
  let mut pathbuf = path.to_path_buf();
  pathbuf.set_file_name(filename);
  pathbuf.set_extension("ics");
  pathbuf
}

fn prepare_backup_dir(backupdir: &Path) -> Result<(), std::io::Error> {
  if !backupdir.exists() {
    info!("Creating backup directory: {}", backupdir.to_string_lossy());
    fs::create_dir(&backupdir)?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use testdata;
  use chrono::{Utc, Local, TimeZone};

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use itertools::Itertools;

  #[test]
  fn backup_test() {
    let testdir = prepare_testdir("testdir");

    let khline = "12345 twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();
    let event = khline.to_event().unwrap();

    let new_path = event.backup().unwrap();

    testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate::path::exists());
    testdir.child(new_path.clone()).assert(predicate::path::exists());

    //let predicate_file = predicate::path::eq_file(new_path);
    //testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate_file);
  }
}
