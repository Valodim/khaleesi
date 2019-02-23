use crate::defaults;
use crate::KhResult;
use crate::utils::stdioutils;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn do_undo(_args: &[&str]) -> KhResult<()> {
  let backupdir = defaults::get_backupdir();

  let source_dir = get_most_recent_backup()?;

  let backup_id = source_dir.strip_prefix(backupdir)?;

  info!("Restoring {:?}", backup_id);

  let files = WalkDir::new(source_dir.clone())
    .into_iter()
    .flatten()
    .filter(|dir_entry| dir_entry.path().is_file());

  for file in files {
    restore_file_from_backup(&source_dir, &file.path())?;
  };

  Ok(())
}

fn restore_file_from_backup(source_prefix: &Path, file_path: &Path) -> KhResult<()> {
  let caldir = defaults::get_caldir();
  let path_in_cal = file_path.strip_prefix(source_prefix)?;

  let mut target_path = caldir.clone();
  target_path.push(path_in_cal);

  if target_path.exists() && !ask_overwrite(&target_path) {
    info!("ignoring {}", target_path.display());
    return Ok(());
  }
  fs::create_dir_all(&target_path.parent().ok_or_else(|| "error creating calendar directory")?)?;
  fs::copy(file_path, &target_path)?;

  info!("Restore {} to {}", file_path.display(), target_path.display());

  Ok(())
}

fn get_most_recent_backup() -> KhResult<PathBuf> {
  let backupdir = defaults::get_backupdir();
  backupdir
    .read_dir()?
    .filter_map(|result| result.ok())
    .map(|dir_entry| dir_entry.path())
    .max()
    .ok_or("there are no backups, nothing to undo!".into())
}

fn ask_overwrite(path: &Path) -> bool {
  if cfg!(test) { return true};
  println!("File exists:\n{}", path.display());
  println!("Overwrite? y/n:");

  match stdioutils::read_single_char_from_stdin().unwrap() {
    'y' => true,
    _ => false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::khline::KhLine;
  use crate::testutils::prepare_testdir;
  use crate::utils::stdioutils;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_get_most_recent_backup() {
    let _testdir = prepare_testdir("testdir_with_backup");
    let result = get_most_recent_backup().unwrap();
    assert_eq!("backup_id", result.file_name().unwrap().to_str().unwrap());
  }

  #[test]
  #[should_panic]
  fn test_get_most_recent_backup_negative() {
    let _testdir = prepare_testdir("testdir");
    get_most_recent_backup().unwrap();
  }

  #[test]
  fn test_restore_file_from_backup() {
    let testdir = prepare_testdir("testdir_with_backup");
    let source_file = testdir.child(".khaleesi/backup/backup_id/my_calendar/twodaysacrossbuckets.ics");
    let source_folder = testdir.child(".khaleesi/backup/backup_id");
    let target_folder = testdir.child(".khaleesi/cal/my_calendar/twodaysacrossbuckets.ics");

    let result = restore_file_from_backup(source_folder.path(), source_file.path()).unwrap();
    target_folder.assert(predicate::path::exists());
  }
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::khline::KhLine;
  use crate::testutils::prepare_testdir;
  use crate::utils::stdioutils;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_do_undo() {
    let testdir = prepare_testdir("testdir_with_backup");
    do_undo(&[]).unwrap();
  }
}
