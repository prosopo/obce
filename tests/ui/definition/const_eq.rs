pub trait ConstEq<const A: u16, const B: u16> {}

impl<const A: u16> ConstEq<A, A> for () {}

pub fn assert_const_eq<const A: u16, const B: u16>() where (): ConstEq<A, B> {}
