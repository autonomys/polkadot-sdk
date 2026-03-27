// This file is part of Substrate.

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

fn main() {
	#[cfg(feature = "std")]
	{
		// Try to build WASM, fall back to dummy if outside polkadot-sdk workspace.
		// with_current_project() panics when this crate is used as a transitive
		// dependency in an external workspace (e.g. via frame-benchmarking-cli).
		let result = std::panic::catch_unwind(|| {
			substrate_wasm_builder::WasmBuilder::new()
				.with_current_project()
				.export_heap_base()
				.import_memory()
				.disable_runtime_version_section_check()
				.build();
		});
		if result.is_err() {
			let out_dir = std::env::var("OUT_DIR").unwrap();
			let path = std::path::Path::new(&out_dir).join("wasm_binary.rs");
			std::fs::write(
				path,
				"pub const WASM_BINARY: Option<&[u8]> = None;\n\
				 pub const WASM_BINARY_BLOATY: Option<&[u8]> = None;\n",
			)
			.unwrap();
		}
	}
}
