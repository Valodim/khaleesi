use input;
use khline::KhLine;
use utils::fileutil;

pub fn do_show(_args: &[String]) -> Result<(), String> {
  info!("do_show");
  let lines = input::default_input_multiple()?;

  for line in lines {
    let khline = line.parse::<KhLine>().map_err(|err| err.to_string())?;
    let output = fileutil::read_file_to_string(&khline.path).unwrap();
    println!("{}", output);
  }

  Ok(())
}
