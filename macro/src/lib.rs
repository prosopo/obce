// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![cfg_attr(not(feature = "std"), no_std)]

use proc_macro::TokenStream;

use obce_codegen::{
    definition,
    error,
    mock,
    ChainExtensionImplementation,
};

/// Chain extension definition for use with Substrate-based nodes and ink! smart contracts.
///
/// # Description
///
/// This macro generates code based on activated OBCE features.
///
/// When used with `ink` feature, [`#[obce::definition]`](macro@definition) generates
/// a glue code to correctly call your chain extension from ink! smart contracts.
///
/// The behaviour of [`#[obce::definition]`](macro@definition) with `substrate` feature enabled
/// is to leave everything as-is, without any additional modifications.
///
/// ```ignore
/// pub struct ChainExtension;
///
/// #[obce::definition]
/// pub trait ChainExtensionDefinition {
///     fn some_method(&self, argument: u32) -> u64;
/// }
/// ```
///
/// # Custom identifiers
///
/// You can use `#[obce::definition(id = ...)]` and `#[obce(id = ...)]` to override
/// the automatically generated chain extension identifier and chain extension method identifier
/// correspondingly.
///
/// `id` accepts literals of type [`&str`] and [`u16`].
#[proc_macro_attribute]
pub fn definition(attrs: TokenStream, trait_item: TokenStream) -> TokenStream {
    match definition::generate(attrs.into(), trait_item.into()) {
        Ok(traits) => traits.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Chain extension implementation for use with Substrate-based nodes.
///
/// # Description
///
/// This macro generates the necessary trait implementations for you to use
/// your chain extension with Substrate runtime.
///
/// This macro checks for the generics that you use in your impl block.
///
/// ```ignore
/// use obce::substrate::{
///     frame_system::Config as SysConfig,
///     pallet_contracts::{
///         chain_extension::Ext,
///         Config as ContractConfig,
///     },
///     sp_core::crypto::UncheckedFrom,
///     sp_runtime::traits::StaticLookup,
///     ExtensionContext
/// };
///
/// pub struct ChainExtension;
///
/// #[obce::definition]
/// pub trait ChainExtensionDefinition {
///     fn extension_method(&self);
/// }
///
/// #[obce::implementation]
/// impl<'a, 'b, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, Env, ChainExtension>
/// where
///     T: SysConfig + ContractConfig,
///     <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
///     E: Ext<T = T>,
///     <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
///     Env: ChainExtensionEnvironment<E, T>
/// {
///     fn extension_method(&self) {
///         // Do awesome stuff!
///     }
/// }
/// ```
///
/// The `E: Ext<T = T>` trait bound is optional. This means that you can create your custom `ChainExtensionEnvironment`
/// implementations to test your chain extension on the Substrate side via `obce::substrate::CallableChainExtension` trait (for ink! testing see [`#[obce::mock]`](macro@mock)).
///
/// At the same time, OBCE will automatically generate `pallet_contracts::chain_extension::ChainExtension` impl with `E: Ext<T = T>` bound implied.
///
/// # Weight charging
///
/// You can use `#[obce(weight(dispatch = ...))]` to automatically charge
/// weight based on a pallet call dispatch information.
///
/// `dispatch` accepts a full path to pallet's call (for example, `pallet_example::Pallet::<T>::my_call`).
///
/// OBCE will attempt to automatically obtain dispatch info based on the arguments passed
/// to your chain extension method.
///
/// If pallet's call arguments and your chain extension method
/// arguments are different, you can use `args` to override them:
/// `#[obce(weight(dispatch = "pallet_example::Pallet::<T>::my_call", args = "some_val,123"))]`.
///
/// You can also use `#[obce(weight(expr = ...))]` to charge weight without pallet calls.
/// In this case, you can simply provide any expression which returns `Weight`:
/// `#[obce(weight(expr = "Weight::from_ref_time(some_val)"))]`.
///
/// ## Usage example
///
/// ```ignore
/// use obce::substrate::{
///     frame_system::{Config as SysConfig, RawOrigin},
///     pallet_contracts::{
///         chain_extension::Ext,
///         Config as ContractConfig,
///     },
///     sp_core::crypto::UncheckedFrom,
///     sp_runtime::traits::StaticLookup,
///     ExtensionContext
/// };
///
/// pub struct ChainExtension;
///
/// #[obce::definition]
/// pub trait ChainExtensionDefinition {
///     fn extension_method(&mut self, val: u64);
/// }
///
/// #[obce::implementation]
/// impl<'a, 'b, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, Env, ChainExtension>
/// where
///     T: SysConfig + ContractConfig + pallet_example::Config,
///     <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
///     E: Ext<T = T>,
///     <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
///     Env: ChainExtensionEnvironment<E, T>
/// {
///     #[obce(weight(dispatch = "pallet_example::Pallet::<T>::test_method", args = "123"))]
///     fn extension_method(&mut self, val: u64) {
///         // ...
///     }
/// }
/// ```
///
/// # `RetVal` handling
///
/// To correctly return `RetVal` on compatible errors, that have their variants marked with
/// `#[obce(ret_val = "...")]` mark your method with `#[obce(ret_val)]`:
///
/// ```ignore
/// #[obce::error]
/// pub enum MyCustomError {
///     #[obce(ret_val = "10_001")]
///     First
/// }
///
/// #[obce::implementation]
/// impl<'a, 'b, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, Env, ChainExtension>
/// where
///     T: SysConfig + ContractConfig + pallet_example::Config,
///     <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
///     E: Ext<T = T>,
///     <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
///     Env: ChainExtensionEnvironment<E, T>
/// {
///     #[obce(ret_val)]
///     fn extension_method(&mut self, _: u64) -> Result<(), MyCustomError> {
///         Err(MyCustomError::First)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn implementation(attrs: TokenStream, impl_item: TokenStream) -> TokenStream {
    match ChainExtensionImplementation::generate(attrs.into(), impl_item.into()) {
        Ok(impls) => impls.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Chain extension error.
///
/// # Description
///
/// Using [`#[obce::error]`](macro@error) you can generate custom chain extension
/// errors.
///
/// Errors marked with [`#[obce::error]`](macro@error) have [`Debug`], [`Copy`], [`Clone`], [`PartialEq`], [`Eq`], `scale::Encode` and `scale::Decode`
/// automatically derived for them.
///
/// ```ignore
/// #[obce::error]
/// enum Error {
///     FirstError,
///     SecondError(u32)
/// }
/// ```
///
/// # Critical errors
///
/// [`#[obce::error]`](macro@error) can automatically generate `SupportCriticalError`
/// implementation for variant that you mark with `#[obce(critical)]`:
///
/// ```ignore
/// use obce::substrate::CriticalError;
///
/// #[obce::error]
/// enum Error {
///     FirstError,
///
///     #[obce(critical)]
///     Two(CriticalError)
/// }
/// ```
///
/// Only one enum variant can be marked as `#[obce(critical)]`.
///
/// # `RetVal`-convertible errors
///
/// You can mark error variants with `#[obce(ret_val = "...")]` to create an implementation of
/// [`TryFrom<YourError>`](::core::convert::TryFrom) for `pallet_contracts::chain_extension::RetVal`,
/// which will automatically convert suitable error variants to `RetVal` on implementation methods marked with `#[obce(ret_val)]`.
///
/// Error variant's `#[obce(ret_val = "...")]` accepts an expression that evaluates to [`u32`]:
///
/// ```ignore
/// #[obce::error]
/// enum Error {
///     #[obce(ret_val = "10_001")]
///     First,
///
///     Second
/// }
/// ```
#[proc_macro_attribute]
pub fn error(attrs: TokenStream, enum_item: TokenStream) -> TokenStream {
    match error::generate(attrs.into(), enum_item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Chain extension mocking utility.
///
/// # Description
///
/// You can use [`#[obce::mock]`](macro@mock) to automatically generate `register_chain_extensions`
/// function, which accepts a context and automatically registers mocked chain extension methods
/// for off-chain ink! smart contract testing.
///
/// ```ignore
/// // ink! smart contract definition is omitted.
///
/// #[obce::definition]
/// pub trait MyChainExtension {
///     fn test_method(&mut self, val: u32, another_val: u32) -> u32;
/// }
///
/// #[obce::mock]
/// impl MyChainExtension for () {
///     fn test_method(&mut self, val: u32, another_val: u32) -> u32 {
///         val + another_val
///     }
/// }
///
/// #[test]
/// fn call_contract() {
///     register_chain_extensions(());
///     let mut contract = SimpleContract::new();
///     assert_eq!(contract.call_test_method(100, 200), 300);
/// }
/// ```
///
/// When using [`#[obce::mock]`](macro@mock), you are not required to fill every single
/// method for testing. Glue code to register chain extension methods will only apply to
/// those methods, that you listed in a mock macro call:
///
/// ```ignore
/// #[obce::definition]
/// pub trait MyChainExtension {
///     fn first_method(&mut self, val: u32) -> u32;
///     fn second_method(&mut self) -> u64;
/// }
///
/// #[obce::mock]
/// impl MyChainExtension for () {
///     fn first_method(&mut self, val: u32) -> u32 {
///         // ...
///     }
///
///     // second_method is not required to be present here
/// }
/// ```
///
/// # Context
///
/// The item that you implement your definition trait for becomes your testing context.
///
/// You will receive the same testing context when calling methods multiple times,
/// thus it can be used as your chain extension testing state.
///
/// # General guidelines
///
/// Since [`#[obce::mock]`](macro@mock) is designed for off-chain testing, you are
/// limited by off-chain testing facilities that [ink! library provides](https://use.ink/basics/contract-testing).
///
/// # Complete example
///
/// ```ignore
/// #[obce::definition(id = 123)]
/// pub trait ChainExtension {
///     fn method(&mut self, val: u32, another_val: u32) -> u32;
///
///     #[obce(id = 456)]
///     fn another_method(&mut self, val: u32) -> u32;
/// }
///
/// struct MyChainExtension;
///
/// impl ChainExtension for MyChainExtension {}
///
/// #[ink::contract]
/// mod simple_contract {
///     use crate::{
///         ChainExtension,
///         MyChainExtension,
///     };
///
///     #[ink(storage)]
///     pub struct SimpleContract {}
///
///     impl SimpleContract {
///         #[ink(constructor)]
///         pub fn new() -> Self {
///             SimpleContract {}
///         }
///
///         #[ink(message)]
///         pub fn call_method(&mut self, val: u32, another_val: u32) -> u32 {
///             MyChainExtension.method(val, another_val)
///         }
///
///         #[ink(message)]
///         pub fn call_another_method(&mut self, val: u32) -> u32 {
///             MyChainExtension.another_method(val)
///         }
///     }
/// }
///
/// mod simple_test {
///     #[obce::mock]
///     impl crate::ChainExtension for () {
///         fn method(&mut self, val: u32, another_val: u32) -> u32 {
///             val + another_val
///         }
///     }
///
///     #[test]
///     fn call_contract() {
///         register_chain_extensions(());
///         let mut contract = crate::simple_contract::SimpleContract::new();
///         assert_eq!(contract.call_method(100, 200), 300);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn mock(attrs: TokenStream, enum_item: TokenStream) -> TokenStream {
    match mock::generate(attrs.into(), enum_item.into()) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
