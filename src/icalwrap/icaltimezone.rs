use std::ops::Deref;
use ical;


pub struct IcalTimeZone {
  timezone: *mut ical::icaltimezone,
}

impl Deref for IcalTimeZone {
  type Target = *mut ical::icaltimezone;

  fn deref(&self) -> &*mut ical::icaltimezone {
    &self.timezone
  }
}


impl IcalTimeZone {
  fn from_ptr(timezone: *mut ical::icaltimezone) -> Self {
    IcalTimeZone{ timezone }
  }

  pub fn local() -> Self {
    // TODO
    let timezone = unsafe { ical::icaltimezone_get_utc_timezone() };
    Self::from_ptr(timezone)
  }

  pub fn utc() -> Self {
    let timezone = unsafe { ical::icaltimezone_get_utc_timezone() };
    Self::from_ptr(timezone)
  }
}

