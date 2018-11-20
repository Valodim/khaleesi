use utils;
use itertools::Itertools;
use defaults::SEQFILE;

pub fn do_seq(args: &[String]) {
  if atty::isnt(atty::Stream::Stdin) {
    write_stdin_to_file()
  } else {
    println!("stdin is tty")
  }
}

fn write_stdin_to_file() {
  if let Err(error) = utils::write_file(&SEQFILE.to_owned(), utils::read_filenames_from_stdin().join("\n")) {
    error!("Could not write seqfile: {}", error);
  } 
}
