#![cfg_attr(not(feature = "std"), no_std)]



#[ink::contract]

mod erc777 {

    use  ink_lang as ink;
    use  ink_prelude::{AccountId, Balance};
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use  ink_env::call::FromAccountId;
    use ink::primitives::AccountId as OtherAccountId;
    use scale::alloc::collections::HashMap;
    use ink_env::call::FromAccountId;



    #[ink(storage)]
    pub  struct Erc777 {

        balance: ink_storage::collection::HashMap<AccountId, Balance>,
        operator_approvals: ink_storage::collection::HashMap<(AccountId, AccountId), bool>,
        
    }

    impl Erc777 {
        /// Constructor that initialalizes the `bool` value to the given `init_value`.
        #[ink(constructor)]

        pub fn new() -> Self {

            Self { 
                balances: ink_storage::collections::HashMap::new(),
                operator_approvals: ink_storage::collections::HashMap::new()
             }
        }

        
        #[ink(message)]

        pub fn balance_of(&self , owner: AccountId) -> Balance {

            *Self.balances.get(&owner).unwrap_or(&0)
        }

    
        #[ink(message)]
        pub fn tranffer(&mut self, operator: AccountId, approval: bool) -> bool{
            let  owmer = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        
        #[ink(message)]

        pub fn approve(&mut self, operator, approved: bool) {

            let  owmer = self.env().caller();
            self.operator_approvals.insert((owmer, operator), approved);
        }
        
        #[ink(message)]

        pub fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {

            let  caller = self.env().caller();
            if caller != from &&!self.opprovals.get(&(from, caller).unwrap_or(&false)) {
                return false;
            }
            self.tranfer_from_to(from, to, value)
        }

        #[ink(message)]

        pub fn tranfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {

            let from_balance = self.balances.get(&from).unwrap_or(&0);
            let to_balance = self.balances.get(&to).unwrap_or(&0);
            if from_balance < value {
                return false;
            }
            let to_balance = self.balances_of(&to).unwrap_or(&0);
            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Tranfer {
                from,
                to,
                value,
            });
            true
        }
}
    #[cfg(test)]
    mod tests {
        use ink::primitives::AccountId;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let erc777 = Erc777::default();
            assert_eq!(erc777.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc777 = Erc777::new();
            assert_eq!(erc777.get(), false);
            erc777.set(true);
            assert_eq!(erc777.get(), true);
        }
        #[ink::test]
        fn it_works_transfer_from_to() {
            let mut erc777 =  Erc777::new();
            let caller = AccountId::from([0x1; 32]);
            let recipient = AccountId::from([0x2; 32]);
            erc777.balances.insert(caller, 100  );

            assert_eq!(erc777.tranfer_from_to(caller, recipient, 50),  true);
            assert_eq!(erc777.balance_of(caller), 50);
            assert_eq!(erc777.balance_of(recipient), 50);

            assert_eq!(erc777.tranfer_from_to(caller, recipient, 50),  false);
            assert_eq!(erc777.balance_of(caller), 50);
            assert_eq!(erc777.balance_of(recipient), 50);
            assert_eq!(erc777.tranfer_from_to(caller, recipient, 100),  true);

        }
    }
}
