pub const VERSION: &str = "1.1.9";
pub const INSTALL_DESC: &str = "install the latest <type> of discord";
pub const INSTALL_ALIASES: &[&str] = &["i", "in", "get"];
pub const INSTALL_OPENASAR_DESC: &str = "install openasar for <type> of discord";
pub const INSTALL_OPENASAR_ALIASES: &[&str] = &["asar", "oa"];
pub const UPDATE_DESC: &str = "update to the latest <type> of discord";
pub const UPDATE_ALIASES: &[&str] = &["u", "up", "upgrade"];
pub const REMOVE_DESC: &str = "remove the installed <type> of discord";
pub const REMOVE_ALIASES: &[&str] = &["r", "rm", "d", "del", "un", "uninstall"];
pub const SHOW_DESC: &str = "show all installed versions";
pub const SHOW_ALIASES: &[&str] = &["s", "installed", "all", "a", "versions", "types"];
pub const COMP_DESC: &str = "get shell completions";
pub const COMP_ALIASES: &[&str] = &["c", "comp"];
pub const RUN_DESC: &str = "run discord with specific options";
pub const RUN_ALIASES: &[&str] =  &["r", "start"];

pub const POSSIBLE_SHELLS: &[&str] = &[
  "bash", "b",
  "elvish", "e",
  "fish", "f",
  "powershell", "pwsh", "ps", "p",
  "zsh", "z"
];

pub const POSSIBLE_VALUES: &[&str] = &[
  "stable",
  "discord-stable",
  "s",

  "canary",
  "discord-canary",
  "c",
  
  "ptb",
  "discord-ptb",
  "p",
  
  "development",
  "dev",
  "discord-development",
  "d",
];
