mod const_eq;

use const_eq::*;

use obce::id;

#[obce::definition(id = "pallet-assets-chain-extension@v0.1")]
pub trait Trait {}

fn main() {
    assert_const_eq::<{ id!(Trait) }, 0x48f6>();
}
