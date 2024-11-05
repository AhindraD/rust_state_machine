use std::collections::BTreeMap;
pub struct Pallet {
    //a key-val pair storing balance
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    //Creating a new instance of the balances module
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn get_balance(&self, who: &String) {
        self.balances.get(who).unwrap_or(&0);
    }
}
