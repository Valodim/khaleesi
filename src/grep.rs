use icalwrap::IcalVCalendar;
use utils;

pub fn grep(filenames: &mut Iterator<Item = String>, args: &[String]) {
  if args.len() == 0 {
    println!("Usage: grep term+");
    return;
  }

  let cals = utils::iterate_calendars_from_files(filenames);

  let predicate = predicate_contains_term(&args[0]);
  let cals = cals.filter(predicate);

  utils::print_cals(cals);
}

fn predicate_contains_term(term: &str) -> impl Fn(&IcalVCalendar) -> bool {
  let term = term.to_lowercase();
  move |cal| {
    let event = cal.get_principal_event();
    if let Some(summary) = event.get_summary() {
      if summary.to_lowercase().contains(&term) {
        return true;
      }
    }
    if let Some(description) = event.get_description() {
      if description.to_lowercase().contains(&term) {
        return true;
      }
    }
    false
  }
}
