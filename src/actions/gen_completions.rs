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
    Bash,
    Zsh,
    Fish,
    Elvish
  }
}

pub fn gen_completions(args: &GenCompletionsArgs) -> KhResult<()> {
  let mut app = CommandLine::clap();
  let binary_name = "khaleesi";
  match args.shell {
    ShellArg::Bash => app.gen_completions_to(binary_name, Shell::Bash, &mut io::stdout()),
    ShellArg::Zsh => app.gen_completions_to(binary_name, Shell::Zsh, &mut io::stdout()),
    ShellArg::Fish => app.gen_completions_to(binary_name, Shell::Fish, &mut io::stdout()),
    ShellArg::Elvish => app.gen_completions_to(binary_name, Shell::Elvish, &mut io::stdout()),
  }
  Ok(())
}
