// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! Runtime modules for parachains code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod configuration;
pub mod inclusion;
pub mod inclusion_inherent;
pub mod initializer;
pub mod paras;
pub mod scheduler;
pub mod session_info;
pub mod origin;
pub mod dmp;
pub mod ump;
pub mod hrmp;
pub mod reward_points;

pub mod runtime_api_impl;

mod util;

#[cfg(test)]
mod mock;

pub use origin::{Origin, ensure_parachain};
use primitives::v1::Id as ParaId;
use parity_scale_codec::{Encode, Decode};
use sp_core::RuntimeDebug;

/// Schedule a para to be initialized at the start of the next session with the given genesis data.
pub fn schedule_para_initialize<T: paras::Config>(
	id: ParaId,
	genesis: paras::ParaGenesisArgs,
) {
	<paras::Module<T>>::schedule_para_initialize(id, genesis);
}

/// Trait to trigger parachain cleanup.
#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait ParachainCleanup {
	fn schedule_para_cleanup(id: ParaId);
}

/// Helper struct which contains all the needed parachain cleanups.
pub struct AllParachainCleanup<T>(core::marker::PhantomData<T>);
impl<T> ParachainCleanup for AllParachainCleanup<T>
where
	T: paras::Config
	+ dmp::Config
	+ ump::Config
	+ hrmp::Config,
{
	fn schedule_para_cleanup(id: ParaId) {
		<paras::Module<T>>::schedule_para_cleanup(id);
		<dmp::Module<T>>::schedule_para_cleanup(id);
		<ump::Module<T>>::schedule_para_cleanup(id);
		<hrmp::Module<T>>::schedule_para_cleanup(id);
	}
}

/// TODO: doc
pub fn schedule_para_upgrade<T: paras::Config>(id: ParaId) {
	paras::Module::<T>::schedule_para_upgrade(id);
}

/// TODO: doc
pub fn schedule_para_downgrade<T: paras::Config>(id: ParaId) {
	paras::Module::<T>::schedule_para_downgrade(id);
}

/// The possible states of a para, to take into account delayed lifecycle changes.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum ParaLifecycle {
	/// Para ID is new and is onboarding as a Parathread.
	OnboardingAsParathread,
	/// Para ID is new and is onboarding as a Parachain.
	OnboardingAsParachain,
	/// Para ID is a Parathread.
	Parathread,
	/// Para ID is a Parachain.
	Parachain,
	/// Para ID is a Parathread which is upgrading to a Parachain.
	UpgradingToParachain,
	/// Para ID is a Parachain which is downgrading to a Parathread.
	DowngradingToParathread,
	/// Para ID is being offboarded.
	Outgoing,
}
