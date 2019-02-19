use crate::input;
use crate::backup::backup;
use crate::KhResult;
use crate::khline::KhLine;
use crate::utils::stdioutils;

use std::path::PathBuf;
use std::fs::remove_file;

pub fn do_delete(_args: &[&str]) -> KhResult<()> {
  info!("do_delete");

  let cursor_khline = input::default_input_khline()?;

  delete_file(cursor_khline)
}

fn delete_file(khline: KhLine) -> KhResult<()> {

  if ask_really_delete(&khline.path) {
    let backup_path = backup(&khline).unwrap();
    info!("Backup written to {}", backup_path.display());

    remove_file(khline.path.clone())?;
    info!("deleted {:#?}", khline.get_normalized_path());
  }

  Ok(())
}

fn ask_really_delete(path: &PathBuf) -> bool {
  if cfg!(test) { return true };

  println!("Really delete {:#?}? y/n:", path);

  match stdioutils::read_single_char_from_stdin().unwrap() {
    'y' => true,
    _ => false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::testutils::*;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_do_delete_cursor() {
    let testdir = prepare_testdir("testdir_with_cursor");

    do_delete(&[]).unwrap();

    let predicate = predicate::path::missing();
    testdir.child(".khaleesi/cal/twodaysacrossbuckets").assert(predicate);
    
  }

  #[test]
  #[should_panic]
  fn test_do_delete_no_cursor() {
    let _testdir = prepare_testdir("testdir");

    do_delete(&[]).unwrap();
  }
}
