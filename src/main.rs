use std::env;
use std::process;
use betting::Config;

mod model;
mod engine;

use model::bet;
use model::race;
use engine::math;
use engine::ticker;

// main entry function
fn main() {

    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Failure to parse arguments: {}", err);
    //     process::exit(1);
    // });

    let result = math::height_of_normal(4.0, 2.0, 2.0);

    math::generate_winners();
    math::seed_favourites(); //generates a initial seed for favourites for ongoing game

    println!("Normal CALC {:?}", result);

    // println!("Console Running @ {}", config.console_size);
    // println!("Server running: {}", config.start_server);

    if let Err(e) = betting::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}