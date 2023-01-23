#[obce::error]
enum Error {
    One,
    Two
}

fn assert_encode_holds<T: scale::Encode>(_: T) {}

fn main() {
    assert_encode_holds(Error::One);
}
