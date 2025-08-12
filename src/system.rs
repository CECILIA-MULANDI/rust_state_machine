use std::collections::BTreeMap;
type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;
#[derive(Debug)]
pub struct Pallet {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}
impl Pallet {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let current_nonce: u32 = *self.nonce.get(who).unwrap_or(&0);
		let new_nonce = current_nonce + 1;
		self.nonce.insert(who.clone(), new_nonce);
	}
}
#[cfg(test)]
mod test {
	fn init_system() {
		let mut system = super::Pallet::new();
		system.inc_block_number();
		let alice_nonce = system.inc_nonce(&"alice".to_string());
		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
