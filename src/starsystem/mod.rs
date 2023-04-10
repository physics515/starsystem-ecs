use super::EnumIndex;
pub use super::Uid;
use super::World;
pub use ascend::{AscendedBeing, AscendedComponent, AscendedEntity};
pub use being::Being;
use serde::{Deserialize, Serialize};
pub use starentity::{StarEntity, StarEntityLocation, StarEntityProperty};
use std::collections::BTreeMap;
use std::fmt::Debug;
use strum::IntoEnumIterator;

mod ascend;
mod being;
mod starentity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem<T> {
	pub worlds: BTreeMap<Uid, World<T>>,
	pub beings: Vec<Being>,
}

impl<T: 'static + Sync + Send + Serialize + for<'a> Deserialize<'a> + IntoEnumIterator + PartialEq + EnumIndex + Clone + Default + Debug> StarSystem<T> {
	/*
			StarSystem actions
	*/

	// Create a new starsystem
	pub async fn new() -> Self {
		Self { worlds: BTreeMap::new(), beings: Vec::new() }
	}

	// Create a new world
	async fn create_world(&mut self) -> Result<Uid, String> {
		let world = World::<T>::new().await;
		self.worlds.insert(world.id.clone(), world.clone());
		Ok(world.id)
	}

	// Create a new being
	pub async fn conceive_being(&mut self, name: String) -> Result<Uid, String> {
		let being = Being::new(name);
		self.beings.push(being.clone());
		Ok(being.id)
	}

	// set being
	// create a new being with provided id and name
	// if the being already exists, it will be overwritten
	pub async fn set_being(&mut self, id: Uid, name: String) -> Result<Uid, String> {
		let being = Being { id, entities: Vec::new(), name };
		self.beings.push(being.clone());
		Ok(being.id)
	}

	// kill being
	// remove all entities from each world that the being owns
	// remove being from beings
	pub async fn kill_being(&mut self, id: Uid) -> Result<(), String> {
		let mut being = None;
		let mut being_index = None;
		for (i, b) in self.beings.iter().enumerate() {
			if b.id == id {
				being = Some(b);
				being_index = Some(i);
				break;
			}
		}

		if let Some(being) = being {
			for entity in being.entities.iter() {
				if let Some(world) = self.worlds.get_mut(&entity.location.world) {
					world.remove_entity(entity.id.clone()).await.unwrap();
				}
			}
			self.beings.remove(being_index.unwrap());
			Ok(())
		} else {
			Err(format!("Being with id {} does not exist", id))
		}
	}

	// get being by id
	pub async fn get_being(&self, id: Uid) -> Result<Being, String> {
		for being in self.beings.iter() {
			if being.id == id {
				return Ok(being.clone());
			}
		}
		Err(format!("Being with id {} does not exist", id))
	}

	// constitue being
	// if no worlds exist, create one
	// create a new entity on a random world
	// add entity to being
	pub async fn constitute_being(&mut self, being: Uid, entity_name: String) -> Result<Uid, String> {
		if self.worlds.is_empty() {
			self.create_world().await.unwrap();
		}

		// if enitity exist on being with the same name, remove it
		if let Some(b) = self.beings.iter_mut().find(|b| b.id == being) {
			if let Some(e) = b.entities.iter_mut().find(|e| e.name == entity_name) {
				// get world
				if let Some(world) = self.worlds.get_mut(&e.location.world) {
					// remove entity
					world.remove_entity(e.id.clone()).await.unwrap();
				}
			}

			// remove entity from being
			b.entities.retain(|e| e.name != entity_name);
		}

		let world = self.worlds.iter().nth(rand::random::<usize>() % self.worlds.len()).unwrap().0.clone();
		let ent = self.worlds.get_mut(&world).unwrap().create_entity(entity_name.clone()).await.unwrap();
		let entity = StarEntity { location: StarEntityLocation { world, entity: ent.clone() }, id: ent, name: entity_name, properties: Vec::new() };
		for b in self.beings.iter_mut() {
			if b.id == being {
				b.entities.push(entity.clone());
				break;
			}
		}
		Ok(entity.id)
	}

	// dissolve entity
	// remove entity from being
	// remove entity from world
	pub async fn dissolve_entity(&mut self, being: Uid, entity: Uid) -> Result<(), String> {
		if let Some(b) = self.beings.iter_mut().find(|b| b.id == being) {
			if let Some(e) = b.entities.iter_mut().find(|e| e.id == entity) {
				// get world
				if let Some(world) = self.worlds.get_mut(&e.location.world) {
					// remove entity
					world.remove_entity(e.id.clone()).await.unwrap();
				}
			}

			// remove entity from being
			b.entities.retain(|e| e.id != entity);
		}
		Ok(())
	}

	// add property to entity
	pub async fn add_property(&mut self, being: Uid, entity: Uid, property: T, property_name: String) -> Result<Uid, String> {
		let mut id: Option<Uid> = None;
		if let Some(b) = self.beings.iter_mut().find(|b| b.id == being) {
			if let Some(e) = b.entities.iter_mut().find(|e| e.id == entity) {
				// get world
				if let Some(world) = self.worlds.get_mut(&e.location.world) {
					// add property
					id = Some(world.add_component_to_entity(entity.clone(), property, property_name.clone()).await.unwrap());
				}
			}
		}

		// add property to entity
		if let Some(id) = id {
			for b in self.beings.iter_mut() {
				if b.id == being {
					for e in b.entities.iter_mut() {
						if e.id == entity {
							let location: StarEntityLocation = StarEntityLocation { world: e.location.world.clone(), entity: id.clone() };
							let prop: StarEntityProperty = StarEntityProperty { location, id: id.clone(), name: property_name };
							e.properties.push(prop);
							break;
						}
					}
					break;
				}
			}
			Ok(id)
		} else {
			Err("Could not add property to entity".to_string())
		}
	}

	// set property
	pub async fn set_property(&mut self, being: Uid, entity: Uid, property: Uid, value: T, name: String) -> Result<Uid, String> {
		let mut id: Option<Uid> = None;
		if let Some(b) = self.beings.iter_mut().find(|b| b.id == being) {
			if let Some(e) = b.entities.iter_mut().find(|e| e.id == entity) {
				// get world
				if let Some(world) = self.worlds.get_mut(&e.location.world) {
					// add property
					id = Some(world.set_component_to_entity(entity.clone(), value.clone(), name.clone(), property.clone()).await.unwrap());

					// update property
					for p in e.properties.iter_mut() {
						if p.id == property {
							p.name = name;
							break;
						}
					}
				}
			}
		}
		Ok(id.unwrap())
	}

	// remove property by id
	pub async fn remove_property(&mut self, property: Uid) -> Result<(), String> {
		for b in self.beings.iter_mut() {
			for e in b.entities.iter_mut() {
				if let Some(p) = e.properties.iter_mut().find(|p| p.id == property) {
					// get world
					if let Some(world) = self.worlds.get_mut(&e.location.world) {
						// remove property
						world.remove_component_from_entity(e.id.clone(), p.id.clone()).await.unwrap();
					}
					// remove property from entity
					e.properties.retain(|p| p.id != property);
					break;
				}
			}
		}
		Ok(())
	}

	// set property given property id
	pub async fn set_property_by_id(&mut self, property_id: Uid, property_value: T) -> Result<Uid, String> {
		if let Some(world) = self.worlds.iter_mut().find(|w| w.1.has_component(property_id.clone())) {
			world.1.set_component(property_id.clone(), property_value.clone()).await.unwrap();
		}
		Ok(property_id)
	}

	// develop being
	// accepts a being id and a vecor tuple of (entity_name, entity)
	// if no worlds exist, create one
	// loop over entities and create a new entity on a random world
	// add entities to being
	pub async fn develop_being(&mut self, being: Uid, ascended_beings: Vec<AscendedBeing<T>>) -> Result<Vec<Uid>, String> {
		if self.worlds.is_empty() {
			self.create_world().await.unwrap();
		}

		let mut entities = Vec::new();
		for ascended_being in ascended_beings.iter() {
			let world = self.worlds.iter().nth(rand::random::<usize>() % self.worlds.len()).unwrap().0.clone();
			let be = self.get_being(being.clone()).await.unwrap();
			for e in ascended_being.entities.iter() {
				let ent = self.worlds.get_mut(&world.clone()).unwrap().set_entity(e.id.clone(), e.name.clone()).await.unwrap();
				let mut entity = StarEntity { location: StarEntityLocation { world: world.clone(), entity: ent.clone() }, id: ent.clone(), name: e.name.clone(), properties: Vec::new() };

				for b in self.beings.iter_mut() {
					if b.id == be.id.clone() {
						b.entities.push(entity.clone());
						break;
					}
				}

				// add properties
				for c in e.components.iter() {
					//let prop = self.set_property(being.clone(), ent.clone(), c.id.clone(), c.data.clone(), c.name.clone()).await.unwrap();
					entity.properties.push(StarEntityProperty { location: StarEntityLocation { world: world.clone(), entity: ent.clone() }, id: c.id.clone(), name: c.name.clone() });
				}

				// update entities on being
				for b in self.beings.iter_mut() {
					if b.id == be.id.clone() {
						for e in b.entities.iter_mut() {
							if e.id == ent.clone() {
								e.properties = entity.properties.clone();
								break;
							}
						}
						break;
					}
				}

				/* for c in e.components {
					let prop = self.set_property(being.clone(), ent.clone(), c.id.clone(), c.data.clone(), c.name.clone()).await.unwrap();
					entity.properties.push(StarEntityProperty { location: StarEntityLocation { world: world.clone(), entity: ent.clone() }, id: c.id.clone(), name: c.name });
				} */
				entities.push(entity.id.clone());
			}
		}
		Ok(entities)
	}

	// ascend being
	// accepts a being id
	// returns BTreeMap<bening_name, AscendedBeing<T> { id: being_id, entities: BTreeMap<entity_name, T> }>>
	pub async fn ascend_being(&mut self, being: Uid) -> Result<Vec<AscendedBeing<T>>, String> {
		let mut res: Vec<AscendedBeing<T>> = Vec::new();
		for b in self.beings.iter() {
			if b.id == being {
				let mut entities: Vec<AscendedEntity<T>> = vec![];
				for entity in b.entities.iter() {
					let world = self.worlds.get_mut(&entity.location.world).unwrap();
					let components = world.get_entity_components(entity.id.clone()).await.unwrap();
					let mut new_component: Vec<AscendedComponent<T>> = vec![];
					for (id, component) in components.iter() {
						new_component.push(AscendedComponent { id: id.clone(), name: component.name.clone(), data: component.data.clone() });
					}
					entities.push(AscendedEntity { id: entity.id.clone(), name: entity.name.clone(), components: new_component });
				}
				res.push(AscendedBeing { name: b.name.clone(), id: b.id.clone(), entities });
			}
		}
		Ok(res)
	}
}
