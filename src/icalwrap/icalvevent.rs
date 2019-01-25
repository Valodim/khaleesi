use std::ffi::CStr;

use super::IcalComponent;
use super::IcalVCalendar;
use super::IcalTime;
use super::IcalTimeZone;
use super::IcalDuration;
use ical;

pub struct IcalVEvent {
  ptr: *mut ical::icalcomponent,
  parent: Option<IcalVCalendar>,
  instance_timestamp: Option<IcalTime>,
}

impl Drop for IcalVEvent {
  fn drop(&mut self) {
    unsafe {
      // println!("free");
      ical::icalcomponent_free(self.ptr);
    }
  }
}

impl IcalComponent for IcalVEvent {
  fn get_ptr (&self) -> *mut ical::icalcomponent {
    self.ptr
  }
  fn as_component(&self) -> &dyn IcalComponent {
    self
  }
}

impl IcalVEvent {
  pub fn from_ptr_with_parent(
      ptr: *mut ical::icalcomponent,
      parent: &IcalVCalendar,
      ) -> IcalVEvent {
    IcalVEvent {
      ptr,
      parent: Some(parent.shallow_copy()),
      instance_timestamp: None,
    }
  }

  pub fn get_dtend(&self) -> Option<IcalTime> {
    match self.instance_timestamp {
      Some(ref timestamp) => unsafe {
        let icalduration = ical::icalcomponent_get_duration(self.ptr);
        let dtend = ical::icaltime_add(**timestamp, icalduration);
        Some(IcalTime::from(dtend))
      },
      None =>
        unsafe {
          let dtend = ical::icalcomponent_get_dtend(self.ptr);
          trace!("{:?}", dtend);
          if ical::icaltime_is_null_time(dtend) == 1 {
            None
          } else {
            Some(IcalTime::from(dtend))
          }
        }
    }
  }

  pub fn get_duration(&self) -> Option<IcalDuration> {
    unsafe {
      let duration = ical::icalcomponent_get_duration(self.ptr);
      if ical::icaldurationtype_is_bad_duration(duration) == 0 {
        Some(IcalDuration::from(duration))
      } else {
        None
      }
    }
  }

  pub fn get_dtstart(&self) -> Option<IcalTime> {
    match self.instance_timestamp {
      Some(ref timestamp) => Some(timestamp.clone()),
      None => unsafe {
        let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
        if ical::icaltime_is_null_time(dtstart) == 1 {
          None
        } else {
          Some(IcalTime::from(dtstart))
        }
      }
    }
  }

  pub fn get_last_relevant_date(&self) -> Option<IcalTime> {
    if self.is_allday() {
      self.get_dtend().map(|dtend| dtend.pred())
    } else {
      self.get_dtend().map(|dtend| dtend)
    }
  }

  pub fn is_recur_master(&self) -> bool {
    self.is_recur() && self.instance_timestamp.is_none()
  }

  pub fn is_recur(&self) -> bool {
    !self.get_properties(ical::icalproperty_kind_ICAL_RRULE_PROPERTY).is_empty()
  }

  pub fn get_recur_datetimes(&self) -> Vec<IcalTime> {
    let mut result: Vec<IcalTime> = vec!();
    let result_ptr: *mut ::std::os::raw::c_void = &mut result as *mut _ as *mut ::std::os::raw::c_void;

    let dtstart = self.get_dtstart().unwrap();
    unsafe {
      let mut dtend = ical::icalcomponent_get_dtend(self.ptr);

      //unroll up to 1 year in the future
      dtend.year += 1;

      ical::icalcomponent_foreach_recurrence(self.ptr, *dtstart, dtend, Some(recur_callback), result_ptr);
    }

    if dtstart.is_date() {
      result = result.into_iter().map(|time| time.as_date()).collect();
    }

    result
  }

  pub fn is_recur_valid(&self) -> bool {
    if self.is_recur_master() {
      true
    } else if let Some(ref timestamp) = self.instance_timestamp {
      let recur_times = self.get_recur_datetimes();
      recur_times.contains(timestamp)
    } else {
      self.instance_timestamp.is_none()
    }
  }

  pub fn with_internal_timestamp(&self, datetime: &IcalTime) -> IcalVEvent {
    IcalVEvent {
      ptr: self.ptr,
      parent: self.parent.as_ref().map(|parent| parent.shallow_copy()),
      instance_timestamp: Some(datetime.clone()),
    }
  }

  pub fn get_recur_instances(&self) -> impl Iterator<Item = IcalVEvent> + '_ {
    self.get_recur_datetimes().into_iter()
      .map(|recur_utc| recur_utc.with_timezone(&IcalTimeZone::local()))
      .map(move |recur_local| self.with_internal_timestamp(&recur_local))
  }

  pub fn get_parent(&self) -> Option<&IcalVCalendar> {
    self.parent.as_ref()
  }

  pub fn get_summary(&self) -> Option<String> {
    unsafe {
      let ptr = ical::icalcomponent_get_summary(self.ptr);
      if !ptr.is_null() {
        Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
      } else {
        None
      }
    }
  }

  pub fn get_description(&self) -> Option<String> {
    unsafe {
      let ptr = ical::icalcomponent_get_description(self.ptr);
      if !ptr.is_null() {
        Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
      } else {
        None
      }
    }
  }

  pub fn get_location(&self) -> Option<String> {
    unsafe {
      let ptr = ical::icalcomponent_get_location(self.ptr);
      if !ptr.is_null() {
        Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
      } else {
        None
      }
    }
  }

  pub fn get_uid(&self) -> String {
    unsafe {
      let cstr = CStr::from_ptr(ical::icalcomponent_get_uid(self.ptr));
      cstr.to_string_lossy().into_owned()
    }
  }

  pub fn is_allday(&self) -> bool {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      dtstart.is_date == 1
    }
  }
}

