use std::{collections::HashMap, path::Path};

use serde::{self, Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Output {
    pub description: String,
    pub value: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    pub description: String,
    pub default: Option<String>,
    pub required: Option<bool>,

    #[serde(alias = "deprecationMessage")]
    pub deprecation_message: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Action {
    pub name: Option<String>,
    pub description: Option<String>,

    pub outputs: Option<Outputs>,
    pub inputs: Option<Inputs>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outputs(pub HashMap<String, Output>);

#[derive(Deserialize, Debug, Clone)]
pub struct Inputs(pub HashMap<String, Input>);

pub trait SortedKeys {
    fn sorted_keys(&self) -> Vec<&str>;
}

impl SortedKeys for Inputs {
    fn sorted_keys(&self) -> Vec<&str> {
        let mut sorted: Vec<&str> = self.0.keys().map(|k| k.as_str()).collect();
        sorted.sort();

        sorted
    }
}

impl SortedKeys for Outputs {
    fn sorted_keys(&self) -> Vec<&str> {
        let mut sorted: Vec<&str> = self.0.keys().map(|k| k.as_str()).collect();
        sorted.sort();

        sorted
    }
}

impl Action {
    pub fn read_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let action: Action = serde_yaml::from_reader(reader)?;

        Ok(action)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            description: String::new(),
            default: None,
            required: Some(false),
            deprecation_message: None,
        }
    }
}

impl Input {
    pub fn to_markdown(&self, name: &str) -> String {
        let def = self.default.clone().unwrap_or(String::from("`nd`"));
        let is_required = self.required.unwrap_or(false);

        let required_as_str = is_required.to_string();

        format!(
            "| `{}` | {} | {} | `{}` |",
            name, self.description, def, required_as_str
        )
    }
}

impl Output {
    pub fn to_markdown(&self, name: &str) -> String {
        format!("| `{}` | {} |", name, self.description)
    }
}
