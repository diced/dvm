use std::{fs, path::Path};

use crate::{
  branch::DiscordBranch,
  error, info,
  path as dvm_path,
  success,
  util::install_version,
  Res,
};

pub async fn update(release_type: DiscordBranch, verbose: bool) -> Res<()> {
  fs::create_dir_all(dvm_path::dvm_bin_dir()?)?;
  if verbose {
    info!("created .dvm dir")
  }

  let exists = Path::new(&dvm_path::install_dir(release_type)?).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  let (latest, version) = install_version(true, release_type, verbose).await?;

  success!(
    "updated {}:{} -> {}:{}",
    release_type,
    version,
    release_type,
    latest
  );

  Ok(())
}
