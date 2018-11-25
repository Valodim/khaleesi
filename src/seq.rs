extern crate atty;

use utils;
use itertools::Itertools;
use defaults::*;
use std::path::{Path,PathBuf};
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
  let tmpfilename = "tmpseq";

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
  if let Err(error) = utils::write_file(&tmpfilename.to_owned(), lines) {
    error!("Could not write seqfile: {}", error);
    return
  }

  if let Err(error) = rename(Path::new(&tmpfilename), seqfile) {
    error!("{}", error)
  }
}

pub fn read_seqfile() -> impl Iterator<Item = String> {
  let seqfile = get_seqfile();
  utils::read_lines_from_file(&seqfile).unwrap()
}

fn get_seqfile() -> PathBuf {
  [DATADIR, SEQFILE].iter().collect()
}

fn write_seqfile_to_stdout() {
  let seq = read_seqfile();
  for line in seq {
    println!("{}", line)
  }
}
