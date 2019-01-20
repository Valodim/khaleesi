use std::ops::Deref;
use std::ffi::CStr;
use chrono::prelude::*;
use ical;
use utils::dateutil;
use super::IcalTimeZone;
use std::fmt::{Error,Display,Formatter};

pub struct IcalTime {
  time: ical::icaltimetype,
}

impl IcalTime {
  pub fn now() -> Self {
    dateutil::now().into()
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

impl<T: Into<IcalTime> + Clone> From<&T> for IcalTime {
  fn from(time: &T) -> IcalTime {
    time.clone().into()
  }
}

impl From<DateTime<Local>> for IcalTime {
  fn from(time: DateTime<Local>) -> IcalTime {
    let timestamp = time.timestamp();
    let is_date = 0;
    let timezone = IcalTimeZone::local();
    let time = unsafe {
      ical::icaltime_from_timet_with_zone(timestamp, is_date, *timezone)
    };

    IcalTime{ time }
  }
}

impl From<DateTime<Utc>> for IcalTime {
  fn from(time: DateTime<Utc>) -> IcalTime {
    let timestamp = time.timestamp();
    let is_date = 0;
    let timezone = IcalTimeZone::utc();
    let time = unsafe {
      ical::icaltime_from_timet_with_zone(timestamp, is_date, *timezone)
    };

    IcalTime{ time }
  }
}

impl<T: TimeZone> From<Date<T>> for IcalTime {
  fn from(date: Date<T>) -> IcalTime {
    let timestamp = date.with_timezone(&Utc).and_hms(0, 0, 0).timestamp();
    let is_date = 1;
    let timezone = IcalTimeZone::utc(); //TODO this should correspond to the timezone T
    let time = unsafe {
      ical::icaltime_from_timet_with_zone(timestamp, is_date, *timezone)
    };

    IcalTime{ time }
  }
}

