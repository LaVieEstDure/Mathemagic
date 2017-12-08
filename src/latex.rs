use ::reqwest;
use ::serde_json;
use ::File;

// Structure of JSON request
#[derive(Serialize, Deserialize)]
struct JSON {
    code: String,
    format: String
}

// Structure of JSON response
#[derive(Serialize, Deserialize)]
struct ResJ {
    status: String,
    log: String,
    filename: String
}

/// Sends POST request to LateX server. Sends text, requested file-format etc
/// Parameters:
///     client (&reqwest::Client) -> reqwest client instance
///     text (&str) -> text to parse into LaTeX
///     file_format (&str) -> type of file to recieve back. Accepts pdf, png or jpg
/// Returns:
///     filename (String) -> Filename on server
fn send_post(client: &reqwest::Client, text: &str, file_format:&str) -> String {
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

/// Uses filename from POST request to GET file from LaTeX server
/// Parameters:
///     filename (&str) -> Name of the file on the server from POST
///     filenname (&str) -> Name of the file to output
/// Returns nothing
fn send_get(filename: &str, filenname: &str){
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

/// Wrapper function for API requests
/// Parameters:
///     latex (String) -> LaTeX content to render
///     format (&str) -> File format, accepts pdf, png and jpg
/// Returns:
///     file (File) -> File to send
pub fn render_latex(latex: String, format:&str) -> File {
    let mut client = reqwest::Client::new();
    let filename = send_post(&client, &latex, format);
    send_get(&filename, "out.png");
    File::open("out.png").expect("Cant open file")
}