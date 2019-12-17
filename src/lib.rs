use std::error::Error;
use std::io;

mod engine;
use engine::ticker;

pub fn run() -> Result<(), Box<dyn Error>> {
  println!("{}","\x1B[7;49;39m Running betting terminal: 🐎 1, 🐕 2, Mixed 3");
  
  println!("{}", "Choose a game number");
  let mut game_input = String::new();
  io::stdin().read_line(&mut game_input).expect("Failed to read line");
  println!("Running game: {}", game_input);

  println!("{}", "Selections in format from top to bottom: <1,2,3,4,5...>");
  let mut selections = String::new();
  io::stdin().read_line(&mut selections).expect("Failed to read line");

  println!("{}", "Bet Type: <W=Win,P=Place,S=Show>");
  let mut bet_type = String::new();
  io::stdin().read_line(&mut bet_type).expect("Failed to read line");

  match game_input.trim() {
    "1" => ticker::spawn_race("🐎"),
    "2" => ticker::spawn_race("🐈"),
    "3" => ticker::spawn_race("🐢"),
    _ => println!("SAD! You didn't choose a race..."),
  }

  // loop {
  //   println!("{}", "Choose a winning number: 1-8");

  //   io::stdin().read_line(&mut winner_input)
  //     .expect("Failed to read line");

  //   winner_matched_input = match winner_input.trim().parse() {
  //       Ok(_) => break,
  //       Err(_) => continue,
  //   };
  // }

  println!("You chose: {} on a {} bet", selections, bet_type);

  Ok(())
}