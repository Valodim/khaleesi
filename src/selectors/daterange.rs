use chrono::*;
use std::cmp;
use std::str::FromStr;

use super::*;

use dateutil;

#[derive(Debug)]
pub struct SelectFilterFrom {
  pub date: Option<Date<Local>>,
  pub bucket: Option<String>,
}

#[derive(Debug)]
pub struct SelectFilterTo {
  pub date: Option<Date<Local>>,
  pub bucket: Option<String>,
}

impl SelectFilterFrom {
  pub fn includes_date(&self, cmp_date: DateTime<Local>) -> bool {
    self.date.map_or(true, |date| date <= cmp_date.date())
  }

  fn from_date(date: Option<Date<Local>>) -> Self {
    Self { date, bucket: date.map(|date| utils::get_bucket_for_date(&date))  }
  }

  pub fn combine_with(self, other: Self) -> Self {
    let date = if self.date.is_some() {
      cmp::max(self.date, other.date)
    } else {
      other.date
    };
    Self::from_date(date)
  }
}

impl SelectFilterTo {
  pub fn includes_date(&self, cmp_date: DateTime<Local>) -> bool {
    self.date.map_or(true, |date| cmp_date.date() <= date)
  }

  fn from_date(date: Option<Date<Local>>) -> Self {
    Self { date, bucket: date.map(|date| utils::get_bucket_for_date(&date))  }
  }

  pub fn combine_with(self, other: Self) -> Self {
    let date = if self.date.is_some() {
      cmp::min(self.date, other.date)
    } else {
      other.date
    };
    Self::from_date(date)
  }
}

impl FromStr for SelectFilterFrom {
  type Err = String;

  fn from_str(s: &str) -> Result<SelectFilterFrom, Self::Err> {
    if let Ok(date) = dateutil::date_from_str(s) {
      return Ok(SelectFilterFrom::from_date(Some(date)));
    }
    if let Ok(weekdate) = dateutil::week_from_str_begin(s) {
      return Ok(SelectFilterFrom::from_date(Some(weekdate)));
    }
    Err(format!("Could not parse date '{}'", s).to_string())
  }
}

impl FromStr for SelectFilterTo {
  type Err = String;

  fn from_str(s: &str) -> Result<SelectFilterTo, Self::Err> {
    if let Ok(date) = dateutil::date_from_str(s) {
      return Ok(SelectFilterTo::from_date(Some(date)));
    }
    if let Ok(weekdate) = dateutil::week_from_str_end(s) {
      return Ok(SelectFilterTo::from_date(Some(weekdate)));
    }
    Err(format!("Could not parse date '{}'", s).to_string())
  }
}

impl Default for SelectFilterTo {
  fn default() -> SelectFilterTo {
    SelectFilterTo::from_date(None)
  }
}

impl Default for SelectFilterFrom {
  fn default() -> SelectFilterFrom {
    SelectFilterFrom::from_date(None)
  }
}

#[cfg(test)]
use self::test::test_filter_event;
#[test]
fn test_from_ends_before() {
  let filtered = test_filter_event(&["from", "2007-08-01"]);
  assert_eq!(false, filtered)
}
#[test]
fn test_from_begins_after() {
  let filtered = test_filter_event(&["from", "2007-06-01"]);
  assert_eq!(true, filtered);
}
#[test]
fn test_from_begins_before_ends_after() {
  let filtered = test_filter_event(&["from", "2007-07-01"]);
  assert_eq!(true, filtered);
}
#[test]
fn test_to_ends_before() {
  let filtered = test_filter_event(&["to", "2007-08-01"]);
  assert_eq!(true, filtered);
}
#[test]
fn test_to_begins_after() {
  let filtered = test_filter_event(&["to", "2007-06-01"]);
  assert_eq!(false, filtered);
}
#[test]
fn test_to_begins_before_ends_after() {
  let filtered = test_filter_event(&["to", "2007-07-01"]);
  assert_eq!(true, filtered);
}
