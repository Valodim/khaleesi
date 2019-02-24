use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
  author = "me",
  name = "khalessi",
  about = "Command line calendar tool."
)]
pub struct CommandLine {
  /// verbosity
  #[structopt(short = "v", parse(from_occurrences))]
  pub verbosity: u64,
  #[structopt(subcommand)]
  pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
  /// Show agenda view
  #[structopt(name = "agenda")]
  Agenda(Agenda),
  /// Copy event
  #[structopt(name = "copy")]
  Copy,
  /// Interact with the cursor
  #[structopt(name = "cursor")]
  Cursor(Cursor),
  /// Delete event
  #[structopt(name = "copy")]
  Delete,
  /// Rebuild index
  #[structopt(name = "index")]
  Index(Index),
}

#[derive(Debug, StructOpt)]
pub struct Agenda {
  /// Show agenda view 
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct Cursor {
  /// Move the cursor on the selection. Uses "next" and "prev".
  #[structopt(name = "direction")]
  pub direction: Option<String>,
}

#[derive(Debug, StructOpt)]
pub struct Index {
  /// Rebuild index
  #[structopt(short = "r", long = "reindex")]
  pub reindex: bool,
  /// index path
  #[structopt(name = "path", parse(from_os_str))]
  pub path: Option<PathBuf>,
}
