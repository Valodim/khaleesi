use std::fs;
use std::path::{Path,PathBuf};
use fs2::FileExt;

pub struct FileLock {
  path: PathBuf,
  lockfile: fs::File
}

impl Drop for FileLock {
  fn drop(&mut self) {
    debug!("Dropping lock on {}", self.path.to_string_lossy());

    self.lockfile.unlock().unwrap();
  }
}

pub fn lock_file_exclusive(path: &Path) -> Result<FileLock ,()> {
  debug!("Locking index ({})", path.to_string_lossy());

  let lockfile = fs::File::create(path).unwrap();
  lockfile.try_lock_exclusive().map_err(|_| ())?;

  Ok(FileLock  { path: PathBuf::from(path), lockfile })
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::NamedTempFile;

  #[test]
  fn test_lock() {
    let lockfile = NamedTempFile::new().unwrap();
    let lock = lock_file_exclusive(lockfile.path());
    assert!(lock.is_ok());
  }

  #[test]
  fn test_lock_fail() {
    let lockfile = NamedTempFile::new().unwrap();
    let lock = lock_file_exclusive(lockfile.path());
    let lock_err = lock_file_exclusive(lockfile.path());
    assert!(lock.is_ok());
    assert!(lock_err.is_err());
  }
}
