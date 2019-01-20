use atty;
use std::io;

use seqfile;
use cursorfile;
use khline::KhLine;
use utils::fileutil;

pub fn default_input_khlines() -> Result<Box<dyn Iterator<Item = KhLine>>, String> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("Taking input from Stdin");
    let lines = fileutil::read_lines_from_stdin().unwrap().into_iter();
    let khlines = lines.map(|line| line.parse::<KhLine>()).flatten();
    Ok(Box::new(khlines))
  } else {
    let lines = seqfile::read_seqfile().map_err(|_| "Invalid input".to_string())?;
    let khlines = lines.map(|line| line.parse::<KhLine>()).flatten();
    Ok(Box::new(khlines))
  }
}

pub fn default_input_khline() -> io::Result<KhLine> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("Taking input from Stdin");

    let lines = fileutil::read_lines_from_stdin()?;
    if lines.len() > 1 {
      Err(io::Error::new(io::ErrorKind::Other, "too many lines in cursorfile"))
    } else {
      lines[0].parse::<KhLine>().map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
  } else {
    cursorfile::read_cursorfile()
  }
}
