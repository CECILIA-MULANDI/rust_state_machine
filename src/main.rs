// use crate::balances::Pallet;
// use crate::system::Pallet;

mod balances;
mod proof_of_existence;
mod support;
mod system;
use crate::support::Dispatch;
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}
pub enum RuntimeCall {
	// BalancesTransfer { to: types::AccountId, amount: types::Balance },
	Balances(balances::Call<Runtime>),
}
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected");
		}
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}
impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		Ok(())
	}
}
fn main() {
	// create runtime instance
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();
	runtime.balances.set_balance(&alice, 1000);
	// // block emulation
	// runtime.system.inc_block_number();
	// assert_eq!(runtime.system.block_number(), 1);
	// //first transaction
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| eprintln!("{e}"));
	// runtime.system.inc_nonce(&alice);
	// let _res = runtime
	// 	.balances
	// 	.transfer(alice.clone(), charlie, 20)
	// 	.map_err(|e| eprintln!("{e}"));
	//Advanced Block creation logic
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 30 }),
			},
		],
	};
	runtime.execute_block(block_1).expect("Invalid block");
	println!("The current runtime: {:#?}", runtime);
}
