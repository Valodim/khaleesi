use selectors::SelectFilters;
use utils;

/* TODO port range queries
if args.len() == 1 {
  let rangeargs: Vec<&str> = args[0].splitn(2, '-').collect();
  match rangeargs.len() {
    1 => {
      if let Ok(num) = rangeargs[0].parse::<usize>() {
        return Ok(ListFilters {from, to, range: Some((num, num)), calendar} );
      } else {
        return Err("list [num] | [from|to|cal parameter]+".to_string())
      }
    },
    2 => {
      let lower = rangeargs[0].parse::<usize>();
      let upper = rangeargs[1].parse::<usize>();
      if lower.is_ok() && upper.is_ok() {
        return Ok(ListFilters {from, to, range: Some((lower.unwrap(), upper.unwrap())), calendar} );
      } else {
        return Err("list [num] | [from|to|cal parameter]+".to_string())
      }
    }
    _ => {
        return Err("list [num] | [from|to|cal parameter]+".to_string())
    }
  }
}
  // if let Some(range) = filters.range {
    // filenames
      // .take(range.1 + 1)
      // .skip(range.0)
      // .for_each( |line| println!("{}", line));
    // return;
  // }

*/

// TODO port cal query
// "cal"  => calendar = Some(chunk[1].clone()) ,
// .map_or(false,  |path| path.parent().map_or(false, |path| path.ends_with(calendar)))

pub fn list_by_args(filenames: &mut Iterator<Item = String>, args: &[String]) {
  let filters = match SelectFilters::parse_from_args(args) {
    Err(error) => { println!("{}", error); return; },
    Ok(parsed_filters) => parsed_filters,
  };

  let cals = utils::read_calendars_from_files(filenames).unwrap();

  let events = cals.into_iter()
    .map(|cal| cal.get_principal_event())
    .filter(filters.predicate_line_is_from())
    .filter(filters.predicate_line_is_to())
    .filter(filters.predicate_others());

  for event in events {
    if let Some(line) = event.get_khaleesi_line() {
      println!("{}", line);
    }
  }
}

