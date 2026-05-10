use std::{
  env,
  fs,
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

pub fn app_dir(branch: DiscordBranch) -> Res<PathBuf> {
  let install = install_dir(branch)?;
  let direct = install.join("resources").join("app.asar");
  if direct.exists() {
    return Ok(install);
  }

  let mut best_match: Option<(Vec<u32>, PathBuf)> = None;
  for entry in fs::read_dir(&install)? {
    let entry = entry?;
    let path = entry.path();
    if !path.is_dir() {
      continue;
    }

    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
      continue;
    };
    let Some(version) = name.strip_prefix("app-") else {
      continue;
    };

    let candidate = path.join("resources").join("app.asar");
    if !candidate.exists() {
      continue;
    }

    let parsed = version
      .split('.')
      .map(|part| part.parse::<u32>().unwrap_or(0))
      .collect::<Vec<_>>();

    match &best_match {
      Some((best_version, _)) if &parsed <= best_version => {}
      _ => best_match = Some((parsed, path)),
    }
  }

  if let Some((_, path)) = best_match {
    return Ok(path);
  }

  Ok(install)
}
