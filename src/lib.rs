pub mod branch;
pub mod cli;
pub mod common;
pub mod util;

pub type Res<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
