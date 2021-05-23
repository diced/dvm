pub mod install;
pub mod remove;
pub mod update;
pub mod show;
pub mod r#type;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use termion::{
  color::{Fg, Green, Red},
  style::{Bold, Reset}
};

pub fn info(text: impl Into<String>) {
  println!("{}info:{} {}", Bold, Reset, text.into());
}

pub fn success(text: impl Into<String>) {
  println!("\n\t{}{}success:{} {}\n", Bold, Fg(Green), Reset, text.into());
}

pub fn error(text: impl Into<String>) {
  println!("{}{}error:{} {}", Bold, Fg(Red), Reset, text.into());
}
