pub mod icalwrap;
pub mod prettyprint;
pub mod utils;
pub mod ical;
pub mod index;

extern crate chrono;
extern crate libc;

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::env;
use std::path::Path;

fn main() {
  simple_logger::init().unwrap();

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "index" => action_index(&args[2..]),
    "print" => action_prettyprint(&args[2..]),
    _  => println!("Usage: {} index|action", args[0])
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn action_prettyprint(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  prettyprint::prettyprint_file(filepath)
}

fn action_index(args: &[String]) {
  let dir = &args[0];
  index::index_dir(dir)
}
