use std::{
  fs,
  io::ErrorKind,
  path::Path,
};

use crate::{branch::DiscordBranch, error, info, path as dvm_path, success, Res};

pub async fn remove(release_type: DiscordBranch, verbose: bool) -> Res<()> {
  fs::create_dir_all(dvm_path::dvm_bin_dir()?)?;

  let exists = Path::new(&dvm_path::install_dir(release_type)?).exists();
  if verbose {
    info!("checking if installation exists")
  }

  if !exists {
    error!("{} not installed", release_type);
  }

  let version = fs::read_to_string(dvm_path::version_file(release_type)?)?;
  if verbose {
    info!("reading version file")
  }

  info!("removing version {}:{}", release_type, version);

  // remove all {release type} associated files
  if let Err(e) = fs::remove_dir_all(dvm_path::install_dir(release_type)?) {
    if e.kind() != ErrorKind::NotFound {
      return Err(e.into());
    }
  }
  if verbose {
    info!("removed data dirs")
  }

  let bin_file = dvm_path::dvm_bin_dir()?.join(dvm_path::pkg_name(release_type));
  if let Err(e) = fs::remove_file(&bin_file) {
    if e.kind() != ErrorKind::NotFound {
      return Err(e.into());
    }
  }
  if verbose {
    info!("removed bin file")
  }

  let desktop_file = dvm_path::home_dir()?
    .join(".local")
    .join("share")
    .join("applications")
    .join(format!("{}.desktop", dvm_path::pkg_name(release_type)));
  if let Err(e) = fs::remove_file(&desktop_file) {
    if e.kind() != ErrorKind::NotFound {
      return Err(e.into());
    }
  }
  if verbose {
    info!("removed desktop file")
  }

  let icon_file = dvm_path::home_dir()?
    .join(".local")
    .join("share")
    .join("icons")
    .join(format!("{}.png", dvm_path::pkg_name(release_type)));
  if let Err(e) = fs::remove_file(&icon_file) {
    if e.kind() != ErrorKind::NotFound {
      return Err(e.into());
    }
  }
  if verbose {
    info!("removed icon")
  }

  success!("removed version {}:{}", release_type, version);
  Ok(())
}
