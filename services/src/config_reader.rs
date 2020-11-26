use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct ConnectionConfig{
    pub this_machine_binding: String,
    pub target_machine_binding: String
}

impl ConnectionConfig {
    pub fn get_connection_config(file_path: &str ) -> ConnectionConfig{
        let mut file =  File::open(file_path).expect("Failed to open the config.json file!");
        
        let mut json = String::new();

        file.read_to_string(&mut json).expect("Failed reading the file!");

        let config: ConnectionConfig = serde_json::from_str(&json).expect("The json was not well formated.");

        return config;
    }
}