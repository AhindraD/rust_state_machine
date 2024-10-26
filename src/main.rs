use balances::Pallet;

mod balances;
fn main() {
    println!("Hello from Rust State Machine!");
    let mut pallet: Pallet = balances::Pallet::new();
}
