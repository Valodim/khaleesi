extern crate atty;
extern crate khaleesi;
extern crate stderrlog;

#[macro_use]
extern crate log;

use khaleesi::prettyprint;
use khaleesi::agenda;
use khaleesi::cal;
use khaleesi::index;
use khaleesi::sort;
use khaleesi::select;
use khaleesi::seq;
use khaleesi::utils;
use khaleesi::unroll;

use std::env;
use std::path::Path;

fn main() {
  stderrlog::new().timestamp(stderrlog::Timestamp::Second).verbosity(3).init().unwrap();
  //            0 => LevelFilter::Error,
  //            1 => LevelFilter::Warn,
  //            2 => LevelFilter::Info,
  //            3 => LevelFilter::Debug,
  //            _ => LevelFilter::Trace,

  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
    print_usage(&args[0])
  } else {
    match args[1].as_str() {
      "index" => action_index(&args[2..]),
      "print" => action_prettyprint(&args[2..]),
      "short" => action_prettyprint_all(&args[2..]),
      "agenda" => action_agenda(&args[2..]),
      "sort" => action_sort(&args[2..]),
      "cal" => cal::printcal(),
      "dbg" => cal::dbg(),
      "select" => action_select(&args[2..]),
      "seq" => action_sequence(&args[2..]),
      "unroll" => action_unroll(&args[2..]),
      _  => print_usage(&args[0])
    }
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn print_usage(name: &String) {
  println!("Usage: {} index|print|short|sort|agenda|cal|dbg|select", name)
}

fn action_sequence(args: &[String]) {
  seq::do_seq(args); 
}

fn action_select(args: &[String]) {
  select::select_by_args(&mut default_input(), &args);
}

fn action_sort(args: &[String]) {
  if args.len() == 0 {
    sort::sort_filenames_by_dtstart(&mut default_input())
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    let mut lines = utils::read_lines_from_file(filepath).unwrap();
    sort::sort_filenames_by_dtstart(&mut lines);
  }
}

fn action_agenda(args: &[String]) {
  if args.len() == 0 {
    agenda::show_events(&mut default_input());
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    agenda::show_events(&mut utils::read_lines_from_file(filepath).unwrap());
  }
}

fn action_unroll(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  unroll::do_unroll(filepath)
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

fn default_input() -> Box<dyn Iterator<Item = String>> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("stdin");
    Box::new(utils::read_lines_from_stdin().unwrap())
  } else {
    debug!("seqfile");
    Box::new(seq::read_seqfile())
  }
}
