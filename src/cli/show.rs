use std::{
  collections::HashMap,
  fs::{self, DirEntry},
  path::PathBuf,
};

use colored::*;

use crate::{branch::DiscordBranch, path as dvm_path, Res};

// fn dirent(dirent: DirEntry) -> Res<(String, Type)> {
//   let rl_type = match dirent.file_name().to_str().unwrap() {
//     "Discord" => Type::STABLE,
//     "DiscordCanary" => Type::CANARY,
//     "DiscordPTB" => Type::PTB,
//     "DiscordDevelopment" => Type::DEVELOPMENT,
//     _ => Type::STABLE,
//   };
//   let version = fs::read_to_string(dirent.path().join("version"))?.replace("\n", "");

//   Ok((version, rl_type))
// }

fn dirent_verbose(dirent: DirEntry) -> Res<(String, DiscordBranch, PathBuf)> {
  let rl_type = match dirent.file_name().to_str().unwrap() {
    "Discord" => DiscordBranch::STABLE,
    "DiscordCanary" => DiscordBranch::CANARY,
    "DiscordPTB" => DiscordBranch::PTB,
    "DiscordDevelopment" => DiscordBranch::DEVELOPMENT,
    _ => DiscordBranch::STABLE,
  };
  let path = dirent.path();
  let version = fs::read_to_string(path.join("version"))?.replace("\n", "");

  Ok((version, rl_type, path))
}

async fn needs_update(version: String, release_type: DiscordBranch) -> Res<(bool, String)> {
  let res = reqwest::get(format!(
    "https://discordapp.com/api/v8/updates/{}?platform=linux",
    release_type
  ))
  .await?
  .json::<HashMap<String, String>>()
  .await?;

  let latest = match res.get("name") {
    Some(v) => v,
    None => std::process::exit(1),
  };

  Ok((version.ne(latest), latest.to_string()))
}

pub async fn show(verbose: bool, check: bool) -> Res<()> {
  fs::create_dir_all(dvm_path::dvm_bin_dir()?)?;

  let mut types = fs::read_dir(dvm_path::dvm_dir()?)?
    .map(|x| x.unwrap())
    .filter(|x| x.file_name() != "bin")
    .map(|x| dirent_verbose(x).unwrap());

  while let Some(x) = types.next() {
    if check {
      let ver = needs_update(x.0.clone(), x.1.clone()).await?;
      if verbose {
        println!(
          "{}:{} -> {}",
          x.1
            .to_string()
            .color(if ver.0 { "red" } else { "green" })
            .bold(),
          x.0.bold().white(),
          x.2.display()
        );
      } else {
        println!(
          "{}:{}",
          x.1
            .to_string()
            .color(if ver.0 { "red" } else { "green" })
            .bold(),
          x.0.bold().white()
        );
      }
    } else {
      if verbose {
        println!(
          "{}:{} -> {}",
          x.1.to_string().white().bold(),
          x.0.bold().white(),
          x.2.display()
        );
      } else {
        println!("{}:{}", x.1.to_string().white().bold(), x.0.bold().white());
      }
    }
  }

  Ok(())
}
