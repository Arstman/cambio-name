//Copyright 2022 Cambio Network
//This file is part of cambio-name, heavily inspired by RMRK sbustrate.
//License: Apache 2.0

use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};

use crate::primitives::*;
use sp_std::result::Result;

/// TLD info, a TLD is like a nft collection, but only the sudoer can mint
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TLDInfo<BoundedString, BoundedSymbol, AccountId> {
	/// Current bidder and bid price.
	pub issuer: AccountId,
	pub metadata: Option<BoundedString>,
	pub max: Option<u32>,
	pub symbol: BoundedSymbol,
	pub domain_count: u32,
}

/// Abstraction over a TLD system.
#[allow(clippy::upper_case_acronyms)]
pub trait Tld<BoundedString, BoundedSymbol, AccountId> {
	fn issuer(tld_id: TldId) -> Option<AccountId>;
	fn tld_create(
		issuer: AccountId,
		metadata: Option<BoundedString>,
		max: Option<u32>,
		symbol: BoundedSymbol,
	) -> Result<TldId, DispatchError>;
	fn tld_burn(issuer: AccountId, collection_id: TldId) -> DispatchResult;
	fn tld_change_issuer(
		tld_id: TldId,
		new_issuer: AccountId,
	) -> Result<(AccountId, TldId), DispatchError>;
	fn tld_lock(
		sender: AccountId,
		tld_id: TldId,
	) -> Result<TldId, DispatchError>;
}

