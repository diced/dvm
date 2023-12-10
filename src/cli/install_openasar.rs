use std::{env, fs, path::Path};

use crate::{branch::DiscordBranch, error, info, success, Res};

pub async fn install_openasar(release_type: DiscordBranch, verbose: bool) -> Res<()> {
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

  let exists = Path::new(&format!("/home/{}/.dvm/{}", &user, &pascal_pkg)).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  let asar_file = format!("/home/{}/.dvm/{}/resources/app.asar", user, pascal_pkg);

  fs::rename(&asar_file, format!("{}.bak", &asar_file))?;
  info!("renamed app.asar to app.asar.bak (if discord doesn't work after this, rename it back)");

  let res = reqwest::get("https://github.com/GooseMod/OpenAsar/releases/download/nightly/app.asar")
    .await?
    .bytes()
    .await?;

  fs::write(&asar_file, res)?;

  success!("installed openasar, if discord is open, restart it");

  Ok(())
}
