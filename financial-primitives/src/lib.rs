// Copyright (C) 2020 equilibrium.
// SPDX-License-Identifier: Apache-2.0
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
// http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod capvec;

use frame_support::dispatch::DispatchError;
use sp_std::iter::Iterator;

pub struct Asset;

pub trait OnPriceSet {
    type Asset;
    type Price;

    fn on_price_set(asset: Self::Asset, value: Self::Price) -> Result<(), DispatchError>;
}

pub trait IntoTypeIterator: Sized {
    type Iterator: Iterator<Item = Self>;

    fn into_type_iter() -> Self::Iterator;
}
