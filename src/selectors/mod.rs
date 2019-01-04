use std::collections::HashMap;

use utils;
use icalwrap::IcalVEvent;

use self::daterange::{SelectFilterFrom,SelectFilterTo};
use self::cal::CalendarFilter;
use self::grep::GrepFilter;
use self::prop::PropFilter;

mod cal;
mod grep;
mod prop;
mod test;
pub mod daterange;

pub struct SelectFilters {
  pub from: SelectFilterFrom,
  pub to: SelectFilterTo,
  others: Vec<Box<dyn SelectFilter>>,
}

pub trait SelectFilter {
  fn add_term(&mut self, it: &mut dyn Iterator<Item = &String>);
  fn is_not_empty(&self) -> bool;
  fn includes(&self, event: &IcalVEvent) -> bool;
}

impl SelectFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut from: SelectFilterFrom = Default::default();
    let mut to: SelectFilterTo = Default::default();
    let mut others: HashMap<&str, Box<dyn SelectFilter>> = HashMap::with_capacity(3);
    others.insert("grep", Box::new(GrepFilter::default()));
    others.insert("cal", Box::new(CalendarFilter::default()));
    others.insert("prop", Box::new(PropFilter::default()));

    let mut it = args.into_iter();
    loop {
      if let Some(arg) = it.next() {
        match arg.as_str() {
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
            } else {
              return Err("select [from|to|in|on|grep|cal parameter]+".to_string())
            }
          }
        }
      } else {
        break;
      }
    }

    let others = others.drain()
      .map(|x| x.1)
      .filter(|filter| filter.is_not_empty())
      .collect();

    Ok(SelectFilters { from, to, others })
  }

  fn line_is_from(&self, event: &IcalVEvent) -> bool {
    let starts_after = self.from.includes_date(event.get_dtstart().unwrap());
    let ends_after = self.from.includes_date(event.get_dtend().unwrap());
    starts_after || ends_after
  }

  fn line_is_to(&self, event: &IcalVEvent) -> bool {
    self.to.includes_date(event.get_dtstart().unwrap())
  }

  fn others(&self, event: &IcalVEvent) -> bool {
    self.others.is_empty() || self.others.iter().any(|filter| filter.includes(event))
  }

  pub fn predicate(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.line_is_from(event) && self.line_is_to(event) && self.others(event)
    }
  }
}
