use icalwrap::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path,PathBuf};
use std::time::SystemTime;
use walkdir::DirEntry;

use defaults::*;
use utils::lock;
use utils::fileutil;
use chrono::prelude::*;

use indextime;

fn add_buckets_for_calendar(buckets: &mut HashMap<String, Vec<String>>, cal: &IcalVCalendar) {
  use bucketable::Bucketable;
  use bucketable::Merge;

  match cal.get_buckets() {
    Ok(cal_buckets) => buckets.merge(cal_buckets),
    Err(error) => {
      warn!("{}", error)
    }
  }
}

pub fn index_dir(dir: &Path, reindex: bool) {
  use std::time::Instant;

  let lock = lock::lock_file_exclusive(&get_indexlockfile());
  if lock.is_err() {
    error!("Failed to obtain index lock!");
    return;
  }

  info!("Recursively indexing '.ics' files in directory: {}", dir.to_string_lossy());
  if !dir.exists() {
    error!("Directory doesn't exist: {}", dir.to_string_lossy());
    return;
  }

  let now = Instant::now();
  let start_time = Utc::now();

  let last_index_time = match reindex {
    true => {
      debug!("Forced reindex, indexing all files");
      None
    },
    false => {
      let last_index_time = indextime::get_index_time();
      match last_index_time {
        Some(time) => debug!("Previously indexed {}, indexing newer files only", time.with_timezone(&Local)),
        None => debug!("No previous index time, indexing all files"),
      }
      last_index_time
    }
  };

  let modified_since = last_index_time.map(|time| time.timestamp()).unwrap_or(0);
  let ics_files = get_ics_files(dir, modified_since);

  let buckets = read_buckets(ics_files);

  let indexdir = get_indexdir();
  let clear_index_dir = last_index_time.is_none();
  if let Err(error) = prepare_index_dir(&indexdir, clear_index_dir) {
    error!("{}", error);
    return;
  }

  write_index(&indexdir, &buckets);
  info!("Index written in {}ms", fileutil::format_duration(&now.elapsed()));

  indextime::write_index_time(&start_time);
}

pub fn get_ics_files(dir: &Path, modified_since: i64) -> impl Iterator<Item = PathBuf> {
  use walkdir::WalkDir;

  WalkDir::new(dir).into_iter()
    .filter_entry(move |entry| accept_entry(entry, modified_since))
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .filter(|e| e.path().extension().map_or(false, |extension| extension == "ics"))
    .map(|entry| entry.into_path())
}

fn accept_entry(dir_entry: &DirEntry, modified_since: i64) -> bool {
  if dir_entry.path().is_dir() {
    return true;
  }
  dir_entry.metadata()
    .map_err(|err| err.into()) // transform to io::Error
    .and_then(|metadata| metadata.modified())
    .map(|modified| modified.duration_since(SystemTime::UNIX_EPOCH).unwrap())
    .map(|modified| modified.as_secs() as i64)
    .map(|modified| modified > modified_since)
    .unwrap_or(false)
}

fn read_buckets(ics_files: impl Iterator<Item = PathBuf>) -> HashMap<String, Vec<String>> {
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  let mut total_files = 0;
  for file in ics_files {
    trace!("File: {:?}", file);
    match fileutil::read_file_to_string(&file) {
      Ok(content) => {
        total_files += 1;
        let file_cpy = file.clone();
        match IcalVCalendar::from_str(&content, Some(file)) {
          Ok(mut cal) => add_buckets_for_calendar(&mut buckets, &cal),
          Err(error) => error!("{:?}: {}", file_cpy, error)
        }
      }
      Err(error) => error!("{}", error),
    }
  }

  info!("Loaded {} files into {} buckets", total_files, buckets.len());
  buckets
}

fn write_index(index_dir: &Path, buckets: &HashMap<String, Vec<String>>) {
  for (key, val) in buckets.iter() {
    let bucketfile = bucket_file(index_dir, key);
    trace!("Writing bucket: {}", key);
    let content = &[&val.join("\n"), "\n"].concat();
    if let Err(error) = fileutil::append_file(&bucketfile, content) {
      error!("{}", error);
      return;
    }
  }
}

fn bucket_file(index_dir: &Path, key: &str) -> PathBuf {
  let mut result = PathBuf::from(index_dir);
  result.push(key);
  result
}

fn prepare_index_dir(indexdir: &Path, clear_index_dir: bool) -> Result<(), std::io::Error> {
  if indexdir.exists() && clear_index_dir {
    info!("Clearing index directory: {}", indexdir.to_string_lossy());
    fs::remove_dir_all(&indexdir)?
  }

  if !indexdir.exists() {
    info!("Creating index directory: {}", indexdir.to_string_lossy());
    fs::create_dir(&indexdir)?;
  }

  Ok(())
}
