#[cfg(test)]
use super::*;

#[cfg(test)]
use testdata;

#[cfg(test)]
pub fn test_filter_event(args: &[&str]) -> bool {
  // DTSTART: 2007-06-28
  // DTEND: 2007-07-09
  let event = testdata::get_test_event(testdata::TEST_EVENT_MULTIDAY);

  let args: Vec<String> = args.into_iter().map(|x| x.to_string()).collect();
  let filters = SelectFilters::parse_from_args(&args).unwrap();
  let predicate = filters.predicate();
  predicate(&event)
}
