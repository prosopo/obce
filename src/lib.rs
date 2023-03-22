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

//! OBCE is a library that provides tools to create custom chain extensions
//! with automatic generation of bindings for both ink! smart contracts and
//! Substrate-based chains.
//!
//! # Usage
//!
//! [`obce::definition`](macro@definition) macro is OBCE's entrypoint. Using this macro you can
//! define the API of your chain extension for usage in both ink! and Substrate:
//!
//! ```ignore
//! #[obce::definition]
//! pub trait MyChainExtension {
//!     fn chain_extension_method(&self, val: u32) -> u64;
//! }
//! ```
//!
//! With `ink` feature enabled, [`obce::definition`](macro@definition) automatically produces
//! glue code to correctly call Substrate part of a chain extension. This glue code
//! takes care of argument encoding/decoding, identifier matching, etc.
//!
//! On the other hand, when `substrate` feature is enabled, the usage of [`obce::implementation`](macro@implementation)
//! is required to complete the chain extension implementation.
//!
//! [`obce::implementation`](macro@implementation) is used on an `impl` block to
//! generate the code necessary for usage in Substrate:
//!
//! ```ignore
//! use obce::substrate::{
//!     frame_system::Config as SysConfig,
//!     pallet_contracts::Config as ContractConfig,
//!     sp_runtime::traits::StaticLookup,
//!     ChainExtensionEnvironment,
//!     ExtensionContext
//! };
//!
//! #[obce::definition]
//! pub trait MyChainExtension {
//!     fn chain_extension_method(&self, val: u32) -> u64;
//! }
//!
//! pub struct ChainExtension;
//!
//! #[obce::implementation]
//! impl<'a, E, T, Env> MyChainExtension for ExtensionContext<'a, E, T, Env, ChainExtension>
//! where
//!     T: SysConfig + ContractConfig,
//!     <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
//!     Env: ChainExtensionEnvironment<E, T>,
//! {
//!     fn chain_extension_method(&self, val: u32) -> u64 {
//!         val as u64
//!     }
//! }
//! ```
//!
//! There are various configuration options available for both [`obce::definition`](macro@definition)
//! and [`obce::implementation`](macro@implementation), all of which are documented
//! in corresponding API sections.
//!
//! # Custom errors
//!
//! Your chain extension may have chain-specific errors, some of which
//! may terminate contract execution itself. You may use [`obce::error`](macro@error)
//! macro to create your custom error type, with an optional variant that holds critical errors:
//!
//! ```ignore
//! use obce::substrate::CriticalError;
//!
//! #[obce::error]
//! enum Error {
//!     One(u32),
//!
//!     #[obce(critical)]
//!     Two(CriticalError)
//! }
//! ```
//!
//! # Testing
//!
//! OBCE also provides infrastructure for testing your chain extension
//! using [`obce::mock`](macro@mock).
//!
//! To start testing your chain extension, mark chain extension definition
//! `impl` block as [`obce::mock`](macro@mock), and fill the `impl` block
//! with the required methods:
//!
//! ```ignore
//! #[obce::definition]
//! pub trait MyChainExtension {
//!     fn chain_extension_method(&self, val: u32) -> u64;
//! }
//!
//! // Contract code...
//!
//! mod simple_test {
//!     struct Context;
//!
//!     #[obce::mock]
//!     impl crate::ChainExtension for Context {
//!         fn chain_extension_method(&self, val: u32) -> u64 {
//!             val as u64
//!         }
//!     }
//!
//!     #[test]
//!     fn call_contract() {
//!         register_chain_extensions(Context);
//!         // Call the contract as usual
//!     }
//! }
//! ```
//!
//! For a complete usage example, as well as more details on how to use the macro
//! correctly see the [corresponding API section](macro@mock).

#![cfg_attr(not(feature = "std"), no_std)]

/// ink!-specific OBCE types
#[cfg(feature = "ink")]
pub mod ink_lang;

/// Substrate-specific OBCE types
#[cfg(feature = "substrate")]
pub mod substrate;

/// Automatically generated traits that provide the necessary information
/// about the chain extension.
pub mod codegen;

pub use obce_macro::{
    definition,
    error,
    hash,
    implementation,
    mock,
};

/// Chain extension identifier lookup.
///
/// # Description
///
/// Using [`obce::id!`](macro@id) macro, you can lookup chain extension and chain extension method identifiers.
///
/// # Example
///
/// ```
/// #[obce::definition(id = 123)]
/// pub trait ChainExtension {
///     #[obce(id = 456)]
///     fn method(&self);
/// }
///
/// assert_eq!(obce::id!(ChainExtension), 123);
/// assert_eq!(obce::id!(ChainExtension::method), 456);
/// ```
#[macro_export]
macro_rules! id {
    ($extension:ident) => {{
        <dyn $extension as ::obce::codegen::ExtensionDescription>::ID
    }};

    ($extension:ident::$method:ident) => {{
        <dyn $extension as ::obce::codegen::MethodDescription<{ ::obce::hash!($method) }>>::ID
    }};
}
