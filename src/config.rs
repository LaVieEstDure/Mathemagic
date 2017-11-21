use std::fs::OpenOptions;

struct Config {
    token: String,
    latex: bool
}

fn read_config(filename: String){
    let file = OpenOptions::new().open("config.json");
}