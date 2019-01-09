use std::ffi::CString;

use super::IcalProperty;
use ical;

pub trait IcalComponent {
  fn get_ptr(&self) -> *mut ical::icalcomponent;
  fn as_component(&self) -> &dyn IcalComponent;

  fn get_property(&self, property_kind: ical::icalproperty_kind) -> Option<IcalProperty<'_>> {
    let property  = unsafe {
      ical::icalcomponent_get_first_property(self.get_ptr(), property_kind)
    };
    if !property.is_null() {
      Some(IcalProperty::from_ptr(property, self.as_component()))
    } else {
      None
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
      let c_str = CString::new(property_name).unwrap();
      ical::icalproperty_string_to_kind(c_str.as_ptr())
    };
    self.get_properties(property_kind)
  }

  fn get_property_by_name(&self, property_name: &str) -> Option<IcalProperty> {
    let property_kind = unsafe {
      let c_str = CString::new(property_name).unwrap();
      ical::icalproperty_string_to_kind(c_str.as_ptr())
    };
    self.get_property(property_kind)
  }
}

