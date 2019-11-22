use std::error::Error;
use std::io;
use rand::Rng;

mod engine;
use engine::ticker;

pub struct Config {
  pub console_size: String,
  pub start_server: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {

      let console_size = args[1].clone();
      let start_server = args[2].clone();

      Ok(Config { console_size, start_server })
  }
}

pub fn run() -> Result<(), Box<dyn Error>> {
  // print!("\x1B[2J");
  // print!("\x1B[K");
  println!("{}","\x1B[7;49;39m Running betting terminal: 🐎 1, 🐕 2");
  println!("{}", "Choose a game number");
  let mut game_input = String::new();

  io::stdin().read_line(&mut game_input)
    .expect("Failed to read line");

  println!("Running game: {}", game_input);
  let winner = rand::thread_rng().gen_range(1,8);

  println!("{}", "Choose a winner number");
  let mut winner_input = String::new();
  io::stdin().read_line(&mut winner_input)
    .expect("Failed to read line");

  ticker::spawn();

  // loop {
  //   println!("{}", "Choose a winning number: 1-8");

  //   io::stdin().read_line(&mut winner_input)
  //     .expect("Failed to read line");

  //   winner_matched_input = match winner_input.trim().parse() {
  //       Ok(_) => break,
  //       Err(_) => continue,
  //   };
  // }

  println!("Winner was: {}, you guessed: {}", winner, winner_input);

  Ok(())
}