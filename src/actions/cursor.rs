extern crate atty;

use cursorfile;
use utils::fileutil;
use KhResult;

pub fn do_cursor(_args: &[String]) -> KhResult<()> {
  if atty::isnt(atty::Stream::Stdin) {
    write_stdin_to_cursorfile();
  } else {
    //println!("stdin is tty")
  }

  if atty::isnt(atty::Stream::Stdout) || atty::is(atty::Stream::Stdin) {
    write_cursorfile_to_stdout();
  }

  Ok(())
}

fn write_stdin_to_cursorfile() {
  let lines = match fileutil::read_lines_from_stdin() {
    Ok(input) => input,
    Err(error) => {
      error!("Error reading from stdin: {}", error);
      return
    }
  };

  if lines.len() > 1 {
    error!("Too many lines on stdin");
    return
  };

  cursorfile::write_cursorfile(&lines[0]).unwrap();
}

fn write_cursorfile_to_stdout() {
  if let Ok(cursor) = cursorfile::read_cursorfile() {
    println!("{}", cursor);
  }
}
