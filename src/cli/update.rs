use std::{env, fs, path::Path};

use crate::{Res, error, info, r#type::Type, success, util::install_version};

pub async fn update(release_type: Type, verbose: bool) -> Res<()> {
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
    Type::STABLE => "Discord",
    Type::PTB => "DiscordPTB",
    Type::CANARY => "DiscordCanary",
    Type::DEVELOPMENT => "DiscordDevelopment",
  };

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  let (latest, version) = install_version(true, release_type.clone(), verbose, user).await?;

  success!(
    "updated {}:{} -> {}:{}",
    release_type, version, release_type, latest
  );

  Ok(())
}
