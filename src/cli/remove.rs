use std::{env, fs, path::Path};

use crate::{error, info, r#type::Type, success, Res};

pub async fn remove(release_type: Type, verbose: bool) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;

  let pascal_pkg = match release_type {
    Type::STABLE => "Discord",
    Type::PTB => "DiscordPTB",
    Type::CANARY => "DiscordCanary",
    Type::DEVELOPMENT => "DiscordDevelopment",
  };

  let pkg_name = match release_type {
    Type::STABLE => "discord",
    Type::PTB => "discord-ptb",
    Type::CANARY => "discord-canary",
    Type::DEVELOPMENT => "discord-development",
  };

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();
  if verbose {
    info!("checking if installation exists")
  }

  if !exists {
    error!("{} not installed", release_type);
  }

  let version = fs::read_to_string(format!("/home/{}/.dvm/{}/version", user, pascal_pkg))
    .expect("could not read version file: malformed installation detected");
  if verbose {
    info!("reading version file")
  }

  info!("removing version {}:{}", release_type, version);

  // remove all {release type} associated files
  fs::remove_dir_all(format!("/home/{}/.dvm/{}", user, pascal_pkg))
    .expect("error when removing data dirs");
  if verbose {
    info!("removed data dirs")
  }

  fs::remove_file(format!("/home/{}/.dvm/bin/{}", user, pkg_name))
    .expect("error when removing bin file");
  if verbose {
    info!("removed bin file")
  }

  fs::remove_file(format!(
    "/home/{}/.local/share/applications/{}.desktop",
    user, pkg_name
  ))
  .expect("error when removing desktop file");
  if verbose {
    info!("removed desktop file")
  }

  fs::remove_file(format!(
    "/home/{}/.local/share/icons/{}.png",
    user, pkg_name
  ))
  .expect("error when removing icon");
  if verbose {
    info!("removed icon")
  }

  success!("removed version {}:{}", release_type, version);
  Ok(())
}
