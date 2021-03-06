/*
 Copyright (c) 2022 ParallelChain Lab

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/// List of methods available for the macro in ParallelChain Mainnet SDK.

// `contract_init` macro transforms idiomatic rust smart contracts into
// contracts that are readable and deployable by ParallelChain Mainnet Fullnode.
mod contract_init;
#[allow(unused_imports)]
pub use contract_init::*;

// `sdk_method_transform` provides typed methods to ParallelChain Mainnet SDK
// that allow direct interaction with the world state using custom data types
// such as structs.
mod sdk_method_transform;
#[allow(unused_imports)]
pub use sdk_method_transform::*;

mod compilation_error;
#[allow(unused_imports)]
pub use compilation_error::*;

mod contract;
#[allow(unused_imports)]
pub use contract::*;

mod use_contract;
#[allow(unused_imports)]
pub use use_contract::*;
