use super::Uid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AscendedComponent<T> {
	pub name: String,
	pub id: Uid,
	pub data: T,
}
