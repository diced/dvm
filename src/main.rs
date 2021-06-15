#[cfg(not(target_os = "linux"))]
compile_error!("can only be compiled on linux ;)");

use clap::{AppSettings, Clap};
use dvm::{Res, cli::{install, remove, show, update}, error, r#type::Type};

const POSSIBLE_VALUES: &[&str] = &[
  "stable",
  "discord-stable",
  "s",

  "canary",
  "discord-canary",
  "c",
  
  "ptb",
  "discord-ptb",
  "p",
  
  "development",
  "dev",
  "discord-development",
  "d",
];

#[derive(Clap, Debug)]
#[clap(version = "1.1.4", setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  command: Command
}

#[derive(Clap, Debug)]
enum Command {
  #[clap(about = "install the latest <type> of discord", aliases = &["i", "in", "get"])]
  Install(InstallOption),

  #[clap(about = "update to the latest <type> of discord", aliases = &["u", "up", "upgrade"])]
  Update(UpdateOption),

  #[clap(about = "remove the installed <type> of discord", aliases = &["r", "rm", "d", "del", "un", "uninstall"])]
  Remove(RemoveOption),

  #[clap(about = "show all installed versions", aliases = &["s", "installed", "all", "a", "versions", "types"])]
  Show(ShowOption),
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
  };

  Ok(())
}
