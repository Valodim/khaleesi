use std::io;
use structopt::clap::Shell;
use structopt::StructOpt;

use crate::cli::CommandLine;
use crate::KhResult;

#[derive(Debug, StructOpt)]
pub struct GenCompletionsArgs {
  /// the shell
  #[structopt(name = "shell", raw(possible_values = "&ShellArg::variants()"))]
  pub shell: ShellArg,
}

arg_enum! {
#[derive(Debug)]
  pub enum ShellArg{
    bash,
    zsh,
    fish,
    elvish
  }
}

pub fn gen_completions(args: &GenCompletionsArgs) -> KhResult<()> {
  let mut app = CommandLine::clap();
  let binary_name = "khaleesi";
  match args.shell {
    ShellArg::bash => app.gen_completions_to(binary_name, Shell::Bash, &mut io::stdout()),
    ShellArg::zsh => app.gen_completions_to(binary_name, Shell::Zsh, &mut io::stdout()),
    ShellArg::fish => app.gen_completions_to(binary_name, Shell::Fish, &mut io::stdout()),
    ShellArg::elvish => app.gen_completions_to(binary_name, Shell::Elvish, &mut io::stdout()),
  }
  Ok(())
}
