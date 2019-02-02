extern crate atty;

use cursorfile;
use utils::stdioutils;
use KhResult;
use khline::KhLine;
use seqfile;

pub fn do_cursor(args: &[&str]) -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_cursorfile()?;
  } else {
    //println!("stdin is tty")
    if args.len() > 0 && args[0] == "prev" {
      return cursor_sequence_prev();
    }
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

fn cursor_sequence_prev() -> KhResult<()> {
  let cursor_event = cursorfile::read_cursorfile()?.to_event().unwrap();
  let mut seq = seqfile::read_seqfile_backwards()?
    .map(|line| line.parse::<KhLine>().unwrap());
  seq.find(|line| line.matches(&cursor_event));
  if let Some(next_elem) = seq.next() {
    cursorfile::write_cursorfile(&next_elem.to_string())?;
  } else {
    warn!("Already at end of sequence");
  }
  Ok(())
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
