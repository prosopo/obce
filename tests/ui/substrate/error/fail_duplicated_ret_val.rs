use obce::substrate::pallet_contracts::chain_extension::RetVal;

#[obce::error]
enum Error {
    #[obce(ret_val = "100")]
    One,

    #[obce(ret_val = "100")]
    Two
}

fn assert_encode_holds<T: scale::Encode>(_: T) {}
fn assert_try_from_holds<T>(_: T) where RetVal: TryFrom<T> {}

fn main() {
    assert_encode_holds(Error::One);
    assert_try_from_holds(Error::One);
}
