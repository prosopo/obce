#[obce::definition]
pub trait Trait {
    fn method();
}

#[obce::mock]
impl Trait for () {
    fn method() {}
}

fn main() {}
