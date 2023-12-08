use std::{env, fs, path::Path};

use crate::{branch::DiscordBranch, error, info, success, util::install_version, Res};

pub async fn install(release_type: DiscordBranch, verbose: bool, open_asar: bool) -> Res<()> {
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

  if exists {
    error!("{} is already installed", release_type);
  }

  let (latest, _) = install_version(false, release_type.clone(), verbose, user.clone()).await?;

  if open_asar {
    let asar_file = format!("/home/{}/.dvm/{}/resources/app.asar", user, pascal_pkg);

    fs::rename(&asar_file, format!("{}.bak", &asar_file))?;
    info!("renamed app.asar to app.asar.bak (if discord doesn't work after this, rename it back)");

    let res =
      reqwest::get("https://github.com/GooseMod/OpenAsar/releases/download/nightly/app.asar")
        .await?
        .bytes()
        .await?;

    fs::write(&asar_file, res)?;

    info!("downloaded openasar, if discord is open, restart it");
  }

  success!("installed {}:{}", release_type, latest);
  Ok(())
}
