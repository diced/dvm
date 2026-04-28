use std::{
  env,
  path::PathBuf,
};

use crate::{branch::DiscordBranch, Res};

pub fn home_dir() -> Res<PathBuf> {
  if let Ok(home) = env::var("HOME") {
    if !home.is_empty() {
      return Ok(PathBuf::from(home));
    }
  }

  let user = env::var("USER")?;
  Ok(PathBuf::from("/home").join(user))
}

pub fn dvm_dir() -> Res<PathBuf> {
  Ok(home_dir()?.join(".dvm"))
}

pub fn dvm_bin_dir() -> Res<PathBuf> {
  Ok(dvm_dir()?.join("bin"))
}

pub fn pkg_name(branch: DiscordBranch) -> &'static str {
  match branch {
    DiscordBranch::STABLE => "discord",
    DiscordBranch::PTB => "discord-ptb",
    DiscordBranch::CANARY => "discord-canary",
    DiscordBranch::DEVELOPMENT => "discord-development",
  }
}

pub fn pascal_pkg(branch: DiscordBranch) -> &'static str {
  match branch {
    DiscordBranch::STABLE => "Discord",
    DiscordBranch::PTB => "DiscordPTB",
    DiscordBranch::CANARY => "DiscordCanary",
    DiscordBranch::DEVELOPMENT => "DiscordDevelopment",
  }
}

pub fn install_dir(branch: DiscordBranch) -> Res<PathBuf> {
  Ok(dvm_dir()?.join(pascal_pkg(branch)))
}

pub fn version_file(branch: DiscordBranch) -> Res<PathBuf> {
  Ok(install_dir(branch)?.join("version"))
}
