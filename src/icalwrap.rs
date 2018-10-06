use chrono::{NaiveDate};
use libc::{c_char, c_int, c_void, time_t};
use std::ffi::CStr;
extern crate libc;

pub struct Icalcomponent {
  iterating: bool,
  pub ptr: *const c_void,
}

impl Drop for Icalcomponent {
  fn drop(&mut self) {
    unsafe {
      icalcomponent_free(self.ptr);
    }
  }
}

impl Icalcomponent {
  fn from_ptr(ptr: *const c_void) -> Icalcomponent {
    Icalcomponent {
      ptr: ptr,
      iterating: false, }
  }

  pub fn get_dtstart_unix(self: &Icalcomponent) -> i64 {
    unsafe {
      let dtstart = icalcomponent_get_dtstart(self.ptr);
      icaltime_as_timet(dtstart)
    }
  }

  pub fn get_dtend(self: &Icalcomponent) -> NaiveDate {
    unsafe {
      let dtend = icalcomponent_get_dtend(self.ptr);
      NaiveDate::from_ymd(dtend.year, dtend.month as u32, dtend.day as u32)
    }
  }

  pub fn get_dtstart(self: &Icalcomponent) -> NaiveDate {
    unsafe {
      let dtstart = icalcomponent_get_dtstart(self.ptr);
      NaiveDate::from_ymd(dtstart.year, dtstart.month as u32, dtstart.day as u32)
    }
  }

  pub fn get_uid(self: &Icalcomponent) -> String {
    unsafe {
      let foo = CStr::from_ptr(icalcomponent_get_uid(self.ptr));
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
        icalcomponent_get_first_component(self.ptr, ICAL_VEVENT_COMPONENT)
      } else {
        icalcomponent_get_next_component(self.ptr, ICAL_VEVENT_COMPONENT)
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

pub fn parse_component(str: &String) -> Icalcomponent {
  unsafe {
    let parsed_event = icalparser_parse_string(str.as_ptr());
    Icalcomponent::from_ptr(parsed_event)
  }
}

#[repr(C)]
pub struct icaltimetype {
  // Actual year, e.g. 2001.
  pub year: c_int,
  // 1 (Jan) to 12 (Dec).
  pub month: c_int,
  pub day: c_int,
  pub hour: c_int,
  pub minute: c_int,
  pub second: c_int,
  //< 1 -> interpret this as date.
  pub is_date: c_int,
  //< 1 -> time is in daylight savings time.
  pub is_daylight: c_int,
  //< timezone
  pub zone: *const c_void,
}

pub type IcalcomponentKind = u32;

pub const ICAL_NO_COMPONENT: IcalcomponentKind = 0;
pub const ICAL_ANY_COMPONENT: IcalcomponentKind = 1;
pub const ICAL_XROOT_COMPONENT: IcalcomponentKind = 2;
pub const ICAL_XATTACH_COMPONENT: IcalcomponentKind = 3;
pub const ICAL_VEVENT_COMPONENT: IcalcomponentKind = 4;
pub const ICAL_VTODO_COMPONENT: IcalcomponentKind = 5;
pub const ICAL_VJOURNAL_COMPONENT: IcalcomponentKind = 6;
pub const ICAL_VCALENDAR_COMPONENT: IcalcomponentKind = 7;
pub const ICAL_VAGENDA_COMPONENT: IcalcomponentKind = 8;
pub const ICAL_VFREEBUSY_COMPONENT: IcalcomponentKind = 9;
pub const ICAL_VALARM_COMPONENT: IcalcomponentKind = 10;
pub const ICAL_XAUDIOALARM_COMPONENT: IcalcomponentKind = 11;
pub const ICAL_XDISPLAYALARM_COMPONENT: IcalcomponentKind = 12;
pub const ICAL_XEMAILALARM_COMPONENT: IcalcomponentKind = 13;
pub const ICAL_XPROCEDUREALARM_COMPONENT: IcalcomponentKind = 14;
pub const ICAL_VTIMEZONE_COMPONENT: IcalcomponentKind = 15;
pub const ICAL_XSTANDARD_COMPONENT: IcalcomponentKind = 16;
pub const ICAL_XDAYLIGHT_COMPONENT: IcalcomponentKind = 17;
pub const ICAL_X_COMPONENT: IcalcomponentKind = 18;
pub const ICAL_VSCHEDULE_COMPONENT: IcalcomponentKind = 19;
pub const ICAL_VQUERY_COMPONENT: IcalcomponentKind = 20;
pub const ICAL_VREPLY_COMPONENT: IcalcomponentKind = 21;
pub const ICAL_VCAR_COMPONENT: IcalcomponentKind = 22;
pub const ICAL_VCOMMAND_COMPONENT: IcalcomponentKind = 23;
pub const ICAL_XLICINVALID_COMPONENT: IcalcomponentKind = 24;
pub const ICAL_XLICMIMEPART_COMPONENT: IcalcomponentKind = 25;
pub const ICAL_VAVAILABILITY_COMPONENT: IcalcomponentKind = 26;
pub const ICAL_XAVAILABLE_COMPONENT: IcalcomponentKind = 27;
pub const ICAL_VPOLL_COMPONENT: IcalcomponentKind = 28;
pub const ICAL_VVOTER_COMPONENT: IcalcomponentKind = 29;
pub const ICAL_XVOTE_COMPONENT: IcalcomponentKind = 30;
pub const ICAL_VPATCH_COMPONENT: IcalcomponentKind = 31;
pub const ICAL_XPATCH_COMPONENT: IcalcomponentKind = 32;

#[link(name = "ical")]
extern "C" {
  //  icalcomponent* icalparser_parse_string  (  const char *   str )
  pub fn icalparser_parse_string(str: *const u8) -> *const c_void;

  pub fn icalcomponent_free(component: *const c_void);

  // struct icaltimetype icalcomponent_get_dtstart(icalcomponent *comp);
  pub fn icalcomponent_get_dtstart(icalcomponent: *const c_void) -> icaltimetype;
  pub fn icalcomponent_get_dtend(icalcomponent: *const c_void) -> icaltimetype;

  // const char *icalcomponent_get_uid(icalcomponent *comp);
  pub fn icalcomponent_get_uid(comp: *const c_void) -> *const c_char;

  // icalcomponent *icalcomponent_get_first_component(icalcomponent *component, IcalcomponentKind kind);
  pub fn icalcomponent_get_first_component(
    comp: *const c_void,
    kind: IcalcomponentKind,
  ) -> *const c_void;

  // icalcomponent *icalcomponent_get_next_component(icalcomponent *component, IcalcomponentKind kind);
  pub fn icalcomponent_get_next_component(
    comp: *const c_void,
    kind: IcalcomponentKind,
  ) -> *const c_void;

  // time_t icaltime_as_timet  (  const struct icaltimetype    )
  pub fn icaltime_as_timet(foo: icaltimetype) -> time_t;
}

