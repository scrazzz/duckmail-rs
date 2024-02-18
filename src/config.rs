pub mod data {
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, hash::Hash};

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
    pub struct Email(pub String);

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Note(pub String);

    #[derive(Default, Serialize, Deserialize, Debug)]
    pub struct Config {
        pub access_token: String,
        pub emails: HashMap<Email, Note>,
    }
}

pub mod db {
    use anyhow::Context;
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::data::{Config, Email, Note};
    use crate::network;

    pub struct Database {
        path: PathBuf,
    }

    impl Database {
        pub fn new(filename: &str) -> anyhow::Result<Self> {
            let path = dirs::config_dir().unwrap().join(filename);
            if path.exists() {
                // println!("[DEBUG] Config FOUND at: {:?}", path);
                return Ok(Database { path });
            }
            let file = File::create(&path)?;
            // Initialize
            serde_json::to_writer(file, &Config::default())?;
            // println!("[DEBUG] config CREATED at: {:?}", path);
            Ok(Database { path })
        }

        pub fn load_config(&self) -> anyhow::Result<Config> {
            let file = File::open(&self.path)?;
            let reader = BufReader::new(file);
            let config_data = serde_json::from_reader(reader)?;
            Ok(config_data)
        }

        pub fn write_config(&self, data: &Config) -> anyhow::Result<()> {
            let writer = File::create(&self.path)?;
            serde_json::to_writer(writer, &data)
                .with_context(|| "Failed to write new config".to_string())
        }

        pub fn set_token(&self, token: String) -> anyhow::Result<()> {
            let mut config_data = self.load_config()?;
            config_data.access_token = token;
            self.write_config(&config_data)?;
            Ok(())
        }

        pub fn create_email(&self, note: String) -> anyhow::Result<String> {
            let config_data = self.load_config()?;
            let token = config_data.access_token;
            if token.is_empty() {
                anyhow::bail!("No access token found. Please run `duckmail token <token>` to set the access token");
            }
            let created_email = network::create_email(token)?;
            self.add_email(&created_email, note)?;
            Ok(created_email + "@duck.com")
        }

        pub fn add_email(&self, email: &String, note: String) -> anyhow::Result<bool> {
            let email = if email.contains("@duck.com") {
                Email(email.to_string())
            } else {
                Email(email.to_string() + "@duck.com")
            };
            let note = Note(note);
            let mut config_data = self.load_config()?;
            config_data.emails.entry(email).or_insert(note);
            self.write_config(&config_data)?;
            // println!("[DEBUG] Updated config.data.emails: {:?}", config_data);
            Ok(true)
        }

        pub fn remove_email(&self, email: &String) -> anyhow::Result<bool> {
            let mut config_data = self.load_config()?;
            if config_data
                .emails
                .remove(&Email(email.to_string()))
                .is_some()
            {
                self.write_config(&config_data)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }

        pub fn return_emails(&self) -> anyhow::Result<Vec<(String, String)>> {
            let config = self.load_config()?;
            let emails = config
                .emails
                .iter()
                .map(|(email, note)| (email.0.clone(), note.0.clone()))
                .collect();
            Ok(emails)
        }
    }
}
