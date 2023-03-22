#[obce::definition]
pub trait Trait {
    #[obce(id = 123)]
    fn extension_method(&self);

    #[obce(id = 456)]
    fn another_extension_method(&self);
}

fn main() {}
