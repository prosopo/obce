use obce::substrate::ExtensionContext;

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {}

#[obce::implementation]
impl<'a, 'b, E, T> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, ChainExtension> {}

fn main() {}
