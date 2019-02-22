use log::error;

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

  #[cfg(not(debug_assertions))] {
    if let Some(dir) = dirs::home_dir() {
      use khaleesi::defaults;
      defaults::set_khaleesi_dir(&dir);
    }
  }

  let args: Vec<String> = env::args().collect();
  let config = Config::read_config();

  let binary_name = &args[0].split('/').last().unwrap();
  let mut args = args[1..].iter().map(|s| s.as_str()).collect::<Vec<&str>>();
  if *binary_name != "khaleesi" && binary_name.starts_with("kh") {
    let command = &binary_name[2..];
    args.push(command);
    args.rotate_right(1);
  }
  let result = main_internal(binary_name, &args[..], &config);
  if let Err(error) = result {
    error!("{}", error)
  }
}

fn main_internal(binary_name: &str, args: &[&str], config: &Config) -> KhResult<()> {
  if args.is_empty() {
    print_usage(&binary_name);
    Ok(())
  } else {
    let cmd = args[0];
    let args = &args[1..];
    match cmd {
      "agenda" => agenda::show_events(&config, args),
      "copy" => copy::do_copy(args),
      "cursor" => cursor::do_cursor(args),
      "delete" => delete::do_delete(args),
      "edit" => edit::do_edit(args),
      "get" => get::action_get(args),
      "index" => index::action_index(args),
      "list" => list::list_by_args(args),
      "modify" => modify::do_modify(args),
      "new" => new::do_new(args),
      "select" => select::select_by_args(args),
      "seq" => seq::action_seq(args),
      "pretty" => prettyprint::prettyprint(),
      "show" => show::do_show(args),
      "undo" => undo::do_undo(args),
      "unroll" => unroll::action_unroll(args),
      _  => { print_usage(cmd); Ok(()) }
    }
  }
}

fn print_usage(name: &str) {
  error!("Usage: {} index|select|list|agenda|copy|new|edit|show", name)
}
