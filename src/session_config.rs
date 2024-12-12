use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TabConfig {
    pub name: String,
    #[serde(default)]
    pub focus: bool,
    #[serde(default)]
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionConfig {
    pub session_name: String,
    pub shell_command_before: String,
    pub tabs: Vec<TabConfig>,
}

impl SessionConfig {
    pub fn new(path: &Path) -> Self {
        load_config(path).unwrap()
    }
}

/// Function to load the RON config file into the SessionConfig struct.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<SessionConfig, ron::Error> {
    let mut file = File::open(&path).map_err(|e| {
        ron::Error::Message(format!(
            "Failed to open file {}: {}",
            path.as_ref().display(),
            e
        ))
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|_| {
        panic!("Failed to open file: {}", path.as_ref().display())
    });
    let config: SessionConfig = ron::from_str(&contents)?;

    Ok(config)
}

pub fn save_config(
    path: &Path,
    session_config: &SessionConfig,
) -> Result<(), io::Error> {
    // Serialize it to a string in RON format
    let pretty_config = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let serialized = to_string_pretty(&session_config, pretty_config)
        .expect("Serialization failed");

    // Write the string to a file
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}
