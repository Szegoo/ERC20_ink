#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ERC20 {
    #[ink(storage)]
    pub struct MyContract {
        total_supply: u32,
        balances: ink_storage::collections::HashMap<AccountId, u32>,
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), u32>
    }
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: u32
    }
    #[ink(event)] 
    pub struct Allowance {
        from: AccountId,
        spender: AccountId,
        amount: u32
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(total_supply: u32) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, total_supply);
            Self {
                total_supply,
                balances,
                allowances: Default::default()
            }
        }
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
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
        pub fn allowance(&self, from: AccountId) -> u32 {
            let caller = self.env().caller();
            *self.allowances.get(&(from, caller)).unwrap_or(&0)
        }
        #[ink(message)]
        pub fn allowance_of(&self, from: AccountId, spender: AccountId) -> u32 {
            *self.allowances.get(&(from, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allow(&mut self, spender: AccountId, amount: u32) -> bool {
            let caller = self.env().caller();
            let balance_of_caller = self.get_balance(caller);
            if balance_of_caller < amount {
                return false;
            }
            self.allowances.insert((caller, spender), amount);
            self.env().emit_event(Allowance {
                from: caller,
                spender,
                amount
            });
            true
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
            //emitujem event
            self.env().emit_event(Transfer{
                from: caller,
                to,
                amount
            });
            return true;
        }
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u32) -> bool {
            let caller = self.env().caller();
            let mut allowed_balance = self.allowance_of(from, caller);
            if allowed_balance < amount {
                return false;
            }
            let mut balance_of_from = self.get_balance(from);
            let mut balance_of_receiver = self.get_balance(to);
            balance_of_from -= amount;
            balance_of_receiver += amount;
            allowed_balance -= amount;
            self.allowances.insert((from, caller), allowed_balance);
            self.balances.insert(from, balance_of_from);
            self.balances.insert(to, balance_of_receiver);
            return true;
        }
    }
}