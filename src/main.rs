#![allow(unused_variables)]
mod fungible_token;
mod coin;
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
    let mut coin = coin::Coin::new();
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
