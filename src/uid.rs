use random_string::generate;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash, Ord)]
#[serde(untagged)]
pub enum Uid {
	Value(String),
}

impl FromStr for Uid {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Uid::Value(s.to_string()))
	}
}

impl Display for Uid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Uid::Value(value) => write!(f, "{}", value),
		}
	}
}

impl Uid {
	pub fn new() -> Self {
		let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
		Self::Value(generate(128, charset))
	}
}

impl Default for Uid {
	fn default() -> Self {
		Self::new()
	}
}
