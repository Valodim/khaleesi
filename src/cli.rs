use structopt::StructOpt;

use crate::actions::gen_completions::GenCompletionsArgs;
use crate::actions::agenda::AgendaArgs;
use crate::actions::cursor::CursorArgs;
use crate::actions::get::GetArgs;
use crate::actions::index::IndexArgs;
use crate::actions::list::ListArgs;
use crate::actions::modify::ModifyArgs;
use crate::actions::select::SelectArgs;
use crate::actions::unroll::UnrollArgs;
use crate::actions::new::NewArgs;

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
  ///
  /// deletes a single event, either from stdin or the current cursor
  #[structopt(name = "delete", author = "")]
  Delete,
  /// Edit event
  #[structopt(name = "edit", author = "")]
  Edit,
  /// Get info about the calendar data
  #[structopt(name = "get", author = "")]
  Get(GetArgs),
  /// Print shell completions script to stdout
  #[structopt(name = "gen-completions", author = "")]
  GenCompletions(GenCompletionsArgs),
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
