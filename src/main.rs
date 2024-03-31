use crate::config::db::Database;

mod config;
mod network;
mod utils;

use clap::Parser;
use prettytable::{format, row, Table};

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum DuckMailCli {
    /// Sets the access token in the config file
    Token(TokenArg),
    /// Creates a new email address. Make sure to set the access token first
    New(NewArg),
    /// Adds an email address to the config file. Optionally, you can add a note to the email address
    Add(AddEmailArgs),
    /// Removes an email address from the config file
    Remove(RemoveEmailArg),
    /// Shows all the email addresses in the config file
    Show,
    /// Removes all email addresses from the config file. Use with caution.
    Nuke,
}

#[derive(clap::Args)]
struct TokenArg {
    /// The access token to add to the config file
    token: String,
}

#[derive(clap::Args)]
struct NewArg {
    /// An optional note to add to the email address
    note: Option<String>,
}

#[derive(clap::Args)]
struct AddEmailArgs {
    /// The email address to add to the config file
    email: String,
    ///  An optional note to add to the email address
    note: Option<String>,
}

#[derive(clap::Args)]
struct RemoveEmailArg {
    /// The email address to remove from the config file
    email: String,
}

fn main() -> anyhow::Result<()> {
    let configdb = Database::new("duckemail.config.json")?;
    let args = DuckMailCli::parse();
    match args {
        DuckMailCli::New(args) => {
            let email = configdb.create_new_email(args.note.unwrap_or_default().as_str())?;
            println!("[*] Created new email: {}", email)
        }
        DuckMailCli::Add(args) => {
            let new_note = args.note.as_deref().unwrap_or_default();
            let is_email_added = configdb.add_email(&args.email, new_note)?;
            if !is_email_added {
                println!("[!] {} already exists", &args.email)
            } else if new_note.is_empty() {
                println!("[*] Added {} to database", &args.email)
            } else {
                println!(
                    "[*] Added {} to database with note: \"{}\"",
                    &args.email, new_note
                )
            }
        }
        DuckMailCli::Remove(args) => {
            if configdb.remove_email(&args.email)? {
                println!("[*] Removed {} from database", &args.email)
            } else {
                println!("[!] {} not found in database", &args.email)
            }
        }
        DuckMailCli::Show => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_BOX_CHARS);
            table.add_row(row![bBYc => "ID", "EMAIL", "NOTE"]);
            let emails = configdb.return_emails()?;
            emails.iter().enumerate().for_each(|(idx, (email, note))| {
                table.add_row(row![idx + 1, email, note]);
            });
            table.printstd()
        }
        DuckMailCli::Token(args) => {
            println!(
                "[*] Token ({}) added to config file\n\
            [!] WARNING: If this token is leaked there is no way revoke/invalidate it!",
                args.token
            );
            configdb.set_token(args.token)?
        }
        DuckMailCli::Nuke => {
            let emails = configdb.return_emails()?;
            emails.iter().for_each(|(email, _)| {
                configdb.remove_email(email).unwrap();
                println!("[*] Removing email: {}", email)
            })
        }
    }
    Ok(())
}
