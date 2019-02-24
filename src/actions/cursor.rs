use crate::cursorfile;
use crate::utils::stdioutils;
use crate::KhResult;
use crate::seqfile;
use crate::cli::Cursor;

enum Direction {
  Up,
  Down,
}

pub fn do_cursor(args: &Cursor) -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_cursorfile()?;
  } else {
    //println!("stdin is tty")
    if let Some(direction) = &args.direction {
      match direction.as_str() {
        "prev" => return cursor_sequence_move(&Direction::Up),
        "next" => return cursor_sequence_move(&Direction::Down),
        &_ => {}
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

    do_cursor(&[]).unwrap();

    testdir.child(".khaleesi/cursor").assert(expected_str);
  }

  #[test]
  fn test_cursor_sequence_move_next() {
    let testdir = testutils::prepare_testdir("testdir_with_seq_and_cursor");
    do_cursor(&["next"]).unwrap();

    let out = "1182988800 rfc_multi_day_allday.ics";
    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }

  #[test]
  fn test_cursor_sequence_move_prev_at_end() {
    let testdir = testutils::prepare_testdir("testdir_with_seq_and_cursor");
    do_cursor(&["prev"]).unwrap();

    let out = "1544740200 twodaysacrossbuckets.ics\n";
    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/cursor").assert(predicate);
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
