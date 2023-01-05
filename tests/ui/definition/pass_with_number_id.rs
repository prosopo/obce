mod const_eq;

use const_eq::*;

#[obce::definition(id = 0x13)]
pub trait Trait {}

fn main() {
    assert_const_eq::<{ <dyn Trait as obce::codegen::ExtensionDescription>::ID }, 0x13>();
}
