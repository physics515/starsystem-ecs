use super::{AscendedComponent, Uid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AscendedEntity<T> {
	pub name: String,
	pub id: Uid,
	pub components: Vec<AscendedComponent<T>>,
}
