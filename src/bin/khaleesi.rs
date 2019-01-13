extern crate khaleesi;
extern crate stderrlog;

#[macro_use]
extern crate log;

use khaleesi::config::Config;
use khaleesi::actions::*;

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

  match main_internal(&args[0], &args[1..], &config) {
    Err(error) => error!("{}", error),
      Ok(_) => (),
  }
}

fn main_internal(binary_name: &str, args: &[String], config: &Config) -> Result<(), String> {
  if args.is_empty() {
    print_usage(&binary_name);
    Ok(())
  } else {
    let cmd = args[0].as_str();
    let args = &args[1..];
    match cmd {
      "agenda" => agenda::show_events(&config, args),
      "cal" => cal::printcal(),
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

#[cfg(test)]
mod tests {
  extern crate assert_fs;
  extern crate predicates;

  use std::path::PathBuf;
  use self::assert_fs::prelude::*;
  use self::assert_fs::TempDir;

  use super::*;
  use khaleesi::defaults;

  fn path_to(artifact: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
  }

  fn prepare_testdir(template: &str) -> TempDir {
    let testdir = TempDir::new().unwrap();
    testdir.child(".khaleesi/").copy_from(path_to(template), &["*"]).unwrap();
    testdir
  }

  fn run(testdir: &TempDir, args: &[&str], config: Option<Config>) {
    defaults::set_khaleesi_dir(testdir.path());

    let config = config.unwrap_or_default();
    let args: Vec<String> = args.iter().map(|x| x.to_string()).collect();
    main_internal("khaleesi", &args, &config).unwrap();
  }

  #[test]
  fn test_index() {
    let testdir = prepare_testdir("testdir");

    run(&testdir, &["index"], None);

    testdir.child(".khaleesi/index/2018-W50").assert("1544740200 twodaysacrossbuckets.ics\n");
    testdir.child(".khaleesi/index/2018-W51").assert("1544740200 twodaysacrossbuckets.ics\n");
  }
}
