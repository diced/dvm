use std::{collections::HashMap, env, fs, path::Path};

use crate::{Res, error, info, r#type::Type, success};
use tokio::process::Command;

pub async fn install(release_type: Type) -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;

  let pkg_name = match release_type {
    Type::STABLE => "discord",
    Type::PTB => "discord-ptb",
    Type::CANARY => "discord-canary",
    Type::DEVELOPMENT => "discord-development"
  };

  let pascal_pkg = match release_type {
    Type::STABLE => "Discord",
    Type::PTB => "DiscordPTB",
    Type::CANARY => "DiscordCanary",
    Type::DEVELOPMENT => "DiscordDevelopment"
  };

  let dl_sub = match release_type {
    Type::STABLE => "dl",
    Type::PTB => "dl-ptb",
    Type::CANARY => "dl-canary",
    Type::DEVELOPMENT => "dl-development"
  };

  let exists = Path::new(&format!("/home/{}/.dvm/{}", user, pascal_pkg)).exists();

  if exists {
    error(format!("{} is already installed", release_type));
    std::process::exit(1);
  }

  // request api for latest version
  let res = reqwest::get(format!(
    "https://discordapp.com/api/v8/updates/{}?platform=linux",
    release_type
  ))
  .await?
  .json::<HashMap<String, String>>()
  .await?;

  // exit if the api doesn't return a name (latest version)
  let latest = match res.get("name") {
    Some(v) => v,
    None => std::process::exit(55)
  };
  info(format!("found latest version {}:{}", release_type, latest));

  // download tarball
  let tar_name = format!("{}-{}", pkg_name, latest);
  info(format!("downloading version {}:{}", release_type, latest));

  let tar_bytes = reqwest::get(format!(
    "https://{}.discordapp.net/apps/linux/{}/{}.tar.gz",
    dl_sub, latest, tar_name
  ))
  .await?
  .bytes()
  .await?;

  // write tar to /tmp
  let tmp_file = format!("/tmp/{}.tar.gz", tar_name);
  fs::write(&tmp_file, tar_bytes)?;

  // extract tar to .dvm
  Command::new("tar")
    .arg("xf")
    .arg(&tmp_file)
    .arg("-C")
    .arg(format!("/home/{}/.dvm/", user))
    .spawn()?
    .wait()
    .await?;
  info(format!(
    "extracting components to {}",
    format!("/home/{}/.dvm/{}", user, pascal_pkg)
  ));

  // change Exec= to dvm path from the desktop file
  Command::new("sed")
    .arg("-i")
    .arg(format!(
      "s#/usr/share/{}/{}#/home/{}/.dvm/bin/{}#",
      pkg_name, pascal_pkg, user, pkg_name
    ))
    .arg(format!(
      "/home/{}/.dvm/{}/{}.desktop",
      user, pascal_pkg, pkg_name
    ))
    .spawn()?
    .wait()
    .await?;

  // write a shell script to .dvm/bin to run discord
  let bin_path = format!("/home/{}/.dvm/bin/{}", user, pkg_name);
  fs::write(
    &bin_path,
    format!(
      "#!/bin/sh\n/home/{}/.dvm/{}/{} \"$@\"\n",
      user, pascal_pkg, pascal_pkg
    )
  )?;

  // make bin executable
  Command::new("chmod")
    .arg("+x")
    .arg(bin_path)
    .spawn()?
    .wait()
    .await?;

  // copy desktop file to .local/share/applications
  Command::new("install")
    .arg("-Dm644")
    .arg(format!(
      "/home/{}/.dvm/{}/{}.desktop",
      user, pascal_pkg, pkg_name
    ))
    .arg(format!("/home/{}/.local/share/applications", user))
    .spawn()?
    .wait()
    .await?;
  info("installing desktop file");

  // copy icon to .local/share/icons
  fs::create_dir_all(format!("/home/{}/.local/share/icons", user))?;
  Command::new("cp")
    .arg(format!("/home/{}/.dvm/{}/discord.png", user, pascal_pkg))
    .arg(format!(
      "/home/{}/.local/share/icons/{}.png",
      user, pkg_name
    ))
    .spawn()?
    .wait()
    .await?;
  info("installing icons");

  // create a file that contains the current version to use for updating
  fs::write(
    format!("/home/{}/.dvm/{}/version", user, pascal_pkg),
    latest
  )?;

  // remove tmp tar ball
  fs::remove_file(tmp_file)?;

  success(format!("installed {}:{}", pkg_name, latest));
  Ok(())
}
