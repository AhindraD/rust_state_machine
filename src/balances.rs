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
}
