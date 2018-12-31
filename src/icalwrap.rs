use chrono::{NaiveDate, Duration, DateTime, Date, Utc, TimeZone, Local};
use std::ffi::{CStr,CString};
use std::fmt;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

use ical;

pub trait IcalComponent {
  fn get_ptr(&self) -> *mut ical::icalcomponent;
  fn as_component(&self) -> &dyn IcalComponent;

  fn get_property(&self, property_kind: ical::icalproperty_kind) -> IcalProperty<'_> {
    unsafe {
      let property = ical::icalcomponent_get_first_property(self.get_ptr(), property_kind);
      IcalProperty::from_ptr(property, self.as_component())
    }
  }

  fn get_properties(self: &Self, property_kind: ical::icalproperty_kind) -> Vec<IcalProperty<'_>> {
    let mut properties = Vec::new();
    unsafe {
      let mut property_ptr = ical::icalcomponent_get_first_property(self.get_ptr(), property_kind);
      while !property_ptr.is_null() {
        let property = IcalProperty::from_ptr(property_ptr, self.as_component());
        properties.push(property);
        property_ptr = ical::icalcomponent_get_next_property(self.get_ptr(), property_kind);
      }
    }
    properties
  }

  fn get_properties_all(&self) -> Vec<IcalProperty<'_>> {
    self.get_properties(ical::icalproperty_kind_ICAL_ANY_PROPERTY)
  }

  fn get_properties_by_name(&self, property_name: &str) -> Vec<IcalProperty> {
    let property_kind = unsafe {
      ical::icalproperty_string_to_kind(CString::new(property_name).unwrap().as_ptr())
    };
    self.get_properties(property_kind)
  }
}

struct IcalComponentOwner {
  ptr: *mut ical::icalcomponent
}

impl Deref for IcalComponentOwner {
  type Target = *mut ical::icalcomponent;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}

#[derive(Clone)]
pub struct IcalVCalendar {
  comp: Rc<IcalComponentOwner>,
  path: Option<PathBuf>,
  instance_timestamp: Option<DateTime<Utc>>,
}

pub struct IcalVEvent {
  ptr: *mut ical::icalcomponent,
  parent: Option<IcalVCalendar>,
  instance_timestamp: Option<DateTime<Utc>>,
}

pub struct IcalProperty<'a> {
  ptr: *mut ical::icalproperty,
  _parent: &'a dyn IcalComponent,
}

pub struct IcalEventIter<'a> {
  iter: ical::icalcompiter,
  parent: &'a IcalVCalendar,
}

impl Drop for IcalComponentOwner {
  fn drop(&mut self) {
    unsafe {
      // println!("free");
      ical::icalcomponent_free(self.ptr);
    }
  }
}

impl Drop for IcalVEvent {
  fn drop(&mut self) {
    unsafe {
      // println!("free");
      ical::icalcomponent_free(self.ptr);
    }
  }
}

impl<'a> Drop for IcalProperty<'a> {
  fn drop(&mut self) {
    unsafe {
      ical::icalproperty_free(self.ptr);
    }
  }
}

impl<'a> IcalProperty<'a> {
  fn from_ptr(ptr: *mut ical::icalproperty, parent: &'a dyn IcalComponent) -> Self {
    IcalProperty { ptr, _parent: parent }
  }

  pub fn get_name(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalproperty_get_property_name(self.ptr));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn get_value(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalproperty_get_value_as_string(self.ptr));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn as_ical_string(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalproperty_as_ical_string(self.ptr));
      foo.to_string_lossy().trim().to_owned()
    }
  }

  pub fn get_value_as_date(&self) -> Option<NaiveDate> {
    unsafe {
      let date = ical::icaltime_from_string(ical::icalproperty_get_value_as_string(self.ptr));
      NaiveDate::from_ymd_opt(date.year, date.month as u32, date.day as u32)
    }
  }
}

impl<'a> fmt::Debug for IcalProperty<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_ical_string())
  }
}

