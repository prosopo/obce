use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_core::crypto::UncheckedFrom,
    sp_runtime::traits::StaticLookup,
    ChainExtensionEnvironment,
    ExtensionContext
};

#[obce::error]
pub enum Error {
    #[obce(ret_val = "10_001")]
    FirstError,
    
    #[obce(ret_val = "10_002")]
    SecondError
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
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    Env: ChainExtensionEnvironment<E, T>
{
    #[obce(ret_val)]
    fn extension_method(&self) -> Result<(), Error> {
        todo!()
    }
}

fn main() {}
