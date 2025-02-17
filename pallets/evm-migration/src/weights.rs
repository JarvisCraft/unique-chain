// Template adopted from https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs

//! Autogenerated weights for pallet_evm_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 29.0.0
//! DATE: 2023-11-29, STEPS: `50`, REPEAT: `80`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-host`, CPU: `Intel(R) Core(TM) i7-8700 CPU @ 3.20GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/production/unique-collator
// benchmark
// pallet
// --pallet
// pallet-evm-migration
// --wasm-execution
// compiled
// --extrinsic
// *
// --template=.maintain/frame-weight-template.hbs
// --steps=50
// --repeat=80
// --heap-pages=4096
// --output=./pallets/evm-migration/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_evm_migration.
pub trait WeightInfo {
	fn begin() -> Weight;
	fn set_data(b: u32, ) -> Weight;
	fn finish(b: u32, ) -> Weight;
	fn insert_eth_logs(b: u32, ) -> Weight;
	fn insert_events(b: u32, ) -> Weight;
}

/// Weights for pallet_evm_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:1)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodes` (r:1 w:0)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn begin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `3593`
		// Minimum execution time: 12_831_000 picoseconds.
		Weight::from_parts(13_198_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:0)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountStorages` (r:0 w:80)
	/// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 80]`.
	fn set_data(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `96`
		//  Estimated: `3494`
		// Minimum execution time: 6_654_000 picoseconds.
		Weight::from_parts(7_176_462, 3494)
			// Standard Error: 650
			.saturating_add(Weight::from_parts(1_606_380, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
	}
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:1)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodes` (r:0 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 80]`.
	fn finish(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `96`
		//  Estimated: `3494`
		// Minimum execution time: 9_339_000 picoseconds.
		Weight::from_parts(9_803_079, 3494)
			// Standard Error: 62
			.saturating_add(Weight::from_parts(1_632, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// The range of component `b` is `[0, 200]`.
	fn insert_eth_logs(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_902_000 picoseconds.
		Weight::from_parts(4_080_566, 0)
			// Standard Error: 347
			.saturating_add(Weight::from_parts(1_089_412, 0).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 200]`.
	fn insert_events(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_098_000 picoseconds.
		Weight::from_parts(4_486_259, 0)
			// Standard Error: 688
			.saturating_add(Weight::from_parts(2_019_978, 0).saturating_mul(b.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:1)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodes` (r:1 w:0)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn begin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `3593`
		// Minimum execution time: 12_831_000 picoseconds.
		Weight::from_parts(13_198_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:0)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountStorages` (r:0 w:80)
	/// Proof: `EVM::AccountStorages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 80]`.
	fn set_data(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `96`
		//  Estimated: `3494`
		// Minimum execution time: 6_654_000 picoseconds.
		Weight::from_parts(7_176_462, 3494)
			// Standard Error: 650
			.saturating_add(Weight::from_parts(1_606_380, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(b.into())))
	}
	/// Storage: `EvmMigration::MigrationPending` (r:1 w:1)
	/// Proof: `EvmMigration::MigrationPending` (`max_values`: None, `max_size`: Some(29), added: 2504, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodes` (r:0 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[0, 80]`.
	fn finish(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `96`
		//  Estimated: `3494`
		// Minimum execution time: 9_339_000 picoseconds.
		Weight::from_parts(9_803_079, 3494)
			// Standard Error: 62
			.saturating_add(Weight::from_parts(1_632, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// The range of component `b` is `[0, 200]`.
	fn insert_eth_logs(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_902_000 picoseconds.
		Weight::from_parts(4_080_566, 0)
			// Standard Error: 347
			.saturating_add(Weight::from_parts(1_089_412, 0).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 200]`.
	fn insert_events(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_098_000 picoseconds.
		Weight::from_parts(4_486_259, 0)
			// Standard Error: 688
			.saturating_add(Weight::from_parts(2_019_978, 0).saturating_mul(b.into()))
	}
}

