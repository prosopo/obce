#[obce::definition]
pub trait Trait {}

#[obce::mock]
impl Trait for () {
    fn method(&self) {}
}

fn main() {}
