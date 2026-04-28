use std::{fs, path::Path};

use tokio::process::Command;

use crate::{branch::DiscordBranch, error, info, path as dvm_path, Res};

pub async fn run(release_type: DiscordBranch, args: Vec<String>, verbose: bool) -> Res<()> {
  fs::create_dir_all(dvm_path::dvm_bin_dir()?)?;
  if verbose {
    info!("created .dvm dir")
  }

  let exists = Path::new(&dvm_path::install_dir(release_type)?).exists();

  if !exists {
    error!("{} is not installed", release_type);
  }

  Command::new(dvm_path::install_dir(release_type)?.join(dvm_path::pkg_name(release_type)))
    .args(&args)
    .spawn()?
    .wait_with_output()
    .await?;

  Ok(())
}
