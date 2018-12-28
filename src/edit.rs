use utils;
use std::fs;
use std::process::Command;

pub fn do_edit(filenames: &mut Iterator<Item = String>, _args: &[String]) {

  let mut paths: Vec<String> = filenames.map( |line| {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    match utils::datetime_from_timestamp(parts[0]) {
      Some(_) => parts[1].to_string(),
      None => parts[0].to_string(),
    }
  }).collect();
  paths.sort_unstable();
  paths.dedup();

  Command::new("vim")
    .args(paths)
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status()
    .expect("vim command failed to start");
}
