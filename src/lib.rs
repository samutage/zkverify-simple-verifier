#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Fake proof verifier — always accepts the proof as valid ✅
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn verify_proof(
            origin: OriginFor<T>,
            _proof: Vec<u8>,   // proof is ignored in this simple version
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            log::info!("User {:?} submitted a proof. Marked as valid ✅", who);
            Ok(())
        }
    }
}
