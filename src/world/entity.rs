use super::Uid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct Entity {
	pub location: Vec<(usize, Uid)>,
	pub name: String,
}
