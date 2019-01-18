extern crate khaleesi;
extern crate stderrlog;

#[macro_use]
extern crate log;

use khaleesi::config::Config;
use khaleesi::actions::*;
use khaleesi::KhResult;

use std::env;

fn main() {
  stderrlog::new()
    .timestamp(stderrlog::Timestamp::Off)
    .verbosity(3)
    .init()
    .unwrap();
  //            0 => LevelFilter::Error,
  //            1 => LevelFilter::Warn,
  //            2 => LevelFilter::Info,
  //            3 => LevelFilter::Debug,
  //            _ => LevelFilter::Trace,

  let args: Vec<String> = env::args().collect();
  let config = Config::read_config();

  let result = main_internal(&args[0], &args[1..], &config);
  if let Err(error) = result {
    error!("{}", error)
  }
}

fn main_internal(binary_name: &str, args: &[String], config: &Config) -> KhResult<()> {
  if args.is_empty() {
    print_usage(&binary_name);
    Ok(())
  } else {
    let cmd = args[0].as_str();
    let args = &args[1..];
    match cmd {
      "agenda" => agenda::show_events(&config, args),
      "cal" => cal::printcal(),
      "get" => get::action_get(&args),
      "copy" => copy::do_copy(&args),
      "cursor" => cursor::do_cursor(args),
      "new" => new::do_new(&args),
      "dbg" => cal::dbg(),
      "edit" => edit::do_edit(&args),
      "index" => index::action_index(&args),
      "list" => list::list_by_args(&args),
      "modify" => modify::do_modify(&args),
      "select" => select::select_by_args(args),
      "seq" => seq::do_seq(args),
      "pretty" => prettyprint::prettyprint(),
      "show" => show::do_show(&args),
      "unroll" => unroll::action_unroll(&args),
      _  => { print_usage(cmd); Ok(()) }
    }
  }
}

fn print_usage(name: &str) {
  error!("Usage: {} index|select|list|agenda|copy|new|edit|show|cal|dbg", name)
}
