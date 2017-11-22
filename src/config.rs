use std::fs::OpenOptions;
use ::File;
use ::serde_json::{from_reader, to_writer_pretty, from_str};
use std::io::Read;
use std::io::prelude::*;


#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    latex: bool
}

fn read_config() {
    let file = File::open("config.json"); 
    let config_file = match file {
        Ok(file) => file,

        Err(_) => {
            let new_config = File::create("config.json").expect("Failed to create file");

            let default = Config{
                token: String::from("Insert token here"),
                latex: true
            };

            let _ = to_writer_pretty(&new_config, &default);
            new_config
        },
    };
}

pub fn token() -> String {
    read_config();
    let mut file = File::open("config.json").expect("Failed to open config");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config");
    let json: Config = from_str(&contents).expect("Failed to parse JSON");
    json.token
}