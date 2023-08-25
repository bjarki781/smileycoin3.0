// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `6164`
		// Minimum execution time: 54_122_000 picoseconds.
		Weight::from_parts(54_834_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:0)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `6164`
		// Minimum execution time: 41_749_000 picoseconds.
		Weight::from_parts(42_193_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3577`
		// Minimum execution time: 16_008_000 picoseconds.
		Weight::from_parts(16_328_000, 0)
			.saturating_add(Weight::from_parts(0, 3577))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `393`
		//  Estimated: `3593`
		// Minimum execution time: 26_277_000 picoseconds.
		Weight::from_parts(26_932_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:1)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `6196`
		// Minimum execution time: 57_020_000 picoseconds.
		Weight::from_parts(57_661_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: NisCounterpartBalances Account (r:2 w:2)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: NisCounterpartBalances TotalIssuance (r:1 w:0)
	/// Proof: NisCounterpartBalances TotalIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `6164`
		// Minimum execution time: 50_630_000 picoseconds.
		Weight::from_parts(51_191_000, 0)
			.saturating_add(Weight::from_parts(0, 6164))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: NisCounterpartBalances Account (r:1 w:1)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391`
		//  Estimated: `3593`
		// Minimum execution time: 21_915_000 picoseconds.
		Weight::from_parts(22_295_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NisCounterpartBalances Account (r:999 w:999)
	/// Proof: NisCounterpartBalances Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: System Account (r:999 w:999)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `u` is `[1, 1000]`.
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + u * (256 ±0)`
		//  Estimated: `990 + u * (2603 ±0)`
		// Minimum execution time: 21_290_000 picoseconds.
		Weight::from_parts(21_622_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 13_372
			.saturating_add(Weight::from_parts(15_527_611, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(u.into()))
	}
}