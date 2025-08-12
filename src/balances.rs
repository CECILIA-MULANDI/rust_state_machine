use std::collections::BTreeMap;
type AccountId = String;
type Balance = u128;
//state and entry point
#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}
impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
	pub fn transfer(
		&mut self,
		from: &String,
		to: &String,
		amount: u128,
	) -> Result<(), &'static str> {
		//get user balance
		let current_senders_new_bal =
			self.balance(from).checked_sub(amount).ok_or("Not enough funds.")?;
		//get recipients balance
		let recipients_bal = self.balance(to).checked_add(amount).ok_or("Overflow")?;
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
		let mut balances = super::Pallet::new();
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
