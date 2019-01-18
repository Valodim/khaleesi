use chrono::prelude::*;
use icalwrap::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path,PathBuf};
use std::time::SystemTime;
use walkdir::DirEntry;

use defaults::*;
use super::indextime;
use utils::fileutil;
use utils::lock;
use utils::misc;
use KhResult;

pub fn action_index(mut args: &[String]) -> KhResult<()> {
  let reindex = !args.is_empty() && args[0] == "--reindex";
  if reindex {
    args = &args[1..];
  }
  let indexpath = if args.is_empty() {
    get_caldir()
  } else {
    PathBuf::from(&args[0])
  };

  index_dir(&indexpath, reindex)
}

fn add_buckets_for_calendar(buckets: &mut HashMap<String, Vec<String>>, cal: &IcalVCalendar) {
  use super::bucketable::Bucketable;
  use super::bucketable::Merge;

  match cal.get_buckets() {
    Ok(cal_buckets) => buckets.merge(cal_buckets),
    Err(error) => {
      warn!("{}", error)
    }
  }
}

fn index_dir(dir: &Path, reindex: bool) -> KhResult<()> {
  use std::time::Instant;

  let _lock = lock::lock_file_exclusive(&get_indexlockfile())?;

  info!("Recursively indexing '.ics' files in directory: {}", dir.to_string_lossy());
  if !dir.exists() {
    Err(format!("Directory doesn't exist: {}", dir.to_string_lossy()))?;
  }

  let now = Instant::now();
  let start_time = Utc::now();

  let last_index_time = if reindex {
    debug!("Forced reindex, indexing all files");
    None
  } else {
    let last_index_time = indextime::get_index_time();
    match last_index_time {
      Some(time) => debug!("Previously indexed {}, indexing newer files only", time.with_timezone(&Local)),
        None => debug!("No previous index time, indexing all files"),
    }
    last_index_time
  };

  let modified_since = last_index_time.map(|time| time.timestamp()).unwrap_or(0);
  let ics_files = get_ics_files(dir, modified_since);

  let buckets = read_buckets(ics_files);

  let indexdir = get_indexdir();
  let clear_index_dir = last_index_time.is_none();
  prepare_index_dir(&indexdir, clear_index_dir)?;

  write_index(&indexdir, &buckets);
  info!("Index written in {}ms", misc::format_duration(&now.elapsed()));

  indextime::write_index_time(&start_time);

  Ok(())
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
    debug!("Indexing file: {:?}", file);
    match fileutil::read_file_to_string(&file) {
      Ok(content) => {
        total_files += 1;
        match IcalVCalendar::from_str(&content, Some(&file)) {
          Ok(mut cal) => add_buckets_for_calendar(&mut buckets, &cal),
          Err(error) => error!("{:?}: {}", file, error)
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

#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;

  #[test]
  fn test_index() {
    let testdir = prepare_testdir("testdir");

    action_index(&[]).unwrap();

    testdir.child(".khaleesi/index/2018-W50").assert("1544740200 twodaysacrossbuckets.ics\n");
    testdir.child(".khaleesi/index/2018-W51").assert("1544740200 twodaysacrossbuckets.ics\n");
  }
}
