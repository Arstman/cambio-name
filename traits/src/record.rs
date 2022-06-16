// Copyright 2022 Cambio Network
// This file is part of Cambio Name
// License: Apache 2.0

#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use frame_support::pallet_prelude::MaxEncodedLen;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{DispatchError, DispatchResult, RuntimeDebug};
use sp_std::{cmp::Eq, result::Result, vec::Vec};
use async_std::net::IpAddr;
use xcm::latest::{MultiAsset, MultiLocation};
use crate::primitives::*;

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RecordInfo<BoundedString> {
	pub id: RecordId,
	pub target: RecordTarget,
    //the name which the record is belong to.
	pub name_id: NameId,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RecordTarget<BoundedString, LinkString> {
	///the LinkString is for Web3 xoss chain only, for example it can be "moonbeam", "PolKadot", 
    /// to point out which chain this record is target to. 
	/// for Web2 records no needed.
	Web3Contract(LinkString, BoundedString),
	Web3Account(LinkString, BoundedString),
	Web3NFT(LinkString,BoundedString),
	//Key types to represent something like only shows as hash
	Web3Key(LinkString, BoundedString),
	//for something like ipfs://xxx.xxx.xx
	Web3URI(LinkString, BoundedString),
	MultiAsset,
	//most of web2 dns record types
	Web2A(IpAddr::V4),
	Web2AAA(IpAddr::V6),
	Web2Cname(BoundedString),
	Web2Txt(BoundedString),
	Web2MX(BoundedString, MXPriority),
	Web2URL(BoundedString),
}


pub trait Record<BoundedString, AccountId, BoundedPart> {

	fn record_add(
		sender: AccountId,
		nft_id: NftId,
		sender: RecordTarget<BoundedString>
	) -> Result<RecordId, DispatchError>;

	fn record_remove(
		sender: AccountId,
		nft_id: NftId,
		record_id: RecordId,
	) -> DispatchResult;

	
}