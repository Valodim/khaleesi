use utils;
use prettyprint;
use std::path::{Path};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn sort_stuff (files: &mut Iterator<Item = String>) {
  let mut comps = utils::read_comps_from_files(files);
  comps.sort_unstable_by_key(|comp| comp.get_dtstart());
  for comp in comps {
    prettyprint::shortprint_comp(&comp);
  }
}

pub fn sort_file(filepath: &Path) {
  let f = File::open(filepath).expect("Unable to open file");
  let f = BufReader::new(f);
  let mut lines = f.lines().map(|x| x.expect("Unable to read line"));
  sort_stuff(&mut lines);
}
