extern crate atty;

use utils;
use std::io;
use itertools::Itertools;
use defaults::*;
use std::fs::rename;

pub fn do_seq(_args: &[String]) {
  if atty::isnt(atty::Stream::Stdin) {
    write_stdin_to_seqfile()
  } else {
    //println!("stdin is tty")
  }

  write_seqfile_to_stdout()
}

fn write_stdin_to_seqfile() {
  let tmpfilename = get_datafile("tmpseq");

  let seqfile = get_seqfile();
  let mut lines;
  match utils::read_lines_from_stdin() {
    Ok(mut input) => lines = input.join("\n"),
    Err(error) => {
      error!("Error reading from stdin: {}", error);
      return
    }
  }
  lines.push_str("\n");
  if let Err(error) = utils::write_file(&tmpfilename, lines) {
    error!("Could not write seqfile: {}", error);
    return
  }

  if let Err(error) = rename(tmpfilename, seqfile) {
    error!("{}", error)
  }
}

pub fn read_seqfile() -> io::Result<impl Iterator<Item = String>> {
  let seqfile = get_seqfile();
  debug!("Reading sequence file: {}", seqfile.to_string_lossy());
  utils::read_lines_from_file(&seqfile)
}

fn write_seqfile_to_stdout() {
  if let Ok(sequence) = read_seqfile() {
    for line in sequence {
      println!("{}", line);
    }
  }
}
