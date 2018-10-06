use chrono::{NaiveDate};
use std::ffi::{CStr,CString};

use ical;

pub struct Icalcomponent {
  iterating: bool,
  pub ptr: *mut ical::icalcomponent,
}

pub struct IcalProperty {
  pub ptr: *mut ical::icalproperty,
}

impl Drop for Icalcomponent {
  fn drop(&mut self) {
    unsafe {
      ical::icalcomponent_free(self.ptr);
    }
  }
}

impl Icalcomponent {
  fn from_ptr(ptr: *mut ical::icalcomponent) -> Icalcomponent {
    Icalcomponent {
      ptr,
      iterating: false, }
  }

  pub fn from_str(str: &str) -> Icalcomponent {
    unsafe {
      let parsed_event = ical::icalparser_parse_string(CString::new(str).unwrap().as_ptr());
      Icalcomponent::from_ptr(parsed_event)
    }
  }

  pub fn get_dtstart_unix(self: &Icalcomponent) -> i64 {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      ical::icaltime_as_timet(dtstart)
    }
  }

  pub fn get_dtend(self: &Icalcomponent) -> NaiveDate {
    unsafe {
      let dtend = ical::icalcomponent_get_dtend(self.ptr);
      NaiveDate::from_ymd(dtend.year, dtend.month as u32, dtend.day as u32)
    }
  }

  pub fn get_dtstart(self: &Icalcomponent) -> NaiveDate {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      NaiveDate::from_ymd(dtstart.year, dtstart.month as u32, dtstart.day as u32)
    }
  }

  pub fn get_uid(self: &Icalcomponent) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalcomponent_get_uid(self.ptr));
      foo.to_string_lossy().into_owned()
    }
  }

}

impl Iterator for Icalcomponent {
  type Item = Icalcomponent;

  fn next(&mut self) -> Option<Icalcomponent> {
    unsafe {
      let ptr = if !self.iterating {
        self.iterating = true;
        ical::icalcomponent_get_first_component(self.ptr, ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT)
      } else {
        ical::icalcomponent_get_next_component(self.ptr, ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT)
      };
      if ptr.is_null() {
        None
      } else {
        let comp = Icalcomponent::from_ptr(ptr);
        Some(comp)
      }
    }
  }
}
