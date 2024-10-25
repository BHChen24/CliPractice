use core::panic;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    println!("Looking for {}", config.query);
    println!("in {}", config.file_path);

    let content = fs::read_to_string(config.file_path).unwrap();
    println!("Read:\n{content}")
    // dbg!(args);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough length!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
