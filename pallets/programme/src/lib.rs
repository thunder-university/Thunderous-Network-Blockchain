#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage,
	dispatch::DispatchResult,
	ensure,
	traits::{Get, IsType, Randomness},
	StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_root, ensure_signed};
use primitives::{Balance, CurrencyId, ProgrammeId};
use sp_core::H256;
use sp_runtime::{
	print,
	traits::{AccountIdConversion, Zero, One, Hash},
	DispatchError, ModuleId, RuntimeDebug,
};
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Programme<AccountId> {
	pub owner: AccountId,
	pub metadata: Vec<u8>,
	pub fee: Balance,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct ProgrammeFund<AccountId, Balance> {
	pub vault: AccountId,
	pub value: Balance,
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type ModuleId: Get<ModuleId>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Programme {

		pub NextProgrammeId get(fn next_programme_id): ProgrammeId;
		pub AllProgrammesCount get(fn all_programmes_count): u64;
		pub FreezingProgrammes get (fn get_freezing_programme): map hasher(twox_64_concat) ProgrammeId => Option<()>;
		pub Programmes get(fn get_programme): map hasher(twox_64_concat) ProgrammeId => Option<Programme<T::AccountId>>;
		pub ProgrammeTresury get (fn get_programme_treasury): map hasher(twox_64_concat) ProgrammeId => Option<ProgrammeFund<T::AccountId, Balance>>;
		pub ProgrammeOwner get(fn get_programme_owner): double_map hasher(twox_64_concat) ProgrammeId, hasher(twox_64_concat) T::AccountId => Option<()>;

		Init get(fn is_init): bool;
		Nonce get(fn nonce): u32;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		NewProgrammeCreated(ProgrammeId),
		TransferredProgramme(ProgrammeId, AccountId, AccountId),
		ProgrammeFreezed(ProgrammeId),
		ProgrammeUnFreezed(ProgrammeId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		ProgrammeInfoNotFound,
		//No permission
		NoPermission,
		NoAvailableProgrammeId,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		#[weight = 10_000]
		fn create_programme(origin, fee: Balance ,metadata: Vec<u8>) -> DispatchResult {

			let owner = ensure_signed(origin)?;

			let programme_id = Self::new_programme(&owner, fee ,metadata)?;

			// All programme will be held into treasury fund, only release once the programme completed
			let fund_id = T::ModuleId::get().into_sub_account(programme_id);

			//Programme treasury
			let programme_fund = ProgrammeFund {
				vault: fund_id,
				value: 0,
			};
			ProgrammeTresury::<T>::insert(programme_id, programme_fund);

			ProgrammeOwner::<T>::insert(programme_id, owner, ());

			let total_programmes_count = Self::all_programmes_count();

			let new_total_programmes_count = total_programmes_count.checked_add(One::one()).ok_or("Overflow adding new count to total programmes")?;
			AllProgrammesCount::put(new_total_programmes_count);
			Self::deposit_event(RawEvent::NewProgrammeCreated(programme_id.clone()));

			Ok(())
		}

		#[weight = 10_000]
		fn freeze_programme(origin, programme_id: ProgrammeId) -> DispatchResult {
			//Only Council can free a programme
			ensure_root(origin)?;

			FreezingProgrammes::insert(programme_id, ());
			Self::deposit_event(RawEvent::ProgrammeFreezed(programme_id));

			Ok(())
		}

		#[weight = 10_000]
		fn unfreeze_programme(origin, programme_id: ProgrammeId) -> DispatchResult {
			//Only Council can free a programme
			ensure_root(origin)?;

			FreezingProgrammes::try_mutate(programme_id, |freeze_programme| -> DispatchResult{
				// ensure!(freeze_programme.take().is_some(), Error::<T>::ProgrammeInfoNotFound);

				Self::deposit_event(RawEvent::ProgrammeUnFreezed(programme_id));
				Ok(())
			})
		}
	}
}

impl<T: Trait> Module<T> {
	/// Reads the nonce from storage, increments the stored nonce, and returns
	/// the encoded nonce to the caller.

	fn new_programme(owner: &T::AccountId, fee: Balance ,metadata: Vec<u8>) -> Result<ProgrammeId, DispatchError> {
		let programme_id = NextProgrammeId::try_mutate(|id| -> Result<ProgrammeId, DispatchError> {
			let current_id = *id;
			*id = id
				.checked_add(One::one())
				.ok_or(Error::<T>::NoAvailableProgrammeId)?;
			Ok(current_id)
		})?;

		let programme_info = Programme {
			owner: owner.clone(),
			fee: fee,
			metadata,
		};

		Programmes::<T>::insert(programme_id, programme_info);

		Ok(programme_id)
	}
}
