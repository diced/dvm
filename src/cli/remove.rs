use std::{fs, path::Path};

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

  let version = fs::read_to_string(dvm_path::version_file(release_type)?)
    .expect("could not read version file: malformed installation detected");
  if verbose {
    info!("reading version file")
  }

  info!("removing version {}:{}", release_type, version);

  // remove all {release type} associated files
  fs::remove_dir_all(dvm_path::install_dir(release_type)?)
    .expect("error when removing data dirs");
  if verbose {
    info!("removed data dirs")
  }

  fs::remove_file(dvm_path::dvm_bin_dir()?.join(dvm_path::pkg_name(release_type)))
    .expect("error when removing bin file");
  if verbose {
    info!("removed bin file")
  }

  fs::remove_file(
    dvm_path::home_dir()?
      .join(".local")
      .join("share")
      .join("applications")
      .join(format!("{}.desktop", dvm_path::pkg_name(release_type))),
  )
    .expect("error when removing desktop file");
  if verbose {
    info!("removed desktop file")
  }

  fs::remove_file(
    dvm_path::home_dir()?
      .join(".local")
      .join("share")
      .join("icons")
      .join(format!("{}.png", dvm_path::pkg_name(release_type))),
  )
  .expect("error when removing icon");
  if verbose {
    info!("removed icon")
  }

  success!("removed version {}:{}", release_type, version);
  Ok(())
}
