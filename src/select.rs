use chrono::*;
use std::cmp;
use std::path::PathBuf;
use std::str::FromStr;

use dateutil;
use defaults;
use icalwrap::IcalVEvent;
use utils;

struct SelectFilters {
  from: SelectFilterFrom,
  to: SelectFilterTo,
}

#[derive(Debug)]
struct SelectFilterFrom {
  date: Option<Date<Local>>,
  bucket: Option<String>,
}

#[derive(Debug)]
struct SelectFilterTo {
  date: Option<Date<Local>>,
  bucket: Option<String>,
}

impl SelectFilterFrom {
  fn is_bucket_before(&self, bucketname: &str) -> bool {
    self.bucket.as_ref().map_or(false, |bucket| bucketname < &bucket)
  }

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
  fn is_bucket_while(&self, bucketname: &str) -> bool {
    self.bucket.as_ref().map_or(true, |bucket| bucketname <= &bucket)
  }

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
        _ => return Err("select [from|to parameter]+".to_string())
      }
    }

    // debug!("from: {:?}, to: {:?}", from, to);
    Ok(SelectFilters { from, to })
  }

  pub fn predicate_path_skip_while(&self) -> impl Fn(&PathBuf) -> bool + '_ {
    move |path| {
      let bucketname = path.file_name().expect(&format!("{:?} not a file", path)).to_string_lossy();
      self.from.is_bucket_before(&bucketname)
    }
  }

  pub fn predicate_path_take_while<'a>(&'a self) -> impl Fn(&PathBuf) -> bool + 'a {
    move |path| {
      let bucketname = path.file_name().expect(&format!("{:?} not a file", path)).to_string_lossy();
      self.to.is_bucket_while(&bucketname)
    }
  }

  pub fn predicate_line_is_from(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.from.includes_date(event.get_dtstart().unwrap())
    }
  }

  pub fn predicate_line_is_to(&self) -> impl Fn(&IcalVEvent) -> bool + '_ {
    move |event| {
      self.to.includes_date(event.get_dtend().unwrap())
    }
  }
}

pub fn select_by_args(args: &[String]) {
  let filters: SelectFilters;

  match SelectFilters::parse_from_args(args) {
    Err(error) => {
      println!("{}", error);
      return
    },
    Ok(parsed_filters) => filters = parsed_filters,
  }

  let indexdir = defaults::get_indexdir();

  let mut buckets: Vec<PathBuf> = utils::file_iter(&indexdir)
    .collect();
  buckets.sort_unstable();
  let buckets = buckets.into_iter()
    .skip_while(filters.predicate_path_skip_while())
    .take_while(filters.predicate_path_take_while());

  let cals = buckets.map(|bucket| utils::read_lines_from_file(&bucket))
    .filter_map(|lines| lines.ok())
    .flatten()
    .map(|line| utils::read_khaleesi_line(&line))
    .filter_map(|cal| cal.ok())
    .map(|cal| cal.get_principal_event())
    ;

  let mut lines: Vec<String> = cals
    .filter( filters.predicate_line_is_from() )
    .filter( filters.predicate_line_is_to() )
    .map(|event| event.get_khaleesi_line())
    .flatten()
    .collect();

  lines.sort_unstable();
  lines.dedup();

  for line in lines {
    println!("{}", line);
  }
}
