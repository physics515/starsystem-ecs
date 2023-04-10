pub use super::Uid;
pub use location::StarEntityLocation;
pub use property::StarEntityProperty;
use serde::{Deserialize, Serialize};

mod location;
mod property;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarEntity {
	pub location: StarEntityLocation,
	pub id: Uid,
	pub name: String,
	pub properties: Vec<StarEntityProperty>,
}
