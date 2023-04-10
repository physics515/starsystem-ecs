use super::Uid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarEntityLocation {
	pub world: Uid,
	pub entity: Uid,
}
