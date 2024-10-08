// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod imports {
	pub use codec::Encode;
	// Substrate
	pub use frame_support::{
		assert_ok,
		sp_runtime::{AccountId32, DispatchResult},
		traits::fungibles::Inspect,
	};

	// Polkadot
	pub use xcm::prelude::*;

	// Cumulus
	pub use asset_test_utils::xcm_helpers;
	pub use emulated_integration_tests_common::xcm_emulator::{
		assert_expected_events, bx, Chain, Parachain as Para, RelayChain as Relay, Test, TestArgs,
		TestContext, TestExt,
	};
	pub use parachains_common::Balance;
	pub use westend_system_emulated_network::{
		self,
		people_westend_emulated_chain::{
			genesis::ED as PEOPLE_WESTEND_ED,
			people_westend_runtime::{
				people, xcm_config::XcmConfig as PeopleWestendXcmConfig,
				ExistentialDeposit as PeopleWestendExistentialDeposit, Runtime as PeopleRuntime,
			},
			PeopleWestendParaPallet as PeopleWestendPallet,
		},
		westend_emulated_chain::{
			genesis::ED as WESTEND_ED,
			westend_runtime::{
				xcm_config::XcmConfig as WestendXcmConfig, BasicDeposit, ByteDeposit,
				MaxAdditionalFields, MaxSubAccounts, Runtime as WestendRuntime,
				RuntimeOrigin as WestendOrigin, SubAccountDeposit,
			},
			WestendRelayPallet as WestendPallet,
		},
		PeopleWestendPara as PeopleWestend, PeopleWestendParaReceiver as PeopleWestendReceiver,
		PeopleWestendParaSender as PeopleWestendSender, WestendRelay as Westend,
		WestendRelayReceiver as WestendReceiver, WestendRelaySender as WestendSender,
	};

	pub type RelayToSystemParaTest = Test<Westend, PeopleWestend>;
	pub type SystemParaToRelayTest = Test<PeopleWestend, Westend>;
}

#[cfg(test)]
mod tests;
