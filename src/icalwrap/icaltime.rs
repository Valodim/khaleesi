use std::ops::Deref;
use std::ffi::{CStr,CString};
use chrono::{Date,DateTime,TimeZone,Utc,Local};
use ical;
use utils::dateutil;
use super::IcalTimeZone;
use super::TZ_MUTEX;
use std::fmt::{Error,Display,Formatter};
use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct IcalTime {
  time: ical::icaltimetype,
}

impl IcalTime {
  pub fn utc() -> Self {
    dateutil::now().into()
  }

  pub fn local() -> Self {
    dateutil::now().with_timezone(&Local).into()
  }

  pub fn from_ymd(year: i32, month: i32, day: i32) -> Self {
    let utc = IcalTimeZone::utc();
    let time = ical::icaltimetype{
      year, month, day,
      hour: 0, minute: 0, second: 0,
      is_date: 1,
      is_daylight: 0,
      zone: *utc
    };
    IcalTime{ time }
  }

  pub fn from_ymdhms(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
    let utc = IcalTimeZone::utc();
    let time = ical::icaltimetype{
      year, month, day,
      hour, minute, second,
      is_date: 0,
      is_daylight: 0,
      zone: *utc
    };
    IcalTime{ time }
  }

  pub fn from_timestamp(timestamp: i64) -> Self {
    let _lock = TZ_MUTEX.lock();
    let utc = IcalTimeZone::utc();
    let is_date = 0;
    let time = unsafe {
      ical::icaltime_from_timet_with_zone(timestamp, is_date, *utc)
    };
    IcalTime{ time }
  }

  pub fn timestamp(&self) -> i64 {
    let _lock = TZ_MUTEX.lock();
    unsafe { ical::icaltime_as_timet_with_zone(self.time, self.time.zone) }
  }

  pub fn is_date(&self) -> bool {
    self.time.is_date != 0
  }

  pub fn as_date(&self) -> IcalTime {
    let mut time = self.time.clone();
    time.is_date = 1;
    IcalTime{ time }
  }

  pub fn get_timezone(&self) -> IcalTimeZone {
    let tz_ptr = unsafe {
      ical::icaltime_get_timezone(self.time)
    };
    IcalTimeZone::from_ptr_copy(tz_ptr)
  }

  pub fn with_timezone(&self, timezone: &IcalTimeZone) -> IcalTime {
    let _lock = TZ_MUTEX.lock();
    let time = unsafe {
      ical::icaltime_convert_to_zone(self.time, **timezone)
    };
    let result = IcalTime { time };
    result
  }

  pub fn pred(&self) -> IcalTime {
    let mut time = self.time;
    time.day -= 1;
    let time = unsafe { ical::icaltime_normalize(time) };
    IcalTime{ time }
  }

  pub fn succ(&self) -> IcalTime {
    let mut time = self.time;
    time.day += 1;
    let time = unsafe { ical::icaltime_normalize(time) };
    IcalTime{ time }
  }
}

impl Deref for IcalTime {
  type Target = ical::icaltimetype;

  fn deref(&self) -> &ical::icaltimetype {
    &self.time
  }
}

impl Display for IcalTime {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    let cstr = unsafe {
      CStr::from_ptr(ical::icaltime_as_ical_string(self.time))
    };
    let string = cstr.to_string_lossy().into_owned();
    write!(f, "{}", string)
  }
}

impl FromStr for IcalTime {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c_str = CString::new(s).unwrap();
    let time = unsafe {
      let time = ical::icaltime_from_string(c_str.as_ptr());
      if ical::icaltime_is_null_time(time) != 0 {
        Some(time)
      } else {
        None
      }
    };
    if let Some(time) = time {
      Ok(IcalTime { time })
    } else {
      return Err(format!("Could not parse time {}", s));
    }
  }
}

