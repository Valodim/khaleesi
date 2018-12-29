use std::path::Path;

use utils;

pub fn do_show(filenames: &mut Iterator<Item = String>, _args: &[String]) {
  info!("do_show");

  for line in filenames {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let path = match utils::datetime_from_timestamp(parts[0]) {
      Some(_) => Path::new(parts[1]),
      None => Path::new(parts[0]),
    };
    let output = utils::read_file_to_string(path).unwrap();
    println!("{}", output);
  }
}
