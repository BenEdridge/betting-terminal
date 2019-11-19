use std::env;
use std::process;
use betting::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failure to parse arguments: {}", err);
        process::exit(1);
    });

    println!("Console Running @ {}", config.console_size);
    println!("Server running: {}", config.start_server);

    if let Err(e) = betting::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}