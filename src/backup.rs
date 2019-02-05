use std::io;
use std::fs;
use chrono::Local;
use std::path::{Path,PathBuf};

use crate::defaults;
use crate::khline::KhLine;

pub fn backup(khline: &KhLine) -> io::Result<PathBuf> {
  let backupdir = defaults::get_backupdir();
  let backup_path = backupdir.join(with_timestamp(khline.get_normalized_path()));

  if backup_path == khline.path {
    Err(io::Error::new(io::ErrorKind::Other, "backup dir same as source dir"))
  } else {
    let backup_path_parent = backup_path.parent().unwrap();
    prepare_backup_dir(&backup_path_parent)?;
    fs::copy(&khline.path, backup_path.clone())?;
    Ok(backup_path.clone())
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

fn prepare_backup_dir(backupdir: &Path) -> io::Result<()> {
  if !backupdir.exists() {
    info!("Creating backup directory: {}", backupdir.to_string_lossy());
    fs::create_dir_all(&backupdir)?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn backup_test() {
    let testdir = prepare_testdir("testdir");

    let khline = "twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();

    let new_path = backup(&khline).unwrap();

    testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate::path::exists());
    testdir.child(new_path.clone()).assert(predicate::path::exists());

    let predicate_file = predicate::path::eq_file(testdir.child(new_path.clone()).path());
    testdir.child(".khaleesi/cal/twodaysacrossbuckets.ics").assert(predicate_file);
  }
}
