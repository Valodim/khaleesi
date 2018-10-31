pub mod icalwrap;
pub mod prettyprint;
pub mod agenda;
pub mod utils;
pub mod ical;
pub mod cal;
pub mod index;
pub mod calutil;
pub mod sort;

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
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main() {
  simple_logger::init().unwrap();

  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "index" => action_index(&args[2..]),
    "print" => action_prettyprint(&args[2..]),
    "short" => action_prettyprint_all(&args[2..]),
    "agenda" => action_agenda(&args[2..]),
    "sort" => action_sort(&args[2..]),
    "cal" => cal::printcal(),
    "dbg" => cal::dbg(),
    _  => println!("Usage: {} index|print|short|sort|agenda|cal|dbg", args[0])
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn action_sort(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  sort::sort_file(filepath)
}

fn action_agenda(args: &[String]) {
  if args.len() == 0 {
    agenda::show_events(&mut read_filenames_from_stdin());
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    agenda::show_events(&mut read_filenames(filepath));
  }
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

fn read_filenames(filepath: &Path) -> impl Iterator<Item = String> {
  let f = File::open(filepath).expect("Unable to open file");
  let f = BufReader::new(f);
  let lines = f.lines().map(|x| x.expect("Unable to read line"));
  //show_lines(&mut lines);
  lines
}

fn read_filenames_from_stdin() -> impl Iterator<Item = String> {
  let stdin = std::io::stdin();
  let handle = stdin.lock();

  let lines = handle.lines().map(|x| x.expect("Unable to read line")).collect::<Vec<String>>().into_iter();
  //let lines = handle.lines().map(|x| x.unwrap());
  lines
}
