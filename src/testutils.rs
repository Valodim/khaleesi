use assert_fs::prelude::*;
use assert_fs::TempDir;
use std::path::PathBuf;

use defaults;

pub fn path_to(artifact: &str) -> PathBuf {
  [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
}

pub fn prepare_testdir(template: &str) -> TempDir {
  let testdir = TempDir::new().unwrap();
  defaults::set_khaleesi_dir(testdir.path());
  testdir.child(".khaleesi/").copy_from(path_to(template), &["*"]).unwrap();
  testdir
}
