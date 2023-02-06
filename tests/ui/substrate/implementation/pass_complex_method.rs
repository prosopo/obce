use obce::substrate::{
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_core::crypto::UncheckedFrom,
    sp_runtime::traits::StaticLookup,
    ChainExtensionEnvironment,
    CriticalError,
    ExtensionContext
};

#[obce::error]
pub enum Error {
    SomeError,
    #[obce(critical)]
    SomeCriticalError(CriticalError)
}

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&self, val: u32) -> Result<u32, Error>;
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
    fn extension_method(&self, val: u32) -> Result<u32, Error> {
        Ok(val)
    }
}

fn main() {}
