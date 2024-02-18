use std::io::Write;

use crate::config::db::Database;

mod config;
mod network;

use clap::Parser;
use tabwriter::TabWriter;

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
            let email = configdb.create_email(args.note.unwrap_or_default())?;
            println!("[*] Created new email: {}", email)
        }
        DuckMailCli::Add(args) => {
            configdb.add_email(&args.email, args.note.unwrap_or_default())?;
            println!("[*] Added {} to config", args.email)
        }
        DuckMailCli::Remove(args) => {
            if configdb.remove_email(&args.email)? {
                println!("[*] Removed {} from config", args.email)
            } else {
                println!("[!] {} not found in config", args.email)
            }
        }
        DuckMailCli::Show => {
            let emails = configdb.return_emails()?;
            let mut tw = TabWriter::new(vec![]);
            let mut fmt = String::from("EMAIL\tNOTE\n");
            for (email, note) in emails {
                fmt.push_str(&format!("{}\t{}\n", email, note));
            }
            tw.write_all(fmt.as_bytes())?;
            tw.flush()?;
            println!("{}", String::from_utf8(tw.into_inner()?)?);
        }
        DuckMailCli::Token(args) => {
            println!(
                "[*] Token ({}) added to config file\n\
            [!] WARNING: If this token is leaked there is no way revoke/invalidate it!",
                args.token
            );
            configdb.set_token(args.token)?;
        }
        DuckMailCli::Nuke => {
            let emails = configdb.return_emails()?;
            emails.iter().for_each(|(email, _)| {
                configdb.remove_email(email).unwrap();
                println!("[*] Removing email: {}", email);
            });
        }
    }
    Ok(())
}
