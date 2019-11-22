use std::f64::consts::*;
use rand::{ thread_rng };
use rand_distr::{ Normal, Distribution, Alphanumeric };

pub fn height_of_normal(mean: f64, sd: f64, x: f64) -> f64 {
  let sd2 = sd.powf(2.0);
  1.0 
    /
  (PI * 2.0 * sd2).sqrt() * E.powf(
  -(x - mean).powf(2.0)
    /
  2.0 * sd2)
}

// mean race time is 5 seconds standard deviation of 1 giving 99.7% 
// of race times between 2 and 8 secs
pub fn generate_winners() {
  let normal = Normal::new(5.0, 3.0).unwrap();

  let mut v: Vec<f32> = normal.sample_iter(thread_rng())
  .take(5)
  .collect();
  v.sort_by(|x, y| x.partial_cmp(y).unwrap());

  println!("{:?} is from a normal distribution", v);
}

// Creates initial seed for favourite racers based on time or argument
pub fn seed_favourites() {
  let seed: String = Alphanumeric.sample_iter(thread_rng()).take(64).collect();
  println!("Game seed : {}", seed);
}

