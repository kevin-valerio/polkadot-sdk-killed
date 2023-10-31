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

//! A set of well-known keys used for testing.

use core::ops::Deref;
use lazy_static::lazy_static;
pub use sp_core::bandersnatch;
#[cfg(feature = "std")]
use sp_core::bandersnatch::Signature;
use sp_core::{
	bandersnatch::{Pair, Public},
	crypto::UncheckedFrom,
	ByteArray, Pair as PairT,
};
#[cfg(feature = "std")]
use std::sync::Mutex;

extern crate alloc;
use alloc::{
	collections::btree_map::BTreeMap, fmt, format, str::FromStr, string::String, vec::Vec,
};

#[cfg(not(feature = "std"))]
compile_error!("No support for bandersnatch in no-std yet.");

/// Set of test accounts.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, Ord, PartialOrd,
)]
pub enum Keyring {
	Alice,
	Bob,
	Charlie,
	Dave,
	Eve,
	Ferdie,
	One,
	Two,
}

const PUBLIC_RAW_LEN: usize = <Public as ByteArray>::LEN;

impl Keyring {
	pub fn from_public(who: &Public) -> Option<Keyring> {
		Self::iter().find(|&k| &Public::from(k) == who)
	}

	pub fn from_raw_public(who: [u8; PUBLIC_RAW_LEN]) -> Option<Keyring> {
		Self::from_public(&Public::unchecked_from(who))
	}

	pub fn to_raw_public(self) -> [u8; PUBLIC_RAW_LEN] {
		*Public::from(self).as_ref()
	}

	pub fn to_raw_public_vec(self) -> Vec<u8> {
		Public::from(self).to_raw_vec()
	}

	#[cfg(feature = "std")]
	pub fn sign(self, msg: &[u8]) -> Signature {
		Pair::from(self).sign(msg)
	}

	pub fn pair(self) -> Pair {
		Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
			.expect("static values are known good; qed")
	}

	/// Returns an iterator over all test accounts.
	pub fn iter() -> impl Iterator<Item = Keyring> {
		<Self as strum::IntoEnumIterator>::iter()
	}

	pub fn public(self) -> Public {
		self.pair().public()
	}

	pub fn to_seed(self) -> String {
		format!("//{}", self)
	}

	/// Create a crypto `Pair` from a numeric value.
	pub fn numeric(idx: usize) -> Pair {
		Pair::from_string(&format!("//{}", idx), None).expect("numeric values are known good; qed")
	}
}

impl From<Keyring> for &'static str {
	fn from(k: Keyring) -> Self {
		match k {
			Keyring::Alice => "Alice",
			Keyring::Bob => "Bob",
			Keyring::Charlie => "Charlie",
			Keyring::Dave => "Dave",
			Keyring::Eve => "Eve",
			Keyring::Ferdie => "Ferdie",
			Keyring::One => "One",
			Keyring::Two => "Two",
		}
	}
}

#[derive(Debug)]
pub struct ParseKeyringError;

impl fmt::Display for ParseKeyringError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ParseKeyringError")
	}
}

impl FromStr for Keyring {
	type Err = ParseKeyringError;

	fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
		match s {
			"Alice" => Ok(Keyring::Alice),
			"Bob" => Ok(Keyring::Bob),
			"Charlie" => Ok(Keyring::Charlie),
			"Dave" => Ok(Keyring::Dave),
			"Eve" => Ok(Keyring::Eve),
			"Ferdie" => Ok(Keyring::Ferdie),
			"One" => Ok(Keyring::One),
			"Two" => Ok(Keyring::Two),
			_ => Err(ParseKeyringError),
		}
	}
}

lazy_static! {
	static ref PRIVATE_KEYS: Mutex<BTreeMap<Keyring, Pair>> =
		Mutex::new(Keyring::iter().map(|who| (who, who.pair())).collect());
	static ref PUBLIC_KEYS: BTreeMap<Keyring, Public> = PRIVATE_KEYS
		.lock()
		.unwrap()
		.iter()
		.map(|(&who, pair)| (who, pair.public()))
		.collect();
}

impl From<Keyring> for Public {
	fn from(k: Keyring) -> Self {
		*(*PUBLIC_KEYS).get(&k).unwrap()
	}
}

impl From<Keyring> for Pair {
	fn from(k: Keyring) -> Self {
		k.pair()
	}
}

impl From<Keyring> for [u8; PUBLIC_RAW_LEN] {
	fn from(k: Keyring) -> Self {
		*(*PUBLIC_KEYS).get(&k).unwrap().as_ref()
	}
}

impl From<Keyring> for &'static [u8; PUBLIC_RAW_LEN] {
	fn from(k: Keyring) -> Self {
		PUBLIC_KEYS.get(&k).unwrap().as_ref()
	}
}

impl AsRef<[u8; PUBLIC_RAW_LEN]> for Keyring {
	fn as_ref(&self) -> &[u8; PUBLIC_RAW_LEN] {
		PUBLIC_KEYS.get(self).unwrap().as_ref()
	}
}

impl AsRef<Public> for Keyring {
	fn as_ref(&self) -> &Public {
		PUBLIC_KEYS.get(self).unwrap()
	}
}

impl Deref for Keyring {
	type Target = [u8; PUBLIC_RAW_LEN];
	fn deref(&self) -> &[u8; PUBLIC_RAW_LEN] {
		PUBLIC_KEYS.get(self).unwrap().as_ref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::{bandersnatch::Pair, Pair as PairT};

	#[test]
	fn should_work() {
		assert!(Pair::verify(
			&Keyring::Alice.sign(b"I am Alice!"),
			b"I am Alice!",
			&Keyring::Alice.public(),
		));
		assert!(!Pair::verify(
			&Keyring::Alice.sign(b"I am Alice!"),
			b"I am Bob!",
			&Keyring::Alice.public(),
		));
		assert!(!Pair::verify(
			&Keyring::Alice.sign(b"I am Alice!"),
			b"I am Alice!",
			&Keyring::Bob.public(),
		));
	}
}
