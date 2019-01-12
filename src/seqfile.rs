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

#[cfg(test)]
mod tests {
  use super::*;

  use testutils::prepare_testdir;
  use assert_fs::prelude::*;
  use predicates::prelude::*;
  use itertools::Itertools;

  #[test]
  fn read_seqfile_test() {
    let testdir = prepare_testdir("testdir_with_seq");
    let mut seqfile_read = read_seqfile().unwrap().join("\n");
    seqfile_read.push('\n');

    let predicate = predicate::str::similar(seqfile_read);
    testdir.child(".khaleesi/seq").assert(predicate);
  }

  #[test]
  fn write_to_seqfile_test() {
    let testdir = prepare_testdir("testdir");
    let teststr = "Teststr äöüß\n";

    write_to_seqfile(teststr);
    testdir.child(".khaleesi/seq").assert(teststr);
  }
}
