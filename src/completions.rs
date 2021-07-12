use clap_generate::{generate, generators::{Bash, Elvish, Fish, PowerShell, Zsh}};

use crate::error; 

const BIN: &str = "dvm";

pub fn give_completions(shell: &str) {
  match shell {
    "bash" | "b" => return_bash(),
    "elvish" | "e" => return_bash(),
    "fish" | "f" => return_bash(),
    "powershell" | "pwsh" | "ps" | "p" => return_bash(),
    "zsh" | "z" => return_bash(),
    _ => {
      error(format!("shell \"{}\" is not supported", shell));
      std::process::exit(1);
    }
  };
}

pub fn return_bash() -> Bash {
  let mut app = super::build_cli();

  generate::<Bash, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_zsh() -> Zsh {
  let mut app = super::build_cli();

  generate::<Zsh, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_elvish() -> Elvish {
  let mut app = super::build_cli();

  generate::<Elvish, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_fish() -> Fish {
  let mut app = super::build_cli();

  generate::<Fish, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_powershell() -> PowerShell {
  let mut app = super::build_cli();

  generate::<PowerShell, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}