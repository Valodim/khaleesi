pub mod action;
mod indextime;
mod bucketable;

pub use self::action::action_index;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct IndexArgs {
  /// Rebuild index
  #[structopt(short = "r", long = "reindex")]
  pub reindex: bool,
  /// index path
  #[structopt(name = "path", parse(from_os_str))]
  pub path: Option<PathBuf>,
}
