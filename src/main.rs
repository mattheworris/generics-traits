#![allow(unused_variables)]
mod fungible_token;
use fungible_token::FungibleToken;

/// Represents an address in the system.
pub type Address = u64;

/// Represents a balance of a token.
pub type Balance = u64;

/// Defines the behavior of a fungible token.
pub trait Fungible {
    /// Transfers a specified amount of tokens from the caller's account to the specified address.
    fn transfer(&mut self, from: &Address, to: &Address, amount: Balance) -> Result<(), String>;

    /// Retrieves the balance of tokens owned by the specified address.
    fn balance_of(&self, owner: &Address) -> Balance;

    /// Sets the balance of tokens for the specified address.
    fn set_balance(&mut self, owner: &Address, amount: Balance);
}   

fn main() {
    let mut token = FungibleToken::new();
    let staker: Address = 1;
    let provider: Address = 2;

    println!("Balance of address 1: {}", token.balance_of(&staker));
    println!("Balance of address 2: {}", token.balance_of(&provider));

    token.set_balance(&staker, 100);
    let result = token.transfer(&staker, &provider, 50);

    println!("Balance of address 1: {}", token.balance_of(&staker));
    println!("Balance of address 2: {}", token.balance_of(&provider));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_successful() {
        let mut token = FungibleToken::new();
        let staker: Address = 1;
        let provider: Address = 2;

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
        let staker: Address = 1;
        let provider: Address = 2;

        token.set_balance(&staker, 10);
        assert_eq!(token.balance_of(&staker), 10);

        let result = token.transfer(&staker, &provider, 50);
        assert_eq!(result, Err("Insufficient balance".to_string()));
        assert_eq!(token.balance_of(&staker), 10);
        assert_eq!(token.balance_of(&provider), 0);
    }
}
