use chrono::*;
use std::fmt::Display;
use std::time;

pub fn joinlines(first: &str, second: &str) -> String {
  use itertools::Itertools;

  let first = first.split(|x| x == '\n');
  let second = second.split(|x| x == '\n');
  let maxlen = first.clone().map(|x| x.len()).max().unwrap();

  first
    .zip(second)
    .map(|(fst, snd)| format!("{:width$} {}", fst, snd, width = maxlen))
    .join("\n")
}

pub fn datetime_from_timestamp(timestamp: &str) -> Option<DateTime<Utc>> {
  let timestamp_i64 = timestamp.parse::<i64>().ok()?;
  let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp_i64, 0)?;
  Some(DateTime::from_utc(naive_datetime, Utc))
}

pub fn format_duration(duration: &time::Duration) -> impl Display {
  //TODO replace this with duration.as_millis() when it becomes stable
  duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
}

pub fn get_bucket_for_date(date: Date<Local>) -> String {
  date.format("%G-W%V").to_string()
}

pub fn make_new_uid() -> String {
  use uuid::Uuid;

  let suffix = "@khaleesi";
  format!("{}{}", Uuid::new_v4().to_hyphenated_ref(), suffix)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn make_new_uid_test() {
    let mut uid = make_new_uid();
    assert_eq!(45, uid.len());
    assert_eq!("@khaleesi".to_string(), uid.split_off(36));
  }

  #[test]
  fn joinlines_test() {
    let first = ["123", "ß", "1234"].join("\n");
    let second = ["abc", "1", "Otto"].join("\n");
    let expected = indoc!("
      123  abc
      ß    1
      1234 Otto");
    assert_eq!(expected, joinlines(first.as_str(), second.as_str()));
  }
}
