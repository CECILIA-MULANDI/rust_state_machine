use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;
pub trait Config: crate::system::Config {
	type Content: Debug + Ord;
}

pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}
impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		self.claims.insert(claim, caller);

		Ok(())
	}
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if caller != *owner {
			return Err("this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}
#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = String;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		/*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/
		let mut new_poe = super::Pallet::<TestConfig>::new();
		assert_eq!(new_poe.get_claim(&"Hello Claims".to_string()), None);
		assert_eq!(new_poe.create_claim("alice".to_string(), "Hello Claims".to_string()), Ok(()));
		assert_eq!(new_poe.get_claim(&"Hello Claims".to_string()), Some(&"alice".to_string()));
		assert_eq!(
			new_poe.create_claim("bob".to_string(), "Hello Claims".to_string()),
			Err("this content is already claimed")
		);
		assert_eq!(new_poe.revoke_claim("alice".to_string(), "Hello Claims".to_string()), Ok(()));
		assert_eq!(new_poe.create_claim("bob".to_string(), "Hello Claims".to_string()), Ok(()));
	}
}
