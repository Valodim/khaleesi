extern crate atty;

use std::fs::rename;
use std::io;

use defaults::*;
use utils::fileutil;

pub fn write_to_seqfile(lines: &str) {
  let tmpfilename = get_datafile("tmpseq");

  if let Err(error) = fileutil::write_file(&tmpfilename, lines) {
    error!("Could not write seqfile: {}", error);
    return
  }

  let seqfile = get_seqfile();
  if let Err(error) = rename(tmpfilename, seqfile) {
    error!("{}", error)
  }
}

pub fn read_seqfile() -> io::Result<impl Iterator<Item = String>> {
  let seqfile = get_seqfile();
  debug!("Reading sequence file: {}", seqfile.to_string_lossy());
  fileutil::read_lines_from_file(&seqfile)
}

