use super::*;
use std::collections::HashMap;

pub struct FungibleToken {
	balances: HashMap<u64, u64>,
}

impl FungibleToken {
	pub fn new() -> FungibleToken {
		FungibleToken {
			balances: HashMap::new(),
		}
	}
}

impl Fungible for FungibleToken {
	type Address = u64;
	type Balance = u64;

	fn transfer(&mut self, from: &Self::Address, to: &Self::Address, amount: Self::Balance) -> Result<(), String> {
		let from_balance = self.balance_of(&from);
		if from_balance < amount {
			return Err("Insufficient balance".to_string());
		}
		self.set_balance(from, from_balance - amount);
		let to_balance = self.balance_of(&to);
		self.set_balance(to, to_balance + amount);
		Ok(())
	}

	fn balance_of(&self, owner: &Self::Address) -> Self::Balance {
		*self.balances.get(owner).unwrap_or(&0)
	}

	fn set_balance(&mut self, owner: &Self::Address, amount: Self::Balance) {
		self.balances.insert(*owner, amount);
	}
}


