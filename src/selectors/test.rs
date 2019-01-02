#[cfg(test)]
use super::*;
#[cfg(test)]
use testdata;
#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub fn test_filter_event(event_str: &str, path: Option<PathBuf>, args: &[&str]) -> bool {
  let event = testdata::get_test_event(event_str, path);

  let args: Vec<String> = args.into_iter().map(|x| x.to_string()).collect();
  let filters = SelectFilters::parse_from_args(&args).unwrap();
  let predicate = filters.predicate();
  predicate(&event)
}
