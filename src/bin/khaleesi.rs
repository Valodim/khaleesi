extern crate atty;
extern crate khaleesi;
extern crate stderrlog;

#[macro_use]
extern crate log;

use khaleesi::agenda;
use khaleesi::cal;
use khaleesi::copy;
use khaleesi::config::Config;
use khaleesi::defaults::*;
use khaleesi::edit;
use khaleesi::index;
use khaleesi::list;
use khaleesi::new;
use khaleesi::prettyprint;
use khaleesi::select;
use khaleesi::seq;
use khaleesi::show;
use khaleesi::sort;
use khaleesi::unroll;
use khaleesi::utils;

use std::env;
use std::path::{Path,PathBuf};

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

  main_internal(&args[0], &args[1..], config);
}

fn main_internal(binary_name: &str, args: &[String], config: Config) {
  if args.len() == 0 {
    print_usage(&binary_name)
  } else {
    match args[0].as_str() {
      "agenda" => action_agenda(&config, &args[1..]),
      "cal" => cal::printcal(),
      "copy" => action_copy(&args[1..]),
      "new" => action_new(&args[1..]),
      "dbg" => cal::dbg(),
      "edit" => action_edit(&args[1..]),
      "index" => action_index(&args[1..]),
      "list" => action_list(&args[1..]),
      "select" => action_select(&args[1..]),
      "seq" => action_sequence(&args[1..]),
      "short" => action_prettyprint_all(&args[1..]),
      "show" => action_show(&args[1..]),
      "sort" => action_sort(&args[1..]),
      "unroll" => action_unroll(&args[1..]),
      _  => print_usage(&args[0])
    }
  }

}

fn print_usage(name: &str) {
  error!("Usage: {} index|select|list|agenda|copy|new|edit|show|cal|sort|dbg|short", name)
}

fn action_sequence(args: &[String]) {
  seq::do_seq(args); 
}

fn action_list(args: &[String]) {
  //lists from sequence file or stdin
  if let Some(mut input) = default_input() {
    list::list_by_args(&mut input, &args);
  }
}

fn action_show(args: &[String]) {
  if let Some(mut input) = default_input() {
    show::do_show(&mut input, &args);
  }
}

fn action_edit(args: &[String]) {
  if let Some(mut input) = default_input() {
    edit::do_edit(&mut input, &args);
  }
}

fn action_select(args: &[String]) {
  //selects from index
  select::select_by_args(args);
}

fn action_sort(args: &[String]) {
  if args.is_empty() {
    if let Some(mut input) = default_input() {
        sort::sort_filenames_by_dtstart(&mut input)
    }
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    let mut lines = utils::read_lines_from_file(filepath).unwrap();
    sort::sort_filenames_by_dtstart(&mut lines);
  }
}

fn action_agenda(config: &Config, args: &[String]) {
  if args.is_empty() {
    if let Some(mut input) = default_input() {
      agenda::show_events(&config, &mut input);
    }
  } else {
    let file = &args[0];
    let filepath = Path::new(file);
    agenda::show_events(&config, &mut utils::read_lines_from_file(filepath).unwrap());
  }
}

fn action_unroll(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  unroll::do_unroll(filepath)
}

fn action_prettyprint_all(args: &[String]) {
  let file = &args[0];
  let filepath = Path::new(file);
  prettyprint::shortprint_dir(filepath)
}

fn action_index(args: &[String]) {
  let indexpath = if args.is_empty() {
    get_caldir()
  } else {
    PathBuf::from(&args[0])
  };
  index::index_dir(&indexpath)
}

fn action_copy(args: &[String]) {
  if let Some(mut input) = default_input() {
    copy::do_copy(&mut input, &args);
  }
}

fn action_new(args: &[String]) {
  if let Some(mut input) = default_input() {
    new::do_new(&mut input, &args);
  }
}

fn default_input() -> Option<Box<dyn Iterator<Item = String>>> {
  if atty::isnt(atty::Stream::Stdin) {
    debug!("stdin");
    Some(Box::new(utils::read_lines_from_stdin().unwrap()))
  } else {
    match seq::read_seqfile() {
      Ok(sequence) => Some(Box::new(sequence)),
      Err(err) => {
        error!("{}", err);
        None
      }
    }

  }
}

#[cfg(test)]
mod tests {
  extern crate tempfile;
  extern crate walkdir;

  use std::env;
  use std::fs;
  use std::path::PathBuf;
  use self::tempfile::{tempdir,TempDir};
  use std::path::Path;
  use self::walkdir::WalkDir;

  use super::*;

  fn path_to(artifact: &str) -> PathBuf {
      [env!("CARGO_MANIFEST_DIR"), "testdata", artifact].iter().collect()
  }

  fn append_path(base: &Path, path: impl AsRef<Path>) -> PathBuf {
      let mut result = PathBuf::from(base);
      result.push(path);
      result
  }

  fn prepare_testdir() -> TempDir {
      let testdir = tempdir().unwrap();
      let testdir_khaleesi = append_path(testdir.path(), ".khaleesi");
      let testdir_cal = append_path(testdir.path(), ".khaleesi/cal");

      println!("preparing test dir: {:?}", testdir.path());
      fs::create_dir(testdir_khaleesi).unwrap();
      fs::create_dir(testdir_cal.clone()).unwrap();

      for direntry in WalkDir::new(path_to("cal")).into_iter() {
          if let Ok(file) = direntry {
              if file.file_type().is_file() {
                  println!("copying {:?}", file.path());
                  fs::copy(file.path(), append_path(&testdir_cal, file.file_name())).unwrap();
              }
          }
      }

      testdir
  }

  fn run(testdir: &TempDir, args: &[&str], config: Option<Config>) {
      env::set_current_dir(testdir).unwrap();

      let config = config.unwrap_or_default();
      let args: Vec<String> = args.iter().map(|x| x.to_string()).collect();
      main_internal("khaleesi", &args, config);
  }

  fn list_files(path: &Path) -> Vec<String> {
    let mut list: Vec<String> = fs::read_dir(path).unwrap()
      .into_iter()
      .flatten()
      .map(|x| x.path().file_name().unwrap().to_string_lossy().into_owned())
      .collect();
    list.sort();
    list
  }

  #[test]
  fn test_index() {
      let testdir = prepare_testdir();
      run(&testdir, &["index"], None);

      let index_files = list_files(&append_path(testdir.path(), ".khaleesi/index"));
      assert_eq!(vec!("2018-W50".to_string(), "2018-W51".to_string()), index_files);
  }

}
