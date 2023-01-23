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

/// Chain extension description.
///
/// This trait is automatically implemented with `#[obce::definition]` macro expansion.
pub trait ExtensionDescription {
    /// Unique chain extension identifier.
    const ID: u16;
}

/// Chain extension method description.
///
/// `METHOD_HASH` generic is dependent solely on the method name,
/// while [`ID`](MethodDescription::ID) can be changed via `#[obce(id = ...)]` macro.
pub trait MethodDescription<const METHOD_HASH: u32> {
    /// Unique chain extension method identifier.
    const ID: u16;

    /// Method input type, that is required for chain extension calls.
    type Input;

    /// Method output type, that you can use to obtain results from chain extension calls.
    type Output;
}
