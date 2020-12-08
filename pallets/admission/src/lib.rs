#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, 
	StorageValue,
	StorageMap,
	dispatch::DispatchResult, 
	ensure,
	traits::Randomness
};
use sp_core::H256;
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;
use country:: {CountryOwner};
use primitives::{Balance, CurrencyId, ProgrammeId, StudentId};
use unique_asset;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Section<Hash> {
	id: Hash,
	block_id: Hash,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Student {
	metadata: Vec<u8>,
}

#[cfg(test)]
mod tests;

pub trait Trait: system::Trait + country::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type BlockRandomnessSource: Randomness<H256>;
	type Currency: Get<CurrencyId>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Admission {

		pub NextStudentId get(fn next_student_id): StudentId;
		pub SectionOwner get(fn get_section_owner): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;
		pub Sections get(fn get_section): map hasher(blake2_128_concat) T::Hash => Section<T::Hash>;
		pub AllSectionCount get(fn all_section_count): u64;
		pub Students get(fn get_student): map hasher(blake2_128_concat) StudentId => Student;

		Init get(fn is_init): bool;

		Nonce get(fn nonce): u32;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		Initialized(AccountId),
		BlockRandomnessSource(H256, H256),
		StudentCreated(StudentId),
	}

);

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Attempted to initialize the token after it had already been initialized.
		AlreadyInitialized,
		//No permission section issuance
		NoPermissionSectionIssuance,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;


		#[weight = 10_000]
		fn add_student(origin, metadata: Vec<u8>) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			let student_id = Self::new_student(&owner ,metadata)?;
			
			Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}

		#[weight = 10_000]
		fn enrol_programme(origin, programme_id; ProgrammeId) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			// //Get programme info, fee for reserve
			// let native_currency_id = T::GetNativeCurrencyId::get();
			// Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}


		#[weight = 10_000]
		fn sponsor_student(origin, programme_id: ProgrammeId, student_id: StudentId, kpi_required: u8) -> DispatchResult{
			let owner = ensure_signed(origin)?;

			// //Get programme info, fee for reserve
			// let native_currency_id = T::GetNativeCurrencyId::get();
			// Self::deposit_event(RawEvent::StudentCreated(student_id.clone()));
			Ok(())
		}			
	}
}


impl<T: Trait> Module<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.

	fn new_student(owner: &T::AccountId, metadata: Vec<u8>) -> Result<StudentId, DispatchError> {
		let student_id = NextStudentId::try_mutate(|id| -> Result<StudentId, DispatchError> {
			let current_id = *id;
			*id = id
				.checked_add(One::one())
				.ok_or(Error::<T>::NoAvailableProgrammeId)?;
			Ok(current_id)
		})?;

		let student_info = Student {
			metadata,
		};

		Students::<T>::insert(student_id, student_info);

		Ok(student_id)
	}
}
