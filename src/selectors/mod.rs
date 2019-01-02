use utils;
use icalwrap::IcalVEvent;

use self::daterange::{SelectFilterFrom,SelectFilterTo};

mod cal;
mod grep;
mod test;
pub mod daterange;

pub struct SelectFilters {
  pub from: SelectFilterFrom,
  pub to: SelectFilterTo,
  others: Vec<Box<dyn SelectFilter>>,
}

pub trait SelectFilter {
  fn includes(&self, event: &IcalVEvent) -> bool;
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
        "grep" => {
          let grep_filter = grep::GrepFilter::new(&args[1]);
          others.push(Box::new(grep_filter));
          args = &args[2..];
        }
        "cal" => {
          let cal_filter = cal::CalendarFilter::new(&args[1]);
          others.push(Box::new(cal_filter));
          args = &args[2..];
        }
        _ => return Err("select [from|to|in|on|grep|cal parameter]+".to_string())
      }
    }

    // debug!("from: {:?}, to: {:?}", from, to);
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
    for filter in &self.others {
      if ! filter.includes(event) {
        return false;
      }
    }
    return true;
  }

  pub fn predicate(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.line_is_from(event) && self.line_is_to(event) && self.others(event)
    }
  }
}
