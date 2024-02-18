use super::*;
use std::collections::HashMap;

pub struct FungibleToken {
	balances: HashMap<Address, Balance>,
}

impl FungibleToken {
	pub fn new() -> FungibleToken {
		FungibleToken {
			balances: HashMap::new(),
		}
	}
}

impl Fungible for FungibleToken {

	fn transfer(&mut self, from: &Address, to: &Address, amount: Balance) -> Result<(), String> {
		let from_balance = self.balance_of(&from);
		if from_balance < amount {
			return Err("Insufficient balance".to_string());
		}
		self.set_balance(from, from_balance - amount);
		let to_balance = self.balance_of(&to);
		self.set_balance(to, to_balance + amount);
		Ok(())
	}

	fn balance_of(&self, owner: &Address) -> Balance {
		*self.balances.get(owner).unwrap_or(&0)
	}

	fn set_balance(&mut self, owner: &Address, amount: Balance) {
		self.balances.insert(*owner, amount);
	}
}


