# dvm (Discord Version Manager)
Allowing you to manage all of your discord versions. This was made as I was impatient to waiting for AUR maintainers (no offense 😂 we have lives) to update discord packages. This solves that problem: since you can update your version by just doing `dvm update <type>`. This utility is heavily inspired by the [`rustup` command line tool](https://rustup.rs)


# Usage
```
dvm 0.1.1

USAGE:
    dvm <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    install the latest <type> of discord
    remove     remove the installed <type> of discord
    show       show all installed versions
    update     update to the latest <type> of discord
```

# Installing
To install a specific version just type in
```
dvm install stable
```
This will do the following:
1. Download the latest stable tarball from discord
2. Extract it into $HOME/.dvm
3. Create a bin file that executes the executable
4. Copy desktop and icons to respectful folders

## Installing multiple versions at once
You can install multiple versions at once, they will be executed one after the other.
```
dvm install stable ptb canary development
```

# Removing
Removing installations is as easy as installing them
```
dvm remove stable
```
This will do the following:
1. Remove the $HOME/.dvm/<installation type>
2. Remove desktop entries and icons
3. Remove the bin file

# Update
Updating installations is as easy as installing them
```
dvm update stable
```
This will do the following:
1. Check if discord actually needs to be updated
2. Download the latest tarball
3. Remove the $HOME/.dvm/<installation type>
4. Do everything the installation does.