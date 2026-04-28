use std::{fs, path::Path};

use crate::{
  branch::DiscordBranch,
  error, info,
  path as dvm_path,
  success,
  util::install_version,
  Res,
};

pub async fn install(release_type: DiscordBranch, verbose: bool, open_asar: bool) -> Res<()> {
  fs::create_dir_all(dvm_path::dvm_bin_dir()?)?;
  if verbose {
    info!("created .dvm dir")
  }

  let exists = Path::new(&dvm_path::install_dir(release_type)?).exists();

  if exists {
    error!("{} is already installed", release_type);
  }

  let (latest, _) = install_version(false, release_type, verbose).await?;

  if open_asar {
    let asar_file = dvm_path::app_dir(release_type)?
      .join("resources")
      .join("app.asar");

    let asar_bak = asar_file
      .file_name()
      .map(|f| {
        let mut s = f.to_os_string();
        s.push(".bak");
        asar_file.with_file_name(s)
      })
      .unwrap_or_else(|| asar_file.with_file_name("app.asar.bak"));
    fs::rename(&asar_file, asar_bak)?;
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
