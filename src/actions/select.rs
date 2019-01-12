use std::path::PathBuf;

use defaults;
use selectors::{SelectFilters,daterange::SelectFilterFrom,daterange::SelectFilterTo};
use utils::fileutil as utils;

impl SelectFilters {
  fn predicate_path_skip_while(&self) -> impl Fn(&PathBuf) -> bool + '_ {
    move |path| {
      let bucketname = match path.file_name() {
        Some(path_os_str) => path_os_str.to_string_lossy(),
        None => panic!("{:?} not a file", path),
      };
      self.from.is_bucket_before(&bucketname)
    }
  }

  fn predicate_path_take_while<'a>(&'a self) -> impl Fn(&PathBuf) -> bool + 'a {
    move |path| {
      let bucketname = match path.file_name() {
        Some(path_os_str) => path_os_str.to_string_lossy(),
        None => panic!("{:?} not a file", path),
      };
      self.to.is_bucket_while(&bucketname)
    }
  }
}

impl SelectFilterFrom {
  fn is_bucket_before(&self, bucketname: &str) -> bool {
    self.bucket.as_ref().map_or(false, |bucket| bucketname < bucket)
  }
}

impl SelectFilterTo {
  fn is_bucket_while(&self, bucketname: &str) -> bool {
    self.bucket.as_ref().map_or(true, |bucket| bucketname <= bucket)
  }
}

pub fn select_by_args(args: &[String]) {
  let filters = match SelectFilters::parse_from_args(args) {
    Err(error) => { println!("{}", error); return; },
    Ok(parsed_filters) => parsed_filters,
  };

  let indexdir = defaults::get_indexdir();

  let mut buckets: Vec<PathBuf> = utils::file_iter(&indexdir)
    .collect();
  buckets.sort_unstable();
  let buckets = buckets.into_iter()
    .skip_while(filters.predicate_path_skip_while())
    .take_while(filters.predicate_path_take_while());

  let cals = buckets.map(|bucket| utils::read_lines_from_file(&bucket))
    .filter_map(|lines| lines.ok())
    .flatten()
    .map(|line| utils::read_khaleesi_line(&line))
    .filter_map(|cal| cal.ok())
    .map(|cal| cal.get_principal_event())
    ;

  let mut lines: Vec<String> = cals
    .filter(|event| filters.is_selected(event))
    .map(|event| event.get_khaleesi_line())
    .flatten()
    .collect();

  lines.sort_unstable();
  lines.dedup();

  for line in lines {
    println!("{}", line);
  }
}
