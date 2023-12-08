use std::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone, Copy)]
pub enum DiscordBranch {
  STABLE,
  PTB,
  CANARY,
  DEVELOPMENT,
}

impl fmt::Display for DiscordBranch {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_possible_value().unwrap().get_name())
  }
}

impl clap::ValueEnum for DiscordBranch {
  fn value_variants<'a>() -> &'a [Self] {
    &[
      DiscordBranch::STABLE,
      DiscordBranch::CANARY,
      DiscordBranch::PTB,
      DiscordBranch::DEVELOPMENT,
    ]
  }

  fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
    match self {
      DiscordBranch::STABLE => Some(clap::builder::PossibleValue::new("stable")),
      DiscordBranch::CANARY => Some(clap::builder::PossibleValue::new("canary")),
      DiscordBranch::PTB => Some(clap::builder::PossibleValue::new("ptb")),
      DiscordBranch::DEVELOPMENT => Some(clap::builder::PossibleValue::new("development")),
    }
  }
}
