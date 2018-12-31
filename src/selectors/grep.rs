use super::*;

use icalwrap::IcalVEvent;

pub struct GrepFilter {
  term: String
}

impl GrepFilter {
  pub fn new(term: &str) -> Self {
    Self { term: term.to_lowercase().to_owned() }
  }
}

impl SelectFilter for GrepFilter  {
  fn includes(&self, event: &IcalVEvent) -> bool {
    if let Some(summary) = event.get_summary() {
      if summary.to_lowercase().contains(&self.term) {
        return true;
      }
    }
    if let Some(description) = event.get_description() {
      if description.to_lowercase().contains(&self.term) {
        return true;
      }
    }
    if let Some(location) = event.get_location() {
      if location.to_lowercase().contains(&self.term) {
        return true;
      }
    }
    false
  }
}

#[cfg(test)]
use super::test::test_filter_event;

#[test]
fn test_grep() {
  let filtered = test_filter_event(&["grep", "International"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_location() {
  let filtered = test_filter_event(&["grep", "Lobby"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_case() {
  let filtered = test_filter_event(&["grep", "InTeRnAtIOnAl"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_negative() {
  let filtered = test_filter_event(&["grep", "nonexistent term"]);
  assert_eq!(false, filtered);
}

