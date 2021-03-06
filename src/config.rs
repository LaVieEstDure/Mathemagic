use ::File;
use std::io::Read;
use ::serde_json::{to_writer_pretty, from_str};


#[derive(Serialize, Deserialize)]
pub struct Config {
    token: String,
    latex: bool
}

fn read_config() {
    let file = File::open("config.json");
    let _ = match file {
        Ok(file) => file,

        Err(_) => {
            let new_config = File::create("config.json") 
                    .expect("Failed to create file");

            let default = Config{
                token: String::from("Insert token here"),
                latex: true
            };

            let _ = to_writer_pretty(&new_config, &default);
            new_config
        },
    };
}

pub fn config() -> Config{
        read_config();
    let mut file = File::open("config.json")
            .expect("Failed to open config");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
            .expect("Failed to read config");

    let json: Config = from_str(&contents)
            .expect("Failed to parse JSON");
    json
}

pub fn token() -> String {
    config().token
}
