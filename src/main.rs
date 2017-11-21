extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;


use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;


static TEXT: &'static str = r"\documentclass{article}
\begin{document}
\pagenumbering{gobble}
\section{Hello, World!}
This is \LaTeX!
\end{document}
";

#[derive(Serialize, Deserialize)]
struct JSON {
    code: String,
    format: String
}

#[derive(Serialize, Deserialize)]
struct ResJ {
    status: String,
    log: String,
    filename: String
}

fn send_post(client: reqwest::Client, text: String, file_format:&str, filename:&str) -> String {
    let jsont = JSON{code: String::from(text), format: String::from(file_format)};
    let json = serde_json::to_string(&jsont).expect("no");

    println!("JSON data sent: \n\n{} \n\n", &json);

    let mut req = client.post("http://63.142.251.124:80/api/v2").body(json).send().expect("no");
    let dd: ResJ = req.json().expect("no");
    dd.filename
}

fn send_get(filename: &String){
    let url = String::from("http://63.142.251.124:80/api/v2/") + filename;
    println!("{}", &url);
    let mut resp = reqwest::get(&url).expect("No");
    // let mut content = String::new();
    // resp.read_to_string(&mut content).expect("no");
    // println!("{:?}",resp.text().expect("no") );
    // let mut file = File::create("out.pdf").expect("no");
    // file.write_all(content.as_bytes()).expect("no");
    // file.sync_all().expect("no");
}

fn main() {
    let clnt = reqwest::Client::new();
    let filename = send_post(clnt, String::from(TEXT), "pdf", "out.pdf");
    send_get(&filename);
}