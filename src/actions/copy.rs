use khline::KhLine;
use utils::fileutil;
use utils::misc;

pub fn do_copy(lines: &mut Iterator<Item = String>, _args: &[String]) {

  let lines = lines.collect::<Vec<String>>();
  if lines.len() > 1 {
    println!("copy only one event!");
    return;
  };

  let khline = lines[0].parse::<KhLine>();
  let cal = match khline.and_then(|khline| khline.to_cal()) {
    Ok(calendar) => calendar,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = match cal.with_uid(&misc::make_new_uid()) {
    Ok(new_cal) => new_cal,
    Err(error) => {
      error!("{}", error);
      return
    },
  };
  let new_cal = new_cal.with_dtstamp_now();

  match fileutil::write_cal(&new_cal) {
    Ok(_) => info!("Successfully wrote file: {}", new_cal.get_path().unwrap().display()),
    Err(error) => {
      error!("{}", error);
      return
    },
  }

  println!("{}", KhLine::from(&new_cal.get_principal_event()).unwrap());
}
