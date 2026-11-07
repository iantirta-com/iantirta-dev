
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::collections::HashMap;

use crate::constants;


#[derive(Debug, Serialize, Deserialize)]
struct _Config {
    global: AppConfig,
    projects: HashMap<String, AppConfig>,
}

impl _Config {
    fn default() -> Self {
        Self {
            global: AppConfig::default(),
            projects: HashMap::new(),
        }
    }
    /// Load the raw .json file into object
    fn load() -> Self {
        let path = constants::get_config_path();

        if path.exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                // If the file exists and is valid JSON, load it
                return serde_json::from_str(&contents).unwrap_or_else(|_| Self::default());
            }
        }

        let default_config: _Config = Self::default();
        default_config.save();
        default_config
    }

    fn save(&self) {
        let path = constants::get_config_path();
        let mut temp_path = path.clone();
        temp_path.set_extension("tmp");

        let json = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        fs::write(&temp_path, json).expect("Failed to write temp config file");
        fs::rename(temp_path, path).expect("Failed to atomically swap config file");
    }

    fn get_mut_config(&mut self, project_name: Option<&str>) -> &mut AppConfig {
        if let Some(name) = project_name {
            self.projects.entry(name.to_string()).or_insert_with(AppConfig::default)
        } else {
            &mut self.global
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {

    // Project prefix
    prefix: String,

    // Python
    venv_dir: Option<String>,
    venv_pattern: Option<String>,
}


impl Default for AppConfig {
    fn default() -> Self {
        Self {
            prefix: "iantirta|i".to_string(),
            venv_dir: Some(constants::get_home_dir().join(".venv").to_string_lossy().to_string()),
            venv_pattern: Some("{prefix}_{project_name}_venv".to_string()),
        }
    }
}


pub struct Config {
    root: _Config,
    pub project_name: Option<String>,
}


impl Config {
    fn _get_app_config(&mut self) -> &mut AppConfig {
        self.root.get_mut_config(self.project_name.as_deref())
    }

    pub fn load(project_name: Option<&str>) -> Self {
        let root: _Config = _Config::load();
        Config {
            root: root,
            project_name: project_name.map(String::from),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        let app_config: &mut AppConfig = self._get_app_config();

        let mut config_map: Value = serde_json::to_value(&app_config)
            .map_err(|e: serde_json::Error| format!("Failed to parse config into map: {}", e))?;
    
        if let Some(map) = config_map.as_object_mut() {
            if value.is_empty() || value == "null" {
                map.insert(key.to_string(), Value::Null);
            } else {
                map.insert(key.to_string(), Value::String(value.to_string()));
            }
        } else {
            return Err("Underlying config is not a JSON object".to_string());
        }

        *app_config = serde_json::from_value(config_map)
            .map_err(|e| format!("Failed to set '{}': {}", key, e))?;

        self.root.save();
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>, String> {
        let app_config: &mut AppConfig = self._get_app_config();

        let config_map: Value = serde_json::to_value(&app_config)
            .map_err(|e: serde_json::Error| format!("Failed to parse config into map: {}", e))?;

        match config_map.get(key) {
            Some(value) => {
                // 3. Extract the underlying data from the JSON Value enum
                match value {
                    Value::String(s) => Ok(Some(s.clone())), // Extract the owned string
                    Value::Null => Ok(None),                 // Handle your Option<String> None values
                    _ => Ok(Some(value.to_string())),        // Fallback for booleans/numbers if you add them later
                }
            }
            None => Err(format!("Key '{}' does not exist in the configuration", key)),
        }
    }

    pub fn list(&mut self) -> Result<String, String> {
        let app_config: &mut AppConfig = self._get_app_config();
        serde_json::to_string_pretty(&app_config)
            .map_err(|e| format!("Failed to format config: {}", e))
    }
}

