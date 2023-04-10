use super::StarEntity;
use super::Uid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Being {
	pub id: Uid,
	pub entities: Vec<StarEntity>,
	pub name: String,
}

impl Being {
	pub fn new(name: String) -> Self {
		Self { id: Uid::new(), entities: Vec::new(), name }
	}
}
