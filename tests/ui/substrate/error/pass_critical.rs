use obce::substrate::CriticalError;

#[obce::error]
enum Error {
    One(u32),
    #[obce(critical)]
    Two(CriticalError)
}

fn assert_encode_holds<T: scale::Encode>(_: T) {}

fn main() {
    assert_encode_holds(Error::One(123));
}
