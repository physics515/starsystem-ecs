use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize)]
pub struct Component<T> {
	pub name: String,
	pub data: T,
}
