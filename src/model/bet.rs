#[derive(Debug)]
pub enum BetType {
    Win(u32),
    Trifecta(u32, u32, u32),
    Trio(u32, u32, u32),
    Quinella(u32, u32, u32, u32),
}

#[derive(Debug)]
pub struct Bet {
    cost: u32,
    selections: [u32; 2],
    bet_type: (BetType),
}

//Option<T> will be useful in rust

// Method defined within struct above??
// Can then be called as bet.total_cost()
impl Bet {
    fn total_cost(&self) -> u32 {
        self.cost * self.selections[0]
    }
}

pub fn build_bet(cost: u32, selections: [u32; 2], bet_type: BetType) -> Bet {
    Bet {
        cost,
        selections,
        bet_type
    }
}