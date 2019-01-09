use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::env;
use std::path::PathBuf;

pub fn path_to(artifact: &str) -> PathBuf {
  [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
}

pub fn prepare_testdir(template: &str) -> TempDir {
  let testdir = TempDir::new().unwrap();
  env::set_current_dir(testdir.path()).unwrap();
  testdir.child(".khaleesi/").copy_from(path_to(template), &["*"]).unwrap();
  testdir
}
