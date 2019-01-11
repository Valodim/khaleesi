use super::*;

use icalwrap::IcalVEvent;

pub struct GrepFilter {
  terms: Vec<String>
}

impl SelectFilter for GrepFilter  {
  fn add_term(&mut self, it: &mut dyn Iterator<Item = &String>) {
    let term = it.next().unwrap();
    self.terms.push(term.to_lowercase());
  }

  fn is_not_empty(&self) -> bool {
    !self.terms.is_empty()
  }

  fn includes(&self, event: &IcalVEvent) -> bool {
    for term in &self.terms {
      if let Some(summary) = event.get_summary() {
        if summary.to_lowercase().contains(term) {
          return true;
        }
      }
      if let Some(description) = event.get_description() {
        if description.to_lowercase().contains(term) {
          return true;
        }
      }
      if let Some(location) = event.get_location() {
        if location.to_lowercase().contains(term) {
          return true;
        }
      }
    }
    false
  }
}

impl Default for GrepFilter {
  fn default() -> GrepFilter {
    GrepFilter { terms: Vec::new() }
  }
}

#[cfg(test)]
use super::test::test_filter_event;
#[cfg(test)]
use testdata;

#[test]
fn test_grep() {
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["grep", "International"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_location() {
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["grep", "Lobby"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_description() {
  let filtered = test_filter_event(&testdata::TEST_EVENT_ONE_MEETING, None, &["grep", "interoperability"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_case() {
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["grep", "InTeRnAtIOnAl"]);
  assert_eq!(true, filtered);
}

#[test]
fn test_grep_negative() {
  let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["grep", "nonexistent term"]);
  assert_eq!(false, filtered);
}

