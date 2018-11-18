use chrono::{NaiveDate, NaiveDateTime};
use std::ffi::{CStr,CString};
use std::path::PathBuf;

use ical;

pub trait IcalComponent {
  fn get_ptr(&self) -> *mut ical::icalcomponent;
  fn as_component(&self) -> &dyn IcalComponent;

  fn get_first_event(&self) -> IcalVEvent {
    unsafe {
      let property = ical::icalcomponent_get_first_component(
        self.get_ptr(),
        ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT,
      );
      IcalVEvent::from_ptr_with_parent(property, self.as_component())
    }
  }

  fn get_property(&self) -> IcalProperty {
    unsafe {
      let property = ical::icalcomponent_get_first_property(self.get_ptr(), ical::icalproperty_kind_ICAL_DESCRIPTION_PROPERTY);
      IcalProperty::from_ptr(property, self.as_component())
    }
  }

  fn get_properties(self: &Self, property_kind: ical::icalproperty_kind) -> Vec<IcalProperty> {
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

  fn get_properties_all(&self) -> Vec<IcalProperty> {
    self.get_properties(ical::icalproperty_kind_ICAL_ANY_PROPERTY)
  }

  fn get_properties_by_name(&self, property_name: &str) -> Vec<IcalProperty> {
    let property_kind = unsafe {
      ical::icalproperty_string_to_kind(CString::new(property_name).unwrap().as_ptr())
    };
    self.get_properties(property_kind)
  }
}

pub struct IcalVCalendar {
  ptr: *mut ical::icalcomponent,
  path: Option<PathBuf>,
}

pub struct IcalVEvent<'a> {
  ptr: *mut ical::icalcomponent,
  _parent: &'a dyn IcalComponent,
}

pub struct IcalProperty<'a> {
  ptr: *mut ical::icalproperty,
  _parent: &'a dyn IcalComponent,
}

pub struct IcalEventIter<'a> {
  iter: ical::icalcompiter,
  parent: &'a IcalVCalendar,
}

impl Drop for IcalVCalendar {
  fn drop(&mut self) {
    unsafe {
      // println!("free");
      ical::icalcomponent_free(self.ptr);
    }
  }
}

impl<'a> Drop for IcalVEvent<'a> {
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

  pub fn get_value_as_date(&self) -> NaiveDate {
    unsafe {
      let foo = ical::icalproperty_get_value_as_string(self.ptr);
      let time = ical::icaltime_from_string(foo);
      NaiveDate::from_ymd(time.year, time.month as u32, time.day as u32)
    }
  }
}

impl IcalComponent for IcalVCalendar {
  fn get_ptr(&self) -> *mut ical::icalcomponent  {
    self.ptr
  }

  fn as_component(&self) -> &dyn IcalComponent {
    self
  }
}

impl<'a> IcalComponent for IcalVEvent<'a> {
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
      ptr: ptr,
      path: None,
    }
  }

  pub fn from_str(str: &str, path: Option<PathBuf>) -> Result<Self, String> {
    unsafe {
      let parsed_comp = ical::icalparser_parse_string(CString::new(str).unwrap().as_ptr());
      if !parsed_comp.is_null() {
        let mut comp = IcalVCalendar::from_ptr(parsed_comp);
        comp.path = path;
        Ok(comp)
      } else {
        Err("could not read component".to_string())
      }
    }
  }

//this needs to create IcalVEvents
  //pub fn get_inner(&self) -> Self {
  //  unsafe {
  //    let inner_comp = ical::icalcomponent_get_inner(self.ptr);
  //    Icalcomponent::from_ptr_with_parent(inner_comp, self.parent)
  //  }
  //}

//research
  pub fn get_uid(&self) -> String {
    unsafe {
      let foo = CStr::from_ptr(ical::icalcomponent_get_uid(self.ptr));
      foo.to_string_lossy().into_owned()
    }
  }

  pub fn get_path_as_string(&self) -> String {
    format!("{}", self.path.as_ref().unwrap().display())
  }

  pub fn events_iter(&self) -> IcalEventIter {
    IcalEventIter::from_vcalendar(self)
  } 
}

impl<'a> IcalVEvent<'a> {
  fn from_ptr_with_parent<'b>(
    ptr: *mut ical::icalcomponent,
    parent: &'b dyn IcalComponent,
  ) -> IcalVEvent<'b> {
    IcalVEvent {
      ptr,
      _parent: parent,
    }
  }

  pub fn get_dtstart_unix(&self) -> i64 {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      ical::icaltime_as_timet(dtstart)
    }
  }

  pub fn get_dtend(&self) -> NaiveDateTime {
    unsafe {
      let dtend = ical::icalcomponent_get_dtend(self.ptr);
      NaiveDate::from_ymd(dtend.year, dtend.month as u32, dtend.day as u32)
        .and_hms(dtend.hour as u32, dtend.minute as u32, dtend.second as u32)
    }
  }

  pub fn get_dtstart(&self) -> NaiveDateTime {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      NaiveDate::from_ymd(dtstart.year, dtstart.month as u32, dtstart.day as u32)
        .and_hms(dtstart.hour as u32, dtstart.minute as u32, dtstart.second as u32)
    }
  }

  pub fn get_dtstart_date(&self) -> NaiveDate {
    unsafe {
      let dtstart = ical::icalcomponent_get_dtstart(self.ptr);
      NaiveDate::from_ymd(dtstart.year, dtstart.month as u32, dtstart.day as u32)
    }
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
}

impl<'a> IcalEventIter<'a> {
  fn from_vcalendar(comp: &'a IcalVCalendar) -> Self {
    use ical::icalcomponent_kind_ICAL_VEVENT_COMPONENT;
    let vevent_kind = icalcomponent_kind_ICAL_VEVENT_COMPONENT;
    let iter = unsafe {
      ical::icalcomponent_begin_component(comp.get_ptr(), vevent_kind)
    };
    IcalEventIter{iter, parent: &comp}
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

impl <'a> Iterator for IcalEventIter<'a> {
  type Item = IcalVEvent<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      let ptr = ical::icalcompiter_deref(&mut self.iter);
      if ptr.is_null() {
        None
      } else {
        ical::icalcompiter_next(&mut self.iter);
        //let comp = Icalcomponent::from_ptr_with_parent(ptr, self.parent);
        let vevent = IcalVEvent::from_ptr_with_parent(ptr, self.parent);
        Some(vevent)
      }
    }
  }
}

#[test]
fn iterator_element_count() {
  use testdata;
  let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_MULTIDAY, None).unwrap();
  assert_eq!(cal.events_iter().count(), 1)
}
