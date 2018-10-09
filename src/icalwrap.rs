use chrono::{NaiveDate, NaiveDateTime};
use std::ffi::{CStr,CString};
use std::ptr;

use ical;

pub struct Icalcomponent<'a> {
  ptr: *mut ical::icalcomponent,
  iterating: bool,
  parent: &'a *const ical::icalcomponent,
}

pub struct IcalProperty<'a> {
  ptr: *mut ical::icalproperty,
  parent: &'a Icalcomponent<'a>,
}

impl<'a> Drop for Icalcomponent<'a> {
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
  fn from_ptr(ptr: *mut ical::icalproperty, parent: &'a Icalcomponent) -> Self {
    IcalProperty { ptr, parent }
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

impl<'a> Icalcomponent<'a> {
  fn from_ptr<'x>(ptr: *mut ical::icalcomponent) -> Self {
    Icalcomponent {
      ptr: ptr,
      parent: &ptr::null(),
      iterating: false,
    }
  }

  fn from_ptr_with_parent(
    ptr: *mut ical::icalcomponent,
    parent: &'a *const ical::icalcomponent,
  ) -> Icalcomponent<'a> {
    Icalcomponent {
      ptr,
      parent,
      iterating: false,
    }
  }

  pub fn from_str(str: &str) -> Self {
    unsafe {
      let parsed_comp = ical::icalparser_parse_string(CString::new(str).unwrap().as_ptr());
      Icalcomponent::from_ptr(parsed_comp)
    }
  }

  pub fn get_inner(&self) -> Self {
    unsafe {
      let inner_comp = ical::icalcomponent_get_inner(self.ptr);
      Icalcomponent::from_ptr_with_parent(inner_comp, self.parent)
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

  fn get_properties(&self, property_kind: ical::icalproperty_kind) -> Vec<IcalProperty> {
    let mut properties = Vec::new();
    unsafe {
      let mut property_ptr = ical::icalcomponent_get_first_property(self.ptr, property_kind);
      while !property_ptr.is_null() {
        let property = IcalProperty::from_ptr(property_ptr, &self);
        properties.push(property);
        property_ptr = ical::icalcomponent_get_next_property(self.ptr, property_kind);
      }
    }
    properties
  }

  pub fn get_properties_all(&self) -> Vec<IcalProperty> {
    self.get_properties(ical::icalproperty_kind_ICAL_ANY_PROPERTY)
  }

  pub fn get_properties_by_name(&self, property_name: &str) -> Vec<IcalProperty> {
    let property_kind = unsafe {
      ical::icalproperty_string_to_kind(CString::new(property_name).unwrap().as_ptr())
    };
    self.get_properties(property_kind)
  }

  pub fn get_property(&self) -> IcalProperty {
    unsafe {
      let property = ical::icalcomponent_get_first_property(self.ptr, ical::icalproperty_kind_ICAL_DESCRIPTION_PROPERTY);
      IcalProperty::from_ptr(property, &self)
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

impl <'a> Iterator for Icalcomponent<'a> {
  type Item = Icalcomponent<'a>;

  fn next(&mut self) -> Option<Icalcomponent<'a>> {
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
        let comp = Icalcomponent::from_ptr_with_parent(ptr, self.parent);
        Some(comp)
      }
    }
  }
}
