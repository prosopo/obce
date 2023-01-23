#[obce::definition]
pub trait Trait {}

#[obce::mock]
impl Trait for () {}

fn main() {
    register_chain_extensions(());
}
