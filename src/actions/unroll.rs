use std::path::Path;

use khline::KhLine;
use KhResult;

pub fn action_unroll(args: &[String]) -> KhResult<()> {
  let file = &args[0];
  let filepath = Path::new(file);
  do_unroll(filepath);

  Ok(())
}

fn do_unroll(filepath: &Path) {
  let cal = filepath.to_str().ok_or_else(|| "str to path failed".to_string())
    .and_then(|path| path.parse::<KhLine>())
    .and_then(|khline| khline.to_cal())
    .unwrap();
  for event in cal.events_iter() {
    if event.is_recur_master() {
      let recurs = event.get_recur_datetimes();
      for datetime in recurs {
        println!("{} {}", datetime.timestamp(), cal.get_path_as_string().unwrap_or_else(|| "".to_string()));
      }
    }
  }
}
