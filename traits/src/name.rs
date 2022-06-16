// Copyright 2022 Cambio Network
// This file is part of Cambio Name
// License: Apache 2.0

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::cmp::Eq;

use frame_support::pallet_prelude::*;
use sp_runtime::Permill;

use crate::primitives::*;
use sp_std::result::Result;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Royalty information (recipient and amount), inspired by RMRK Royalty, 
/// can be use for rent a name 
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RoyaltyInfo<AccountId> {
	/// Recipient (AccountId) of the royalty
	pub recipient: AccountId,
	/// Amount (Permill) of the royalty
	pub amount: Permill,
}

/// Name info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct NameInfo<AccountId, BoundedString> {
	/// The owner of the Name
	pub owner: AccountId,
    /// The TLD of th Name
    pub tld: TldId,
	/// Royalty (optional)
	pub royalty: Option<RoyaltyInfo<AccountId>>,
	/// Pending state (if sent to Name)
	pub transferable: bool,
}
/// Abstraction over a Name system.
#[allow(clippy::upper_case_acronyms)]
pub trait Name<AccountId, BoundedString> {
	fn name_mint(
		sender: AccountId,
		owner: AccountId,
		tld_id: TldId,
		royalty_recipient: Option<AccountId>,
		royalty_amount: Option<Permill>,
		transferable: bool,
	) -> Result<(TldId, NameId), DispatchError>;
    
	fn name_burn(
		tld_id: TldId,
		name_id: NameId,
		max_recursions: u32,
	) -> Result<(TldId, NameId), DispatchError>;

	fn name_send(
		sender: AccountId,
		tld_id: TldId,
		name_id: NameId,
		new_owner: AccountId,
	) -> Result<(AccountId, bool), DispatchError>;
}
