use std::path::PathBuf;

use crate::defaults;
use crate::selectors::{SelectFilters,daterange::SelectFilterFrom,daterange::SelectFilterTo};
use crate::utils::fileutil as utils;
use crate::khline::KhLine;
use crate::KhResult;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SelectArgs {
  /// the arguments for the selection
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

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
    // cargo check && cargo build are fine,
    // cargo test gives this error:
    // error[E0277]: can't compare `str` with `std::string::String`
    //--> src/actions/select.rs:47:59
    //|
    //|     self.bucket.as_ref().map_or(true, |bucket| bucketname < bucket)
    //|                                                           ^ no implementation for `str < std::string::String` and `str > std::string::String`
    //|
    //= help: the trait `std::cmp::PartialOrd<std::string::String>` is not implemented for `str`
    //= note: required because of the requirements on the impl of `std::cmp::PartialOrd<&std::string::String>` for `&str`
    self.bucket.as_ref().map_or(false, |bucket| bucketname < bucket.as_str())
  }
}

impl SelectFilterTo {
  fn is_bucket_while(&self, bucketname: &str) -> bool {
    self.bucket.as_ref().map_or(true, |bucket| bucketname <= bucket.as_str())
  }
}

pub fn select_by_args(args: &[&str]) -> KhResult<()> {
  let filters = SelectFilters::parse_from_args(args)?;

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
    .map(|line| line.parse::<KhLine>())
    .filter_map(|cal| cal.ok())
    .map(|khline| khline.to_event())
    .flatten()
    ;

  let mut lines: Vec<String> = cals
    .filter(|event| filters.is_selected(event))
    .map(|event| KhLine::from(&event))
    .map(|khline| khline.to_string())
    .collect();

  lines.sort_unstable();
  lines.dedup();

  for line in lines {
    println!("{}", line);
  }

  Ok(())
}
