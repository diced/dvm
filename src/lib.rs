#![feature(format_args_nl)]

pub mod cli;
pub mod util;
pub mod r#type;
pub mod common;
pub mod completions;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use clap::{App, Arg};
use common::*;

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("{}{}", "info: ".white().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! success {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("\n\t{}{}\n", "success: ".green().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("{}{}", "error: ".red().bold(), std::format_args!($($arg)*));
    std::process::exit(1);
  })
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