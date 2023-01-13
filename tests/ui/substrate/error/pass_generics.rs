pub trait Trait {}

impl Trait for i32 {}

#[obce::error]
enum Error<A, B>
where
    A: Trait
{
    One(A),
    Two(B)
}

fn assert_encode_holds<T: scale::Encode>(_: T) {}

fn main() {
    assert_encode_holds(Error::<_, u32>::One(123));
}
