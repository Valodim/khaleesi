use utils;

pub fn do_copy(lines: &mut Iterator<Item = String>, _args: &[String]) {

  let lines = lines.collect::<Vec<String>>();
  if lines.len() > 1 {
    println!("copy only one event!");
    return;
  };

  let cal = match utils::read_khaleesi_line(&lines[0]) {
    Ok(calendar) => calendar,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = match cal.with_uid(&utils::make_new_uid()) {
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
