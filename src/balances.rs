use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}
//state and entry point
#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	/*************  ✨ Windsurf Command ⭐  *************/
	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	/// Returns `Ok(())` if the transfer was successful, or an error if it was not.
	/*******  1d417edd-1e85-4a9a-aaa2-e5ac25ea4abe  *******/
	pub fn transfer(
		&mut self,
		from: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		//get user balance
		let current_senders_new_bal =
			self.balance(&from).checked_sub(&amount).ok_or("Not enough funds.")?;
		//get recipients balance
		let recipients_bal = self.balance(&to).checked_add(&amount).ok_or("Overflow")?;
		// updates to the balances
		self.set_balance(&from, current_senders_new_bal);
		self.set_balance(&to, recipients_bal);
		Ok(())
	}
}
#[cfg(test)]
mod test {
	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
		// balances.transfer(&"alice".to_string(), &"bob".to_string(), 50).is_ok();
		// assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();
		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}
