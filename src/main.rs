#[cfg(not(target_os = "linux"))]
compile_error!("can only be compiled on linux ;)");

use clap::{AppSettings, Clap};
use dvm::{Res, cli::{install, remove, show, update}, common::*, common::VERSION, completions, error, r#type::Type};

#[derive(Clap, Debug)]
#[clap(version = VERSION, setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  command: Command
}

#[derive(Clap, Debug)]
enum Command {
  #[clap(about = INSTALL_DESC, aliases = INSTALL_ALIASES)]
  Install(InstallOption),

  #[clap(about = UPDATE_DESC, aliases = UPDATE_ALIASES)]
  Update(UpdateOption),

  #[clap(about = REMOVE_DESC, aliases = REMOVE_ALIASES)]
  Remove(RemoveOption),

  #[clap(about = SHOW_DESC, aliases = SHOW_ALIASES)]
  Show(ShowOption),

  #[clap(about = COMP_DESC, aliases = COMP_ALIASES)]
  Completions(CompletionsOption)
}

#[derive(Clap, Debug)]
struct InstallOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Clap, Debug)]
struct UpdateOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Clap, Debug)]
struct RemoveOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Clap, Debug)]
struct ShowOption {
  #[clap(short, long)]
  verbose: bool,

  #[clap(short, long)]
  check: bool,
}

#[derive(Clap, Debug)]
struct CompletionsOption {
  #[clap(possible_values = POSSIBLE_SHELLS)]
  shell: String
}

fn str_to_type(s: String) -> Type {
  match s.as_str() {
    "stable" | "discord-stable" | "s" => Type::STABLE,
    "canary" | "discord-canary" | "c" => Type::CANARY,
    "ptb" | "discord-ptb" | "p" => Type::PTB,
    "development" | "dev" | "discord-development" | "d" => Type::DEVELOPMENT,
    _ => {
      error(format!("type \"{}\" does not exist", s));
      std::process::exit(1);
    }
  }
}

#[tokio::main]
async fn main() -> Res<()> {
  let opts = Opts::parse();

  match opts.command {
    Command::Install(opt) => {
      for r#type in opt.r#type {
        install(str_to_type(r#type), opt.verbose).await?;
      }
    }
    Command::Update(opt) => {
      for r#type in opt.r#type {
        update(str_to_type(r#type), opt.verbose).await?;
      }
    }
    Command::Remove(opt) => {
      for r#type in opt.r#type {
        remove(str_to_type(r#type), opt.verbose).await?;
      }
    }
    Command::Show(opt) => {
      show(opt.verbose, opt.check).await?;
    }
    Command::Completions(opt) => {
      completions::give_completions(&opt.shell);
    }
  };

  Ok(())
}
