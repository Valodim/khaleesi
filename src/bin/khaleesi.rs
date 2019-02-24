use log::error;

use khaleesi::actions::*;
use khaleesi::cli;
use khaleesi::config::Config;
use khaleesi::KhResult;

use std::env;
use structopt::StructOpt;


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
    cli::Command::Cursor(x) => cursor::do_cursor(x),
    cli::Command::Delete => delete::do_delete(),
    cli::Command::Edit => edit::do_edit(),
    //      "get" => get::action_get(args),
    cli::Command::Index(x) => index::action_index(x),
    cli::Command::List(x) => {
      list::list_by_args(&x.args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>())
    }
    //      "modify" => modify::do_modify(args),
    cli::Command::New(x) => new::do_new(x),
    cli::Command::Select(x) => {
      select::select_by_args(&x.args.iter().map(|x| x.as_ref()).collect::<Vec<&str>>())
    }
    cli::Command::Seq => seq::action_seq(),
    //      "pretty" => prettyprint::prettyprint(),
    cli::Command::Show => show::do_show(),
    cli::Command::Undo => undo::do_undo(),
    cli::Command::Unroll(x) => unroll::action_unroll(&x),
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
