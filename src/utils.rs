extern crate walkdir;

use chrono::*;
use std::fmt::Display;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::{Path,PathBuf};
use std::{fs, io, time};

use icalwrap::IcalVCalendar;

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

pub fn file_iter(dir: &Path) -> impl Iterator<Item = PathBuf> {
  use self::walkdir::WalkDir;

  WalkDir::new(dir).into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .map(|entry| entry.into_path())
}

pub fn write_file(filepath: &Path, contents: String) -> io::Result<()> {
  let mut file = fs::File::create(filepath)?;
  file.write_all(contents.as_bytes())
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

pub fn read_lines_from_stdin() -> io::Result<impl Iterator<Item = String>> {
  let stdin = io::stdin();
  let handle = stdin.lock();

  let lines: Result<Vec<String>, io::Error> = handle.lines().collect();
  match lines {
    Ok(result) => Ok(result.into_iter()),
    Err(error) => Err(error)
  }
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
  IcalVCalendar::from_str(&content, Some(path.to_path_buf()))
}

pub fn read_calendar_from_file(filepath: &str) -> Result<IcalVCalendar, String> {
  let path = Path::new(filepath);
  read_calendar_from_path(path)
}

pub fn iterate_calendars_from_files(filenames: impl Iterator<Item = String>) -> impl Iterator<Item = IcalVCalendar> {
  let cals = filenames.map(|f| read_khaleesi_line(&f));
  cals.filter_map(|cal| cal.ok())
}

pub fn read_calendars_from_files(files: &mut Iterator<Item = String>) -> Result<Vec<IcalVCalendar>, String> {
  files.map(|file| read_khaleesi_line(&file)).collect()
}

pub fn read_khaleesi_line(kline: &str) -> Result<IcalVCalendar, String> {
  let parts: Vec<&str> = kline.splitn(2, ' ').collect();
  if let Some(timestamp) = datetime_from_timestamp(parts[0]) {
    let path = Path::new(parts[1]);
    let calendar = read_calendar_from_path(path)?;
    let calendar = calendar.with_internal_timestamp(timestamp);
    Ok(calendar)
  } else {
    let path = Path::new(parts[0]);
    let calendar = read_calendar_from_path(path)?;
    Ok(calendar)
  }
}

pub fn datetime_from_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
  let timestamp_i64 = timestamp.parse::<i64>().ok()?;
  let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp_i64, 0)?;
  Some(DateTime::from_utc(naive_datetime, Utc))
}

pub fn format_duration(duration: &time::Duration) -> impl Display {
  duration.as_secs() * 1000 + (duration.subsec_millis() as u64)
}

pub fn get_bucket_for_date(date: &Date<Local>) -> String {
  date.format("%G-W%V").to_string()
}

pub fn print_cals(cals: impl Iterator<Item = IcalVCalendar>) {
  for cal in cals {
    if let Some(line) = cal.get_principal_event().get_khaleesi_line() {
      println!("{}", line);
    }
  }
}

pub fn make_new_uid() -> String {
  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};

  let suffix = "@khaleesi";
  let mut hasher = DefaultHasher::new();
  Local::now().hash(&mut hasher);
  let mut hash_string = hasher.finish().to_string();

  hash_string.push_str(suffix);
  hash_string
}
