extern crate reqwest;
#[macro_use]
extern crate serenity;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use serenity::prelude::*;
use serenity::model::*;
use std::env;


mod latex;
mod config;

static TEXT: &'static str = r"\documentclass{article}
\begin{document}
\pagenumbering{gobble}
\section{Hello, World!}
This is \LaTeX!
\end{document}
";

struct Handler;

impl EventHandler for Handler {

    fn on_ready(&self, _: Context, ready: Ready) {
        println!("Connection successful! \n {} is connected!", ready.user.name);
    }

}

command!()

fn main() {
    
    let token: String = config::token();
    let mut client = Client::new(&token, Handler);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("+="))
            .on("ping", ping))
            .delimiters(vec!(", ", ","))
            .
            

}
