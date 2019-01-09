use std::path::PathBuf;

pub static DATADIR: &str = ".khaleesi";
pub static INDEXDIR: &str = "index";
pub static SEQFILE: &str  = "seq";
pub static CALDIR: &str  = "cal";

pub fn get_datafile(filename: &str) -> PathBuf {
  [DATADIR, filename].iter().collect()
}

pub fn get_seqfile() -> PathBuf {
  [DATADIR, SEQFILE].iter().collect()
}

pub fn get_configfile() -> PathBuf {
  [DATADIR, "config.toml"].iter().collect()
}

pub fn get_indexdir() -> PathBuf {
  [DATADIR, INDEXDIR].iter().collect()
}

pub fn get_indexfile(key: &str) -> PathBuf {
  [DATADIR, INDEXDIR, key].iter().collect()
}

pub fn get_indexlockfile() -> PathBuf {
  [DATADIR, "index-lock"].iter().collect()
}

pub fn get_caldir() -> PathBuf {
  [DATADIR, CALDIR].iter().collect()
}
