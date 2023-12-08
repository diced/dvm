#[cfg(not(target_os = "linux"))]
compile_error!("can only be compiled on linux ;)");

use clap::{
  builder::{
    styling::{AnsiColor, Effects},
    Styles,
  },
  CommandFactory, Parser,
};
use clap_complete::{generate, Shell};
use dvm::{
  branch::DiscordBranch,
  cli::{install, install_openasar, remove, run, show, update},
  error, Res,
};

fn styles() -> Styles {
  Styles::styled()
    .header(AnsiColor::White.on_default() | Effects::BOLD)
    .usage(AnsiColor::White.on_default() | Effects::BOLD)
    .literal(AnsiColor::BrightBlue.on_default())
    .placeholder(AnsiColor::BrightGreen.on_default())
}

#[derive(Parser, Debug, Clone)]
#[clap(
  version = env!("CARGO_PKG_VERSION"),
  name=env!("CARGO_PKG_NAME"),
  bin_name=env!("CARGO_PKG_NAME"),
  author=env!("CARGO_PKG_AUTHORS"),
  about=env!("CARGO_PKG_DESCRIPTION"),
  styles=styles(),
)]
struct Opts {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Parser, Debug, Clone)]
enum Command {
  #[clap(about = "Install the latest <type> of discord")]
  Install(InstallOption),

  #[clap(about = "Install openasar for <type> of discord")]
  InstallOpenAsar(InstallOpenAsarOption),

  #[clap(about = "Update to the latest <type> of discord")]
  Update(UpdateOption),

  #[clap(about = "Remove the installed <type> of discord")]
  Remove(RemoveOption),

  #[clap(about = "Show all installed versions")]
  List(ListOption),

  #[clap(about = "Get shell completions")]
  Completions(CompletionsOption),

  #[clap(about = "Run discord with specific options")]
  Run(RunOptions),
}

#[derive(Parser, Debug, Clone)]
struct InstallOption {
  r#type: Vec<DiscordBranch>,

  #[clap(short, long)]
  verbose: bool,

  #[clap(short, long)]
  open_asar: bool,
}

#[derive(Parser, Debug, Clone)]
struct InstallOpenAsarOption {
  r#type: Vec<DiscordBranch>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Parser, Debug, Clone)]
struct UpdateOption {
  r#type: Vec<DiscordBranch>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Parser, Debug, Clone)]
struct RemoveOption {
  r#type: Vec<DiscordBranch>,

  #[clap(short, long)]
  verbose: bool,
}

#[derive(Parser, Debug, Clone)]
struct ListOption {
  #[clap(short, long)]
  verbose: bool,

  #[clap(short, long)]
  check: bool,
}

#[derive(Parser, Debug, Clone)]
struct CompletionsOption {
  shell: Shell,
}

#[derive(Parser, Debug, Clone)]
struct RunOptions {
  #[clap(short, long)]
  verbose: bool,

  r#type: DiscordBranch,

  #[clap(last = true)]
  args: Vec<String>,
}

fn check_type_len(types: &Vec<DiscordBranch>) -> Res<()> {
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
        install(r#type, opt.verbose, opt.open_asar).await?
      }
    }
    Command::InstallOpenAsar(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        install_openasar(r#type, opt.verbose).await?
      }
    }

    Command::Update(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        update(r#type, opt.verbose).await?
      }
    }
    Command::Remove(opt) => {
      check_type_len(&opt.r#type)?;

      for r#type in opt.r#type {
        remove(r#type, opt.verbose).await?
      }
    }
    Command::List(opt) => show(opt.verbose, opt.check).await?,
    Command::Completions(opt) => {
      let mut cmd = Opts::command_for_update();
      let name = cmd.get_name().to_string();
      generate(opt.shell, &mut cmd, name, &mut std::io::stdout())
    }
    Command::Run(opt) => run(opt.r#type, opt.args.clone(), opt.verbose).await?,
  })
}
