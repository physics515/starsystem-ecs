use super::{StarEntityLocation, Uid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarEntityProperty {
	pub name: String,
	pub id: Uid,
	pub location: StarEntityLocation,
}
