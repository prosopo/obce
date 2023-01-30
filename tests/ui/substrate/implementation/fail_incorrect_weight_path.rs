mod test_pallet;

pub struct ChainExtension;

#[obce::definition]
pub trait ChainExtensionDefinition {
    fn extension_method(&mut self);
}

#[obce::implementation]
impl<'a, 'b, E, T> ChainExtensionDefinition for ExtensionContext<'a, 'b, E, T, ChainExtension>
where
    T: SysConfig + ContractConfig + crate::test_pallet::Config,
    <<T as SysConfig>::Lookup as StaticLookup>::Source: From<<T as SysConfig>::AccountId>,
    E: Ext<T = T>,
{
    #[obce(weight(dispatch = "test_method"))]
    fn extension_method(&mut self) {
        crate::test_pallet::Pallet::<T>::test_method(
            RawOrigin::Signed(self.env.ext().address().clone()).into(),
            123
        ).unwrap();
    }
}

fn main() {}
