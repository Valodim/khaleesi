pub mod icalwrap;
pub mod prettyprint;
pub mod agenda;
pub mod utils;
pub mod ical;
pub mod cal;
pub mod index;
pub mod calutil;

#[cfg(test)]
pub mod testdata;

extern crate chrono;
extern crate yansi;
extern crate libc;
extern crate itertools;
extern crate simple_logger;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate indoc;

use std::env;
use std::path::Path;

fn main() {
  simple_logger::init().unwrap();

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "index" => action_index(&args[2..]),
    "print" => action_prettyprint(&args[2..]),
    "short" => action_prettyprint_all(&args[2..]),
    "agenda" => action_agenda(&args[2..]),
    "cal" => cal::printcal(),
    "dbg" => cal::dbg(),
    _  => println!("Usage: {} index|print|short|cal|dbg", args[0])
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn action_agenda(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  agenda::show_file(filepath)
}

fn action_prettyprint(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  prettyprint::prettyprint_file(filepath)
}

fn action_prettyprint_all(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  prettyprint::shortprint_dir(filepath)
}

fn action_index(args: &[String]) {
  let dir = &args[0];
  let dirpath = Path::new(dir);
  index::index_dir(dirpath)
}
