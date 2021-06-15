use std::{ env, fs, path::Path};

use crate::{error, info, r#type::Type, success, Res, util::install_version};

pub async fn install(release_type: Type, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;
  if verbose {
    info("created .dvm dir")
  }

  let pascal_pkg = match release_type {
    Type::STABLE => "Discord",
    Type::PTB => "DiscordPTB",
    Type::CANARY => "DiscordCanary",
    Type::DEVELOPMENT => "DiscordDevelopment",
  };

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if exists {
    error(format!("{} is already installed", release_type));
    std::process::exit(1);
  }

  let (latest, _) = install_version(false, release_type.clone(), verbose, user).await?;

  success(format!("installed {}:{}", release_type, latest));
  Ok(())
}
