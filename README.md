# dvm (Discord Version Manager)
Allowing you to manage all of your discord versions. This was made as I was impatient to waiting for AUR maintainers (no offense 😂 we have lives) to update discord packages. This solves that problem: since you can update your version by just doing `dvm update <type>`. This utility is heavily inspired by the [`rustup` command line tool](https://rustup.rs)

# Installing
## <img src="https://www.vectorlogo.zone/logos/archlinux/archlinux-icon.svg" height="20"/> Arch Linux
You can install the binary version of dvm through the [`dvm` AUR package](https://aur.archlinux.org/packages/dvm)

Or you can compile from source using the [`dvm-git` AUR package](https://aur.archlinux.org/packages/dvm)

## Other Distros
At the moment I haven't bothered making this available on other package managers as the issue dvm wants to solve usually isn't present in those other distros.
### Compiling from Source
If you prefer compiling from source you can do the following

#### Prequisites
* Rust Nightly ([rustup](https://rustup.rs) or your distro might provide a `rustup` package, then `rustup install nightly`)
```sh
git clone https://github.com/diced/dvm
cd dvm
cargo +nightly build # make sure you have rust nightly installed like stated above
```

### Using prebuilt binary
If you don't want to bother compiling from source then you may use the precompiled binary that I provide on the [releases page](https://github.com/diced/dvm/releases)

*Note: The precompiled binary is not a statically compiled binary, so if it happens to error, run `ldd dvm-x86_64-unknown-linux-gnu` and it will show the libraries it needs, then install them.*

```sh
wget https://github.com/diced/dvm/releases/download/<version>/dvm-x86_64-unknown-linux-gnu
chmod +x dvm
./dvm # you can move it into /usr/bin or move it into somewhere then add it to $PATH
```
# Usage
```
dvm 1.1.8

USAGE:
    dvm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    completions    get shell completions
    help           Prints this message or the help of the given subcommand(s)
    install        install the latest <type> of discord
    remove         remove the installed <type> of discord
    run            run discord with specific options
    show           show all installed versions
    update         update to the latest <type> of discord
```

# Installing Discord
To install a specific version just type in
```sh
dvm install stable
```
This will do the following:
1. Download the latest stable tarball from discord
2. Extract it into $HOME/.dvm
3. Create a bin file that executes the executable
4. Copy desktop and icons to respectful folders

## Installing multiple versions at once
You can install multiple versions at once, they will be executed one after the other.
```sh
dvm install stable ptb canary development
```

# Removing
Removing installations is as easy as installing them
```sh
dvm remove stable
```
This will do the following:
1. Remove the $HOME/.dvm/<installation type>
2. Remove desktop entries and icons
3. Remove the bin file

# Update
Updating installations is as easy as installing them
```sh
dvm update stable
```
This will do the following:
1. Check if discord actually needs to be updated
2. Download the latest tarball
3. Remove the $HOME/.dvm/<installation type>
4. Do everything the installation does.

# Show
This will show all installations that are currently installed, the `--verbose, -v` flag will show the path it's installed to, and the `--check, -c` flag will check if your installations are up-to-date (red = outdated, green = up to date)
```sh
dvm show
dvm show -v
dvm show -c
dvm show -vc
```
```sh
# no flag
canary:0.0.133
development:0.0.198
stable:0.0.17
# -v
canary:0.0.133 -> /home/diced/.dvm/DiscordCanary
development:0.0.198 -> /home/diced/.dvm/DiscordDevelopment
stable:0.0.17 -> /home/diced/.dvm/Discord
```

# Run
You can run discord via command line with extra flags
```sh
dvm run canary <extra args>
# for example
dvm run canary --idk-some-chromium-flag-or-something
```

# Completions
Get shell completions for your shell of choice
```sh
dvm completions zsh
```
