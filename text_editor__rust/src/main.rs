use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    for (i, line) in contents.split("\n").enumerate() {
        println!("{} | {}", i, line);
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        // Clone as Config cannot take ownership of the strings
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

