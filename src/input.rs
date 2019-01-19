use atty;
use std::io;

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

pub fn default_input_single() -> io::Result<KhLine> {
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