impl IcalComponent for IcalVCalendar {
  fn get_ptr(&self) -> *mut ical::icalcomponent  {
    self.comp.ptr
  }

  fn as_component(&self) -> &dyn IcalComponent {
    self
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

impl IcalVCalendar {
  fn from_ptr(ptr: *mut ical::icalcomponent) -> Self {
    IcalVCalendar {
      comp: Rc::new(IcalComponentOwner { ptr }),
      path: None,
      instance_timestamp: None,
    }
  }

  pub fn with_internal_timestamp(mut self, datetime: DateTime<Utc>) -> IcalVCalendar {
    self.instance_timestamp = Some(datetime);
    self
  }

  pub fn from_str(str: &str, path: Option<PathBuf>) -> Result<Self, String> {
    unsafe {
      let parsed_cal = ical::icalparser_parse_string(CString::new(str).unwrap().as_ptr());
      if !parsed_cal.is_null() {
        let kind = ical::icalcomponent_isa(parsed_cal);
        if kind == ical::icalcomponent_kind_ICAL_VCALENDAR_COMPONENT {
          let mut cal = IcalVCalendar::from_ptr(parsed_cal);
          cal.path = path;
          Ok(cal)
        } else {
          let kind = CStr::from_ptr(ical::icalcomponent_kind_to_string(kind)).to_string_lossy();
          Err(format!("expected VCALENDAR component, got {}", kind))
        }
      } else {
        Err("could not read component".to_string())
      }
    }
  }

  pub fn to_string(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalcomponent_as_ical_string(self.get_ptr()));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn get_uid(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalcomponent_get_uid(self.get_principal_event().get_ptr()));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn with_uid(mut self, uid: &str) -> Self {
    {
      let events = self.events_iter();
      let uid_cstr = CString::new(uid).unwrap();
      for event in events {
        unsafe {
          ical::icalcomponent_set_uid(event.get_ptr(), uid_cstr.as_ptr());
        }
      }
    }
    self.path = self.path.map(|path| path.with_file_name(uid));
    return self;
  }

  pub fn get_path_as_string(&self) -> String {
    format!("{}", self.path.as_ref().unwrap().display())
  }

  pub fn get_path(&self) -> Option<&PathBuf> {
    self.path.as_ref()
  }

  pub fn get_calendar_name(&self) -> Option<String> {
      let calendar_name = self.path.as_ref()?.parent()?.file_name()?;
      Some(calendar_name.to_string_lossy().into_owned())
  }

  pub fn events_iter(&self) -> IcalEventIter {
    IcalEventIter::from_vcalendar(self)
  }

  pub fn get_first_event(&self) -> IcalVEvent {
    unsafe {
      let event = ical::icalcomponent_get_first_component(
        self.get_ptr(),
        ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT,
      );
      if self.events_iter().unique_uid_count() > 1 {
        warn!("More than one event in file: {}", self.get_path_as_string())
      }
      IcalVEvent::from_ptr_with_parent(event, self)
    }
  }

  pub fn get_principal_event(&self) -> IcalVEvent {
    let mut event = self.get_first_event();
    if let Some(timestamp) = self.instance_timestamp {
      event = event.with_internal_timestamp(timestamp)
    }
    event
  }

}

impl IcalVEvent {
  fn from_ptr_with_parent(
      ptr: *mut ical::icalcomponent,
      parent: &IcalVCalendar,
      ) -> IcalVEvent {
    IcalVEvent {
      ptr,
      parent: Some(parent.clone()),
      instance_timestamp: None,
    }
  }

  pub fn get_dtend_unix(&self) -> Option<i64> {
    match self.instance_timestamp {
      Some(timestamp) => unsafe {
        let icalduration = ical::icalcomponent_get_duration(self.ptr);
        let duration = Duration::seconds(ical::icaldurationtype_as_int(icalduration) as i64);
        Some(timestamp.checked_add_signed(duration)?.timestamp())
      },
      None =>
        unsafe {
          let dtend = ical::icalcomponent_get_dtend(self.ptr);
          trace!("{:?}", dtend);
          if ical::icaltime_is_null_time(dtend) == 1 {
            None
          } else {
            Some(ical::icaltime_as_timet_with_zone(dtend, dtend.zone))
          }
        }
    }
  }

  pub fn get_dtstart_unix(&self) -> Option<i64> {
    match self.instance_timestamp {
      Some(timestamp) => Some(timestamp.timestamp()),
      None => unsafe {
        let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
        if ical::icaltime_is_null_time(dtstart) == 1 {
          None
        } else {
          Some(ical::icaltime_as_timet_with_zone(dtstart, dtstart.zone))
        }
      }
    }
  }

  pub fn get_dtend(&self) -> Option<DateTime<Local>> {
    let dtend = self.get_dtend_unix()?;
    Some(Utc.timestamp(dtend, 0).with_timezone(&Local))
  }

  pub fn get_dtstart(&self) -> Option<DateTime<Local>> {
    let dtstart = self.get_dtstart_unix()?;
    Some(Utc.timestamp(dtstart, 0).with_timezone(&Local))
  }

  pub fn get_dtstart_date(&self) -> Option<Date<Local>> {
    Some(self.get_dtstart()?.date())
  }

  pub fn get_dtend_date(&self) -> Option<Date<Local>> {
    Some(self.get_dtend()?.date())
  }

  pub fn has_recur(&self) -> bool {
    !self.get_properties(ical::icalproperty_kind_ICAL_RRULE_PROPERTY).is_empty()
    & self.instance_timestamp.is_none()
  }

  pub fn get_recur_datetimes(&self) -> Vec<DateTime<Utc>> {
    let mut result = vec!();
    let result_ptr: *mut ::std::os::raw::c_void = &mut result as *mut _ as *mut ::std::os::raw::c_void;

    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      let mut dtend = ical::icalcomponent_get_dtend(self.ptr);

      //unroll up to 1 year in the future
      dtend.year = dtend.year + 1;

      ical::icalcomponent_foreach_recurrence(self.ptr, dtstart, dtend, Some(recur_callback), result_ptr);
    }

    result
  }

  fn with_internal_timestamp(&self, datetime: DateTime<Utc>) -> IcalVEvent {
    IcalVEvent {
      ptr: self.ptr,
      parent: self.parent.clone(),
      instance_timestamp: Some(datetime),
    }
  }

  pub fn get_recur_instances(&self) -> impl Iterator<Item = IcalVEvent> + '_ {
    self.get_recur_datetimes().into_iter().map(move |rec| self.with_internal_timestamp(rec))
  }

