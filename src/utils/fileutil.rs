use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::fs::OpenOptions;

use icalwrap::IcalVCalendar;

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

pub fn write_cal(cal: &IcalVCalendar) -> io::Result<()> {
  match cal.get_path() {
    Some(path) => write_file(&path, &cal.to_string()),
    None => Err(io::Error::new(io::ErrorKind::Other, "calendar has no path")),
  }
}

pub fn read_lines_from_file(filepath: &Path) -> io::Result<impl Iterator<Item = String>> {
  let f = fs::File::open(filepath)?;
  let f = BufReader::new(f);
  let lines: Result<Vec<String>, io::Error> = f.lines().collect();
  lines.map(|result| result.into_iter())
}

pub fn read_lines_from_file_backwards(filepath: &Path) -> io::Result<impl Iterator<Item = String>> {
  let f = fs::File::open(filepath)?;
  let f = BufReader::new(f);
  let lines: Result<Vec<String>, io::Error> = f.lines().collect();
  lines.map(|result| result.into_iter().rev())
}

pub fn read_file_to_string(path: &Path) -> io::Result<String> {
  let mut file = fs::File::open(&path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
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
}
