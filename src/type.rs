use std::fmt;

#[derive(Debug, Clone)]
pub enum Type {
  STABLE,
  PTB,
  CANARY,
  DEVELOPMENT,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let r = match self {
      Type::STABLE => "stable",
      Type::PTB => "ptb",
      Type::CANARY => "canary",
      Type::DEVELOPMENT => "development",
    };

    write!(f, "{}", r)
  }
}
