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

mod is_critical_error;

pub use frame_support;
pub use frame_system;
pub use is_critical_error::{
    ToCriticalErr,
    ToCriticalErrFallback,
};
pub use pallet_contracts;
pub use sp_core;
pub use sp_runtime;
pub use sp_std;

use core::marker::PhantomData;

use frame_support::weights::Weight;
use frame_system::Config as SysConfig;
use pallet_contracts::chain_extension::{
    BufInBufOutState,
    Environment,
    Ext,
    RetVal,
    UncheckedFrom,
};
use scale::Decode;
use sp_runtime::DispatchError;

pub trait ChainExtensionEnvironment<E, T> {
    type ChargedAmount;

    fn func_id(&self) -> u16;

    fn ext_id(&self) -> u16;

    fn in_len(&self) -> u32;

    fn read_as_unbounded<U: Decode>(&mut self, len: u32) -> Result<U, CriticalError>;

    fn write(&mut self, buffer: &[u8], allow_skip: bool, weight_per_byte: Option<Weight>) -> Result<(), CriticalError>;

    fn ext(&mut self) -> &mut E;

    fn charge_weight(&mut self, amount: Weight) -> Result<Self::ChargedAmount, CriticalError>;

    fn adjust_weight(&mut self, charged: Self::ChargedAmount, actual_weight: Weight);
}

impl<'a, 'b, E, T> ChainExtensionEnvironment<E, T> for Environment<'a, 'b, E, BufInBufOutState>
where
    T: SysConfig,
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{
    type ChargedAmount = pallet_contracts::ChargedAmount;

    fn func_id(&self) -> u16 {
        Environment::func_id(self)
    }

    fn ext_id(&self) -> u16 {
        Environment::ext_id(self)
    }

    fn in_len(&self) -> u32 {
        Environment::in_len(self)
    }

    fn read_as_unbounded<U: Decode>(&mut self, len: u32) -> Result<U, CriticalError> {
        Environment::read_as_unbounded(self, len)
    }

    fn write(&mut self, buffer: &[u8], allow_skip: bool, weight_per_byte: Option<Weight>) -> Result<(), CriticalError> {
        Environment::write(self, buffer, allow_skip, weight_per_byte)
    }

    fn ext(&mut self) -> &mut E {
        Environment::ext(self)
    }

    fn charge_weight(&mut self, amount: Weight) -> Result<Self::ChargedAmount, CriticalError> {
        Environment::charge_weight(self, amount)
    }

    fn adjust_weight(&mut self, charged: Self::ChargedAmount, actual_weight: Weight) {
        Environment::adjust_weight(self, charged, actual_weight)
    }
}

impl<E, T, Env> ChainExtensionEnvironment<E, T> for &mut Env
where
    Env: ChainExtensionEnvironment<E, T>,
{
    type ChargedAmount = Env::ChargedAmount;

    fn func_id(&self) -> u16 {
        <Env as ChainExtensionEnvironment<E, T>>::func_id(self)
    }

    fn ext_id(&self) -> u16 {
        <Env as ChainExtensionEnvironment<E, T>>::ext_id(self)
    }

    fn in_len(&self) -> u32 {
        <Env as ChainExtensionEnvironment<E, T>>::in_len(self)
    }

    fn read_as_unbounded<U: Decode>(&mut self, len: u32) -> Result<U, CriticalError> {
        <Env as ChainExtensionEnvironment<E, T>>::read_as_unbounded(self, len)
    }

    fn write(&mut self, buffer: &[u8], allow_skip: bool, weight_per_byte: Option<Weight>) -> Result<(), CriticalError> {
        <Env as ChainExtensionEnvironment<E, T>>::write(self, buffer, allow_skip, weight_per_byte)
    }

    fn ext(&mut self) -> &mut E {
        <Env as ChainExtensionEnvironment<E, T>>::ext(self)
    }

    fn charge_weight(&mut self, amount: Weight) -> Result<Self::ChargedAmount, CriticalError> {
        <Env as ChainExtensionEnvironment<E, T>>::charge_weight(self, amount)
    }

    fn adjust_weight(&mut self, charged: Self::ChargedAmount, actual_weight: Weight) {
        <Env as ChainExtensionEnvironment<E, T>>::adjust_weight(self, charged, actual_weight)
    }
}

pub trait CallableChainExtension<E, T, Env> {
    fn call(&mut self, env: Env) -> Result<RetVal, CriticalError>;
}

/// Chain extension context that you can use with your implementations.
pub struct ExtensionContext<'a, E, T, Env: ChainExtensionEnvironment<E, T>, Extension> {
    /// Chain extension environment.
    pub env: Env,

    /// Custom chain extension storage.
    pub storage: &'a mut Extension,

    pre_charged: Option<Env::ChargedAmount>,

    _ghost: PhantomData<(E, T)>,
}

impl<'a, E, T, Env, Extension> ExtensionContext<'a, E, T, Env, Extension>
where
    Env: ChainExtensionEnvironment<E, T>,
{
    pub fn new(storage: &'a mut Extension, env: Env, pre_charged: Option<Env::ChargedAmount>) -> Self {
        ExtensionContext {
            env,
            storage,
            pre_charged,
            _ghost: Default::default(),
        }
    }

    pub fn pre_charged(&mut self) -> Option<Env::ChargedAmount> {
        self.pre_charged.take()
    }
}

pub type CriticalError = DispatchError;

/// The trait allows filtering error on critical and non-critical errors.
///
/// Critical errors terminate the execution of the chain extension, while
/// non-critical errors are propagated to the caller contract via buffer.
pub trait SupportCriticalError: Sized {
    /// Try to convert an error to a critical one.
    ///
    /// Returns [`Ok`] if the conversion was successful (and the current
    /// error should be qualified as critical), and [`Err`] otherwise.
    fn try_to_critical(self) -> Result<CriticalError, Self>;
}
