use std::env;
use std::fs;
use std::process::Command;

use utils::fileutil as utils;

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

  let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

  if let Err(error) = Command::new(&editor)
    .args(paths)
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status() {
      error!("{} command failed to start, error: {}", editor, error);
      return
    };
}
