pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {}

#[obce::implementation]
impl ChainExtensionDefinition for ExtensionContext {}

fn main() {}
