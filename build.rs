#[macro_use]
extern crate clap;
extern crate structopt;

use std::env;
use structopt::clap::Shell;

include!("src/cli.rs");

fn main() {
  if env::var_os("PROFILE") == Some("release".into()) {
    let outdir = match env::var_os("OUT_DIR") {
      None => return,
      Some(outdir) => outdir,
    };
    let mut app = CommandLine::clap();
    let binary_name = "khaleesi";
    app.gen_completions(binary_name, Shell::Bash, &outdir);
    app.gen_completions(binary_name, Shell::Zsh, &outdir);
    app.gen_completions(binary_name, Shell::Fish, &outdir);
    app.gen_completions(binary_name, Shell::Elvish, &outdir);
  }
}
