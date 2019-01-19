use super::*;
use testdata;
use std::path::Path;

pub fn test_filter_event(event_str: &str, path: Option<&Path>, args: &[&str]) -> bool {
  let event = testdata::get_test_event(event_str, path);

  let filters = SelectFilters::parse_from_args(args).unwrap();
  filters.is_selected(&event)
}

pub fn test_filter_event_index(event_str: &str, index: usize, args: &[&str]) -> bool {
  let event = testdata::get_test_event(event_str, None);

  let filters = SelectFilters::parse_from_args_with_range(args).unwrap();
  filters.is_selected_index(index, &event)
}

#[test]
fn test_parse_range_check() {
  let args = &["1:5"];
  let ok = SelectFilters::parse_from_args_with_range(args);
  let err = SelectFilters::parse_from_args(args);
  assert!(ok.is_ok());
  assert!(err.is_err());
}
