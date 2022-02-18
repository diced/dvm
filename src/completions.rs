use clap_generate::{
  generate,
  generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

use crate::error;

const BIN: &str = "dvm";

pub fn give_completions(shell: &str) {
  match shell {
    "bash" | "b" => return_bash(),
    "elvish" | "e" => return_elvish(),
    "fish" | "f" => return_fish(),
    "powershell" | "pwsh" | "ps" | "p" => return_powershell(),
    "zsh" | "z" => return_zsh(),
    _ => {
      error!("shell \"{}\" is not supported", shell);
    }
  };
}

pub fn return_bash() {
  let mut app = super::build_cli();

  generate::<Bash, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_zsh() {
  let mut app = super::build_cli();

  generate::<Zsh, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_elvish() {
  let mut app = super::build_cli();

  generate::<Elvish, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_fish() {
  let mut app = super::build_cli();

  generate::<Fish, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}

pub fn return_powershell() {
  let mut app = super::build_cli();

  generate::<PowerShell, _>(&mut app, BIN, &mut std::io::stdout());

  std::process::exit(0);
}
