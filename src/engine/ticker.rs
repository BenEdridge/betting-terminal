use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use rand::Rng;

const TICK_RATE_MS: u64 = 25;

/// Thread for game ticker
pub fn spawn_race(entity: &'static str) {

    // race array example position
    let mut race_results: [usize; 8] = [30; 8];

    let handle = thread::spawn( move || {
        for i in 0..400 {
            thread::sleep(Duration::from_millis(TICK_RATE_MS));

            let pos_update = rand::thread_rng().gen_range(0,8);

            print!("\x1Bc"); // clear screen in between
            println!("Race running for {} seconds", i * TICK_RATE_MS / 1000);
            print_race_state(race_results, entity);
        
            if race_results[pos_update] == 1 {
                println!("{} is the winner!", pos_update + 1);
                break;
            }
            race_results[pos_update] -= 1;
        }

    });

    // Thread joining
    handle.join().unwrap();

    fn print_race_state(race_results: [usize; 8], entity: &str) {
        for (i, pos) in race_results.iter().enumerate() {
            println!(
                "#{number} {text:>position$\u{000A}}", text=entity, position=pos, number=i+1
            );  
        }
    }
}

