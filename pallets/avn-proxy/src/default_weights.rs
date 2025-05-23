
//! Autogenerated weights for pallet_avn_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2025-04-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-2-182`, CPU: `AMD EPYC 7R32`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./avn-parachain-collator
// benchmark
// pallet
// --chain
// dev
// --wasm-execution=compiled
// --template
// frame-weight-template.hbs
// --pallet
// pallet_avn_proxy
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// avn_proxy_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_avn_proxy.
pub trait WeightInfo {
	fn charge_fee() -> Weight;
	fn charge_fee_in_token() -> Weight;
}

/// Weights for pallet_avn_proxy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `AvnProxy::PaymentNonces` (r:1 w:1)
	/// Proof: `AvnProxy::PaymentNonces` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::AVTTokenContract` (r:1 w:0)
	/// Proof: `TokenManager::AVTTokenContract` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn charge_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `3593`
		// Minimum execution time: 133_533_000 picoseconds.
		Weight::from_parts(136_713_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `AvnProxy::PaymentNonces` (r:1 w:1)
	/// Proof: `AvnProxy::PaymentNonces` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::AVTTokenContract` (r:1 w:0)
	/// Proof: `TokenManager::AVTTokenContract` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::Balances` (r:2 w:2)
	/// Proof: `TokenManager::Balances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn charge_fee_in_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `6108`
		// Minimum execution time: 101_182_000 picoseconds.
		Weight::from_parts(102_273_000, 6108)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `AvnProxy::PaymentNonces` (r:1 w:1)
	/// Proof: `AvnProxy::PaymentNonces` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::AVTTokenContract` (r:1 w:0)
	/// Proof: `TokenManager::AVTTokenContract` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn charge_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `3593`
		// Minimum execution time: 133_533_000 picoseconds.
		Weight::from_parts(136_713_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `AvnProxy::PaymentNonces` (r:1 w:1)
	/// Proof: `AvnProxy::PaymentNonces` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::AVTTokenContract` (r:1 w:0)
	/// Proof: `TokenManager::AVTTokenContract` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TokenManager::Balances` (r:2 w:2)
	/// Proof: `TokenManager::Balances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	fn charge_fee_in_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `6108`
		// Minimum execution time: 101_182_000 picoseconds.
		Weight::from_parts(102_273_000, 6108)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}