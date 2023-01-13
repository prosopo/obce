#[obce::definition]
pub trait Trait {
    fn method(&self);
}

#[obce::mock]
impl Trait for () {
    fn method(&self) {}
}

fn main() {
    register_chain_extensions(());
}
