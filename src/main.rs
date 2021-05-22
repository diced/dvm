use dvm::{Res, error, r#type::Type, install::install, remove::remove, update::update, show::show};
use clap::{AppSettings, Clap};
#[derive(Clap, Debug)]
#[clap(version = "0.1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Clap, Debug)]
enum Command {
  #[clap(about = "install the latest <type> of discord")]
  Install(InstallOption),

  #[clap(about = "update to the latest <type> of discord")]
  Update(UpdateOption),

  #[clap(about = "remove the installed <type> of discord")]
  Remove(RemoveOption),

  #[clap(about = "show all installed versions")]
  Show
}

#[derive(Clap, Debug)]
struct InstallOption {
  r#type: String
}

#[derive(Clap, Debug)]
struct UpdateOption {
  r#type: String
}

#[derive(Clap, Debug)]
struct RemoveOption {
  r#type: String
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
      install(str_to_type(opt.r#type)).await?;
    }
    Command::Update(opt) => {
      update(str_to_type(opt.r#type)).await?;
    }
    Command::Remove(opt) => {
      remove(str_to_type(opt.r#type)).await?;
    }
    Command::Show => {
      show().await?;
    }
  };

  Ok(())
}
