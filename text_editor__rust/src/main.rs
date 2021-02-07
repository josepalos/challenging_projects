use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        // Clone as Config cannot take ownership of the strings
        let filename = args[1].clone();

        Config { filename }
    }
}

