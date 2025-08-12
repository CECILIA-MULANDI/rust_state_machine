use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

//state and entry point
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		from: &AccountId,
		to: &AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		//get user balance
		let current_senders_new_bal =
			self.balance(from).checked_sub(&amount).ok_or("Not enough funds.")?;
		//get recipients balance
		let recipients_bal = self.balance(to).checked_add(&amount).ok_or("Overflow")?;
		// updates to the balances
		self.set_balance(from, current_senders_new_bal);
		self.set_balance(to, recipients_bal);
		Ok(())
	}
}
#[cfg(test)]
mod test {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<String, u128>::new();
		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
		// balances.transfer(&"alice".to_string(), &"bob".to_string(), 50).is_ok();
		// assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();
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
