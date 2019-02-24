use log::error;

use khaleesi::actions::*;
use khaleesi::config::Config;
use khaleesi::KhResult;

use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "khalessi", about = "Command line calendar tool.")]
struct Khaleesi {
  /// Verbosity
  #[structopt(short = "v", parse(from_occurrences))]
  verbosity: u64,
  #[structopt(subcommand)]
  cmd: Index,
}

#[derive(Debug, StructOpt)]
enum Index {
  /// Rebuild index
  #[structopt(name = "index")]
  Index {
    /// Rebuild index
    #[structopt(short = "r", long = "reindex")]
    reindex: bool,
  },
}

fn main() {
  let args = Khaleesi::clap().get_matches();
  println!("{:?}", args);

  //let opt = Khaleesi::from_args();
  //println!("{:?}", opt);

  #[cfg(not(debug_assertions))]
  {
    if let Some(dir) = dirs::home_dir() {
      use khaleesi::defaults;
      defaults::set_khaleesi_dir(&dir);
    }
    init_logger(1 + args.occurrences_of("verbosity"));
  }

  //set default log level to INFO in debug builds
  #[cfg(debug_assertions)]
  init_logger(3 + args.occurrences_of("verbosity"));

  //let args: Vec<String> = env::args().collect();
  //let config = Config::read_config();

  //let binary_name = &args[0].split('/').last().unwrap();
  //let mut args = args[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  //if *binary_name != "khaleesi" && binary_name.starts_with("kh") {
  //  let command = &binary_name[2..];
  //  args.push(command);
  //  args.rotate_right(1);
  //}
  //let result = main_internal(binary_name, &args[..], &config);
  //if let Err(error) = result {
  //  error!("{}", error)
  //}
}

fn main_internal(binary_name: &str, args: &[&str], config: &Config) -> KhResult<()> {
  if args.is_empty() {
    print_usage(&binary_name);
    Ok(())
  } else {
    let cmd = args[0];
    let args = &args[1..];
    match cmd {
      //      "agenda" => agenda::show_events(&config, args),
      //      "copy" => copy::do_copy(args),
      //      "cursor" => cursor::do_cursor(args),
      //      "delete" => delete::do_delete(args),
      //      "edit" => edit::do_edit(args),
      //      "get" => get::action_get(args),
      //      "index" => index::action_index(args),
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
        print_usage(cmd);
        Ok(())
      }
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
