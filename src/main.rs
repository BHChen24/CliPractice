use std::env;
use std::process;

use cli_practice::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Deal with the err message that Result<_, _> returns
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Something bad happened. Check your code. Error message: {err}");
        process::exit(1);
    });
    println!("Looking for '{}'", config.query);
    println!("in '{}'\n", config.file_path);

    if let Err(e) = cli_practice::run(config) {
        println!("Error happened {e}");
        process::exit(1);
    };
    // dbg!(args);
}
