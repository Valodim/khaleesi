use seqfile;
use utils::stdioutils;
use KhResult;

pub fn do_seq(_args: &[&str]) -> KhResult<()> {
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
mod tests {
  use super::*;

  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use testutils;
  use utils::stdioutils;

  #[test]
  fn test_write_stdin_to_seqfile() {
    let testdir = testutils::prepare_testdir_empty();
    stdioutils::test_stdin_write("hi\nthere");

    write_stdin_to_seqfile().unwrap();

    testdir.child(".khaleesi/seq").assert("hi\nthere\n");
  }

  #[test]
  fn test_read_seqfile_to_stdout() {
    let testdir = testutils::prepare_testdir("testdir_with_seq");

    write_seqfile_to_stdout();
    let out = stdioutils::test_stdout_clear();

    let predicate = predicate::str::similar(out);
    testdir.child(".khaleesi/seq").assert(predicate);
  }
}

