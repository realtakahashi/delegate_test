#![cfg_attr(not(feature = "std"), no_std)]

pub use self::source::{
    Source,
    SourceRef,
};

use ink_lang as ink;

#[ink::contract]
mod source {
    use destination::DestinationRef;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Source {
        /// Stores a single `bool` value on the storage.
        value: bool,
        destination:DestinationRef,
    }

    impl Source {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, _destination:DestinationRef) -> Self {
            Self { value: init_value, destination:_destination }
        }

        // /// Constructor that initializes the `bool` value to `false`.
        // ///
        // /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Call Destination Flip
        #[ink(message)]
        pub fn destination_flip(&mut self) {
            let caller = self.env().caller();
            let account_id: AccountId = self.env().account_id();
            let caller_is_origin = self.env().caller_is_origin();
            ink_env::debug_println!("########## source caller ############### value is {:?}", caller);
            ink_env::debug_println!("########## source caller_is_origin ############### value is {:?}", caller_is_origin);
            ink_env::debug_println!("########## source account_id ############### value is {:?}", account_id);
            ink_env::debug_println!("########## source destination address ############### value is {:?}", self.destination);

            self.destination.flip();
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
            let source = Source::default();
            assert_eq!(source.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut source = Source::new(false);
            assert_eq!(source.get(), false);
            source.flip();
            assert_eq!(source.get(), true);
        }
    }
}
