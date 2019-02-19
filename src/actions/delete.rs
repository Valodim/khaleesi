//use crate::input;
use crate::KhResult;
use crate::cursorfile;
use std::fs::remove_file;

pub fn do_delete(_args: &[&str]) -> KhResult<()> {
  info!("do_delete");

  let cursor_khline = cursorfile::read_cursorfile()?;

  let path = cursor_khline.path;  

  remove_file(path)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::fs;

  use crate::testutils::*;
  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use crate::utils::fileutil;

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
    let testdir = prepare_testdir("testdir");

    do_delete(&[]).unwrap();
  }
}
