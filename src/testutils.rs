use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::path::PathBuf;

use std::cell::RefCell;
thread_local! {
  pub static STDOUT_BUF: RefCell<String> = RefCell::new(String::new())
}

use defaults;

pub fn path_to(artifact: &str) -> PathBuf {
  [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
}

pub fn prepare_testdir_empty() -> TempDir {
  let testdir = TempDir::new().unwrap();
  defaults::set_khaleesi_dir(testdir.path());
  testdir
}

pub fn prepare_testdir(template: &str) -> TempDir {
  let testdir = prepare_testdir_empty();
  testdir.child(".khaleesi/").copy_from(path_to(template), &["*"]).unwrap();
  testdir
}

pub fn test_stdout_write(line: &str) {
  STDOUT_BUF.with(|cell| cell.borrow_mut().push_str(&line));
}

pub fn test_stdout_clear() -> String {
  STDOUT_BUF.with(|cell| {
    let result = cell.borrow().clone();
    *cell.borrow_mut() = String::new();
    result
  })
}
