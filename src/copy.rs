use utils;

pub fn do_copy(lines: &mut Iterator<Item = String>, _args: &[String]) {

  let lines = lines.collect::<Vec<String>>();
  if lines.len() > 1 {
    println!("copy only one event!");
    return;
  };

  let cal = utils::read_khaleesi_line(&lines[0]).unwrap();  //TODO do not stupidly unwrap
  let mut event = cal.get_principal_event();
    //Ok(event) => event,
    //Err(error) => { 
      //error!("{}", error);
      //return;
    //}
  //};

  debug!("uid: {}", event.get_uid());
  let uid = &utils::make_new_uid();
  event.set_uid(uid);
  debug!("uid: {}", event.get_uid());

  let path = match cal.get_path() {
    Some(path) => path.with_file_name(uid),
    None => {
      error!("Could not get path.");
      return
    }
  };

  match utils::write_file(&path, cal.to_string()) {
    Ok(_) => info!("Successfully wrote file: {}", path.display()),
    Err(err) => error!("{}", err), 
  }

  println!("{}", cal.get_principal_event().get_khaleesi_line().unwrap());
}
