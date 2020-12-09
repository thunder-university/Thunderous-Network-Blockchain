#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, ensure,
	traits::{Currency, ReservableCurrency},
	Parameter,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
	print,
	traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub, Member, One, Printable, Zero},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::vec::Vec;

// mod mock;
// mod tests;

pub type CollectionId = u64;
pub type AssetId = u64;
pub type RentId = u64;

pub trait Trait: frame_system::Trait + pallet_balances::Trait {
	/// The Asset ID type
	// type AssetId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_error! {
	/// Error for non-fungible-token module.
	pub enum Error for Module<T: Trait> {
		// No available Collection Id
		NoAvailableCollectionId,
		/// No available Asset ID
		NoAvailableAssetId,
		/// Collection not found
		CollectionNotFound,
		/// Asset(CollectionId, AssetId) not found
		AssetNotFound,
		/// The operator is not the owner of the token and has no permission
		NoPermission,
		/// Arithmetic calculation overflow
		NumOverflow,
		/// Can not destroy asset
		/// Total issuance is not 0
		CannotDestroyAsset,
	}
}

decl_event!(
	pub enum Event<T>
	where
		AssetId = AssetId,
		AccountId = <T as system::Trait>::AccountId
	{
		NewCertificateCreated(AssetId, AccountId),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as NonFungibleToken {
		/// Next collection id
		pub NextCollectionId get(fn next_collection_id): CollectionId;
		/// Next available asset id per collection.
		pub NextAssetId get(fn next_asset_id): map hasher(twox_64_concat) CollectionId => AssetId;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn mint_certificate(origin, to: T::AccountId, metadata: Vec<u8>) -> DispatchResult{
			let from = ensure_signed(origin)?;

			// ensure!(AssetByOwner::<T>::contains_key(&from, &asset), Error::<T>::NoPermission);

			// Self::transfer_from(from, to, asset.0 ,asset.1)?;

			Ok(())
		}
	}
}