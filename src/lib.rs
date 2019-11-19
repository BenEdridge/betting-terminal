use std::error::Error;

pub struct Config {
  pub console_size: String,
  pub start_server: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {

      if args.len() < 3 {
          return Err("Incorrect args");
      }

      let console_size = args[1].clone();
      let start_server = args[2].clone();

      Ok(Config { console_size, start_server })
  }
}

pub fn run() -> Result<(), Box<dyn Error>> {
  println!("{}","Running betting terminal");
  Ok(())
}