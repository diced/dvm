use std::{env, fs, path::Path};

use tokio::process::Command;

use crate::{branch::DiscordBranch, error, info, Res};

pub async fn run(release_type: DiscordBranch, args: Vec<String>, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;

  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;
  if verbose {
    info!("created .dvm dir")
  }

  let pascal_pkg = match release_type {
    DiscordBranch::STABLE => "Discord",
    DiscordBranch::PTB => "DiscordPTB",
    DiscordBranch::CANARY => "DiscordCanary",
    DiscordBranch::DEVELOPMENT => "DiscordDevelopment",
  };

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  Command::new(format!("/home/{}/.dvm/{}/{}", user, pascal_pkg, pascal_pkg))
    .args(&args)
    .spawn()?
    .wait_with_output()
    .await?;

  Ok(())
}
