use utils;
use itertools::Itertools;
use defaults::*;
use std::path::{Path};
use std::fs::rename;

pub fn do_seq(args: &[String]) {
  if atty::isnt(atty::Stream::Stdin) {
    write_stdin_to_seqfile()
  } else {
    //println!("stdin is tty")
  }

  if atty::isnt(atty::Stream::Stdout) {
    write_seqfile_to_stdout()
  } else {
    //println!("stdout is tty")
  }
}

fn write_stdin_to_seqfile() {
  let tmpfilename = "tmpseq";

  let mut tmpfilepath: String = DATADIR.to_owned();
  tmpfilepath.push_str("/");
  tmpfilepath.push_str(tmpfilename);
  let lines;
  match utils::read_lines_from_stdin() {
    Ok(mut input) => lines = input.join("\n"),
    Err(error) => {
      error!("Error reading from stdin: {}", error);
      return
    }
  }

  if let Err(error) = utils::write_file(&tmpfilename.to_owned(), lines) {
    error!("Could not write seqfile: {}", error);
    return
  } 

  let mut seqfilepath: String = DATADIR.to_owned();
  seqfilepath.push_str("/");
  seqfilepath.push_str(&SEQFILE);
  if let Err(error) = rename(Path::new(&tmpfilepath), Path::new(&seqfilepath)) {
    error!("{}", error)
  }
}

fn write_seqfile_to_stdout() {
  let mut seqfilepath: String = DATADIR.to_owned();
  seqfilepath.push_str("/");
  seqfilepath.push_str(&SEQFILE);
  print!("{}", utils::read_file_to_string(Path::new(&seqfilepath)).unwrap());
}
