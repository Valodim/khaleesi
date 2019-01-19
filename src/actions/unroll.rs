use std::path::Path;

use khline::KhLine;
use KhResult;

pub fn action_unroll(args: &[String]) -> KhResult<()> {
  let file = &args[0];
  let filepath = Path::new(file);
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
