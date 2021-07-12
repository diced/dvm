pub mod cli;
pub mod util;
pub mod r#type;
pub mod common;
pub mod completions;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use clap::{App, Arg};
use colored::*;
use common::*;

pub fn info(text: impl Into<String>) {
  println!("{}{}", "info: ".white().bold(), text.into());
}

pub fn success(text: impl Into<String>) {
  println!(
    "\n\t{}{}\n",
    "success: ".green().bold(),
    text.into()
  );
}

pub fn error(text: impl Into<String>) {
  println!("{}{}", "error: ".red().bold(), text.into());
}

pub fn build_cli() -> App<'static> {
  App::new("dvm")
    .version("1.1.4")
    .subcommand(
      App::new("install")
      .about(INSTALL_DESC)
      .aliases(INSTALL_ALIASES)
      .arg(Arg::new("verbose").short('v').long("verbose").about("verbosity"))
      .arg(Arg::new("type").possible_values(POSSIBLE_VALUES))
    )
    .subcommand(
      App::new("update")
      .about(UPDATE_DESC)
      .aliases(UPDATE_ALIASES)
      .arg(Arg::new("type").possible_values(POSSIBLE_VALUES))
    )
    .subcommand(
      App::new("show")
      .about(SHOW_DESC)
      .aliases(SHOW_ALIASES)
      .arg(Arg::new("type").possible_values(POSSIBLE_VALUES))
    )
    .subcommand(
      App::new("remove")
      .about(REMOVE_DESC)
      .aliases(REMOVE_ALIASES)
      .arg(Arg::new("type").possible_values(POSSIBLE_VALUES))
    )
    .subcommand(
      App::new("completions")
      .about(COMP_DESC)
      .aliases(COMP_ALIASES)
      .arg(Arg::new("type").possible_values(POSSIBLE_SHELLS))
    )
}