# Git User Manager

reference [lexmin0412/gcm.git](https://github.com/lexmin0412/gcm.git)

## Install

```sh
cargo install --git https://github.com/yixiaojiu/git-user-manager
```

## Usage

```
Usage: gum <COMMAND>

Commands:
  add     add user config
  remove  use alias to remove a user config
  list    list all user config
  use     use alias to change git user config
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

For example:

```sh
# add a user
# alias: github
# name: bar
# email: foo@example.com
gum add github bar foo@example.com

# use github alias to change git user config
gum use github
```
