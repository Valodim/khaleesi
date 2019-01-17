use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::fs::OpenOptions;

use icalwrap::IcalVCalendar;
use khline::KhLine;

pub fn file_iter(dir: &Path) -> impl Iterator<Item = PathBuf> {
  use walkdir::WalkDir;

  WalkDir::new(dir).into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .map(|entry| entry.into_path())
}

pub fn dir_iter(dir: &Path) -> impl Iterator<Item = PathBuf> {
  use walkdir::WalkDir;

  let dir = dir.to_path_buf();
  WalkDir::new(&dir).into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_dir())
    .filter(move |f| f.path() != dir)
    .map(|entry| entry.into_path())
}

pub fn write_file(filepath: &Path, contents: &str) -> io::Result<()> {
  let mut file = fs::File::create(filepath)?;
  file.write_all(contents.as_bytes())
}

pub fn append_file(filepath: &Path, contents: &str) -> io::Result<()> {
  let mut file = OpenOptions::new()
              .append(true)
              .create(true)
              .open(filepath)?;
  file.write_all(contents.as_bytes())
}

pub fn write_cal(cal: &IcalVCalendar) -> Result<(), String> {
  match cal.get_path() {
    Some(path) => write_file(&path, &cal.to_string()).map_err(|error| format!("{}", error)),
    None => Err("calendar has no path".to_string()),
  }
}

pub fn read_lines_from_file(filepath: &Path) -> io::Result<impl Iterator<Item = String>> {
  let f = fs::File::open(filepath)?;
  let f = BufReader::new(f);
  let lines: Result<Vec<String>, io::Error> = f.lines().collect();
  match lines {
    Ok(result) => Ok(result.into_iter()),
    Err(error) => Err(error)
  }
}

pub fn read_single_char(mut source: impl BufRead) -> Result<char, String> {
  let mut buf = String::new();
  if let Err(error) = source.read_line(&mut buf) {
    return Err(format!("{}", error));
  }

  match buf.chars().next() {
    Some(c) => Ok(c),
    None => Err("failed to read from stdin".to_string()),
  }
}

pub fn read_lines_from_stdin() -> Result<Vec<String>, io::Error> {
  let stdin = io::stdin();
  let lines = stdin.lock().lines();

  lines.collect()
}

pub fn read_file_to_string(path: &Path) -> Result<String, String> {
  if let Ok(mut file) = fs::File::open(&path) {
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
      Ok(contents)
    } else {
      Err("Something went wrong reading the file".to_string())
    }
  } else {
    Err(format!("Could not open {} for reading", path.display()))
  }
}

pub fn read_calendar_from_path(path: &Path) -> Result<IcalVCalendar, String> {
  trace!("Reading calendar from {}", path.to_string_lossy());
  let content = match fs::read_to_string(path) {
    Ok(content) => content,
    Err(error) => return Err(format!("{} {:?}", error, path))
  };
  IcalVCalendar::from_str(&content, Some(path))
}

pub fn read_calendars_from_files(files: &mut Iterator<Item = String>) -> Result<Vec<IcalVCalendar>, String> {
  files
    .map(|line| line.parse::<KhLine>())
    .flatten()
    .map(|khline| khline.to_cal())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;

  #[test]
  fn test_append_file() {
    let testdir = prepare_testdir("testdir");
    let file = testdir.child("f");

    append_file(file.path(), "x\ny\n").unwrap();
    file.assert("x\ny\n");

    append_file(file.path(), "z\n").unwrap();
    file.assert("x\ny\nz\n");
  }

  #[test]
  fn test_write_file() {
    let testdir = prepare_testdir("testdir");
    let file = testdir.child("f");

    write_file(file.path(), "x\ny\n").unwrap();
    file.assert("x\ny\n");

    write_file(file.path(), "z\n").unwrap();
    file.assert("z\n");
  }

  #[test]
  fn read_single_char_test() {
    let source = "ab".as_bytes();
    let read_char = read_single_char(source).unwrap();
    assert_eq!('a', read_char);
  }
}
