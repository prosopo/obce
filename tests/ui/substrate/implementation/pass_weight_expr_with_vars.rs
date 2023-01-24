use obce::substrate::{
    frame_support::dispatch::Weight,
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_core::crypto::UncheckedFrom,
    sp_runtime::traits::StaticLookup,
    ExtensionContext
};

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&mut self, val: u64);
}

#[obce::implementation]
impl<'a, 'b, E, T> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, ChainExtension>
where
    T: SysConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
    <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
{
    #[obce(weight(expr = "Weight::from_ref_time(_val)"))]
    fn extension_method(&mut self, _val: u64) {}
}

fn main() {}
