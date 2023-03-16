mod const_eq;

use const_eq::*;

use obce::id;

#[obce::definition]
pub trait Trait {}

fn main() {
    assert_const_eq::<{ id!(Trait) }, 0xd401>();
}
