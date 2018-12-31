use utils;
use icalwrap::IcalVCalendar;

pub fn do_copy(lines: &mut Iterator<Item = String>, _args: &[String]) {

  let lines = lines.collect::<Vec<String>>();
  if lines.len() > 1 {
    println!("copy only one event!");
    return;
  };

  let cal: IcalVCalendar;
  match utils::read_khaleesi_line(&lines[0]) {
    Ok(calendar) => cal = calendar,
    Err(error) => {
      error!("{}", error);
      return
    },
  }
  let new_cal = cal.with_uid(&utils::make_new_uid());
  match utils::write_cal(&new_cal) {
    Ok(_) => info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display()),
    Err(err) => {
      error!("{}", err);
      return
    },
  }

  println!("{}", new_cal.get_principal_event().get_khaleesi_line().unwrap());
}
