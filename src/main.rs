extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::fs::File;

mod latex;
mod config;

static TEXT: &'static str = r"\documentclass{article}
\begin{document}
\pagenumbering{gobble}
\section{Hello, World!}
This is \LaTeX!
\end{document}
";


fn main() {
    // let clnt = reqwest::Client::new();
    // let filename = latex::send_post(clnt, String::from(TEXT), "png");
    // latex::send_get(&filename, "out.png");
    print!("");
    println!("TOKEN: {}", config::token());
}
