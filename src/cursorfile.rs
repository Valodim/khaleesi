use std::fs::rename;
use std::io;

use crate::defaults::*;
use crate::khline::KhLine;
use crate::khevent::KhEvent;
use crate::utils::fileutil;
use crate::KhResult;

pub fn write_cursorfile(line: &str) -> KhResult<()> {
  let tmpfilename = get_datafile("tmpcursor");

  fileutil::write_file(&tmpfilename, line)?;

  let cursorfile = get_cursorfile();
  rename(tmpfilename, cursorfile)?;

  Ok(())
}

pub fn read_cursorfile() -> io::Result<KhLine> {
  let cursorfile = get_cursorfile();
  debug!("Reading cursor file: {}", cursorfile.to_string_lossy());
  let lines = fileutil::read_lines_from_file(&cursorfile)?.collect::<Vec<String>>();
  if lines.len() > 1 {
    Err(io::Error::new(io::ErrorKind::Other, "too many lines in cursorfile"))
  } else {
    lines[0].parse::<KhLine>().map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
  }
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
  fn read_cursorfile_ok() {
    let testdir = prepare_testdir("testdir_with_cursor");
    let khline = read_cursorfile().unwrap();
    let mut khline_string = khline.to_string();
    khline_string.push('\n');

    let predicate = predicate::str::similar(khline_string);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }

  #[test]
  fn read_cursorfile_empty() {
    let _testdir = prepare_testdir("testdir");

    let cursorfile = read_cursorfile();

    assert!(cursorfile.is_err());
  }

  #[test]
  fn read_cursorfile_broken() {
    let testdir = prepare_testdir("testdir_with_cursor");

    fileutil::append_file(testdir.child(".khaleesi/cursor").path(), "\nx").unwrap();
    let cursorfile = read_cursorfile();

    assert!(cursorfile.is_err());
  }

  #[test]
  fn write_cursorfile_ok() {
    let testdir = prepare_testdir("testdir");
    let teststr = "Teststr äöüß\n";

    let result = write_cursorfile(teststr);
    testdir.child(".khaleesi/cursor").assert(teststr);

    assert!(result.is_ok());
  }

  #[test]
  fn write_cursorfile_failed() {
    let testdir = prepare_testdir("testdir");

    fs::create_dir(testdir.child(".khaleesi/cursor").path()).unwrap();
    let result = write_cursorfile("abc");

    assert!(result.is_err());
  }
}
