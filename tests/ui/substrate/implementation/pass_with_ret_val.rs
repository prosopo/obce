use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_runtime::traits::StaticLookup,
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
impl<'a, 'b, E, T> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, ChainExtension>
where
    T: SysConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
{
    #[obce(ret_val)]
    fn extension_method(&self) -> Result<(), Error> {
        todo!()
    }
}

fn main() {}
