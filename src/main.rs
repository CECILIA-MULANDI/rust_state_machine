// use crate::balances::Pallet;
// use crate::system::Pallet;
mod balances;
mod system;
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}
impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}
fn main() {
	// create runtime instance
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	runtime.balances.set_balance(&alice, 100);
	// block emulation
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);
	//first transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(&alice, &bob, 30).map_err(|e| eprintln!("{e}"));
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| eprintln!("{e}"));
	println!("The current runtime: {:#?}", runtime);
}
