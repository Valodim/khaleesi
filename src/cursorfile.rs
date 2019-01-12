extern crate atty;

use std::fs::rename;

use defaults::*;
use khline::KhLine;
use utils::fileutil;

pub fn write_cursorfile(lines: &str) -> Result<(), String> {
  let tmpfilename = get_datafile("tmpcursor");

  if let Err(error) = fileutil::write_file(&tmpfilename, lines) {
    return Err(format!("Could not write cursorfile: {}", error));
  }

  let cursorfile = get_cursorfile();
  if let Err(error) = rename(tmpfilename, cursorfile) {
    return Err(format!("{}", error));
  }

  Ok(())
}

pub fn read_cursorfile() -> Result<KhLine, String> {
  let cursorfile = get_cursorfile();
  debug!("Reading cursor file: {}", cursorfile.to_string_lossy());
  let lines = match fileutil::read_lines_from_file(&cursorfile) {
    Ok(lines) => lines.collect::<Vec<String>>(),
    Err(error) => return Err(format!("{}", error)),
  };
  if lines.len() > 1 {
    Err("too many lines in cursorfile".to_string())
  } else {
    lines[0].parse::<KhLine>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::fs;

  use testutils::*;
  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use utils::fileutil;

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
