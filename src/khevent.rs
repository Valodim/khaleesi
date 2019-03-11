use std::path::PathBuf;

use crate::icalwrap::IcalTime;
use crate::icalwrap::IcalProperty;
use crate::icalwrap::IcalTimeZone;
use crate::icalwrap::IcalDuration;
use crate::icalwrap::IcalVEvent;
use crate::icalwrap::IcalComponent;

pub struct KhEvent {
  //TODO event should be private
  event: IcalVEvent,
  instance_timestamp: Option<IcalTime>,
}

impl KhEvent {
  pub fn get_start(&self) -> Option<IcalTime> {
    //TODO: should probably depend on is_recur_master, not the instance timestamp
    match self.instance_timestamp {
      Some(ref timestamp) => Some(timestamp.clone()),
      None => {
        self.event.get_dtstart()
      }
    }
  }

  pub fn get_end(&self) -> Option<IcalTime> {
    //TODO: should probably depend on is_recur_master, not the instance timestamp
    match self.instance_timestamp {
      Some(ref timestamp) => {
        let dur = self.get_duration().unwrap();
        let dtend = timestamp.to_owned() + dur;
        Some(dtend)
      },
      None => self.event.get_dtend()
    }
  }

  pub fn with_internal_timestamp(&self, timestamp: &IcalTime) -> Self {
    Self {
      event: self.event.shallow_copy(),
      instance_timestamp: Some(timestamp.clone())
    }
  }

  pub fn get_calendar_name(&self) -> Option<String> {
    self.event.get_parent().and_then(|cal| cal.get_calendar_name())
  }

  pub fn get_path(&self) -> Option<&PathBuf> {
    self.event.get_parent()?.get_path()
  }

  pub fn is_allday(&self) -> bool {
    self.event.is_allday()
  }

  pub fn get_duration(&self) -> Option<IcalDuration> {
    self.event.get_duration()
  }

  pub fn get_summary(&self) -> Option<String> {
    self.event.get_summary()
  }

  pub fn get_description(&self) -> Option<String> {
    self.event.get_description()
  }

  pub fn get_location(&self) -> Option<String> {
    self.event.get_location()
  }

  pub fn get_uid(&self) -> String {
    self.event.get_uid()
  }

  pub fn get_dtstamp(&self) -> Option<String> {
    let dtstamp_kind = ical::icalproperty_kind_ICAL_DTSTAMP_PROPERTY;
    self.event.get_property(dtstamp_kind).map(|prop| prop.get_value())
  }

  pub fn get_last_modified(&self) -> Option<String> {
    let last_modified_kind = ical::icalproperty_kind_ICAL_LASTMODIFIED_PROPERTY;
    self.event.get_property(last_modified_kind).map(|prop| prop.get_value())
  }

  pub fn get_last_relevant_date(&self) -> Option<IcalTime> {
    //TODO this is still wrong
    //events can end at 00:00
    if self.is_allday() {
      self.get_end().map(|dtend| dtend.pred())
    } else {
      self.get_end().map(|dtend| dtend)
    }
  }

  pub fn is_recur_master(&self) -> bool {
    self.event.has_property_rrule() && self.instance_timestamp.is_none()
  }

  pub fn is_recur_valid(&self) -> bool {
    if self.is_recur_master() {
      true
    } else if let Some(ref timestamp) = self.instance_timestamp {
      let recur_times = self.event.get_recur_datetimes();
      recur_times.contains(timestamp)
    } else {
      self.instance_timestamp.is_none()
    }
  }

  pub fn get_recur_instances(&self) -> impl Iterator<Item = KhEvent> + '_ {
    self.get_recur_datetimes().into_iter()
      .map(|recur_utc| recur_utc.with_timezone(&IcalTimeZone::local()))
      .map(move |recur_local| self.with_internal_timestamp(&recur_local))
  }

  pub fn get_recur_datetimes(&self) -> Vec<IcalTime> {
    self.event.get_recur_datetimes()
  }

  pub fn get_properties_by_name(&self, property_name: &str) -> Vec<IcalProperty> {
    self.event.get_properties_by_name(property_name)
  }

  pub fn from_event(event: IcalVEvent) -> Self {
    Self {
      event,
      instance_timestamp: None,
    }
  }

  pub fn from_event_with_timestamp(event: IcalVEvent, instance_timestamp: Option<IcalTime>) -> Self {
    Self {
      event,
      instance_timestamp,
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testdata;
  use crate::icalwrap::IcalVCalendar;
  use crate::icalwrap::IcalTimeZone;

  #[test]
  fn test_is_recur_valid_master() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_khevent();

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_dtstart() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_khevent();
    let start = &event.get_start().unwrap();

    let event = event.with_internal_timestamp(start);

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_incorrect() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_khevent();

    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2010, 01, 01));

    assert!(!event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_other() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_khevent();

    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2018, 10, 25));

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_valid_nonrecur() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    let event = cal.get_principal_khevent();

    assert!(event.is_recur_valid());
  }

  #[test]
  fn test_is_recur_master_instance() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    let event = cal.get_principal_khevent();
    let event = event.with_internal_timestamp(&IcalTime::floating_ymd(2018, 01, 01));
    assert!(!event.is_recur_master());
  }

  #[test]
  fn test_is_recur_master() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();
    assert!(cal.get_principal_khevent().is_recur_master());
  }

  #[test]
  fn test_is_recur_master_invalid() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_ONE_MEETING, None).unwrap();
    assert!(!cal.get_principal_khevent().is_recur_master());
  }

  #[test]
  fn recur_datetimes_test() {
    let cal = IcalVCalendar::from_str(testdata::TEST_EVENT_RECUR, None).unwrap();

    let event = cal.get_principal_khevent();
    let mut recur_instances = event.get_recur_instances();
    let local = IcalTimeZone::local();
    assert_eq!(IcalTime::floating_ymd(2018, 10, 11).with_timezone(&local), recur_instances.next().unwrap().get_start().unwrap());
    assert_eq!(IcalTime::floating_ymd(2018, 10, 18).with_timezone(&local), recur_instances.next().unwrap().get_start().unwrap());
  }

}
