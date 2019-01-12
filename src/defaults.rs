use std::path::{Path,PathBuf};
use std::cell::RefCell;
use std::env;

pub static DATADIR: &str = ".khaleesi";
pub static INDEXDIR: &str = "index";
pub static SEQFILE: &str  = "seq";
pub static CURSORFILE: &str  = "cursor";
pub static CALDIR: &str  = "cal";
pub static BACKUPDIR: &str  = "backup";

thread_local! {
  static KHALEESI_DIR: RefCell<PathBuf> = RefCell::new(env::current_dir().unwrap())
}

fn get_khaleesi_dir() -> PathBuf {
  KHALEESI_DIR.with(|dir| { dir.borrow().clone() })
}

pub fn set_khaleesi_dir(path: &Path) {
  let path = path.to_path_buf();
  KHALEESI_DIR.with(|dir| {
    *dir.borrow_mut() = path;
  });
}

pub fn get_datafile(filename: &str) -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(filename);
  dir
}

pub fn get_seqfile() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push("seq");
  dir
}

pub fn get_cursorfile() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(CURSORFILE);
  dir
}

pub fn get_configfile() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push("config.toml");
  dir
}

pub fn get_indexdir() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(INDEXDIR);
  dir
}

pub fn get_backupdir() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(BACKUPDIR);
  dir
}

pub fn get_indexfile(key: &str) -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(INDEXDIR);
  dir.push(key);
  dir
}

pub fn get_indexlockfile() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push("index-lock");
  dir
}

pub fn get_indextimefile() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push("index-time");
  dir
}

pub fn get_caldir() -> PathBuf {
  let mut dir = get_khaleesi_dir();
  dir.push(DATADIR);
  dir.push(CALDIR);
  dir
}
