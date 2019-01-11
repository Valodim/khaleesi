use chrono::*;

pub fn date_from_str(date_str: &str) -> ParseResult<Date<Local>> {
  if date_str  == "today" || date_str == "now" {
    return Ok(Local::now().date());
  }
  let naive_date = &NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
  Ok(Local.from_local_date(naive_date).unwrap())
}

pub fn week_from_str_begin(date_str: &str) -> Result<Date<Local>,String> {
  let now = Local::now();
  if date_str == "toweek" || date_str == "thisweek" {
    return Ok(Local.isoywd(now.year(), now.iso_week().week(), Weekday::Mon));
  }
  if let Ok(date) = &NaiveDate::parse_from_str(&format!("{}-1", date_str), "%G-W%V-%u") {
    return Ok(Local.from_local_date(date).unwrap());
  }
  if let Ok(date) = &NaiveDate::parse_from_str(&format!("{}-{}-1", now.year(), date_str), "%G-W%V-%u") {
    return Ok(Local.from_local_date(date).unwrap());
  }
  Err("Could not parse '{}' as week".to_string())
}

pub fn week_from_str_end(date_str: &str) -> Result<Date<Local>,String> {
  let now = Local::now();
  if date_str == "toweek" || date_str == "thisweek"  {
    return Ok(Local.isoywd(now.year(), now.iso_week().week(), Weekday::Sun));
  }
  if let Ok(date) = &NaiveDate::parse_from_str(&format!("{}-7", date_str), "%G-W%V-%u") {
    return Ok(Local.from_local_date(date).unwrap());
  }
  if let Ok(date) = &NaiveDate::parse_from_str(&format!("{}-{}-7", now.year(), date_str), "%G-W%V-%u") {
    return Ok(Local.from_local_date(date).unwrap());
  }
  Err("Could not parse '{}' as week".to_string())
}

#[test]
fn test_date_from_str() {
  let date = date_from_str("2018-12-10").unwrap();
  assert_eq!("2018-12-10", format!("{}", date.format("%F")));
  let date = date_from_str("today").unwrap();
  assert_eq!(Local::now().date(), date);
  let date = date_from_str("now").unwrap();
  assert_eq!(Local::now().date(), date);
}

#[test]
#[should_panic]
fn test_date_from_str_negative() {
  date_from_str("2018-02-30").unwrap();
}

#[test]
fn test_week_from_str_begin() {
  let date = week_from_str_begin("2018-W50").unwrap();
  assert_eq!("2018-12-10", format!("{}", date.format("%F")));
  let date = week_from_str_begin("W50").unwrap();
  assert_eq!("2019-12-09", format!("{}", date.format("%F")));
}

#[test]
fn test_week_from_str_begin_current_year() {
  // TODO test must be adapted once a year. hum.
  let date = week_from_str_begin("W50").unwrap();
  assert_eq!("2019-12-09", format!("{}", date.format("%F")));
}

#[test]
#[should_panic]
fn test_week_from_str_begin_neg() {
  week_from_str_begin("nonsense").unwrap();
}

#[test]
fn test_week_from_str_end() {
  let date = week_from_str_end("W50").unwrap();
  assert_eq!("2019-12-15", format!("{}", date.format("%F")));
}
#[test]
fn test_week_from_str_end_current_year() {
  // TODO test must be adapted once a year. hum.
  let date = week_from_str_end("W50").unwrap();
  assert_eq!("2019-12-15", format!("{}", date.format("%F")));
}

#[test]
#[should_panic]
fn test_week_from_str_end_neg() {
  week_from_str_end("nonsense").unwrap();
}
