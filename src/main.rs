use std::env;
//use std::fs::{File, read_dir};
use chrono::{Datelike, Duration};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

extern crate chrono;
extern crate libc;

pub mod icalwrap;
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

fn read_file_to_string(path: &PathBuf) -> Result<String, String> {
  if let Ok(mut file) = File::open(&path) {
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
      Ok(contents)
    } else {
      //println!("something went wrong reading the file");
      Err("something went wrong reading the file".to_string())
    }
  } else {
    //println!("could not open {} for reading", path.display());
    Err(format!("could not open {} for reading", path.display()))
  }
}

fn vec_from_string(str: String) -> Vec<String> {
  let mut vec: Vec<String> = Vec::new();
  vec.push(str);
  vec
}

fn write_file(filename: &String, contents: String) -> std::io::Result<()> {
  let mut filepath: String = "Index/".to_owned();
  filepath.push_str(&filename);
  let mut file = File::create(filepath)?;
  file.write_all(contents.as_bytes())?;
  Ok(())
}

fn main() {
  let args: Vec<String> = env::args().collect();

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
            if let Ok(contents) = read_file_to_string(&entry.path()) {
              let mut comp = parse_component(&contents); //
              let comp_buckets = get_buckets(&mut comp);
              for bucketid in comp_buckets {
                buckets
                  .entry(bucketid)
                  .and_modify(|items| items.push(comp.get_uid()))
                  .or_insert(vec_from_string(comp.get_uid()));
              }
            }
          }
        }
      }
    }
  }
  for (key, val) in buckets.iter() {
    write_file(key, val.join("\n"));
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
