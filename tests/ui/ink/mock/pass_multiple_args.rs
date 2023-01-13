#[obce::definition]
pub trait Trait {
    fn method(&self, val: u32, another_val: u64) -> u64;
}

#[obce::mock]
impl Trait for () {
    fn method(&self, _: u32, another_val: u64) -> u64 {
        another_val
    }
}

fn main() {
    register_chain_extensions(());
}
