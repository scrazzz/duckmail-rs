# duckmail-rs
A simple CLI program to manage duckduckgo emails.

> [!NOTE]
> *I don't expect anyone to use this program, so there are no instructions on how to obtain your own access token.*

## Installation
```
cargo install --git https://github.com/scrazzz/duckmail-rs.git
```

## Usage
```
$ duckmail --help
Create and manage duckduckgo email addresses

Usage: duckmail <COMMAND>

Commands:
  token   Sets the access token in the config file
  new     Creates a new email address. Make sure to set the access token first
  add     Adds an email address to the config file. Optionally, you can add a note to the email address
  remove  Removes an email address from the config file
  show    Shows all the email addresses in the config file
  nuke    Removes all email addresses from the config file. Use with caution
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Learned
What I have observed/learned from making this program. 

- Utilized `cargo clippy` and `cargo fmt`
- Learned to make CLI subcommands using `clap`
- Learned to use `anyhow` and `prettytable-rs`
- Learned to use rust `mod`ules
- Learned to serialize and deserialize json <=> struct
- Learned the difference between `pub(crate) fn` and `pub fn`
