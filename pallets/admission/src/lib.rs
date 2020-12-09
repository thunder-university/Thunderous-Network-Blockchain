#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{StorageMap, StorageValue, debug, decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_root, ensure_signed};
use primitives::{Balance, CurrencyId, ProgrammeId, StudentId};
use sp_core::H256;
use sp_runtime::{
	print,
	traits::{AccountIdConversion, Zero, One, Hash},
	DispatchError, RuntimeDebug,
};
use sp_std::vec::Vec;

// mod mock;
// mod tests;

pub type CollectionId = u64;
pub type AssetId = u64;
pub type RentId = u64;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Student {
	metadata: Vec<u8>,
}

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
		AccountId = <T as system::Trait>::AccountId,
		AssetId = AssetId,
	{
		StudentCreated(StudentId),
		NewAssetCreated(AssetId),
		TransferedAsset(AccountId, AccountId, AssetId),
		NewAssetRented(AssetId, RentId),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Admission {
		pub NextStudentId get(fn next_student_id): StudentId;
		pub Students get(fn get_student): map hasher(blake2_128_concat) StudentId => Student;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
		
		#[weight= 10_000]
		pub fn add_student(origin, metadata: Vec<u8>) -> DispatchResult{

			// let student_id = Self::new_student(&owner ,metadata)?;
			
			// Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}

		#[weight= 10_000]
		pub fn enrol_programme(origin, programme_id: ProgrammeId) -> DispatchResult{

			// //Get programme info, fee for reserve
			// let native_currency_id = T::GetNativeCurrencyId::get();
			// Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}


		#[weight= 10_000]
		pub fn sponsor_student(origin, programme_id: ProgrammeId, student_id: StudentId, kpi_required: u8) -> DispatchResult{

			// //Get programme info, fee for reserve
			// let native_currency_id = T::GetNativeCurrencyId::get();
			// Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}	
	}
}
