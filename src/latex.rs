use ::reqwest;
use ::serde_json;
use ::File;

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


pub fn send_post(client: reqwest::Client, text: String, file_format:&str) -> String {
    let jsont = JSON{code: String::from(text), format: String::from(file_format)};
    let json = serde_json::to_string(&jsont).expect("no");

    let mut req = client.post("http://63.142.251.124:80/api/v2").body(json).send().expect("no");
    let dd: ResJ = req.json().expect("no");
    dd.filename
}


pub fn send_get(filename: &String, filenname: &str){
    let url = String::from("http://63.142.251.124:80/api/v2/") + filename;
    println!("{}", &url);
    let mut resp = reqwest::get(&url).expect("No");
    let mut file = File::create(filenname).expect("no");
    let _copy = resp.copy_to(&mut file);
    file.sync_all().expect("no"); 
}