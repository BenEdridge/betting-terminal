use crate::engine::math;

#[derive(Debug)]
pub enum BetType {
    Win, //Simple win bet
    LuckyLoser, // Non placing
    Exacta, //two or more to finish in 1st/2nd in exact order
    Quinella, // two or more in any order
    Trifecta, // first 3 to cross finish line
    Trio, // 3 in any order
    FirstFour, // first 4 to cross

    Double, // Winner of two non-consecutive race
    Treble, // Winner of 3 consecutive races or Parlay
    Quaddie, // Winners of 4 consec

    Custom, //Custom bet type with tuned parameters
}

#[derive(Debug)]
pub struct Bet {
    bet_type: (BetType),
    pre_race_selections: Vec<u32>, //selected outcome in order?
    post_race_outcomes: Vec<u32>, //final outcomes in order?
    pub final_cost: f64,
    in_order: bool,
    multi_event: bool, //eg consecutive race or skipped races
    consecutive: bool,
    boxed: bool, //allows in_order to be flexible for winnings
    non_adjusted_prob: f64, //all outcomes equal
    adjusted_prob: f64, //adjusted based on odds?
    permutations_combinations: u128, //total outcomes for bet type
    final_win: f64, // final win amount
}

//Option<T> will be useful in rust

// Method defined within struct above??
// Can then be called as bet.total_cost()
impl Bet {
    fn total_cost(&self) -> u32 {
        self.final_cost as u32 * self.pre_race_selections[0]
    }
}