use crate::input;
use crate::KhResult;

pub fn do_modify(args: &[&str]) -> KhResult<()> {
  info!("do_modify");

  if args[0] == "removeprop" && args[1] == "xlicerror" {
    let lines = input::default_input_khlines()?;
    let output: Vec<String> = lines
      .map(|line| line.to_cal())
      .flatten()
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
