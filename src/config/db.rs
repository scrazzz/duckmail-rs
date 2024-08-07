use anyhow::Context;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::config::data::{Config, Email, Note};
use crate::{network, utils};

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
        // Initialize config file with default values
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

    pub fn create_new_email(&self, note: &str) -> anyhow::Result<String> {
        let config_data = self.load_config()?;
        let token = config_data.access_token;
        if token.is_empty() {
            anyhow::bail!("No access token found. Please run `duckmail token <access_token>` to set the access token");
        }
        let created_email = network::create_email(token)?;
        self.add_email(&created_email, note)?;
        Ok(utils::format_email(&created_email))
    }

    pub fn add_email(&self, email: &str, note: &str) -> anyhow::Result<bool> {
        let email = Email(utils::format_email(email));
        let note = Note(note.to_string());
        let mut config_data = self.load_config()?;

        // Case 1: Email exists and new note to add is not empty
        if config_data.emails.contains_key(&email) && !note.0.is_empty() {
            config_data.emails.insert(email, note);
            self.write_config(&config_data)?;
            Ok(true)
        }
        // Case 2: Email exists and new note to add is empty
        else if config_data.emails.contains_key(&email) && note.0.is_empty() {
            Ok(false)
        }
        // Case 3: Email does not exist
        else {
            config_data.emails.insert(email, note);
            self.write_config(&config_data)?;
            Ok(true)
        }
    }

    pub fn remove_email(&self, email: &str) -> anyhow::Result<bool> {
        let mut config_data = self.load_config()?;
        let email = utils::format_email(email);
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
