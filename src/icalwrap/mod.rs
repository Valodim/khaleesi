mod icalvcalendar;
mod icalvevent;
mod icalproperty;
mod icalcomponent;
mod icaltime;
mod icaltimezone;

// libical does some weird, non-threadsafe things in timezone methods, notably
// icaltime_convert_to_zone (which is also called in icaltime_as_timet_with_zone)
// see these two (independent!) bugs:
// https://github.com/libical/libical/issues/86
// https://github.com/libical/libical/commit/0ebf2d9a7183be94991c2681c6e3f009c64cf7cc
use std::sync::Mutex;
lazy_static! {
  static ref TZ_MUTEX: Mutex<i32> = Mutex::new(0);
}

pub use self::icalvcalendar::IcalVCalendar;
pub use self::icalvcalendar::IcalEventIter;
pub use self::icalvevent::IcalVEvent;
pub use self::icalproperty::IcalProperty;
pub use self::icalcomponent::IcalComponent;
pub use self::icaltime::IcalTime;
pub use self::icaltimezone::IcalTimeZone;
