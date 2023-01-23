mod const_eq;

use const_eq::*;

#[obce::definition]
pub trait Trait {
    #[obce(id = 0x7b)]
    fn extension_method(&self);
}

fn main() {
    assert_const_eq::<{ <dyn Trait as obce::codegen::MethodDescription<0x3eae5bbc>>::ID }, 0x7b>();
}
