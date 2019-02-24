use crate::seqfile;
use crate::utils::stdioutils;
use crate::KhResult;

pub fn action_seq() -> KhResult<()> {
  if !stdioutils::is_stdin_tty() {
    write_stdin_to_seqfile()?;
  } else {
    //println!("stdin is tty")
  }

  if !stdioutils::is_stdout_tty() || stdioutils::is_stdin_tty() {
    write_seqfile_to_stdout();
  }

  Ok(())
}

fn write_stdin_to_seqfile() -> KhResult<()> {
  let mut lines = stdioutils::read_lines_from_stdin()?.join("\n");
  lines.push_str("\n");

  seqfile::write_to_seqfile(&lines)?;

  Ok(())
}

fn write_seqfile_to_stdout() {
  if let Ok(sequence) = seqfile::read_seqfile() {
    for line in sequence {
      khprintln!("{}", line);
    }
  }
}

#[cfg(test)]
mod integration {
  use super::*;

  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use crate::testutils;
  use crate::utils::stdioutils;

  #[test]
  fn test_with_stdin() {
    let testdir = testutils::prepare_testdir_empty();
    stdioutils::test_stdin_write("hi\nthere");

    action_seq().unwrap();

    testdir.child(".khaleesi/seq").assert("hi\nthere\n");
  }

  #[test]
  fn test_no_stdin() {
    let testdir = testutils::prepare_testdir("testdir_with_seq");

    action_seq().unwrap();
    let out = stdioutils::test_stdout_clear();

    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/seq").assert(predicate);
  }

  #[test]
  fn test_with_stdin_stdout() {
    let testdir = testutils::prepare_testdir_empty();
    stdioutils::test_stdin_write("hi\nthere");
    stdioutils::test_stdout_set_tty(false);

    action_seq().unwrap();
    let out = stdioutils::test_stdout_clear();

    testdir.child(".khaleesi/seq").assert("hi\nthere\n");
    assert_eq!("hi\nthere\n", out);
  }
}

