use dvm::{Res, error, install::install, r#type::Type, remove::remove, update::update};
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
  Install(InstallOption),
  Update(UpdateOption),
  Remove(RemoveOption),
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
  }

  Ok(())
}
