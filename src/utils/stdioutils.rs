use std::io;
use std::io::BufRead;

pub fn read_single_char_from_stdin() -> io::Result<char> {
  let stdin = io::stdin();
  let stdinlock = stdin.lock();
  read_single_char(stdinlock)
}

pub fn read_single_char(mut source: impl BufRead) -> io::Result<char> {
  let mut buf = String::new();
  source.read_line(&mut buf)?;

  buf.chars().next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "calendar has no path"))
}

#[cfg(not(test))]
pub use self::production::*;
#[cfg(test)]
pub use self::test::*;
#[cfg(test)]
pub use self::fixtures::*;

#[cfg(not(test))]
mod production {
  use super::*;

  pub fn read_lines_from_stdin() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    lines.collect()
  }

  pub fn is_stdin_tty() -> bool {
    atty::is(atty::Stream::Stdin)
  }

  pub fn is_stdout_tty() -> bool {
    atty::is(atty::Stream::Stdout)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  pub fn read_lines_from_stdin() -> io::Result<Vec<String>> {
    let lines = fixtures::test_stdin_clear();
    Ok(lines)
  }

  pub fn is_stdin_tty() -> bool {
    fixtures::test_stdin_is_tty()
  }

  pub fn is_stdout_tty() -> bool {
    fixtures::test_stdout_is_tty()
  }

}

#[cfg(test)]
pub mod fixtures {
  use std::cell::RefCell;
  thread_local! {
    pub static STDOUT_BUF: RefCell<String> = RefCell::new(String::new());
    pub static STDIN_BUF: RefCell<String> = RefCell::new(String::new());
    pub static STDIN_TTY: RefCell<bool> = RefCell::new(true);
    pub static STDOUT_TTY: RefCell<bool> = RefCell::new(true);
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

  pub fn test_stdout_set_tty(istty: bool) {
    STDOUT_TTY.with(|cell| { *cell.borrow_mut() = istty } );
  }

  pub fn test_stdout_is_tty() -> bool {
    STDOUT_TTY.with(|cell| { *cell.borrow() } )
  }

  pub fn test_stdin_write(text: &str) {
    test_stdin_set_tty(false);
    STDIN_BUF.with(|cell| cell.borrow_mut().push_str(&text));
  }

  pub fn test_stdin_clear() -> Vec<String> {
    STDIN_BUF.with(|cell| {
      let result = cell.borrow().lines().map(|line| line.to_owned()).collect();
      *cell.borrow_mut() = String::new();
      result
    })
  }

  pub fn test_stdin_set_tty(istty: bool) {
    STDIN_TTY.with(|cell| { *cell.borrow_mut() = istty } );
  }

  pub fn test_stdin_is_tty() -> bool {
    STDIN_TTY.with(|cell| { *cell.borrow() } )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_single_char_test() {
    let source = "ab".as_bytes();
    let read_char = read_single_char(source).unwrap();
    assert_eq!('a', read_char);
  }
}
