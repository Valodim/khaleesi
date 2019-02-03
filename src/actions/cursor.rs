extern crate atty;

use cursorfile;
use utils::stdioutils;
use KhResult;
use khline::KhLine;
use seqfile;

enum Direction {
  Up,
  Down,
}

pub fn do_cursor(args: &[&str]) -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_cursorfile()?;
  } else {
    //println!("stdin is tty")
    if !args.is_empty() {
      match args[0] {
        "prev" => return cursor_sequence_move(&Direction::Up),
        "next" => return cursor_sequence_move(&Direction::Down),
        &_ => {}
      }
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

fn cursor_sequence_move(direction: &Direction) -> KhResult<()> {
  let cursor_event = cursorfile::read_cursorfile()?.to_event().unwrap();
  let mut seq = seqfile::read_seqfile()?
    .map(|line| line.parse::<KhLine>().unwrap());
  let next_elem = match direction {
    Direction::Up => {
      let mut seq_rev = seq.rev();
      seq_rev.find(|line| line.matches(&cursor_event));
      seq_rev.next()
    },
    Direction::Down => {
      seq.find(|line| line.matches(&cursor_event));
      seq.next()
    }
  };

  match next_elem {
    Some(next_elem) => cursorfile::write_cursorfile(&next_elem.to_string()),
    None => {
      warn!("Already at end of sequence");
      Ok(())
    }
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
