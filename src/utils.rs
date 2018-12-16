extern crate walkdir;

use std::{fs, io, time};
use std::path::{Path,PathBuf};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::fmt::Display;
use icalwrap::IcalVCalendar;
use chrono::*;

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

pub fn date_from_str(date: &str) -> ParseResult<Date<Local>> {
  let naive_date = &NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
  Ok(Local.from_local_date(naive_date).unwrap())
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
  let cals = filenames.map(|f| read_calendar_from_file(&f));
  cals.filter_map(|cal| cal.ok())
}

pub fn read_calendars_from_files(files: &mut Iterator<Item = String>) -> Result<Vec<IcalVCalendar>, String> {
  files.map(|file| read_khaleesi_line(file)).collect()
}

pub fn read_khaleesi_line(kline: String) -> Result<IcalVCalendar, String> {
  let mut parts = kline.splitn(2, ' ');
  let timestamp_str = parts.next().ok_or("empty line")?;
  let timestamp = datetime_from_timestamp(timestamp_str).ok_or("bad timestamp")?;
  let path = Path::new(parts.next().ok_or("missing path")?);
  let calendar = read_calendar_from_path(path);
  Ok(calendar.unwrap().with_internal_timestamp(timestamp))
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
  let bucket = format!(
      "{}-{:02}",
      date.iso_week().year(),
      date.iso_week().week()
      );
  bucket
}

pub fn print_cals(cals: impl Iterator<Item = IcalVCalendar>) {
  for cal in cals {
    println!("{}", cal.get_path_as_string());
  }
}
