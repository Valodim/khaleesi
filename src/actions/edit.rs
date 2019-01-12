use std::env;
use std::fs;
use std::process::Command;

use khline::KhLine;

pub fn do_edit(khline: &KhLine, _args: &[String]) {

  let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

  if let Err(error) = Command::new(&editor)
    .arg(khline.path.as_os_str())
    .stdin(fs::File::open("/dev/tty").unwrap())
    .status() {
      error!("{} command failed to start, error: {}", editor, error);
      return
    };
}
