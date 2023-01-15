#[obce::definition(id = 123)]
pub trait Trait {
    fn method(&mut self, val: u32, another_val: u32) -> u32;

    #[obce(id = 456)]
    fn another_method(&mut self, val: u32) -> u32;
}

struct TestExtension;

impl Trait for TestExtension {}

#[ink::contract]
mod simple_contract {
    use crate::{
        TestExtension,
        Trait,
    };

    #[ink(storage)]
    pub struct SimpleContract {}

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            SimpleContract {}
        }

        #[ink(message)]
        pub fn call_method(&mut self, val: u32, another_val: u32) -> u32 {
            TestExtension.method(val, another_val)
        }

        #[ink(message)]
        pub fn call_another_method(&mut self, val: u32) -> u32 {
            TestExtension.another_method(val)
        }
    }
}

mod simple_test {
    #[obce::mock]
    impl crate::Trait for () {
        fn method(&mut self, val: u32, another_val: u32) -> u32 {
            val + another_val
        }
    }

    #[test]
    fn call_contract() {
        register_chain_extensions(());
        let mut contract = crate::simple_contract::SimpleContract::new();
        assert_eq!(contract.call_method(100, 200), 300);
    }
}

mod state_access {
    #[derive(Clone, Default)]
    pub struct State {
        call_count: u32,
    }

    #[obce::mock]
    impl crate::Trait for State {
        fn method(&mut self, _: u32, _: u32) -> u32 {
            self.call_count += 1;
            self.call_count
        }
    }

    #[test]
    fn call_contract() {
        register_chain_extensions(State::default());
        let mut contract = crate::simple_contract::SimpleContract::new();
        assert_eq!(contract.call_method(100, 200), 1);
        assert_eq!(contract.call_method(100, 200), 2);
    }
}

mod internal_calls {
    #[obce::mock]
    impl crate::Trait for () {
        fn method(&mut self, val: u32, another_val: u32) -> u32 {
            val + another_val
        }

        fn another_method(&mut self, val: u32) -> u32 {
            self.method(val, val)
        }
    }

    #[test]
    fn call_contract() {
        register_chain_extensions(());
        let mut contract = crate::simple_contract::SimpleContract::new();
        assert_eq!(contract.call_another_method(200), 400);
    }
}

mod unregistered_call {
    #[obce::mock]
    impl crate::Trait for () {}

    #[test]
    #[should_panic(expected = "UnregisteredChainExtension")]
    fn call_contract() {
        register_chain_extensions(());
        let mut contract = crate::simple_contract::SimpleContract::new();
        contract.call_another_method(200);
    }
}
