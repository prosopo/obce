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

use frame_system::Config as SysConfig;
use pallet_contracts::chain_extension::{
    BufInBufOutState,
    Environment,
    Ext,
};
use sp_runtime::DispatchError;

/// Chain extension context that you can use with your implementations.
pub struct ExtensionContext<'a, 'b, E: Ext, T, Extension>
where
    T: SysConfig,
    E: Ext<T = T>,
{
    /// Chain extension environment.
    pub env: Environment<'a, 'b, E, BufInBufOutState>,

    /// Custom chain extension storage.
    pub storage: &'a mut Extension,
}

impl<'a, 'b, E: Ext, T, Extension> ExtensionContext<'a, 'b, E, T, Extension>
where
    T: SysConfig,
    E: Ext<T = T>,
{
    pub fn new(storage: &'a mut Extension, env: Environment<'a, 'b, E, BufInBufOutState>) -> Self {
        ExtensionContext { env, storage }
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
