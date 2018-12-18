use chrono::*;
use std::path::PathBuf;
use utils;
use defaults;

struct SelectFilters {
  from: Option<Date<Local>>,
  to: Option<Date<Local>>,
}

impl SelectFilters {
  pub fn parse_from_args(args: &[String]) -> Result<Self, String> {
    let mut fromarg: Option<Date<Local>> = None;
    let mut toarg: Option<Date<Local>> = None;

    if args.len() < 2 {
      return Err("select [from|to parameter]+".to_string())
    }

    for chunk in args.chunks(2) {
      if chunk.len() == 2 {
        let mut datearg = match utils::date_from_str(&chunk[1]) {
          Ok(datearg) => datearg,
            Err(error) => {
              return Err(format!("{}", error))
            }
        };

        match chunk[0].as_str() {
          "from" => fromarg = Some(datearg),
          "to"   => toarg = Some(datearg),
          _      => return Err("Incorrect!".to_string())
        }
      } else {
        return Err("Syntax error!".to_string());
      }
    }
    Ok(SelectFilters {from: fromarg, to: toarg})
  }

  pub fn predicate_from(&self) -> impl Fn(&PathBuf) -> bool + '_ {
    move |path| {
      let filename = path.file_name().expect(&format!("{:?} not a file", path));
      match &self.from {
        Some(from) => filename < utils::get_bucket_for_date(from).as_str(),
        None => false
      }
    }
  }

  pub fn predicate_to<'a>(&'a self) -> impl Fn(&PathBuf) -> bool + 'a {
    move |path| {
      let filename = path.file_name().expect(&format!("{:?} not a file", path));
      match &self.to {
        Some(to) => filename <= utils::get_bucket_for_date(to).as_str(),
        None => true
      }
    }
  }
}

pub fn select_by_args(args: &[String]) {

  let filters = SelectFilters::parse_from_args(args).unwrap();
  let indexdir = defaults::get_indexdir();

  let mut buckets: Vec<PathBuf> = utils::file_iter(&indexdir)
    .collect();
  buckets.sort_unstable();
  let buckets = buckets.into_iter().skip_while( filters.predicate_from() )
    .take_while( filters.predicate_to() );

  let lines = buckets.map(|bucket| utils::read_lines_from_file(&bucket))
    .filter_map(|lines| lines.ok())
    .flatten();
  for line in lines {
    println!("{}", line);
  }
}
