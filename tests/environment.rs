use obce::ink_lang::env::{
    DefaultEnvironment,
    Environment,
};

#[obce::definition(id = 123)]
pub trait Trait {
    fn method(&self) -> u32;
}

#[obce::ink_lang::extension]
pub struct TestExtension;

impl Trait for TestExtension {}

pub struct CustomEnvironment;

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

    type ChainExtension = TestExtension;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod simple_contract {
    use crate::Trait;

    #[ink(storage)]
    pub struct SimpleContract {}

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            SimpleContract {}
        }

        #[ink(message)]
        pub fn call_method(&self) -> u32 {
            self.env().extension().method()
        }
    }
}

mod call_using_env {
    #[obce::mock]
    impl crate::Trait for () {
        fn method(&self) -> u32 {
            123
        }
    }

    #[test]
    fn call_contract() {
        register_chain_extensions(());
        let contract = crate::simple_contract::SimpleContract::new();
        assert_eq!(contract.call_method(), 123);
    }
}