  pub fn get_parent(&self) -> Option<&IcalVCalendar> {
    self.parent.as_ref()
  }

  pub fn get_khaleesi_line(&self) -> Option<String> {
    let dtstart = self.get_dtstart()?.timestamp();
    let dtstart_string = format!("{:010}", dtstart);
    let path_string = self.parent.as_ref()?.get_path_as_string();
    Some([dtstart_string, path_string].join(" "))
  }

  pub fn get_summary(&self) -> Option<String> {
    unsafe {
      let ptr = ical::icalcomponent_get_summary(self.ptr);
      if ! ptr.is_null() {
          Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
      } else {
          None
      }
    }
  }

  pub fn get_description(&self) -> Option<String> {
    unsafe {
      let ptr = ical::icalcomponent_get_description(self.ptr);
      if ! ptr.is_null() {
          Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
      } else {
          None
      }
    }
  }

  pub fn get_uid(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalcomponent_get_uid(self.ptr));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn is_allday(&self) -> bool {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
//      let dtend = ical::icalcomponent_get_dtend(self.ptr);
      dtstart.is_date == 1
    }
  }
}

impl<'a> IcalEventIter<'a> {
  fn from_vcalendar(cal: &'a IcalVCalendar) -> Self {
    let vevent_kind = ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT;
    let iter = unsafe {
      ical::icalcomponent_begin_component(cal.get_ptr(), vevent_kind)
    };
    IcalEventIter{iter, parent: &cal}
  }

