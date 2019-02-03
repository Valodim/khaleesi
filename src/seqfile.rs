use std::fs::rename;
use std::io;

use defaults::*;
use khline::KhLine;
use utils::fileutil;
use KhResult;
use errors::KhError;

pub fn write_to_seqfile(lines: &str) -> io::Result<()> {
  let tmpfilename = get_datafile("tmpseq");

  fileutil::write_file(&tmpfilename, lines)?;

  let seqfile = get_seqfile();
  rename(tmpfilename, seqfile)?;

  Ok(())
}

pub fn read_seqfile() -> io::Result<impl DoubleEndedIterator<Item = String>> {
  let seqfile = get_seqfile();
  debug!("Reading sequence file: {}", seqfile.to_string_lossy());
  fileutil::read_lines_from_file(&seqfile)
}

pub fn read_seqfile_khlines() -> KhResult<impl DoubleEndedIterator<Item = KhLine>> {
  read_seqfile()?
    .map(|line| line.parse::<KhLine>())
    .collect::<Result<Vec<KhLine>, String>>()
    .map_err(KhError::from)
    .map(|lines| lines.into_iter())
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
  fn read_seqfile_khlines_test() {
    let _testdir = prepare_testdir("testdir_with_seq");
    let mut khlines = read_seqfile_khlines().unwrap();

    let khline_expected = "1544740200 twodaysacrossbuckets.ics".parse::<KhLine>().unwrap();
    assert_eq!(khline_expected, khlines.next().unwrap())
  }

  #[test]
  fn write_to_seqfile_test() {
    let testdir = prepare_testdir("testdir");
    let teststr = "Teststr äöüß\n";

    write_to_seqfile(teststr).unwrap();

    testdir.child(".khaleesi/seq").assert(teststr);
  }
}
