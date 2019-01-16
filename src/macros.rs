#[macro_export]
macro_rules! khprint {
  () => ();
  ($($arg:tt)*) => ({
    let line = format!($($arg)*);
    #[cfg(test)] {
      use testutils;
      testutils::test_stdout_write(&line);
    }
    print!("{}", line);
  })
}

#[macro_export]
macro_rules! khprintln {
  () => ({
    #[cfg(test)] {
      use testutils;
      testutils::test_stdout_write("\n");
    }
    println!();
  });
  ($($arg:tt)*) => ({
    let line = format!($($arg)*);
    #[cfg(test)] {
      use testutils;
      testutils::test_stdout_write(&line);
      testutils::test_stdout_write("\n");
    }
    println!("{}", line);
  })
}
