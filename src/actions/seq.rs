extern crate atty;

use seqfile;
use utils::fileutil;
use KhResult;

pub fn do_seq(_args: &[&str]) -> KhResult<()> {
  if atty::isnt(atty::Stream::Stdin) {
    write_stdin_to_seqfile();
  } else {
    //println!("stdin is tty")
  }

  if atty::isnt(atty::Stream::Stdout) || atty::is(atty::Stream::Stdin) {
    write_seqfile_to_stdout();
  }

  Ok(())
}

fn write_stdin_to_seqfile() {
  let mut lines;
  match fileutil::read_lines_from_stdin() {
    Ok(input) => lines = input.join("\n"),
    Err(error) => {
      error!("Error reading from stdin: {}", error);
      return
    }
  }
  lines.push_str("\n");

  seqfile::write_to_seqfile(&lines);
}

fn write_seqfile_to_stdout() {
  if let Ok(sequence) = seqfile::read_seqfile() {
    for line in sequence {
      println!("{}", line);
    }
  }
}
