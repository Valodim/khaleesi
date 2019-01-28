use std::collections::HashMap;

use super::*;

use icalwrap::IcalVEvent;
use icalwrap::IcalComponent;

pub struct PropFilter {
  terms: HashMap<String,Vec<String>>
}

impl SelectFilter for PropFilter  {
  fn add_term(&mut self, it: &mut dyn Iterator<Item = &&str>) {
    let term = it.next().unwrap().to_uppercase();
    let value = it.next().unwrap();
    self.terms.entry(term)
      .and_modify(|x| x.push(value.to_lowercase()))
      .or_insert_with(|| vec!(value.to_lowercase()));
  }

  fn is_not_empty(&self) -> bool {
    !self.terms.is_empty()
  }

  fn includes(&self, event: &IcalVEvent) -> bool {
    for (term, values) in &self.terms {
      for prop in event.get_properties_by_name(term) {
        let value = prop.get_value().to_lowercase();
        if values.iter().any(|x| value.contains(x)) {
          return true;
        }
      }
    }
    false
  }
}

impl Default for PropFilter {
  fn default() -> Self {
    PropFilter { terms: HashMap::new() }
  }
}

#[cfg(test)]
mod tests {
  use super::test::test_filter_event;
  use testdata;

  #[test]
  fn test_prop() {
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["prop", "TRANSP", "TRANSPARENT"]);
    assert_eq!(true, filtered);
  }

  #[test]
  fn test_prop_nocase() {
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["prop", "tRaNsP", "tRaNsPaReNt"]);
    assert_eq!(true, filtered);
  }

  #[test]
  fn test_prop_negative() {
    let filtered = test_filter_event(&testdata::TEST_EVENT_MULTIDAY, None, &["prop", "TRANSP", "nonexistent term"]);
    assert_eq!(false, filtered);
  }
}
