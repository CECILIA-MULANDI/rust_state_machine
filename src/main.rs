// use crate::balances::Pallet;
// use crate::system::Pallet;
mod balances;
mod system;
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}
impl Runtime {
	fn new() -> Self {
		Runtime { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
fn main() {
	println!("Hello, world!");
}
