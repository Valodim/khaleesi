use utils;
use itertools::Itertools;
use defaults::*;
use std::path::{Path};

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
  if let Err(error) = utils::write_file(&SEQFILE.to_owned(), utils::read_filenames_from_stdin().join("\n")) {
    error!("Could not write seqfile: {}", error);
  } 
}

fn write_seqfile_to_stdout() {
  let mut filepath: String = DATADIR.to_owned();
  filepath.push_str("/");
  filepath.push_str(SEQFILE);
  print!("{}", utils::read_file_to_string(Path::new(&filepath)).unwrap());
}
