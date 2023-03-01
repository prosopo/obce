mod test_pallet;

use obce::substrate::{
    frame_system::{Config as SysConfig, RawOrigin},
    pallet_contracts::{
        chain_extension::Ext,
        Config as ContractConfig,
    },
    sp_runtime::traits::StaticLookup,
    ChainExtensionEnvironment,
    ExtensionContext
};

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&mut self, val: u64, another_val: u64);
}

#[obce::implementation]
impl<'a, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, E, T, Env, ChainExtension>
where
    T: SysConfig + ContractConfig + crate::test_pallet::Config,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    Env: ChainExtensionEnvironment<E, T>,
    E: Ext<T = T>
{
    #[obce(weight(dispatch = "crate::test_pallet::Pallet::<T>::test_method"))]
    fn extension_method(&mut self, val: u64, another_val: u64) {
        crate::test_pallet::Pallet::<T>::test_method(
            RawOrigin::Signed(self.env.ext().address().clone()).into(),
            val,
            another_val
        ).unwrap();
    }
}

fn main() {}
