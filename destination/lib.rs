#![cfg_attr(not(feature = "std"), no_std)]

pub use self::destination::{
    Destination,
    DestinationRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod destination {
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Destination {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Destination {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value}
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            let caller = self.env().caller();
            let account_id: AccountId = self.env().account_id();
            let caller_is_origin = self.env().caller_is_origin();
            ink_env::debug_println!("########## destination caller ############### value is {:?}", caller);
            ink_env::debug_println!("########## destination caller_is_origin ############### value is {:?}", caller_is_origin);
            ink_env::debug_println!("########## destination account_id ############### value is {:?}", account_id);

            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let destination = Destination::default();
            assert_eq!(destination.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut destination = Destination::new(false);
            assert_eq!(destination.get(), false);
            destination.flip();
            assert_eq!(destination.get(), true);
        }
    }
}
