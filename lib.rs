#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
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
            self.allowance_of(&from, &caller)
        }
        pub fn allowance_of(&self, from: &AccountId, spender: &AccountId) -> u32 {
            *self.allowances.get(&(*from, *spender)).unwrap_or(&0)
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
            let mut allowance_of_caller = self.allowance_of(&from, &caller);
            if allowance_of_caller < amount {
                return false;
            }
            let mut balance_of_receiver = self.get_balance(to);
            let mut balance_of_from = self.get_balance(from);
            balance_of_from -= amount;
            balance_of_receiver += amount;
            allowance_of_caller -= amount;

            self.balances.insert(from, balance_of_from);
            self.balances.insert(to, balance_of_receiver);
            self.allowances.insert((from, caller), allowance_of_caller);
            self.env().emit_event(Transfer{
                from: caller,
                to,
                amount
            });
            return true;
        }
    }
    //testovi :D
    //pokretanje testova: cargo +nightly test
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = MyContract::new(1000);
            assert_eq!(contract.total_supply(), 1000);
        }

        #[ink::test]
        fn balance_works() {
            let contract = MyContract::new(100);
            assert_eq!(contract.get_balance(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.get_balance(AccountId::from([0x0; 32])), 0);
        }
        #[ink::test]
        fn transfer_works() {
            let mut contract = MyContract::new(100);
            assert_eq!(contract.get_balance(AccountId::from([0x1; 32])), 100);
            contract.transfer(AccountId::from([0x0; 32]), 60);
            //proverava balance od primaoca
            assert_eq!(contract.get_balance(AccountId::from([0x0; 32])), 60);
            assert_eq!(contract.get_balance(AccountId::from([0x1; 32])), 40);
        } 
        #[ink::test]
        fn allow_works() {
            let mut contract = MyContract::new(100);
            assert_eq!(contract.get_balance(AccountId::from([0x1; 32])), 100);
            contract.allow(AccountId::from([0x0; 32]), 40);
            assert_eq!(contract.allowance_of(&AccountId::from([0x1; 32]), 
            &AccountId::from([0x0; 32])), 40);
        }
        #[ink::test]
        fn transfer_from_works() { 
            //u ovom testu sam sebi dozvoljavam trosenje neke kolicine tokena
            //ovo inace nema smisla, ali je za test dobro
            let mut contract = MyContract::new(100);
            assert_eq!(contract.get_balance(AccountId::from([0x1; 32])), 100);
            contract.allow(AccountId::from([0x1; 32]), 30);
            assert_eq!(contract.allowance_of(&AccountId::from([0x1; 32]), 
            &AccountId::from([0x1; 32])), 30);
            assert_eq!(contract.get_balance(AccountId::from([0x0; 32])), 0);
            contract.transfer_from(AccountId::from([0x1; 32]), 
            AccountId::from([0x0; 32]), 30);
            //proverava da li je primaoc dobio tokene
            assert_eq!(contract.get_balance(AccountId::from([0x0; 32])), 30);
            //proverava da li se potrosacu smanjio dozvoljen balance
            assert_eq!(contract.allowance_of(&AccountId::from([0x1; 32]), 
            &AccountId::from([0x1; 32])), 0);
        }
    }
}