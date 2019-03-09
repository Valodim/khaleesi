use crate::icalwrap::IcalTime;

pub trait KhEvent {
  fn get_start(&self) -> Option<IcalTime>;
  fn get_end(&self) -> Option<IcalTime>;
  fn is_recur_master(&self) -> bool;
}