  fn unique_uid_count(self) -> usize {
    let mut uids = self.map(|event| {
      event.get_uid()
    }).collect::<Vec<String>>();
    uids.sort_unstable();
    uids.dedup();
    uids.len()
  }
}

//impl<'a> IntoIterator for &'a IcalComponent {
//  type Item = IcalComponent;
//  type IntoIter = IcalCompIter<'a>;
//
//  fn into_iter(self) -> Self::IntoIter {
//    IcalCompIter::from_comp(&self, ical::icalcomponent_kind_ICAL_ANY_COMPONENT)
//  }
//}

extern "C" fn recur_callback(
                         _comp: *mut ical::icalcomponent,
                         span: *mut ical::icaltime_span,
                         data: *mut ::std::os::raw::c_void) {
  let data: &mut Vec<DateTime<Utc>> = unsafe { &mut *(data as *mut Vec<DateTime<Utc>>) };

  let spanstart = unsafe {
    trace!("callback!, {:?}", *span);
    let start = (*span).start;
    Utc.timestamp(start, 0)
  };

  data.push(spanstart);

  ()
}

impl <'a> Iterator for IcalEventIter<'a> {
  type Item = IcalVEvent;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      let ptr = ical::icalcompiter_deref(&mut self.iter);
      if ptr.is_null() {
        None
      } else {
        ical::icalcompiter_next(&mut self.iter);
        let vevent = IcalVEvent::from_ptr_with_parent(ptr, self.parent);
        Some(vevent)
      }
    }
  }
}

#[test]
fn event_iterator_element_count() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
  assert_eq!(cal.events_iter().count(), 1)
}

#[test]
fn event_iterator_element_count_with_other() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_WITH_TIMEZONE_COMPONENT, None).unwrap();
  assert_eq!(cal.events_iter().count(), 1)
}

#[test]
fn load_serialize() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
  let back = unsafe {
    let ical_str = ical::icalcomponent_as_ical_string(cal.get_ptr());
    CStr::from_ptr(ical_str).to_string_lossy().into_owned()
  }.replace("\r\n", "\n");
  assert_eq!(back.trim(), testdata::TEST_EVENT_MULTIDAY)
}

#[test]
fn recur_iterator_test() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
  let event = cal.get_principal_event();
  assert_eq!(format!("{}", event.get_dtstart_date().unwrap().format("%Y%m%d")), "20181011");
  assert_eq!(format!("{}", event.get_dtend_date().unwrap().format("%Y%m%d")), "20181013");
  assert_eq!(event.get_property(ical::icalproperty_kind_ICAL_RRULE_PROPERTY).as_ical_string(), "RRULE:FREQ=WEEKLY;COUNT=10");
  assert_eq!(event.get_recur_datetimes().len(), 10);
  assert_eq!(event.get_recur_instances().count(), 10);
}

#[test]
fn get_khaleesi_line_test() {
  use testdata;
  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, path).unwrap();
  let event = cal.get_principal_event();
  assert_eq!(event.get_khaleesi_line().unwrap(), String::from("1182988800 test/path"))
}

#[test]
fn has_recur_test() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
  assert!(cal.get_principal_event().has_recur());
}

#[test]
fn recur_datetimes_test() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();

  let event = cal.get_principal_event();
  let mut recur_instances = event.get_recur_instances();
//TODO also check times
  assert_eq!(format!("{}", recur_instances.next().unwrap().get_dtstart().unwrap().format("%Y%m%d")), "20181011");
  assert_eq!(format!("{}", recur_instances.next().unwrap().get_dtstart().unwrap().format("%Y%m%d")), "20181018");
}

#[test]
fn with_uid_test() {
  use testdata;
  let path = Some(PathBuf::from("test/path"));
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, path.clone()).unwrap();

  let uid = "my_new_uid";
  let new_cal = cal.with_uid(uid);

  for event in new_cal.events_iter() {
    assert_eq!(uid, event.get_uid());
  }
  assert_eq!(Some(path.unwrap().with_file_name(uid)), new_cal.path);
}