extern "C" fn recur_callback(
                         _comp: *mut ical::icalcomponent,
                         span: *mut ical::icaltime_span,
                         data: *mut ::std::os::raw::c_void) {
  let data: &mut Vec<IcalTime> = unsafe { &mut *(data as *mut Vec<IcalTime>) };

  let spanstart = unsafe {
    let start = (*span).start;
    IcalTime::from_timestamp(start)
  };

  data.push(spanstart);
}

#[cfg(test)]
mod tests {
  use super::*;
  use testdata;
  use chrono::NaiveDate;

  #[test]
  fn recur_iterator_test() {
    testdata::setup();
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();
    assert_eq!(IcalTime::floating_ymd(2018, 10, 11), event.get_dtstart().unwrap());
    assert_eq!(IcalTime::floating_ymd(2018, 10, 13), event.get_dtend().unwrap());
    assert_eq!("RRULE:FREQ=WEEKLY;COUNT=10", event.get_property(ical::icalproperty_kind_ICAL_RRULE_PROPERTY).unwrap().as_ical_string());
    assert_eq!(10, event.get_recur_datetimes().len());
    assert_eq!(10, event.get_recur_instances().count());
  }

  #[test]
  fn test_get_all_properties() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();

    let event = cal.get_principal_event();
    let props = event.get_properties_all();
    assert_eq!(7, props.len());
  }

  #[test]
  fn test_get_property_get_value() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, None).unwrap();
    let event = cal.get_principal_event();
    let prop = event.get_properties_by_name("DTSTART");

    assert_eq!(1, prop.len());
    assert_eq!("DTSTART", prop[0].get_name());
    assert_eq!("20070628", prop[0].get_value());
    assert_eq!(NaiveDate::from_ymd_opt(2007,6,28), prop[0].get_value_as_date());
  }

  #[test]
  fn test_get_property_debug() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY_ALLDAY, None).unwrap();
    let event = cal.get_principal_event();
    let prop = event.get_property(ical::icalproperty_kind_ICAL_DTSTART_PROPERTY).unwrap();

    assert_eq!("DTSTART;VALUE=DATE:20070628", format!("{:?}", prop));
  }

  #[test]
  fn test_get_summary() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(Some("Festival International de Jazz de Montreal".to_string()), event.get_summary());
  }

  #[test]
  fn test_get_summary_none() {
    let cal = IcalVCalendar::from_str(testdata::TEST_NO_SUMMARY, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(None, event.get_summary());
  }

  #[test]
  fn test_get_duration() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(Some(IcalDuration::from_seconds(10*24*60*60 + 18*60*60)), event.get_duration());
  }


  #[test]
  fn test_get_description() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(Some("Discuss how we can test c&s interoperability\nusing iCalendar and other IETF standards.".to_string()), event.get_description());
  }

  #[test]
  fn test_get_description_none() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(None, event.get_description());
  }

  #[test]
  fn test_get_location() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(Some("LDB Lobby".to_string()), event.get_location());
  }


  #[test]
  fn test_get_location_none() {
    let cal = IcalVCalendar::from_str(testdata::TEST_NO_SUMMARY, None).unwrap();
    let event = cal.get_principal_event();

    assert_eq!(None, event.get_location());
  }

  #[test]
  fn test_is_recur_master() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    assert!(cal.get_principal_event().is_recur_master());
  }

  #[test]
  fn test_is_recur_master_instance() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();
    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2018, 01, 01));
    assert!(!event.is_recur_master());
  }

  #[test]
  fn test_is_recur_master_invalid() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    assert!(!cal.get_principal_event().is_recur_master());
  }

  #[test]
  fn recur_datetimes_test() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();

    let event = cal.get_principal_event();
    let mut recur_instances = event.get_recur_instances();
    let local = IcalTimeZone::local();
    assert_eq!(IcalTime::floating_ymd(2018, 10, 11).with_timezone(&local), recur_instances.next().unwrap().get_dtstart().unwrap());
    assert_eq!(IcalTime::floating_ymd(2018, 10, 18).with_timezone(&local), recur_instances.next().unwrap().get_dtstart().unwrap());
  }

  #[test]
  fn test_is_recur_valid_master() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_dtstart() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();

    let event = event.with_internal_timestamp(&event.get_dtstart().unwrap());

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_incorrect() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();

    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2010, 01, 01));

    assert!(!event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_other() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_event();

    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2018, 10, 25));

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_nonrecur() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_event();

    assert!(event.is_recur_valid());
  }

}
