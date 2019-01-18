use utils::fileutil;
use input;
use KhResult;

pub fn do_modify(args: &[String]) -> KhResult<()> {
  info!("do_modify");
  let mut lines = input::default_input_multiple()?;

  if args[0] == "removeprop" && args[1] == "xlicerror" {
    let cals = fileutil::read_calendars_from_files(&mut lines)?;
    let output: Vec<String> = cals.into_iter()
      .map(|cal| cal.with_remove_property("X-LIC-ERROR") )
      .filter(|cal| cal.1 > 0)
      .map(|cal| cal.0.to_string())
      .collect();
    println!("{}", output.join("\n"));
  } else {
    error!("not supported");
  }

  Ok(())
}
