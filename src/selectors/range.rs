use std::str::FromStr;

pub struct RangeFilter {
  from: usize,
  to: usize,
}

impl RangeFilter {
  pub fn includes(&self, index: usize) -> bool {
    index >= self.from && index <= self.to
  }
}

impl FromStr for RangeFilter {
  type Err = ();

  fn from_str(s: &str) -> Result<RangeFilter, Self::Err> {
    let bounds: Vec<usize> = s
      .splitn(2, ':')
      .map(|x| x.parse::<usize>())
      .flatten()
      .collect();
    if bounds.len() == 2 {
      return Ok(RangeFilter {
        from: bounds[0],
        to: bounds[1],
      });
    }
    if let Ok(index) = s.parse::<usize>() {
      return Ok(RangeFilter {
        from: index,
        to: index,
      });
    }
    Err(())
  }
}

#[cfg(test)]
mod tests {
  use crate::selectors::test::test_filter_event_index;
  use crate::testdata;

  #[test]
  fn test_index_single() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 1, &["1"]);
    assert!(filtered)
  }

  #[test]
  fn test_index_single_negative() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 5, &["1"]);
    assert_eq!(false, filtered)
  }

  #[test]
  fn test_index_range_lower() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 1, &["1:3"]);
    assert!(filtered)
  }

  #[test]
  fn test_index_range_middle() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 2, &["1:3"]);
    assert!(filtered)
  }

  #[test]
  fn test_index_range_upper() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 3, &["1:3"]);
    assert!(filtered)
  }

  #[test]
  fn test_index_range_negative() {
    let filtered = test_filter_event_index(&testdata::TEST_EVENT_MULTIDAY, 4, &["1:3"]);
    assert_eq!(false, filtered)
  }
}
