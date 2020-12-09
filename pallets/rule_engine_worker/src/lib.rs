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
use frame_system::{self as system, ensure_signed, ensure_root};
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;
use primitives::{ Balance, CurrencyId, ProgrammeId, BlockNumber, StudentId, RuleId};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Section<Hash> {
	id: Hash,
	block_id: Hash,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
	start_block: BlockNumber,
	end_block: BlockNumber,
	metadata: Vec<u8>,
}

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as SectionModule {

		pub NextRuleId get(fn next_rule_id): RuleId;
		pub Rules get(fn get_section): map hasher(blake2_128_concat) RuleId => Rule;
		pub AllRuleCount get(fn all_section_count): u64;

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

		#[weight= 10_000]
		pub fn create_rule(origin, metadata: Vec<u8>) -> DispatchResult{
			let sender = ensure_signed(origin)?;

			Ok(())
		}

		#[weight= 10_000]
		pub fn log_kpi(origin, student_id: StudentId, programme_id: ProgrammeId ,metadata: Vec<u8>) -> DispatchResult{
			let sender = ensure_root(origin)?;

			Ok(())
		}

		#[weight= 10_000]
		pub fn refund_admission(origin, student_id: StudentId, programme_id: ProgrammeId) -> DispatchResult{
			let sender = ensure_root(origin)?;

			Ok(())
		}
			
	}
}

