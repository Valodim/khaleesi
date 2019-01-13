use atty;

use seqfile;
use cursorfile;
use khline::KhLine;
use utils::fileutil;

pub fn default_input_multiple() -> Result<Box<dyn Iterator<Item = String>>, String> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("Taking input from Stdin");
    Ok(Box::new(fileutil::read_lines_from_stdin().unwrap().into_iter()))
  } else {
    let seq = seqfile::read_seqfile().map_err(|_| "Invalid input".to_string())?;
    Ok(Box::new(seq))
  }
}

pub fn default_input_single() -> Result<KhLine, String> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("Taking input from Stdin");

    let lines = match fileutil::read_lines_from_stdin() {
      Ok(lines) => lines,
      Err(error) => {
        return Err(format!("{}", error));
      }
    };
    if lines.len() > 1 {
      Err("too many lines in cursorfile".to_string())
    } else {
      lines[0].parse::<KhLine>()
    }
  } else {
    cursorfile::read_cursorfile()
  }
}
