mod const_eq;

use const_eq::*;

#[obce::definition]
pub trait Trait {}

fn main() {
    assert_const_eq::<{ <dyn Trait as obce::codegen::ExtensionDescription>::ID }, 0xd401>();
}
