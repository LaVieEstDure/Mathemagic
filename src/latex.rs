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


fn send_post(client: reqwest::Client, text: String, file_format:&str) -> String {
    let jsont = JSON{code: String::from(text), format: String::from(file_format)};
    let json = serde_json::to_string(&jsont)
            .expect("Failed to convert request to JSON");

    let mut req = client.post("http://63.142.251.124:80/api/v2")
            .body(json).send()
            .expect("Failed to send POST Request");
    let dd: ResJ = req.json()
            .expect("Failed to parse POST response");
    dd.filename
}


fn send_get(filename: &String, filenname: &str){
    let url = String::from("http://63.142.251.124:80/api/v2/") + filename;
    println!("{}", &url);
    let mut resp = reqwest::get(&url)
            .expect("Failed GET request");
    let mut file = File::create(filenname)
            .expect("Failed to create output file");
    let _copy = resp.copy_to(&mut file);
    file.sync_all()
            .expect("Failed to write output file"); 
}

pub fn render_latex(latex: String, format:&str) -> File {
    let mut client = ::reqwest::Client::new();
    let filename = send_post(client, latex, format);
    send_get(&filename, "out.png");
    File::open(&filename).expect("Couldnt open file")
}