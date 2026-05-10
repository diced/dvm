use std::{collections::HashMap, fs};

use tokio::process::Command;

use crate::{branch::DiscordBranch, error, info, path as dvm_path, Res};

fn get_version(branch: DiscordBranch) -> Res<String> {
  Ok(
    fs::read_to_string(dvm_path::version_file(branch)?)
      .expect("could not read version file: malformed installation detected")
      .replace("\n", ""),
  )
}

pub async fn install_version(
  update: bool,
  release_type: DiscordBranch,
  verbose: bool,
) -> Res<(String, String)> {
  let pkg_name = dvm_path::pkg_name(release_type);
  let pascal_pkg = dvm_path::pascal_pkg(release_type);

  let dl_sub = match release_type {
    DiscordBranch::STABLE => "dl",
    DiscordBranch::PTB => "dl-ptb",
    DiscordBranch::CANARY => "dl-canary",
    DiscordBranch::DEVELOPMENT => "dl-development",
  };

  // request api for latest version
  let res = reqwest::get(format!(
    "https://discordapp.com/api/v8/updates/{}?platform=linux",
    release_type
  ))
  .await?
  .json::<HashMap<String, String>>()
  .await?;
  if verbose {
    info!("requested api for latest version")
  }

  // exit if the api doesn't return a name (latest version)
  let latest = match res.get("name") {
    Some(v) => v,
    None => std::process::exit(1),
  };
  info!("found latest version {}:{}", release_type, latest);

  let mut version = String::new();
  if update {
    version = get_version(release_type)?;
    // check if the version is the same & clean file of \n's
    if verbose {
      info!("checking if existing version and latest match")
    }

    if version.eq(latest) {
      error!("you already have the latest version of {}", release_type);
    }

    // remove installed to make room for upgrade
    fs::remove_dir_all(dvm_path::install_dir(release_type)?)?;
    info!("removing old components");
  }

  // download tarball
  let tar_name = format!("{}-{}", pkg_name, latest);
  info!("downloading version {}:{}", release_type, latest);

  let tar_bytes = reqwest::get(format!(
    "https://{}.discordapp.net/apps/linux/{}/{}.tar.gz",
    dl_sub, latest, tar_name
  ))
  .await?
  .bytes()
  .await?;
  if verbose {
    info!("downloaded tarball")
  }

  // write tar to /tmp
  let tmp_file = format!("/tmp/{}.tar.gz", tar_name);
  fs::write(&tmp_file, tar_bytes)?;
  if verbose {
    info!("wrote tar to /tmp")
  }

  // extract tar to .dvm
  Command::new("tar")
    .arg("xf")
    .arg(&tmp_file)
    .arg("-C")
    .arg(dvm_path::dvm_dir()?)
    .spawn()?
    .wait()
    .await?;
  info!(
    "extracting components to {}",
    dvm_path::install_dir(release_type)?.display()
  );

  // newer versions of discord come with a updater bootstrap executable that can be run to dl
  let install_dir = dvm_path::install_dir(release_type)?;
  let bootstrap = install_dir.join("updater_bootstrap");
  if bootstrap.exists() {
    let channel = release_type.to_string();
    let output = Command::new(&bootstrap)
      .arg("--no-zenity")
      .arg(&install_dir)
      .arg(channel)
      .arg("https://updates.discord.com/")
      .output()
      .await?;

    if verbose {
      info!(
        "ran updater_bootstrap (status: {})",
        output.status.code().unwrap_or(-1)
      );
    }

    if !output.status.success() {
      info!("updater_bootstrap failed; continuing with extracted files");
    }
  }

  // patch desktop entry to point to dvm launcher bin
  let desktop_source = install_dir.join(format!("{}.desktop", pkg_name));

  if desktop_source.exists() {
    Command::new("sed")
      .arg("-i")
      .arg(format!(
        "s#/usr/bin/{}#{}/{}#",
        pkg_name,
        dvm_path::dvm_bin_dir()?.display(),
        pkg_name
      ))
      .arg(&desktop_source)
      .spawn()?
      .wait()
      .await?;
    if verbose {
      info!("changing bin locations in desktop entries")
    }
  }

  // write a shell script to .dvm/bin to run discord
  let bin_path = dvm_path::dvm_bin_dir()?.join(pkg_name);
  fs::write(
    &bin_path,
    format!(
      r#"#!/usr/bin/env bash

USER_FLAGS_FILE="$HOME/.dvm/{}-flags.conf"
if [[ -f $USER_FLAGS_FILE ]]; then
  USER_FLAGS="$(cat $USER_FLAGS_FILE | sed 's/#.*//')"
fi

INSTALL_DIR="{}"
BIN_NAME="{}"
APP_DIR="$(find "$INSTALL_DIR" -maxdepth 1 -type d -name 'app-*' 2>/dev/null | sort -V | tail -n 1)"

if [[ -n "$APP_DIR" && -x "$APP_DIR/$BIN_NAME" ]]; then
  exec "$APP_DIR/$BIN_NAME" "$@" $USER_FLAGS
fi

exec "$INSTALL_DIR/$BIN_NAME" "$@" $USER_FLAGS
"#,
      pkg_name,
      install_dir.display(),
      pascal_pkg
    ),
  )?;

  if verbose {
    info!("created executable bin")
  }

  // make bin executable
  Command::new("chmod")
    .arg("+x")
    .arg(&bin_path)
    .spawn()?
    .wait()
    .await?;
  if verbose {
    info!("allowed execution for bin")
  }

  // copy desktop file to .local/share/applications
  let local_apps_dir = dvm_path::home_dir()?.join(".local").join("share").join("applications");
  if desktop_source.exists() {
    fs::create_dir_all(&local_apps_dir)?;
    fs::copy(
      &desktop_source,
      local_apps_dir.join(format!("{}.desktop", pkg_name)),
    )?;
    info!("installing desktop file");
  }

  // copy icon to .local/share/icons
  let local_icons_dir = dvm_path::home_dir()?.join(".local").join("share").join("icons");
  fs::create_dir_all(&local_icons_dir)?;
  let icon_file = install_dir.join("discord.png");
  if icon_file.exists() {
    fs::copy(&icon_file, local_icons_dir.join(format!("{}.png", pkg_name)))?;
    info!("installing icons");
  }

  // create a file that contains the current version to use for updating
  fs::write(dvm_path::version_file(release_type)?, latest)?;
  if verbose {
    info!("created version file")
  }

  // remove tmp tar ball
  fs::remove_file(tmp_file)?;
  if verbose {
    info!("remove tmp tar ball")
  }

  Ok((latest.to_string(), version))
}
