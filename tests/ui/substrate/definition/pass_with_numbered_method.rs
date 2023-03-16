mod const_eq;

use const_eq::*;

use obce::id;

#[obce::definition]
pub trait Trait {
    #[obce(id = 0x7b)]
    fn extension_method(&self);
}

fn main() {
    assert_const_eq::<{ id!(Trait::extension_method) }, 0x7b>();
}
