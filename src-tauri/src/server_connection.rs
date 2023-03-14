use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Protocol {
    Http = 1,
    Https = 2
}

#[derive(Serialize, Deserialize)]
pub struct ServerConnection {
    pub protocol: Protocol,
    pub address: String,
    pub port: String,
    pub extension: String,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    connections: Vec<ServerConnection>
}

pub fn add_connection(conn: ServerConnection) {
    //Get the existing config
    let mut connections = get_connections();
    //Add the new one.
    connections.push(conn);
    
    let new_file = ConfigFile {
        connections: connections
    };

    //Convert config to json and write to file.
    let json = serde_json::to_string(&new_file).unwrap();
    let mut file = File::create(get_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn get_connections() -> Vec<ServerConnection> {
    match File::open(get_path()) {
        Ok(mut res) => {
            let mut contents = String::new();
            res.read_to_string(&mut contents).expect("Error Reading Config File! Exiting...");
            let config: ConfigFile = serde_json::from_str(&contents).unwrap();
            return config.connections;
        },
        Err(_reason) => {
            println!("File not found. Creating a new one!");
            return create_config();
        }
    }
}

fn create_config() -> Vec<ServerConnection> {
    match File::create(get_path()) {
        Ok(mut data) => {
            //Create the default config values
            let initial_connection = ConfigFile {
                connections: vec![]
            };

            //Convert config to json and write to file.
            let json = serde_json::to_string(&initial_connection).unwrap();
            data.write_all(json.as_bytes()).unwrap();

            //Return the default data
            return initial_connection.connections;
        },
        Err(_reason) => {
            //Something is really wrong.
            std::process::exit(1);
        }
    }
}

pub fn delete_connection(name: String) {
    let mut connections = get_connections();
    connections.retain(|conn| conn.name != name);

    let new_file = ConfigFile {
        connections: connections
    };

    //Convert config to json and write to file.
    let json = serde_json::to_string(&new_file).unwrap();
    let mut file = File::create(get_path()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn get_path() -> String {
    #[cfg(unix)]
    let app_data: String = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let mut app_data: String = std::env::var("APPDATA").expect("No APP_DATA directory");

    app_data.push_str("/config.json");
    return app_data;
}