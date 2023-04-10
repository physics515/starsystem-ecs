use super::{AscendedEntity, Uid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AscendedBeing<T> {
	pub name: String,
	pub id: Uid,
	pub entities: Vec<AscendedEntity<T>>,
}
