use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::Config as ContractConfig,
    sp_runtime::traits::StaticLookup,
    ChainExtensionEnvironment,
    ExtensionContext
};

#[obce::error]
pub enum Error {
    #[obce(ret_val = "100")]
    One,

    #[obce(ret_val = "200")]
    Two
}

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&self) -> Result<(), Error>;
}

#[obce::implementation]
impl<'a, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, E, T, Env, ChainExtension>
where
    T: SysConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    Env: ChainExtensionEnvironment<E, T>,
{
    #[obce(ret_val)]
    fn extension_method(&self) -> Result<(), Error> {
        todo!()
    }
}

fn main() {}
