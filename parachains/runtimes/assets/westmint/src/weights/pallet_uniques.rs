// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westmint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_uniques
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/westmint/src/weights/pallet_uniques.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	// Storage: Uniques NextCollectionId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(27_462_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques NextCollectionId (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_create() -> Weight {
		Weight::from_ref_time(16_480_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Storage: Uniques Attribute (r:0 w:1000)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Storage: Uniques InstanceMetadataOf (r:0 w:1000)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Storage: Uniques Account (r:0 w:20)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(10_956_000 as RefTimeWeight).saturating_mul(n as RefTimeWeight))
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(1_654_000 as RefTimeWeight).saturating_mul(m as RefTimeWeight))
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(1_572_000 as RefTimeWeight).saturating_mul(a as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((2 as RefTimeWeight).saturating_mul(n as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(m as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(a as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Storage: Uniques Account (r:0 w:1)
	fn mint() -> Weight {
		Weight::from_ref_time(36_074_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:1)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn burn() -> Weight {
		Weight::from_ref_time(37_583_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Account (r:0 w:2)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(28_134_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques Asset (r:100 w:100)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(12_715_000 as RefTimeWeight).saturating_mul(i as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn freeze() -> Weight {
		Weight::from_ref_time(21_019_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn thaw() -> Weight {
		Weight::from_ref_time(21_907_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn freeze_collection() -> Weight {
		Weight::from_ref_time(16_894_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn thaw_collection() -> Weight {
		Weight::from_ref_time(16_650_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:2)
	fn transfer_ownership() -> Weight {
		Weight::from_ref_time(25_095_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	fn set_team() -> Weight {
		Weight::from_ref_time(17_428_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassAccount (r:0 w:1)
	fn force_item_status() -> Weight {
		Weight::from_ref_time(20_018_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn set_attribute() -> Weight {
		Weight::from_ref_time(41_955_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	// Storage: Uniques Attribute (r:1 w:1)
	fn clear_attribute() -> Weight {
		Weight::from_ref_time(40_201_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn set_metadata() -> Weight {
		Weight::from_ref_time(33_630_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	fn clear_metadata() -> Weight {
		Weight::from_ref_time(34_054_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:1)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn set_collection_metadata() -> Weight {
		Weight::from_ref_time(33_283_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques ClassMetadataOf (r:1 w:1)
	fn clear_collection_metadata() -> Weight {
		Weight::from_ref_time(31_298_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn approve_transfer() -> Weight {
		Weight::from_ref_time(22_430_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Asset (r:1 w:1)
	fn cancel_approval() -> Weight {
		Weight::from_ref_time(23_005_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	fn set_accept_ownership() -> Weight {
		Weight::from_ref_time(20_468_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	fn set_collection_max_supply() -> Weight {
		Weight::from_ref_time(19_948_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:0)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	fn set_price() -> Weight {
		Weight::from_ref_time(19_487_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Storage: Uniques ItemPriceOf (r:1 w:1)
	// Storage: Uniques Class (r:1 w:0)
	// Storage: Uniques Account (r:0 w:2)
	fn buy_item() -> Weight {
		Weight::from_ref_time(39_973_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
}
