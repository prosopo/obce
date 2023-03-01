mod test_pallet;

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&mut self);
}

#[obce::implementation]
impl<'a, E, T, Env> ChainExtensionDefinition for ExtensionContext<'a, E, T, Env, ChainExtension>
where
    T: SysConfig + ContractConfig + crate::test_pallet::Config,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    Env: ChainExtensionEnvironment<E, T>,
    E: Ext<T = T>
{
    #[obce(weight())]
    fn extension_method(&mut self) {
        crate::test_pallet::Pallet::<T>::test_method(
            RawOrigin::Signed(self.env.ext().address().clone()).into(),
            123
        ).unwrap();
    }
}

fn main() {}
