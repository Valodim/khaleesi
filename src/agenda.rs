use prettyprint::*;
use utils;
use std::path::{Path};
use std::fs::File;
use std::io::{BufRead, BufReader};
use yansi::Style;

pub fn show_file(filepath: &Path) {
  let f = File::open(filepath).expect("Unable to open file");
  let f = BufReader::new(f);
  let mut lines = f.lines().map(|x| x.expect("Unable to read line"));
  //show_lines(&mut lines);
  show_events(&mut lines);
}

fn show_lines(lines: &mut Iterator<Item = String>) {
  for (i, line) in lines.enumerate() {
    //let i = i+1;
    let path = Path::new(&line);
    print!("{}  ", i);
    shortprint_file(path)
  }
}

pub fn show_events(lines: &mut Iterator<Item = String>) {
  let style_heading = Style::new().bold();
  let comps = utils::read_comps_from_files(lines);

  let mut cur_day = comps[0].get_dtstart().date();
  println!("{}", style_heading.paint(cur_day));

  for (i, comp) in comps.iter().enumerate() {
    if comp.get_dtstart().date() != cur_day {
      cur_day = comp.get_dtstart().date();
      println!("{}", style_heading.paint(cur_day));
    }
    print!("  {}  ", i);
    shortprint_comp(comp)
  }
}
