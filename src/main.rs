#[cfg(not(target_os = "linux"))]
compile_error!("can only be compiled on linux ;)");

use clap::{AppSettings, Clap};
use dvm::{Res, cli::{install, install_openasar, remove, show, update, run}, common::*, common::VERSION, completions, error, r#type::Type};

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

  #[clap(about = INSTALL_OPENASAR_DESC, aliases = INSTALL_OPENASAR_ALIASES)]
  InstallOpenAsar(InstallOpenAsarOption),

  #[clap(about = UPDATE_DESC, aliases = UPDATE_ALIASES)]
  Update(UpdateOption),

  #[clap(about = REMOVE_DESC, aliases = REMOVE_ALIASES)]
  Remove(RemoveOption),

  #[clap(about = SHOW_DESC, aliases = SHOW_ALIASES)]
  Show(ShowOption),

  #[clap(about = COMP_DESC, aliases = COMP_ALIASES)]
  Completions(CompletionsOption),

  #[clap(about = RUN_DESC, aliases = RUN_ALIASES)]
  Run(RunOptions)
}

#[derive(Clap, Debug)]
struct InstallOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>,

  #[clap(short, long)]
  verbose: bool,

  #[clap(short, long)]
  open_asar: bool
}

#[derive(Clap, Debug)]
struct InstallOpenAsarOption {
  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: Vec<String>,

  #[clap(short, long)]
  verbose: bool
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

#[derive(Clap, Debug)]
struct RunOptions {
  #[clap(short, long)]
  verbose: bool,

  #[clap(possible_values = POSSIBLE_VALUES)]
  r#type: String,

  #[clap(last = true)]
  args: Vec<String>
}

fn str_to_type(s: String) -> Type {
  match s.as_str() {
    "stable" | "discord-stable" | "s" => Type::STABLE,
    "canary" | "discord-canary" | "c" => Type::CANARY,
    "ptb" | "discord-ptb" | "p" => Type::PTB,
    "development" | "dev" | "discord-development" | "d" => Type::DEVELOPMENT,
    _ => {
      error!("type \"{}\" does not exist", s);
    }
  }
}

fn check_type_len(types: &Vec<String>) -> Res<()> {
  if types.len() == 0 {
    error!("no types provided");
  }

  Ok(())
}

#[tokio::main]
async fn main() -> Res<()> {
  let opts = Opts::parse();

  Ok(match opts.command {
    Command::Install(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        install(str_to_type(r#type), opt.verbose, opt.open_asar).await?
      }
    }
    Command::InstallOpenAsar(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        install_openasar(str_to_type(r#type), opt.verbose).await?
      }
    }

    Command::Update(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        update(str_to_type(r#type), opt.verbose).await?
      }
    }
    Command::Remove(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        remove(str_to_type(r#type), opt.verbose).await?
      }
    }
    Command::Show(opt) => {
      show(opt.verbose, opt.check).await?
    }
    Command::Completions(opt) => {
      completions::give_completions(&opt.shell)
    }
    Command::Run(opt) => {
      run(str_to_type(opt.r#type), opt.args.clone(), opt.verbose).await?
    }
  })
}
