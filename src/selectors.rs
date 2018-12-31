use chrono::*;
use std::cmp;
use std::str::FromStr;

use dateutil;
use icalwrap::IcalVEvent;
use utils;

pub struct SelectFilters {
  pub from: SelectFilterFrom,
  pub to: SelectFilterTo,
  others: Vec<Box<dyn SelectFilter>>,
}

trait SelectFilter {
  fn includes(&self, event: &IcalVEvent) -> bool;
}

struct CalendarFilter {
  cal_name: String
}

impl SelectFilter for CalendarFilter {
  fn includes(&self, event: &IcalVEvent) -> bool {
    event.get_parent()
      .and_then(|cal| cal.get_path())
      .and_then(|path| path.parent())
      .map(|path| path.ends_with(&self.cal_name))
      .unwrap_or(false)
  }
}

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
  fn includes_date(&self, cmp_date: DateTime<Local>) -> bool {
    self.date.map_or(true, |date| date <= cmp_date.date())
  }

  fn from_date(date: Option<Date<Local>>) -> Self {
    Self { date, bucket: date.map(|date| utils::get_bucket_for_date(&date))  }
  }

  fn combine_with(self, other: Self) -> Self {
    let date = if self.date.is_some() {
      cmp::max(self.date, other.date)
    } else {
      other.date
    };
    Self::from_date(date)
  }
}

impl SelectFilterTo {
  fn includes_date(&self, cmp_date: DateTime<Local>) -> bool {
    self.date.map_or(true, |date| cmp_date.date() <= date)
  }

  fn from_date(date: Option<Date<Local>>) -> Self {
    Self { date, bucket: date.map(|date| utils::get_bucket_for_date(&date))  }
  }

  fn combine_with(self, other: Self) -> Self {
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

impl SelectFilters {
  pub fn parse_from_args(mut args: &[String]) -> Result<Self, String> {
    let mut from: SelectFilterFrom = Default::default();
    let mut to: SelectFilterTo = Default::default();
    let mut others: Vec<Box<dyn SelectFilter>> = vec!();

    while !args.is_empty() {
      match args[0].as_str() {
        "from" => {
          from = from.combine_with(args[1].parse()?);
          args = &args[2..];
        }
        "to" => {
          to = to.combine_with(args[1].parse()?);
          args = &args[2..];
        }
        "in" | "on" => {
          from = from.combine_with(args[1].parse()?);
          to = to.combine_with(args[1].parse()?);
          args = &args[2..];
        }
        "cal" => {
          let cal_filter = CalendarFilter { cal_name: args[1].to_owned() };
          others.push(Box::new(cal_filter));
          args = &args[2..];
        }
        _ => return Err("select [from|to parameter]+".to_string())
      }
    }

    // debug!("from: {:?}, to: {:?}", from, to);
    Ok(SelectFilters { from, to, others })
  }
  pub fn predicate_line_is_from(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      let starts_after = self.from.includes_date(event.get_dtstart().unwrap());
      let ends_after = self.from.includes_date(event.get_dtend().unwrap());
      starts_after || ends_after
    }
  }

  pub fn predicate_line_is_to(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.to.includes_date(event.get_dtstart().unwrap())
    }
  }

  pub fn predicate_others(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      for filter in &self.others {
        if ! filter.includes(event) {
          return false;
        }
      }
      return true;
    }
  }
}

#[cfg(test)]
use testdata;
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

#[cfg(test)]
fn test_filter_event(args: &[&str]) -> bool {
  // DTSTART: 2007-06-28
  // DTEND: 2007-07-09
  let event = testdata::get_test_event(testdata::TEST_EVENT_MULTIDAY);

  let args: Vec<String> = args.into_iter().map(|x| x.to_string()).collect();
  let filters = SelectFilters::parse_from_args(&args).unwrap();
  let pred_from = filters.predicate_line_is_from()(&event);
  let pred_to = filters.predicate_line_is_to()(&event);

  pred_from && pred_to
}
