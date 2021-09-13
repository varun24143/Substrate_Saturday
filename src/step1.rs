use std::collections::HashMap;

pub struct BalancesModule {
    balances: HashMap<u32, u32>
}

impl BalancesModule {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new()
        }
    }

    pub fn set_balance(&mut self, who: u32, amount: u32) {
        self.balances.insert(who, amount);
    }

    pub fn balance(&self, who: u32) -> u32 {
        *self.balances.get(&who).unwrap_or(&0)
    }
}