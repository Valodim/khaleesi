use std::fs;
use chrono::Local;
use std::path::{Path,PathBuf};

use icalwrap::IcalVEvent;
use khline::KhLine;
use defaults;

impl IcalVEvent {
  pub fn backup(&self) -> Result<PathBuf, String> {
    let khline = KhLine::from(self);
    
    let backupdir = defaults::get_backupdir();
    let backup_path = backupdir.join(with_timestamp(khline.get_normalized_path()));
    if backup_path == khline.path {
      Err("backup dir same as source dir".to_string())
    } else {
      match prepare_backup_dir(&backupdir)
        .and_then(|_| fs::copy(khline.path, backup_path.clone())) {
        Ok(_) => Ok(backup_path),
        Err(err) => Err(format!("{}", err)),
      }
    }
  }
}

fn with_timestamp(path: &Path) -> PathBuf {
  let mut filename = path.file_stem().unwrap().to_owned();
  filename.push(format!("_{}", Local::now().format("%FT%T")));
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

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn backup_test() {
    let testdir = prepare_testdir("testdir");

    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();
    let event = khline.to_event().unwrap();

    let new_path = event.backup().unwrap();

    testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate::path::exists());
    testdir.child(new_path.clone()).assert(predicate::path::exists());

    let predicate_file = predicate::path::eq_file(testdir.child(new_path.clone()).path());
    testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate_file);
  }
}
