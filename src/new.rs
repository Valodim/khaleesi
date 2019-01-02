use utils;
use icalwrap::IcalVCalendar;
use defaults;

pub fn do_new(_lines: &mut Iterator<Item = String>, _args: &[String]) {

  let uid = utils::make_new_uid();
  let path = defaults::get_datafile(&(uid.clone() + ".ics"));

  let new_cal = match IcalVCalendar::from_str(TEMPLATE_EVENT, Some(path)).unwrap().with_uid(&uid) {
    Ok(new_cal) => new_cal,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = match new_cal.with_dtstamp_now() {
    Ok(new_cal) => new_cal,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  
  match utils::write_cal(&new_cal) {
    Ok(_) => info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display()),
    Err(error) => {
      error!("{}", error);
      return
    },
  }

  println!("{}", new_cal.get_principal_event().get_khaleesi_line().unwrap());
}

static TEMPLATE_EVENT: &str = indoc!("
  BEGIN:VCALENDAR
  VERSION:2.0
  PRODID:-//khaleesi //EN
  BEGIN:VEVENT
  SUMMARY:<<EDIT ME>>
  LOCATION:<<EDIT ME>>
  DTSTART;VALUE=DATE-TIME:20181026T133000
  DTEND;VALUE=DATE-TIME:20181026T160000
  DTSTAMP;VALUE=DATE-TIME:20181022T145405Z
  UID:foo
  END:VEVENT
  END:VCALENDAR
");
