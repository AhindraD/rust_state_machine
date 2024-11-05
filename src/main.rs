use balances::Pallet;
use std::collections::BTreeMap;
mod balances;
fn main() {
    println!("Hello from Rust State Machine!");
    let mut pallet: Pallet = balances::Pallet::new();

    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    map.insert("ahindra", 777);
    assert_eq!(map.get(&"ahindra"), Some(&777));
    assert_eq!(map.get(&"anon"), None);

    let maybe_val1 = map.get("ahindra");
    let maybe_val2 = map.get("zyzz");
    match maybe_val1 {
        Some(value) => {
            println!("{}", value)
        }
        None => {
            println!("not present")
        }
    }
    match maybe_val2 {
        Some(value) => {
            println!("{}", value)
        }
        None => {
            println!("not present")
        }
    }
}
