extern crate atty;

use std::fs::rename;

use defaults::*;
use khline::KhLine;
use utils::fileutil;

pub fn write_cursorfile(lines: &str) {
  let tmpfilename = get_datafile("tmpcursor");

  if let Err(error) = fileutil::write_file(&tmpfilename, lines) {
    error!("Could not write cursorfile: {}", error);
    return
  }

  let cursorfile = get_cursorfile();
  if let Err(error) = rename(tmpfilename, cursorfile) {
    error!("{}", error)
  }
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

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn read_cursorfile_test() {
    let testdir = prepare_testdir("testdir_with_cursor");
    let khline = read_cursorfile().unwrap();
    let mut khline_string = khline.to_string();
    khline_string.push('\n');

    let predicate = predicate::str::similar(khline_string);
    testdir.child(".khaleesi/cursor").assert(predicate);
  }

  #[test]
  fn write_cursorfile_test() {
    let testdir = prepare_testdir("testdir");
    let teststr = "Teststr äöüß\n";

    write_cursorfile(teststr);
    testdir.child(".khaleesi/cursor").assert(teststr);
  }
}
