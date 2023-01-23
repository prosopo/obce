#[obce::definition]
pub trait Trait {
    fn method(&self) -> u64;
}

#[obce::mock]
impl Trait for () {
    fn method(&self, val: u32) -> u64 {
        val as u64
    }
}

fn main() {}