impl PartialEq<IcalTime> for IcalTime {
  fn eq(&self, rhs: &IcalTime) -> bool {
    let _lock = TZ_MUTEX.lock();
    let cmp = unsafe { ical::icaltime_compare(self.time, rhs.time) };
    cmp == 0
  }
}

impl Eq for IcalTime {}

impl From<ical::icaltimetype> for IcalTime {
  fn from(time: ical::icaltimetype) -> IcalTime {
    IcalTime { time }
  }
}

impl<T: Into<IcalTime> + Clone> From<&T> for IcalTime {
  fn from(time: &T) -> IcalTime {
    time.clone().into()
  }
}

impl From<DateTime<Local>> for IcalTime {
  fn from(time: DateTime<Local>) -> IcalTime {
    let timestamp = time.timestamp();
    let local = IcalTimeZone::local();
    IcalTime::from_timestamp(timestamp).with_timezone(&local)
  }
}

impl From<DateTime<Utc>> for IcalTime {
  fn from(time: DateTime<Utc>) -> IcalTime {
    let timestamp = time.timestamp();
    IcalTime::from_timestamp(timestamp)
  }
}

impl From<Date<Local>> for IcalTime {
  fn from(date: Date<Local>) -> IcalTime {
    let timestamp = date.with_timezone(&Utc).and_hms(0, 0, 0).timestamp();
    let timezone = IcalTimeZone::local();
    IcalTime::from_timestamp(timestamp).with_timezone(&timezone).as_date()
  }
}

impl From<Date<Utc>> for IcalTime {
  fn from(date: Date<Utc>) -> IcalTime {
    let timestamp = date.and_hms(0, 0, 0).timestamp();
    IcalTime::from_timestamp(timestamp).as_date()
  }
}

impl From<IcalTime> for Date<Local> {
  fn from(time: IcalTime) -> Date<Local> {
    Local.timestamp(time.timestamp(), 0).date()
  }
}

impl From<IcalTime> for DateTime<Local> {
  fn from(time: IcalTime) -> DateTime<Local> {
    Local.timestamp(time.timestamp(), 0)
  }
}

impl From<IcalTime> for Date<Utc> {
  fn from(time: IcalTime) -> Date<Utc> {
    Utc.timestamp(time.timestamp(), 0).date()
  }
}

impl From<IcalTime> for DateTime<Utc> {
  fn from(time: IcalTime) -> DateTime<Utc> {
    Utc.timestamp(time.timestamp(), 0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use testdata;

  #[test]
  fn test_now() {
    let now = IcalTime::utc();

    assert_eq!("20130101T010203Z", now.to_string());
    assert_eq!(1357002123, now.timestamp());
  }

  #[test]
  fn test_from_local() {
    testdata::setup();
    let local_time = Local.ymd(2014, 01, 01).and_hms(01, 02, 03);
    let time = IcalTime::from(local_time);

    assert_eq!("Europe/Berlin", time.get_timezone().get_name());
    assert_eq!(1388534523, time.timestamp());
  }

  #[test]
  fn test_with_timezone_xxx() {
    let utc = IcalTime::utc();
    let tz = IcalTimeZone::from_name("US/Eastern").unwrap();

    let time = utc.with_timezone(&tz);

    assert_eq!("US/Eastern", time.get_timezone().get_name());
    assert_eq!("20121231T200203", time.to_string());
    assert_eq!(1357002123, time.timestamp());
  }

  #[test]
  fn test_from_local_date() {
    testdata::setup();
    let local_date = Local.ymd(2014, 01, 01);
    let time = IcalTime::from(local_date);

    assert_eq!("Europe/Berlin", time.get_timezone().get_name());
    assert_eq!("20140101", time.to_string());
  }

  #[test]
  fn test_from_utc_date() {
    let utc_date = Utc.ymd(2014, 01, 01);
    let time = IcalTime::from(utc_date);

    assert_eq!("UTC", time.get_timezone().get_name());
    assert_eq!("20140101", time.to_string());
  }

}
