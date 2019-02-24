use log::error;

use khaleesi::actions::*;
use khaleesi::config::Config;
use khaleesi::KhResult;

use std::env;
use structopt::clap::ArgMatches;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "khalessi", about = "Command line calendar tool.")]
struct Khaleesi {
  /// Verbosity
  #[structopt(short = "v", parse(from_occurrences))]
  verbosity: u64,
  #[structopt(subcommand)]
  cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
  /// Show agenda view
  #[structopt(name = "agenda")]
  Agenda {
    /// Rebuild index
    #[structopt(name = "args")]
    args: Vec<String>,
  },
  /// Rebuild index
  #[structopt(name = "index")]
  Index {
    /// Rebuild index
    #[structopt(short = "r", long = "reindex")]
    reindex: bool,
    /// index path
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>
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

  let config = Config::read_config();

  //let args: Vec<String> = env::args().collect();
  //let binary_name = &args[0].split('/').last().unwrap();
  //let mut args = args[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  //if *binary_name != "khaleesi" && binary_name.starts_with("kh") {
  //  let command = &binary_name[2..];
  //  args.push(command);
  //  args.rotate_right(1);
  //}
  let result = main_internal(&args, &config);
  if let Err(error) = result {
    error!("{}", error)
  }
}

fn main_internal(args: &ArgMatches, config: &Config) -> KhResult<()> {
  match args.subcommand() {
    ("agenda", Some(sub_args)) => {
      let args = sub_args.values_of("args").map_or_else(|| Vec::new(), |x| x.collect::<Vec<&str>>());
      agenda::show_events(&config, &args)
    }
    //      "copy" => copy::do_copy(args),
    //      "cursor" => cursor::do_cursor(args),
    //      "delete" => delete::do_delete(args),
    //      "edit" => edit::do_edit(args),
    //      "get" => get::action_get(args),
    ("index", Some(sub_args))  => index::action_index(sub_args),
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
