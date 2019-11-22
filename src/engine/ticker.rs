use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use rand::Rng;

const TICK_RATE_MS: u64 = 60;

pub fn spawn() {

    // race array example position
    let mut race_results: [usize; 8] = [40; 8];

    let handle = thread::spawn( move || {
        for i in 0..400 {
            thread::sleep(Duration::from_millis(TICK_RATE_MS));

            let pos_update = rand::thread_rng().gen_range(0,8);

            print!("\x1Bc"); // clear screen in between
            println!("Race has been running for {} seconds", i);

            for horse in &race_results {
                print_state(*horse);
            }
            if race_results[pos_update] == 0 {
                break;
            }
            race_results[pos_update] -= 1;
        }
    });

    handle.join().unwrap();

    fn print_state(status: usize) {
        println!(
            "{text:>width$\u{000A}}", text="🐎", width=status
        );  
    }
}

