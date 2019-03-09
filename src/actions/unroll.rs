use std::path::{PathBuf, Path};
use structopt::StructOpt;

use crate::khline::KhLine;
use crate::KhResult;
use crate::khevent::KhEvent;

#[derive(Debug, StructOpt)]
pub struct UnrollArgs {
  /// The file to unroll
  #[structopt(name = "path", parse(from_os_str))]
  pub path: PathBuf,
}

pub fn action_unroll(args: &UnrollArgs) -> KhResult<()> {
  let filepath = &args.path;
  do_unroll(filepath)?;

  Ok(())
}

fn do_unroll(filepath: &Path) -> KhResult<()> {
  let path = filepath.to_str().ok_or_else(|| "str to path failed")?;
  let khline = path.parse::<KhLine>()?;
  let cal = khline.to_cal()?;

  for event in cal.events_iter() {
    if event.is_recur_master() {
      let recurs = event.get_recur_datetimes();
      for datetime in recurs {
        println!("{} {}", datetime.timestamp(), cal.get_path_as_string().unwrap_or_else(|| "".to_string()));
      }
    }
  }
  Ok(())
}
