use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  author = "",
  name = "khaleesi",
  about = "Command line calendar tool.",
  raw(setting = "structopt::clap::AppSettings::VersionlessSubcommands")
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
  #[structopt(name = "agenda", author = "")]
  Agenda(AgendaArgs),
  /// Copy event
  #[structopt(name = "copy", author = "")]
  Copy,
  /// Interact with the cursor
  #[structopt(name = "cursor", author = "")]
  Cursor(CursorArgs),
  /// Delete event
  #[structopt(name = "delete", author = "")]
  Delete,
  /// Edit event
  #[structopt(name = "edit", author = "")]
  Edit,
  /// Get info about the calendar data
  #[structopt(name = "get", author = "")]
  Get(GetArgs),
  /// Rebuild index
  #[structopt(name = "index", author = "")]
  Index(IndexArgs),
  /// Select from the sequence
  #[structopt(name = "list", author = "")]
  List(ListArgs),
  /// Modify an event
  #[structopt(name = "modify", author = "")]
  Modify(ModifyArgs),
  /// Create new event
  #[structopt(name = "new", author = "")]
  New(NewArgs),
  /// Select from the index
  #[structopt(name = "select", author = "")]
  Select(SelectArgs),
  /// Interact with the sequence
  #[structopt(name = "seq", author = "")]
  Seq,
  /// Show the raw ical file of an event
  #[structopt(name = "show", author = "")]
  Show,
  /// Undo the most recent action
  #[structopt(name = "undo", author = "")]
  Undo,
  /// Unroll a recurring event
  #[structopt(name = "unroll", author = "")]
  Unroll(UnrollArgs),
}

#[derive(Debug, StructOpt)]
pub struct AgendaArgs {
  /// Show agenda view
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct CursorArgs {
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
pub struct GetArgs {
  /// Show information about this
  #[structopt(name = "query", raw(possible_values = "&GetQueryArgs::variants()"))]
  pub query: GetQueryArgs,
}

arg_enum! {
#[derive(Debug)]
  pub enum GetQueryArgs{
    calendars,
  }
}

#[derive(Debug, StructOpt)]
pub struct IndexArgs {
  /// Rebuild index
  #[structopt(short = "r", long = "reindex")]
  pub reindex: bool,
  /// index path
  #[structopt(name = "path", parse(from_os_str))]
  pub path: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct ListArgs {
  /// the arguments for the selection
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct ModifyArgs {
  /// Rebuild index
  #[structopt(short = "n", long = "dry-run")]
  pub dry_run: bool,
  /// index path
  #[structopt(subcommand)]
  pub modify_cmd: ModifyCommand,
}

#[derive(Debug, StructOpt)]
pub enum ModifyCommand {
  /// Show agenda view
  #[structopt(name = "remove-xlicerror", author = "")]
  RemoveXlicerror,
}

#[derive(Debug, StructOpt)]
pub struct SelectArgs {
  /// the arguments for the selection
  #[structopt(name = "args")]
  pub args: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct UnrollArgs {
  /// The file to unroll
  #[structopt(name = "path", parse(from_os_str))]
  pub path: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct NewArgs {
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
