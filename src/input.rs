use std::io;

use seqfile;
use cursorfile;
use khline::{KhLine,khlines_to_events,lines_to_khlines};
use utils::stdioutils;
use icalwrap::IcalVEvent;
use selectors::SelectFilters;

pub fn selection(args: &[&str]) -> Result<Box<dyn Iterator<Item = IcalVEvent>>, String> {
  if args.is_empty() {
    let khlines = default_input_khlines()?;
    let events = khlines_to_events(khlines);
    return Ok(Box::new(events));
  }

  let filters = SelectFilters::parse_from_args_with_range(args)?;
  let khlines = input_khlines_seq()?;
  let events = filters.filter_khlines(khlines);

  Ok(Box::new(events))
}

fn input_khlines_stdin() -> impl Iterator<Item = KhLine> {
  let lines = stdioutils::read_lines_from_stdin().unwrap().into_iter();
  lines_to_khlines(lines)
}

fn input_khlines_seq() -> Result<impl Iterator<Item = KhLine>, String> {
  let lines = seqfile::read_seqfile().map_err(|_| "Invalid input".to_string())?;
  Ok(lines_to_khlines(lines))
}

pub fn default_input_khlines() -> Result<Box<dyn Iterator<Item = KhLine>>, String> {
  let khlines: Box<dyn Iterator<Item = KhLine>> = if !stdioutils::is_stdin_tty() {
    debug!("Taking input from Stdin");
    Box::new(input_khlines_stdin())
  } else {
    Box::new(input_khlines_seq()?)
  };
  Ok(khlines)
}

pub fn default_input_khline() -> io::Result<KhLine> {
  if !stdioutils::is_stdin_tty() {
    debug!("Taking input from Stdin");

    let lines = stdioutils::read_lines_from_stdin()?;
    if lines.len() > 1 {
      Err(io::Error::new(io::ErrorKind::Other, "too many lines in input"))
    } else {
      lines[0].parse::<KhLine>().map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
  } else {
    cursorfile::read_cursorfile()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use utils::stdioutils;

  #[test]
  fn test_default_input_khline() {
    stdioutils::test_stdin_write("a\nb\n");

    assert!( default_input_khline().is_err());
  }
}
