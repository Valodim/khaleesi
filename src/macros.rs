#[macro_export]
macro_rules! khprint {
  () => ();
  ($($arg:tt)*) => ({
    let line = format!($($arg)*);
    #[cfg(test)] {
      use utils::stdioutils;
      testutils::test_stdout_write(&line);
    }
    print!("{}", line);
  })
}

#[macro_export]
macro_rules! khprintln {
  () => ({
    #[cfg(test)] {
      use crate::utils::stdioutils;
      stdioutils::test_stdout_write("\n");
    }
    println!();
  });
  ($($arg:tt)*) => ({
    let line = format!($($arg)*);
    #[cfg(test)] {
      use crate::utils::stdioutils;
      stdioutils::test_stdout_write(&line);
      stdioutils::test_stdout_write("\n");
    }
    println!("{}", line);
  })
}
