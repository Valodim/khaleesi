use std::path::Path;
use utils;

pub fn do_modify(lines: &mut Iterator<Item = String>, args: &[String]) {
  info!("do_modify");
  
  if args[0] == "removeprop" && args[1] == "xlicerror" {
    let cals = utils::read_calendars_from_files(lines).unwrap();
    let output: Vec<String> = cals.into_iter()
      .map(|cal| cal.with_remove_property("X-LIC-ERROR") )
      .filter(|cal| cal.1 > 0)
      .map(|cal| cal.0.to_string())
      .collect();
    println!("{}", output.join("\n"));
  } else {
    error!("not supported")
  }

}
