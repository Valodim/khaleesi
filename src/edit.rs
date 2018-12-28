use utils;
use std::process::Command;

pub fn do_edit(filenames: &mut Iterator<Item = String>, _args: &[String]) {

  let paths: Vec<String> = filenames.map( |line| {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    match utils::datetime_from_timestamp(parts[0]) {
      Some(_) => parts[1].to_string(),
      None => parts[0].to_string(),
    }
  }).collect();

  Command::new("vim")
    .args(paths)
    .status()
    .expect("vim command failed to start");

}
