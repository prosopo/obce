#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod my_ext {
    use chain_extension::Error;
    use ink_env::DefaultEnvironment;

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(MyExtData);

    #[openbrush::wrapper]
    pub type MyExtRef = dyn MyExt;

    #[openbrush::trait_definition]
    pub trait MyExt {
        #[ink(message)]
        fn successful_method(&mut self) -> Result<(), Error<DefaultEnvironment>>;

        #[ink(message)]
        fn erroneous_method(&mut self) -> Result<(), Error<DefaultEnvironment>>;

        #[ink(message)]
        fn critically_erroneous_method(&mut self) -> Result<(), Error<DefaultEnvironment>>;

        #[ink(message)]
        fn multi_arg_method(&mut self, one: u64, two: u64) -> Result<u64, Error<DefaultEnvironment>>;

        #[ink(message)]
        fn weight_linear_method(&mut self, complexity: u64) -> Result<(), Error<DefaultEnvironment>>;
    }

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct MyExtData {
        pub my_ext: chain_extension::ink::Extension,
    }
}

#[openbrush::contract]
mod test_contract {
    use chain_extension::{
        ChainExtension,
        Error,
    };
    use ink_env::DefaultEnvironment;
    use openbrush::traits::Storage;

    use crate::my_ext::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct TestContract {
        #[storage_field]
        my_ext_data: MyExtData,
    }

    impl TestContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            TestContract::default()
        }
    }

    impl MyExt for TestContract {
        #[ink(message)]
        fn successful_method(&mut self) -> Result<(), Error<DefaultEnvironment>> {
            self.my_ext_data.my_ext.successful_method()
        }

        #[ink(message)]
        fn erroneous_method(&mut self) -> Result<(), Error<DefaultEnvironment>> {
            self.my_ext_data.my_ext.erroneous_method()
        }

        #[ink(message)]
        fn critically_erroneous_method(&mut self) -> Result<(), Error<DefaultEnvironment>> {
            self.my_ext_data.my_ext.critically_erroneous_method()
        }

        #[ink(message)]
        fn multi_arg_method(&mut self, one: u64, two: u64) -> Result<u64, Error<DefaultEnvironment>> {
            self.my_ext_data.my_ext.multi_arg_method(one, two)
        }

        #[ink(message)]
        fn weight_linear_method(&mut self, complexity: u64) -> Result<(), Error<DefaultEnvironment>> {
            self.my_ext_data.my_ext.weight_linear_method(complexity)
        }
    }
}
