use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize)]
pub struct TabConfig {
    pub name: String,
    #[serde(default)]
    pub focus: bool,
    #[serde(default)]
    pub commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    pub session_name: String,
    pub tabs: Vec<TabConfig>,
}

impl SessionConfig {
    pub fn new(path: &Path) -> Self {
        load_config(path).unwrap()
    }
}

/// Function to load the RON config file into the SessionConfig struct.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<SessionConfig, ron::Error> {
    let mut file =
        File::open(&path).expect(&format!("Failed to open file: {}", path.as_ref().display()));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Failed to read file: {}", path.as_ref().display()));
    let config: SessionConfig = ron::from_str(&contents)?;

    Ok(config)
}
