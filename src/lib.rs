pub mod install;
pub mod remove;
pub mod show;
pub mod r#type;
pub mod update;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use colored::*;

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
