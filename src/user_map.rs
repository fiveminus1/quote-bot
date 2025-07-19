use std::{collections::HashMap, fs};
use serde::Deserialize;
use crate::types::Error;

#[derive(Debug, Deserialize)]
pub struct UserMap(pub HashMap<String, String>); //todo: move to types if no further logic for this file

impl UserMap {
    pub fn load_from_file(path: &str) -> Result<Self, Error> {
        let file_content = fs::read_to_string(path)?;
        let map: HashMap<String, String> = serde_json::from_str(&file_content)?;
        Ok(UserMap(map))
    }

    pub fn resolve(&self, id: &str) -> String {
        self.0.get(id).cloned().unwrap_or_else(|| id.to_string())
    }
}