extern crate chrono;
extern crate libc;

use icalwrap::*;
use std::collections::HashMap;
use utils;
use std::path::{Path,PathBuf};
use std::fs;
use defaults::*;

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

pub fn index_dir(dir: &Path) {
  use std::time::Instant;

  info!("Recursively indexing '.ics' files in directory: {}", dir.to_string_lossy());
  if !dir.exists() {
    error!("Directory doesn't exist: {}", dir.to_string_lossy());
    return;
  }

  let now = Instant::now();

  let ics_files = get_ics_files(dir);
  let buckets = read_buckets(ics_files);

  match prepare_index_dir() {
    Ok(_) => {
      write_index(buckets);
      info!("Index written in {}ms", utils::format_duration(&now.elapsed()));
    },
    Err(error) => error!("{}", error),
  }

}

fn get_ics_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
  utils::file_iter(dir)
    .filter( |path| path.extension().map_or(false, |extension| extension == "ics"))
}

fn read_buckets(ics_files: impl Iterator<Item = PathBuf>) -> HashMap<String, Vec<String>> {
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  let mut total_files = 0;
  for file in ics_files {
    match utils::read_file_to_string(&file) {
      Ok(content) => {
        total_files += 1;
        match IcalVCalendar::from_str(&content, Some(file)) {
          Ok(mut cal) => add_buckets_for_calendar(&mut buckets, &mut cal),
          Err(error) => error!("{}", error)
        }
      }
      Err(error) => error!("{}", error),
    }
  }

  info!("Loaded {} files into {} buckets", total_files, buckets.len());
  buckets
}

fn write_index(buckets: HashMap<String, Vec<String>>) {
  for (key, val) in buckets.iter() {
    let bucketfile = get_indexfile(key);
    trace!("Writing bucket: {}", key);
    if let Err(error) = utils::write_file(&bucketfile, val.join("\n")) {
      error!("{}", error);
      return;
    }
  }
}

fn prepare_index_dir() -> Result<(), std::io::Error> {
  let indexdir = get_indexdir();
  if indexdir.exists() {
    info!("Deleting index directory: {}", indexdir.to_string_lossy());
    fs::remove_dir_all(&indexdir)?
  }

  info!("Creating index directory: {}", indexdir.to_string_lossy());
  fs::create_dir(&indexdir)?;
  Ok(())
}

