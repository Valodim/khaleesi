use std::path::{Path,PathBuf};
use std::io::prelude::*;
use std::fs;
use std::io;
use std::iter;
use icalwrap::IcalVCalendar;
use chrono::*;
use std::io::{BufRead, BufReader};

pub fn joinlines(first: &str, second: &str) -> String {
    use itertools::Itertools;

    let first = first.split(|x| x == '\n');
    let second = second.split(|x| x == '\n');
    let maxlen = first.clone().map(|x| x.len()).max().unwrap();

    first
        .zip(second)
        .map(|(fst, snd)| format!("{:width$} {}", fst, snd, width = maxlen))
        .join("\n")
}

pub fn file_iter(dir: &Path) -> Box<Iterator<Item = PathBuf>> {
  if let Ok(entries) = fs::read_dir(dir) {
      let valid_entries = entries.filter(|x| x.is_ok());
      let extracted_paths = valid_entries.map(move |x| x.unwrap().path());
      Box::new(extracted_paths)
  } else {
      Box::new(iter::empty())
  }
}

pub fn write_file(relative_path_to_file: &String, contents: String) -> Result<(), io::Error> {
  use defaults;

  let mut filepath: String = defaults::DATADIR.to_owned();
  filepath.push_str("/");
  filepath.push_str(&relative_path_to_file);
  let mut file = fs::File::create(filepath)?;
  file.write_all(contents.as_bytes())
}

pub fn read_filenames_from_file(filepath: &Path) -> impl Iterator<Item = String> {
  let f = fs::File::open(filepath).expect("Unable to open file");
  let f = BufReader::new(f);
  let lines = f.lines().map(|x| x.expect("Unable to read line"));
  lines
}

pub fn read_filenames_from_stdin() -> impl Iterator<Item = String> {
  let stdin = std::io::stdin();
  let handle = stdin.lock();

  let lines = handle.lines().map(|x| x.expect("Unable to read line")).collect::<Vec<String>>().into_iter();
  lines
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

pub fn date_from_str(date: &str) -> NaiveDate {
  NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

pub fn read_calendar_from_file(filepath: &str) -> IcalVCalendar {
  let path = Path::new(filepath);
  let content = fs::read_to_string(path).expect(&format!("Could not read file {}", path.display()));
  IcalVCalendar::from_str(&content, Some(path.to_path_buf())).unwrap()
}

pub fn read_calendars_from_files(files: &mut Iterator<Item = String>) -> Vec<IcalVCalendar> {
  files.map(|file| read_calendar_from_file(&file)).collect()
}
