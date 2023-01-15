pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::OriginFor;

    use crate::test_pallet::weights::WeightInfo;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type WeightInfo: WeightInfo;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[allow(unused_variables)]
        #[pallet::weight(T::WeightInfo::test(*val))]
        pub fn test_method(
            _origin: OriginFor<T>,
            val: u64,
        ) -> DispatchResult {
            Ok(())
        }
    }
}

mod weights {
    use frame_support::weights::Weight;

    pub trait WeightInfo {
        fn test(val: u64) -> Weight;
    }

    impl<I> WeightInfo for I {
        fn test(val: u64) -> Weight {
            Weight::from_ref_time(val)
        }
    }
}
