extern crate reqwest;
#[macro_use]
extern crate serenity;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
mod latex;
mod config;


use std::fs::File;
use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::standard::{StandardFramework};


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



fn main() {
    
    let token: String = config::token();
    let mut client = Client::new(&token, Handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c|c.prefix("~"))
        .before(|ctx, msg, command_name|{
            println!("Got command: {}", command_name);
            true
        })
        .after(|_, _, command_name, error|{
            println!("Processed command!")
        })
        .command("ping", |c| c.exec(ping))
        .command("math", |c| c.exec(math))

    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
 
}

command!(math(context, message) {
    let file = latex::render_latex(String::from(TEXT),  "png");
    let _ = match message.channel() {
            Some(Channel::Category(c)) => panic!("Category channel"),
            Some(Channel::Group(c)) => c.read().unwrap().send_files(vec!((&file, "maths.png")), |m|m.content("maths")),
            Some(Channel::Guild(c)) => c.read().unwrap().send_files(vec!((&file, "maths.png")), |m|m.content("maths")),
            Some(Channel::Private(c)) => c.read().unwrap().send_files(vec!((&file, "maths.png")), |m|m.content("maths")),
            None => panic!("Idk some kind of error")
        };
});

command!(ping(_context, message) {
    let _ = message.channel_id.say("Pong!");
});
