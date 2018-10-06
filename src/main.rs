pub mod icalwrap;
pub mod prettyprint;
pub mod utils;

extern crate chrono;
extern crate libc;

use std::env;
//use std::fs::{File, read_dir};
use chrono::{Datelike, Duration};
use std::collections::HashMap;
use std::path::Path;
use std::fs;

use icalwrap::*;

pub fn get_buckets(comp: &mut Icalcomponent) -> Vec<String> {
  let mut buckets: Vec<String> = comp
    .map(|x| {
      let mut start_date = x.get_dtstart();
      let end_date = x.get_dtend();
      let mut buckets = Vec::new();
      while start_date.iso_week() <= end_date.iso_week() {
        let bucket = format!(
          "{}-{:02}",
          start_date.iso_week().year(),
          start_date.iso_week().week()
        );
        buckets.push(bucket);
        start_date = start_date.checked_add_signed(Duration::days(7)).unwrap();
      }
      buckets
    }).flatten()
    .collect();
  buckets.sort();
  buckets.dedup();
  buckets
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[0].as_ref() {
    "index" => action_index(&args[1..]),
    "print" => action_prettyprint(&args[1..]),
    _  => ()
  }

  // do_other_stuff(args)
  // do_stuff(args)
}

fn action_prettyprint(args: &[String]) {
  let file = &args[1];
  let filepath = Path::new(file);
  prettyprint::prettyprint_file(filepath)
}

fn action_index(args: &[String]) {
  //let filename = &args[1];
  let dir = &args[1];
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        // Here, `entry` is a `DirEntry`.
        if entry.path().is_file() {
          if entry
            .path()
            .extension()
            .map_or(false, |extension| extension == "ics")
          {
            if let Ok(contents) = ::utils::read_file_to_string(&entry.path()) {
              let mut comp = Icalcomponent::from_str(&contents); //
              let comp_buckets = get_buckets(&mut comp);
              for bucketid in comp_buckets {
                buckets
                  .entry(bucketid)
                  .and_modify(|items| items.push(comp.get_uid()))
                  .or_insert(::utils::vec_from_string(comp.get_uid()));
              }
            }
          }
        }
      }
    }
  }
  for (key, val) in buckets.iter() {
    ::utils::write_file(key, val.join("\n"));
  }

  //  //println!("Searching for {}", query);
  //  println!("In file {}", filename);
  //
  //  let mut f = File::open(filename).expect("file not found");
  //
  //  let mut contents = String::new();
  //  f.read_to_string(&mut contents)
  //    .expect("something went wrong reading the file");
  //
  //  println!("With text:\n{}", contents);
  //
  //  let mut comp = parse_component(&contents);
  //
  //  let mut foo = get_buckets(&mut comp);
  //  println!("{}", foo.join("\n"));
}
