extern crate assert_cli;
extern crate tempfile;
extern crate walkdir;

use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::{tempdir,TempDir};
use assert_cli::Assert;
use std::path::Path;
use walkdir::WalkDir;


fn path_to(artifact: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
}

fn append_path(base: &Path, path: impl AsRef<Path>) -> PathBuf {
    let mut result = PathBuf::from(base);
    result.push(path);
    result
}

fn prepare_testdir() -> TempDir {
    let testdir = tempdir().unwrap();
    let testdir_khaleesi = append_path(testdir.path(), ".khaleesi");
    let testdir_cal = append_path(testdir.path(), ".khaleesi/cal");

    println!("preparing test dir: {:?}", testdir.path());
    fs::create_dir(testdir_khaleesi).unwrap();
    fs::create_dir(testdir_cal.clone()).unwrap();

    for direntry in WalkDir::new(path_to("cal")).into_iter() {
        if let Ok(file) = direntry {
            if file.file_type().is_file() {
                println!("copying {:?}", file.path());
                fs::copy(file.path(), append_path(&testdir_cal, file.file_name())).unwrap();
            }
        }
    }

    testdir
}

#[test]
fn test_index() {
    let testdir = prepare_testdir();

    base_cmd()
        .current_dir(testdir.path())
        .with_args(&["index"])
        .stderr().contains("Loaded 1 files into 2 buckets")
        .unwrap();
}

// Adapted from
// https://github.com/rust-lang/cargo/blob/485670b3983b52289a2f353d589c57fae2f60f82/tests/testsuite/support/mod.rs#L507
fn target_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        }).unwrap()
}

fn khaleesi_exe() -> PathBuf {
    target_dir().join(format!("khaleesi{}", env::consts::EXE_SUFFIX))
}

fn base_cmd() -> Assert {
    Assert::command(&[&khaleesi_exe()])
}
