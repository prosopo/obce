use obce::substrate::{
    frame_support::dispatch::Weight,
    frame_system::Config as SysConfig,
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_runtime::traits::StaticLookup,
    ExtensionContext
};

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&mut self);
}

#[obce::implementation]
impl<'a, 'b, E, T> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, ChainExtension>
where
    T: SysConfig + ContractConfig,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
{
    #[obce(weight(expr = "Weight::from_ref_time(123)"))]
    fn extension_method(&mut self) {}
}

fn main() {}
