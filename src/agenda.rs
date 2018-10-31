use prettyprint::*;
use std::path::{Path};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn show_file(filepath: &Path) {
  let f = File::open(filepath).expect("Unable to open file");
  let f = BufReader::new(f);
  let mut lines = f.lines().map(|x| x.expect("Unable to read line"));
  show_lines(&mut lines);
}

fn show_lines(lines: &mut Iterator<Item = String>) {
  for (i, line) in lines.enumerate() {
    //let i = i+1;
    let path = Path::new(&line);
    print!("{}  ", i);
    shortprint_file(path)
  }
}
