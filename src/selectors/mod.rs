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
  fn includes(&self, event: &IcalVEvent) -> bool;
}

impl SelectFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut from: SelectFilterFrom = Default::default();
    let mut to: SelectFilterTo = Default::default();
    let mut cal: Option<CalendarFilter> = None;
    let mut grep: Option<GrepFilter> = None;
    let mut prop: Option<PropFilter> = None;
    let mut others: Vec<Box<dyn SelectFilter>> = vec!();

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
          "cal" => {
            cal.get_or_insert_with(Default::default).add_term(&mut it);
          }
          "grep" => {
            grep.get_or_insert_with(Default::default).add_term(&mut it);
          }
          "prop" => {
            prop.get_or_insert_with(Default::default).add_term(&mut it);
          }
          _ => return Err("select [from|to|in|on|grep|cal parameter]+".to_string())
        }
      } else {
        break;
      }
    }
    if let Some(cal) = cal {
      others.push(Box::new(cal));
    }
    if let Some(grep) = grep {
      others.push(Box::new(grep));
    }
    if let Some(prop) = prop {
      others.push(Box::new(prop));
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
    true
  }

  pub fn predicate(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.line_is_from(event) && self.line_is_to(event) && self.others(event)
    }
  }
}
