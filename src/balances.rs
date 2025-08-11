use std::collections::BTreeMap;
//state and entry point
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}
impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}
}
