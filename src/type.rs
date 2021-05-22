use std::fmt;

#[derive(Debug)]
pub enum Type {
  STABLE,
  PTB,
  CANARY,
  DEVELOPMENT
}

impl Into<String> for Type {
  fn into(self) -> String {
    match self {
      Type::STABLE => "stable",
      Type::PTB => "ptb",
      Type::CANARY => "canary",
      Type::DEVELOPMENT => "development"
    }
    .to_string()
  }
}

impl Into<&str> for Type {
  fn into(self) -> &'static str {
    match self {
      Type::STABLE => "stable",
      Type::PTB => "ptb",
      Type::CANARY => "canary",
      Type::DEVELOPMENT => "development"
    }
  }
}

impl Into<Type> for String {
  fn into(self) -> Type {
    match self.as_str() {
      "stable" => Type::STABLE,
      "ptb" => Type::PTB,
      "canary" => Type::CANARY,
      "development" => Type::DEVELOPMENT,
      _ => Type::STABLE
    }
  }
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let r = match self {
      Type::STABLE => "stable",
      Type::PTB => "ptb",
      Type::CANARY => "canary",
      Type::DEVELOPMENT => "development"
    };

    write!(f, "{}", r)
  }
}
