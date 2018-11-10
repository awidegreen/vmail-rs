# vmail-rs

[![Build Status](https://travis-ci.com/awidegreen/vmail-rs.svg?branch=master)](https://travis-ci.com/awidegreen/vmail-rs)
[![license](https://img.shields.io/badge/license-ISC-brightgreen.svg)](https://www.isc.org/downloads/software-support-policy/isc-license/)

`vmail-rs` is a command line tool and libary for managing a mail-server database
based on the great [HowTo](https://thomas-leister.de/en/mailserver-debian-stretch) ([german version](https://thomas-leister.de/mailserver-debian-stretch/))
from [Thomas Leister](https://thomas-leister.de) written in Rust.

## Prerequisites

Make sure you have a working setup as described in the tutorial as `vmail-rs`
relies on the described database scheme (MySQL or MariaDB). That also includes
that `libmysqlclient-dev` on ubuntu or the counter-part for other
distributions is installed.

Further, as `vmail-rs` is written in Rust, you should have a working
rustup/cargo setup.


# Installation

vmail-rs contains the cli tools `vmail-cli`


## Install from github

```
cargo install --git https://github.com/awidegreen/vmail-rs
```

## Build from sources

Clone the repo and run

```
cargo install
```
or the release version

```
cargo install --release
```

# Usage

vmail-rs uses Rust's [dotenv](https://docs.rs/crate/dotenv/) crate to create
environment configuration from a `.env` file.

Create a `.env` in the current directory containing the `DATABASE_URL`
configuration parameter (be aware of the URI character encoding for the
password).

```
DATABASE_URL=mysql://vmail:vmailpassword@localhost
```
Use the command help to get started, and see which subcommands exists.

```shell
vmail-cli --help
```

NOTE: all subcommands can also be shortcut'd. vmail-cli will automatically defer
the correct command: `vmail-cli u s` equals `vmail-cli user show`

## Subcommand `domain`

Available subcommands are:

* add
* help
* remove
* show

## Subcommand `user`

As the name suggests, this subcommand is used to mange the users/accounts within
the database.  In order to add a new user, the associated domain need to exist.

Available subcommands are:

* add
* edit
* help
* password
* remove
* show

Use help for more information.


## Subcommand `alias`

In order to add a new alias, the associated user and domain need to exist.

Available subcommands are:

* add
* help
* remove
* show

Use help for more information.

# Misc

## Shell completions

For bash, move `shell/vmail-cli.bash` to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`.

For fish, move `shell/vmail-cli.fish` to `$HOME/.config/fish/completions/`.

For zsh, move `shell/_vmail-cli` to one of your `$fpath` directories.

For regenerating the shell completions, run `shell/gen_comp.sh` in the root of
the repository. The files in `shell/` will be updated accordingly. This will use
vmail-cli hidden `completions` subcommand.

# How to contribute

Create new issues if you find bugs or want to a new features. Pull requests are
very welcomed.

# License

Copyright (C) 2018 by Armin Widegreen

This is free software, licensed under the [ISC License](LICENSE).
