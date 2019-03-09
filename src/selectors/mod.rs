use std::collections::HashMap;

use crate::khline::{KhLine,khlines_to_events};
use crate::icalwrap::IcalVEvent;
use crate::khevent::KhEvent;

use self::daterange::{SelectFilterFrom,SelectFilterTo};
use self::cal::CalendarFilter;
use self::grep::GrepFilter;
use self::prop::PropFilter;
use self::range::RangeFilter;

mod cal;
mod grep;
mod prop;
mod range;
pub mod daterange;
#[cfg(test)]
mod test;

pub struct SelectFilters {
  pub from: SelectFilterFrom,
  pub to: SelectFilterTo,
  pub range: Option<RangeFilter>,
  others: Vec<Box<dyn SelectFilter>>,
}

pub trait SelectFilter {
  fn add_term(&mut self, it: &mut dyn Iterator<Item = &&str>);
  fn is_not_empty(&self) -> bool;
  fn includes(&self, event: &IcalVEvent) -> bool;
}

impl SelectFilters {
  pub fn parse_from_args_with_range(args: &[&str]) -> Result<Self, String> {
    Self::parse_from_args_internal(args, true)
  }

  pub fn parse_from_args(args: &[&str]) -> Result<Self, String> {
    Self::parse_from_args_internal(args, false)
  }

  fn parse_from_args_internal(args: &[&str], with_range: bool) -> Result<Self, String> {
    let mut from: SelectFilterFrom = Default::default();
    let mut to: SelectFilterTo = Default::default();
    let mut range: Option<RangeFilter> = None;
    let mut others: HashMap<&str, Box<dyn SelectFilter>> = HashMap::with_capacity(3);
    others.insert("grep", Box::new(GrepFilter::default()));
    others.insert("cal", Box::new(CalendarFilter::default()));
    others.insert("prop", Box::new(PropFilter::default()));

    let mut it = args.iter();
    while let Some(arg) = it.next() {
      match *arg {
        "from" => {
          let term = it.next().unwrap();
          from = from.combine_with(&term.parse()?);
        }
        "to" => {
          let term = it.next().unwrap();
          to = to.combine_with(&term.parse()?);
        }
        "in" | "on" => {
          let term = it.next().unwrap();
          from = from.combine_with(&term.parse()?);
          to = to.combine_with(&term.parse()?);
        }
        term => {
          if let Some(filter) = others.get_mut(term) {
            filter.add_term(&mut it);
          } else if let Ok(parsed_range) = term.parse::<RangeFilter>() {
            if !with_range {
              return Err("Range selector not allowed here!".to_string())
            }
            if range.is_some() {
              return Err("Duplicate range selector!".to_string())
            }
            range = Some(parsed_range);
          } else {
            return Err("select [from|to|in|on|grep|cal parameter]+".to_string())
          }
        }
      }
    }

    let others = others.drain()
      .map(|x| x.1)
      .filter(|filter| filter.is_not_empty())
      .collect();

    Ok(SelectFilters { from, to, range, others })
  }

  fn line_is_from(&self, event: &IcalVEvent) -> bool {
    let starts_after = self.from.includes_date(event.get_start().unwrap().into());
    let ends_after = self.from.includes_date(event.get_end().unwrap().into());
    starts_after || ends_after
  }

  fn line_is_to(&self, event: &IcalVEvent) -> bool {
    self.to.includes_date(event.get_start().unwrap().into())
  }

  fn filter_index(&self, index: usize) -> bool {
    if let Some(range) = self.range.as_ref() {
      range.includes(index)
    } else {
      true
    }
  }

  fn others(&self, event: &IcalVEvent) -> bool {
    self.others.is_empty() || self.others.iter().any(|filter| filter.includes(event))
  }

  pub fn is_selected(&self, event: &IcalVEvent) -> bool {
    self.line_is_from(event) && self.line_is_to(event) && self.others(event)
  }

  pub fn is_selected_index(&self, index: usize, event: &IcalVEvent) -> bool {
    self.filter_index(index) && self.line_is_from(event) && self.line_is_to(event) && self.others(event)
  }

  pub fn filter_khlines(
    self,
    khlines: impl Iterator<Item = KhLine>,
  ) -> impl Iterator<Item = IcalVEvent> {
    let events = khlines_to_events(khlines);
    events
      .enumerate()
      .filter(move |(index, event)| self.is_selected_index(*index, event))
      .map(|(_, event)| event)
  }
}
