#![allow(unused_variables)]
mod fungible_token;
use fungible_token::FungibleToken;

/// Defines the behavior of a fungible token.
pub trait Fungible {
    /// Associated type Address represents an address in the system.
    type Address;
    /// Associated type Balance represents a balance of an Address.
    type Balance;

    /// Sets the balance of tokens for the specified address.
    fn set_balance(&mut self, owner: &Self::Address, amount: Self::Balance);

    /// Transfers a specified amount of tokens from the caller's account to the specified address.
    fn transfer(&mut self, from: &Self::Address, to: &Self::Address, amount: Self::Balance) -> Result<(), String>;

    /// Retrieves the balance of tokens owned by the specified address.
    fn balance_of(&self, owner: &Self::Address) -> Self::Balance;
}   

fn main() {
    let mut token = FungibleToken::new();
    let staker: u64 =  1;
    let provider: u64 = 2;

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
