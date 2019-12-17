use std::f64::consts::*;
use rand::{ thread_rng, SeedableRng };
use rand_distr::{ Normal, Uniform, Distribution, Alphanumeric };
use std::convert::TryInto;
// use bigint::BigUint;
// use bigint::FromPrimitive;

// https://en.wikipedia.org/wiki/Multivariate_normal_distribution
//https://rosettacode.org/wiki/Category:Rust

// let big = BigUint::from_u8(5).unwrap();
// let answer_as_string = format!("{}", pow(big,pow(4,pow(3,2))));

//   // The rest is output formatting.
// let first_twenty: String = answer_as_string.chars().take(20).collect();
// let last_twenty_reversed: Vec<char> = answer_as_string.chars().rev().take(20).collect();
// let last_twenty: String = last_twenty_reversed.into_iter().rev().collect();
// println!("Number of digits: {}", answer_as_string.len());
// println!("First and last digits: {:?}..{:?}", first_twenty, last_twenty);

// important variables

// let true_odds; // true odds based just on probability
// let house_cut; // % cut to profit
// let calculated_odds // probability based on calculated odds which horses are more likely to win

/// mean race time is 5 seconds standard deviation of 1 giving 99.7% 
/// of race times between 2 and 8 secs
pub fn generate_winners() {
  let normal = Normal::new(5.0, 3.0).unwrap();

  // here is where we can make use another distribution to alter normal results?
  let uniform = Uniform::new(1, 10);
  let normal_for_noise = Normal::new(5.0, 1.0).unwrap(); // use a small std to generate noise for mid-pack entities thus winners and loser change less

  let mut v: Vec<f32> = normal.sample_iter(thread_rng())
    .take(5)
    .collect();

  v.sort_by(|x, y| x.partial_cmp(y).unwrap());

  println!("{:?} is from a normal distribution", v);
}

/// Creates initial seed for game, can be used to replicate game state
pub fn global_seed() {
  let seed: String = Alphanumeric.sample_iter(thread_rng())
  .take(32).collect();
  
  // let mut rng = rand::rngs::StdRng::from_seed(seed);

  println!("Intial game seed : {}", seed);
}

//eg
// Pick 3 across 3 independent events of 8 horse races

// 8 * 8 * 8; // 512 => 1/512

//https://www.sportsbettingdime.com/guides/how-to/horse-racing-bet-types/
//https://promathletics.com/blogs/news/understanding-the-probability-and-expected-value-of-parlays

// each wager has a different pool
// 
//
// POOL for PICK 3 across 
// Or everyone else playing Exacta

//
// Straight bets
//

// Win
// 1 / 8 races

// Place 
// 2 / 8

// Show
// 3 / 8


//
// Exotic Horizontal (Multiplicative)
//
// 

//Daily Double
// 1/8 * 1/8
// The wager cost = 1 selection * 1 selection * $<N> 

// PICK 3 eg 3 race parlay
// 1/8 * 1/8 * 1/8

// PICK <N>


// Combinations eg.
// 1/8 * 3/8 (Daily Double with combination)


//
// Exotic Vertical (Multiplicative)
//
//

//Exacta (first 2 in order)
// Same as Win because we remove the 1,2 switch option
// eg. 1/8 for winner
// horse 1 or 2 could 
// So 8!/6! = 56 different permuations
// So 2/56 of probability

// So to calculate These N in order Exotics

//#########
// Simplified prob
// Prob = (SELECTIONS(RUNNERS - SELECTIONS)!) / RUNNERS!

// The above applies to TRI and SUPERFECTA

// Boxing allows for flexibility in Verticals bets
// eg 1,2,3 box is safe as trio??

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn checking_prob_calculations(){
        assert_eq!(true_probability_of_single_event(1.0, 1.0, true), 1.0);
        assert_eq!(true_probability_of_single_event(1.0, 1.0, false), 1.0);

        assert_eq!(true_probability_of_single_event(1.0, 4.0, false), 0.25);
        assert_eq!(true_probability_of_single_event(1.0, 4.0, true), 0.25);

        // order isn't import only 6 permutations
        assert_eq!(true_probability_of_single_event(2.0, 4.0, false), 0.16666666666666666);
        // order is importantant 12 permuations
        assert_eq!(true_probability_of_single_event(2.0, 4.0, true), 0.08333333333333333);

        assert_eq!(true_probability_of_single_event(3.0, 4.0, false), 0.25);
        assert_eq!(true_probability_of_single_event(3.0, 4.0, true), 0.041666666666666664);

        assert_eq!(true_probability_of_single_event(4.0, 4.0, false), 1.0);
        assert_eq!(true_probability_of_single_event(4.0, 4.0, true), 0.041666666666666664);

        assert_eq!(true_probability_of_single_event(3.0, 18.0, false), 0.0012254901960784314);
        assert_eq!(true_probability_of_single_event(3.0, 18.0, true), 0.0002042483660130719);

        // assert_eq!(true_probability_of_single_event(3.0, 20.0, false), 0.0012254901960784314);
        // assert_eq!(true_probability_of_single_event(3.0, 20.0, true), 0.0002042483660130719);
    }
}

// selections = chosen outcomes, outcomes=possible results, in_order=permutation or combination based
// eg in_order is trifecta
pub fn true_probability_of_single_event(selection_count: f64, outcome_count: f64, in_order: bool) -> f64 {
    let selection_combinations = factorial_r(outcome_count as u64 - selection_count as u64) as f64;
    let total_combinations = factorial_r(outcome_count as u64) as f64;
    let selection_count_factorial = factorial_r(selection_count as u64) as f64;

    if in_order {
       selection_combinations  / total_combinations //flip these to get combinations
    } else {
        selection_count_factorial * selection_combinations  / total_combinations
    }
}

// Just flip result to get permutation integer
pub fn true_permutation_of_single_event(selection_count: f64, outcome_count: f64, in_order: bool) -> f64 {
    let selection_combinations = factorial_r(outcome_count as u64 - selection_count as u64) as f64;
    let total_combinations = factorial_r(outcome_count as u64) as f64;
    let selection_count_factorial = factorial_r(selection_count as u64) as f64;

    if in_order {
        total_combinations / selection_combinations  //flip these to get combinations
    } else {
          total_combinations / selection_count_factorial * selection_combinations
    }
}

pub fn factorial_r(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial_r(n-1)
    }
}

// Permutations with repetition
// eg. n types
// n * n * n = n (cubed)
// -> permuations

// Permuations without repetition
// eg n types and remove n-1 for each selection
// 16 pool balls ordering
// Make use of factorial
// n! = permuations


//eg 1st and 2nd place awards for 10 people
// 10! / (10-2)! => 90 different ways

// Does order matter?
// NO => 2! / 90 = 0.0222222
// YES => 1 / 90 = 0.0111111


//Combinations
// 1 -> Repetition is allowed
// 2 -> None (eg lotto)

// LOTTO
// "n choose r" => 
// It is often called "n choose r" (such as "16 choose 3")
// And is also known as the Binomial Coefficient.

// 3 out of 16 gives us 3360 permuations
// 3! = 6
// 16! /(3! * (16-3)!) = 560

//https://www.mathsisfun.com/combinatorics/combinations-permutations.html
//https://www.mathsisfun.com/combinatorics/combinations-permutations-calculator.html