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
use serenity::framework::standard::{Args, Command, DispatchError, StandardFramework, help_commands};
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::sync::Arc;



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

fn owner_check(_: &mut Context, msg: &Message, _: &mut Args, _: &Arc<Command>) -> bool {
    // Replace 7 with your ID
    msg.author.id == 7
}


fn main() {
    
    let token: String = config::token();
    let mut client = Client::new(&token, Handler);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c
        
        .prefix("+=")
        .delimiters(vec![", ", ","]))
        
        .before(|ctx, msg, command_name|{
            print!("Received command {}", command_name); true
        })
        .after(|_, _, command_name, error| {
            match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            }
        })
        .on_dispatch_error(|_ctx, msg, error| {
            if let DispatchError::RateLimited(seconds) = error {
                let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
            }
        })
        .command("ping", |c| c
            .exec_str("Pong!")));
            

}
