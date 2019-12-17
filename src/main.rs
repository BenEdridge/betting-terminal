use std::env;
use std::process;
use std::collections::HashMap;

mod model;
mod engine;
//mod config;

use crate::model::bet;
use crate::model::event;
use crate::engine::math;
use crate::engine::ticker;

//use crate::config::Config;

// https://doc.rust-lang.org/std/collections/struct.HashMap.html
// Use HashMap for memory storage

// main entry function
fn main() {

    let args: Vec<String> = env::args().collect();

    //let config = Config::new(&args).unwrap_or_else(|err| {
    //    eprintln!("Failure to parse arguments: {}", err);
    //    process::exit(1);
    //});

    println!("{}", (1..101).fold(0, |x, y| x + y));

    math::generate_winners();
    math::global_seed(); //generates a initial seed repeatable games

    // println!("Console Running @ {}", config.console_size);
    // println!("Server running: {}", config.start_server);

    if let Err(e) = betting::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
