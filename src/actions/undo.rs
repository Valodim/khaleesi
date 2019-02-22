use crate::backup;
use crate::defaults;
use crate::input;
use crate::utils::fileutil;
use crate::utils::misc;
use crate::KhResult;

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn do_undo(_args: &[&str]) -> KhResult<()> {
  let backupdir = defaults::get_backupdir();

  let mut dirs: Vec<PathBuf> = backupdir
    .read_dir()?
    .filter_map(|result| result.ok())
    .map(|dir_entry| dir_entry.path())
    .collect();

  if dirs.len() < 1 {
    Err("there are no backups, nothing to undo!".to_string())?;
  }
  dirs.sort_unstable();

  //source_dir is most recent
  let source_dir = dirs.pop().unwrap().to_path_buf();
  let backup_id = source_dir.strip_prefix(backupdir)?;

  let files: Vec<PathBuf> = WalkDir::new(source_dir.clone())
    .into_iter()
    .flatten()
    .filter(|dir_entry| dir_entry.path().is_file())
    .map(|x| x.path().to_path_buf())
    .collect();

  let caldir = defaults::get_caldir();
  for file_path in files {
    let path_in_cal = file_path.strip_prefix(source_dir.clone())?;

    let mut target_path = caldir.clone();
    target_path.push(path_in_cal);

    if target_path.exists() {
      //ask
    } else {
      fs::copy(file_path.clone(), target_path.clone())?;
    }
    println!("{}", target_path.display());
    println!("{}", file_path.display());
  }

  println!("{:?}", backup_id);

  Ok(())
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::khline::KhLine;
  use crate::testutils::prepare_testdir;
  use crate::utils::stdioutils;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

}
