use log::error;

use khaleesi::actions::*;
use khaleesi::config::Config;
use khaleesi::KhResult;

use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

use khaleesi::cli;

fn main() {
  //let clap_args = CommandLine::clap().get_matches();
  //println!("{:?}", clap_args);

  let args = cli::CommandLine::from_args();
  println!("{:?}", args);

  #[cfg(not(debug_assertions))]
  {
    if let Some(dir) = dirs::home_dir() {
      use khaleesi::defaults;
      defaults::set_khaleesi_dir(&dir);
    }
    init_logger(1 + args.verbosity);
  }

  //set default log level to INFO in debug builds
  #[cfg(debug_assertions)]
  init_logger(3 + args.verbosity);

  let config = Config::read_config();

  let result = main_internal(&args, &config);
  if let Err(error) = result {
    error!("{}", error)
  }
}

fn main_internal(args: &cli::CommandLine, config: &Config) -> KhResult<()> {
  match &args.cmd {
    cli::Command::Agenda(x) => {
      agenda::show_events(&config, &x.args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>())
    }
    cli::Command::Copy => copy::do_copy(),
    //      "cursor" => cursor::do_cursor(args),
    //      "delete" => delete::do_delete(args),
    //      "edit" => edit::do_edit(args),
    //      "get" => get::action_get(args),
    cli::Command::Index(x) => index::action_index(x),
    //      "list" => list::list_by_args(args),
    //      "modify" => modify::do_modify(args),
    //      "new" => new::do_new(args),
    //      "select" => select::select_by_args(args),
    //      "seq" => seq::action_seq(args),
    //      "pretty" => prettyprint::prettyprint(),
    //      "show" => show::do_show(args),
    //      "undo" => undo::do_undo(args),
    //      "unroll" => unroll::action_unroll(args),
    _ => {
      //print_usage(cmd);
      Ok(())
    }
  }
}

fn init_logger(verbose: u64) {
  stderrlog::new()
    .timestamp(stderrlog::Timestamp::Off)
    .verbosity(verbose as usize)
    .init()
    .unwrap();
  //            0 => LevelFilter::Error,
  //            1 => LevelFilter::Warn,
  //            2 => LevelFilter::Info,
  //            3 => LevelFilter::Debug,
  //            _ => LevelFilter::Trace,
}

fn print_usage(name: &str) {
  error!(
    "Usage: {} index|select|list|agenda|copy|new|edit|show",
    name
  )
}
