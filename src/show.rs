use std::{env, fs::{self, DirEntry}};

use crate::{Res, r#type::Type};

pub fn dirent(dirent: DirEntry) -> Res<(String, Type)> {
  let rl_type = match dirent.file_name().to_str().unwrap() {
    "Discord" => Type::STABLE,
    "DiscordCanary" => Type::CANARY,
    "DiscordPtb" => Type::PTB,
    "DiscordDevelopment" => Type::DEVELOPMENT,
    _ => Type::STABLE
  };
  let version = fs::read_to_string(dirent.path().join("version"))?.replace("\n", "");

  Ok((version, rl_type))
}

pub async fn show() -> Res<()> {
  // create user var & create .dvm dirs
  let user = env::var("USER")?;
  fs::create_dir_all(format!("/home/{}/.dvm/bin", user))?;

  let types = fs::read_dir(format!("/home/{}/.dvm", user))?
    .map(|x| x.unwrap())
    .filter(|x| x.file_name() != "bin")
    .map(|x| dirent(x).unwrap())
    .map(|x| format!("{}:{}", x.1, x.0))
    .collect::<Vec<String>>()
    .join(", ");

  println!("{}", types);

  Ok(())
}
