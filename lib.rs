#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ERC20 {
    #[ink(storage)]
    pub struct MyContract {
        total_supply: u32,
        balances: ink_storage::collections::HashMap<AccountId, u32>,
    }
    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, 1_000_000);
            Self {
                total_supply: 1_000_000,
                balances
            }
        }
        #[ink(message)]
        pub fn get_my_balance(&self) -> u32 {
            self.get_balance(self.env().caller())
        }
        #[ink(message)]
        pub fn get_balance(&self, of: AccountId) -> u32 {
            let value = self.balances.get(&of).unwrap_or(&0);
            *value
        }
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u32) -> bool {
            let caller = self.env().caller();
            let mut balance_of_caller = self.get_balance(caller);
            if balance_of_caller < amount {
                return false;
            }
            let mut balance_of_receiver = self.get_balance(to);
            balance_of_caller -= amount;
            balance_of_receiver += amount;

            self.balances.insert(caller, balance_of_caller);
            self.balances.insert(to, balance_of_receiver);
            return true;
        }
    }
}