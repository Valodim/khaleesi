use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::path::PathBuf;
use std::fs;

use defaults;

pub fn path_to(artifact: &str) -> PathBuf {
  [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
}

pub fn prepare_testdir_empty() -> TempDir {
  let testdir = TempDir::new().unwrap();
  fs::create_dir(testdir.child(".khaleesi").path()).unwrap();
  defaults::set_khaleesi_dir(testdir.path());
  testdir
}

pub fn prepare_testdir(template: &str) -> TempDir {
  let testdir = prepare_testdir_empty();
  testdir.child(".khaleesi/").copy_from(path_to(template), &["*"]).unwrap();
  testdir
}
