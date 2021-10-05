//! Autogenerated weights for lending
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-09-23, STEPS: `[5, ]`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=lending
// --extrinsic=*
// --steps=5
// --repeat=2
// --raw
// --output=./runtime/picasso/src/weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for lending.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> lending::WeightInfo for WeightInfo<T> {
	fn create_new_market() -> Weight {
		(97_886_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn deposit_collateral() -> Weight {
		(120_976_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn withdraw_collateral() -> Weight {
		(132_627_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn borrow() -> Weight {
		(365_975_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(19 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	fn repay_borrow() -> Weight {
		(208_171_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn initialize_block(m: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 1_318_000
			.saturating_add((117_261_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((7 as Weight).saturating_mul(m as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
	}
}
