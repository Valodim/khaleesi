use crate::cursorfile;
use crate::utils::stdioutils;
use crate::KhResult;
use crate::seqfile;
use crate::cli::{CursorArgs,Direction as CursorDirection};

enum Direction {
  Up,
  Down,
}

pub fn do_cursor(args: &CursorArgs) -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_cursorfile()?;
  } else {
    //println!("stdin is tty")
    if let Some(direction) = &args.direction {
      match direction {
        CursorDirection::prev => return cursor_sequence_move(&Direction::Up),
        CursorDirection::next => return cursor_sequence_move(&Direction::Down),
      }
    };
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
  let cursor_event = cursorfile::read_cursorfile()?;
  let mut seq = seqfile::read_seqfile_khlines()?;
  let next_elem = match direction {
    Direction::Up => {
      let mut seq_rev = seq.rev();
      seq_rev.find(|line| line == &cursor_event);
      seq_rev.next()
    },
    Direction::Down => {
      seq.find(|line| line == &cursor_event);
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

  use crate::testutils;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_with_stdin() {
    let testdir = testutils::prepare_testdir_empty();
    let expected_str = "hi there";
    stdioutils::test_stdin_write(expected_str);

    let args = CursorArgs{direction: None};
    do_cursor(&args).unwrap();

    testdir.child(".khaleesi/cursor").assert(expected_str);
  }

  #[test]
  fn test_cursor_sequence_move_next() {
    let testdir = testutils::prepare_testdir("testdir_with_seq_and_cursor");
    let args = CursorArgs{direction: Some(CursorDirection::next)};
    do_cursor(&args).unwrap();

    let out = "1182988800 rfc_multi_day_allday.ics";
    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }

  #[test]
  fn test_cursor_sequence_move_prev_at_end() {
    let testdir = testutils::prepare_testdir("testdir_with_seq_and_cursor");
    let args = CursorArgs{direction: Some(CursorDirection::prev)};
    do_cursor(&args).unwrap();

    let out = "1544740200 twodaysacrossbuckets.ics\n";
    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }

  #[test]
  fn test_with_stdin_linebreak() {
    let _testdir = testutils::prepare_testdir_empty();
    let expected_str = "hi\nthere";
    stdioutils::test_stdin_write(expected_str);

    let args = CursorArgs {direction: None};
    let result = do_cursor(&args);

    assert!(result.is_err());
    //testdir.child(".khaleesi/cursor").assert(expected_str);
  }

  #[test]
  fn test_no_stdin() {
    let testdir = testutils::prepare_testdir("testdir_with_cursor");

    let args = CursorArgs {direction: None};
    do_cursor(&args).unwrap();
    let out = stdioutils::test_stdout_clear();

    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }
}
