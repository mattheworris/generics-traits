#![allow(unused_variables)]
mod fungible_token;
mod coin;
use coin::Coin;
use fungible_token::FungibleToken;
use std::ops::{Add, Sub};
use std::hash::Hash;

/// Defines the behavior of a fungible token.
// Use type parameters to define the Generics Address and Balance.
pub trait Fungible<Address, Balance>
where Address: Eq + Hash + Copy,
      Balance: Default + Copy + PartialOrd + Add<Output = Balance> + Sub<Output = Balance>,
{
    /// Sets the balance of tokens for the specified address.
    /// Default implementation is not allowed to access data directly.
    fn set_balance(&mut self, owner: &Address, amount: Balance);

    /// Transfers a specified amount of tokens from the caller's account to the specified address.
    fn transfer(&mut self, from: &Address, to: &Address, amount: Balance) -> Result<(), String> {
        // Default implementation
        let from_balance = self.balance_of(&from);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.set_balance(from, from_balance - amount);
        let to_balance = self.balance_of(&to);
        self.set_balance(to, to_balance + amount);
        Ok(())
    }

    /// Retrieves the balance of tokens owned by the specified address.
    fn balance_of(&self, owner: &Address) -> Balance;
}   

fn main() {
    let mut token = FungibleToken::<u64, u64>::new();
    let mut coin = Coin::<u32, u32>::new();
    let staker: u64 =  1;
    let provider: u64 = 2;

    let coin_buyer: u32 = 1;
    let coin_seller: u32 = 2;

    token.set_balance(&staker, 100);
    let result = token.transfer(&staker, &provider, 50);

    println!("Balance of token address 1: {}", token.balance_of(&staker));
    println!("Balance of token address 2: {}", token.balance_of(&provider));

    coin.set_balance(&coin_buyer, 100);
    let result = coin.transfer(&coin_buyer, &coin_seller, 50);

    println!("Balance of coin address 1: {}", coin.balance_of(&coin_buyer));
    println!("Balance of coin address 2: {}", coin.balance_of(&coin_seller));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_successful() {
        let mut token = FungibleToken::new();
        let staker: u64 = 1;
        let provider: u64 = 2;

        token.set_balance(&staker, 100);
        assert_eq!(token.balance_of(&staker), 100);

        let result = token.transfer(&staker, &provider, 50);
        assert_eq!(result, Ok(()));
        assert_eq!(token.balance_of(&staker), 50);
        assert_eq!(token.balance_of(&provider), 50);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut token = FungibleToken::new();
        let staker: u64 = 1;
        let provider: u64 = 2;

        token.set_balance(&staker, 10);
        assert_eq!(token.balance_of(&staker), 10);

        let result = token.transfer(&staker, &provider, 50);
        assert_eq!(result, Err("Insufficient balance".to_string()));
        assert_eq!(token.balance_of(&staker), 10);
        assert_eq!(token.balance_of(&provider), 0);
    }
}
