#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub mod my_ext {
    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(MyExtData);

    #[openbrush::wrapper]
    pub type MyExtRef = dyn MyExt;

    #[openbrush::trait_definition]
    pub trait MyExt {
        #[ink(message)]
        fn store_key(&mut self, val: u64) -> u32;
    }

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct MyExtData {
        pub my_ext: chain_extension::ink::Extension,
    }
}

#[openbrush::contract]
mod test_contract {
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
        fn store_key(&mut self, val: u64) -> u32 {
            use chain_extension::ChainExtension;
            self.my_ext_data.my_ext.test_method(val)
        }
    }
}
