extern crate atty;

use cursorfile;
use utils::stdioutils;
use KhResult;

pub fn do_cursor(_args: &[&str]) -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_cursorfile()?;
  } else {
    //println!("stdin is tty")
  }

  if !stdioutils::is_stdout_tty() || stdioutils::is_stdin_tty() {
    write_cursorfile_to_stdout();
  }

  Ok(())
}

fn write_stdin_to_cursorfile() -> KhResult<()> {
  let lines = stdioutils::read_lines_from_stdin()?;

  if lines.len() > 1 {
    Err("Too many lines on stdin")?;
  };

  cursorfile::write_cursorfile(&lines[0])?;

  Ok(())
}

fn write_cursorfile_to_stdout() {
  if let Ok(cursor) = cursorfile::read_cursorfile() {
    khprintln!("{}", cursor);
  }
}

#[cfg(test)]
mod integration {
  use super::*;

  use testutils;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_with_stdin() {
    let testdir = testutils::prepare_testdir_empty();
    let expected_str = "hi there";
    stdioutils::test_stdin_write(expected_str);

    do_cursor(&[]).unwrap();

    testdir.child(".khaleesi/cursor").assert(expected_str);
  }

  #[test]
  fn test_with_stdin_linebreak() {
    let _testdir = testutils::prepare_testdir_empty();
    let expected_str = "hi\nthere";
    stdioutils::test_stdin_write(expected_str);

    let result = do_cursor(&[]);

    assert!(result.is_err());
    //testdir.child(".khaleesi/cursor").assert(expected_str);
  }

  #[test]
  fn test_no_stdin() {
    let testdir = testutils::prepare_testdir("testdir_with_cursor");

    do_cursor(&[]).unwrap();
    let out = stdioutils::test_stdout_clear();

    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }
}
