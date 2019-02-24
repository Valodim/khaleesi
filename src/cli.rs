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
  #[structopt(name = "delete")]
  Delete,
  /// Edit event
  #[structopt(name = "edit")]
  Edit,
  /// Rebuild index
  #[structopt(name = "index")]
  Index(Index),
  /// Select from the sequence
  #[structopt(name = "list")]
  List(List),
  /// Create new event
  #[structopt(name = "new")]
  New(New),
  /// Select from the index
  #[structopt(name = "select")]
  Select(Select),
}

#[derive(Debug, StructOpt)]
pub struct Agenda {
  /// Show agenda view 
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct Cursor {
  /// Move the cursor on the selection. 
  #[structopt(name = "direction", raw(possible_values = "&Direction::variants()"))]
  pub direction: Option<Direction>,
}

arg_enum! {
#[derive(Debug)]
  pub enum Direction {
    next,
    prev,
  }
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

#[derive(Debug, StructOpt)]
pub struct List {
  /// the arguments for the selection
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct Select {
  /// the arguments for the selection
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct New {
  /// the calendar
  #[structopt(name = "calendar")]
  pub calendar: String,
  /// from
  #[structopt(name = "from")]
  pub from: String,
  /// to
  #[structopt(name = "to")]
  pub to: String,
  /// summary
  #[structopt(name = "summary")]
  pub summary: String,
  /// location
  #[structopt(name = "location")]
  pub location: String,
}
