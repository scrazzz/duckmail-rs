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
