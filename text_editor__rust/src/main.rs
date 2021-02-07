use std::env;
use std::process;

use text_editor__rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = text_editor__rust::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

