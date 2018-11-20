extern crate khaleesi;

use khaleesi::prettyprint;
use khaleesi::agenda;
use khaleesi::cal;
use khaleesi::index;
use khaleesi::sort;
use khaleesi::select;
use khaleesi::utils;

use std::env;
use std::path::Path;

fn main() {
  stderrlog::new().timestamp(stderrlog::Timestamp::Second).verbosity(4).init().unwrap();

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
      _  => print_usage(&args[0])
    }
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn print_usage(name: &String) {
  println!("Usage: {} index|print|short|sort|agenda|cal|dbg|select", name)
}

fn action_select(args: &[String]) {
  select::select_by_args(&mut utils::read_filenames_from_stdin(), &args);
}

fn action_sort(args: &[String]) {
  if args.len() == 0 {
    sort::sort_filenames_by_dtstart(&mut utils::read_filenames_from_stdin())
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    sort::sort_filenames_by_dtstart(&mut utils::read_filenames_from_file(filepath));
  }
}

fn action_agenda(args: &[String]) {
  if args.len() == 0 {
    agenda::show_events(&mut utils::read_filenames_from_stdin());
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    agenda::show_events(&mut utils::read_filenames_from_file(filepath));
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
