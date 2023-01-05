mod const_eq;

use const_eq::*;

#[obce::definition(id = "pallet-assets-chain-extension@v0.1")]
pub trait Trait {}

fn main() {
    assert_const_eq::<{ <dyn Trait as obce::codegen::ExtensionDescription>::ID }, 0x48f6>();
}
