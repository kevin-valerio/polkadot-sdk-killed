// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_fast_unstake`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-163-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_fast_unstake
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_fast_unstake`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fast_unstake::WeightInfo for WeightInfo<T> {
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: FastUnstake Head (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking SlashingSpans (r:64 w:0)
	// Storage: Staking Bonded (r:64 w:64)
	// Storage: Staking Validators (r:64 w:0)
	// Storage: Staking Nominators (r:64 w:0)
	// Storage: System Account (r:64 w:64)
	// Storage: Balances Locks (r:64 w:64)
	// Storage: Staking Ledger (r:0 w:64)
	// Storage: Staking Payee (r:0 w:64)
	fn on_idle_unstake() -> Weight {
		// Minimum execution time: 2_522_692 nanoseconds.
		Weight::from_ref_time(2_542_008_000)
			.saturating_add(T::DbWeight::get().reads(389))
			.saturating_add(T::DbWeight::get().writes(321))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: FastUnstake Head (r:1 w:1)
	// Storage: FastUnstake Queue (r:65 w:64)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasStakers (r:4 w:0)
	/// The range of component `x` is `[2, 256]`.
	fn on_idle_check(x: u32, ) -> Weight {
		// Minimum execution time: 2_657_214 nanoseconds.
		Weight::from_ref_time(2_672_893_000)
			// Standard Error: 548_838
			.saturating_add(Weight::from_ref_time(868_095_065).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(72))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(66))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: FastUnstake Queue (r:1 w:1)
	// Storage: FastUnstake Head (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:2 w:2)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	fn register_fast_unstake() -> Weight {
		// Minimum execution time: 159_946 nanoseconds.
		Weight::from_ref_time(164_798_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: FastUnstake Queue (r:1 w:1)
	// Storage: FastUnstake Head (r:1 w:0)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	fn deregister() -> Weight {
		// Minimum execution time: 67_054 nanoseconds.
		Weight::from_ref_time(70_067_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:0 w:1)
	fn control() -> Weight {
		// Minimum execution time: 4_535 nanoseconds.
		Weight::from_ref_time(4_750_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
