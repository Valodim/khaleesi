use std::path::Path;

use khline::KhLine;

pub fn do_unroll(filepath: &Path) {
  let cal = filepath.to_str().unwrap().parse::<KhLine>().unwrap().to_cal().unwrap();
  for event in cal.events_iter() {
    if event.has_recur() {
      let recurs = event.get_recur_datetimes();
      for datetime in recurs {
        println!("{} {}", datetime.timestamp(), cal.get_path_as_string().unwrap_or_else(|| "".to_string()));
      }
    }
  }
}
