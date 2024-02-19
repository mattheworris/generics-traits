use super::*;
use std::hash::Hash;
use std::collections::HashMap;
use std::ops::{Add, Sub};

pub struct Coin<Address, Balance> {
	pub balances: HashMap<Address, Balance>,
}

impl<Address, Balance> Coin<Address, Balance>{
	pub fn new() -> Coin<Address, Balance>{
		Coin{
			balances: HashMap::new(),
		}
	}
}	

impl<Address, Balance> Fungible<Address, Balance> for Coin<Address, Balance>
where
	Address: Eq + Hash + Copy,
	Balance: Default + Copy + PartialOrd + Add<Output = Balance> + Sub<Output = Balance>,
{

	fn set_balance(&mut self, owner: &Address, amount: Balance) {
		self.balances.insert(*owner, amount);
	}

	fn balance_of(&self, owner: &Address) -> Balance {
		*self.balances.get(owner).unwrap_or(&Default::default())
	}
}
