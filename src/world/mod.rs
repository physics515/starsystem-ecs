use super::Uid;
use component::Component;
use entity::Entity;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use strum::IntoEnumIterator;
pub use enum_index::EnumIndex;

mod component;
mod entity;
mod enum_index;

/// A collection of components of a given type.
pub type CompMap<T> = BTreeMap<Uid, Component<T>>;

/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World<T> {
	pub id: Uid,
	pub indexes: Arc<Mutex<Vec<usize>>>,
	pub entities_count: Arc<Mutex<usize>>,
	pub entities: Arc<Mutex<BTreeMap<Uid, Entity>>>,
	pub components: Arc<Mutex<BTreeMap<usize, CompMap<T>>>>,
}

impl<'a, T: 'static + Sync + Send + Serialize + Deserialize<'a> + IntoEnumIterator + PartialEq + EnumIndex + Clone + Default + Debug> World<T> {
	// creates a new world
	pub async fn new() -> Self {
		let entities_count = Arc::new(Mutex::new(0));
		let entities = Arc::new(Mutex::new(BTreeMap::new()));
		let components = Arc::new(Mutex::new(BTreeMap::new()));
		let indexes = Arc::new(Mutex::new(Vec::new()));
		for (i, _) in &mut T::iter().enumerate() {
			indexes.lock().unwrap().push(i);
			components.lock().unwrap().insert(i, BTreeMap::new());
		}
		Self { id: Uid::new(), indexes, entities_count, entities, components }
	}

	// has component
	pub fn has_component(&self, component_id: Uid) -> bool {
		self.components.lock().unwrap().iter().any(|(_, comps)| comps.contains_key(&component_id))
	}

	// has entity
	pub fn has_entity(&self, entity_id: Uid) -> bool {
		self.entities.lock().unwrap().contains_key(&entity_id)
	}

	// creates a new entity in the world
	// adds 1 to the entities_count
	// adds a new entity to the entities vec
	pub async fn create_entity(&mut self, name: String) -> Result<Uid, String> {
		let id = Uid::new();
		let location = Vec::new();
		let entity = Entity { location, name };
		self.entities.lock().unwrap().insert(id.clone(), entity);
		*self.entities_count.lock().unwrap() += 1;
		Ok(id)
	}

	// set entity
	// create a new entity from provided id and name
	// if the entity already exists, it will be overwritten
	pub async fn set_entity(&mut self, id: Uid, name: String) -> Result<Uid, String> {
		let location = Vec::new();
		let entity = Entity { location, name };
		self.entities.lock().unwrap().insert(id.clone(), entity);
		Ok(id)
	}

	// add component to entity
	// adds a component to the component vec where the index is the index of the component in the enum
	// adds the location of the component to the entity
	pub async fn add_component_to_entity(&mut self, entity: Uid, component: T, component_name: String) -> Result<Uid, String> {
		let index = T::index(&component);
		let id = Uid::new();
		let comp: Component<T> = Component { name: component_name, data: component };
		self.components.lock().unwrap().get_mut(&index).unwrap().insert(id.clone(), comp);
		self.entities.lock().unwrap().par_iter_mut().find_any(|e| *e.0 == entity).unwrap().1.location.push((index, id.clone()));
		Ok(id)
	}

	// set component to entity
	// adds a component to the component vec where the index is the index of the component in the enum
	// adds the location of the component to the entity
	pub async fn set_component_to_entity(&mut self, entity: Uid, component: T, component_name: String, component_id: Uid) -> Result<Uid, String> {
		let index = T::index(&component);
		let comp: Component<T> = Component { name: component_name, data: component };
		self.components.lock().unwrap().get_mut(&index).unwrap().insert(component_id.clone(), comp);
		self.entities.lock().unwrap().par_iter_mut().find_any(|e| *e.0 == entity).unwrap().1.location.push((index, component_id.clone()));
		Ok(component_id)
	}

	// removes a component from an entity
	// removes the component from the component vec where the index is the index of the component in the enum
	// removes the location of the component from the entity
	pub async fn remove_component_from_entity(&mut self, entity: Uid, component: Uid) -> Result<(), String> {
		/* let ent: Entity = self
		.entities
		.lock()
		.unwrap()
		.par_iter()
		.find_any(|e| *e.0 == entity)
		.unwrap()
		.1
		.clone(); */
		let mut index = 0;
		let mut component_name = Uid::new();
		let components = self.components.lock().unwrap().clone();
		for (i, c) in components.iter() {
			if c.par_iter().find_any(|c| *c.0 == component).is_some() {
				index = *i;
				component_name = c.par_iter().find_any(|c| *c.0 == component).unwrap().0.clone();
				break;
			}
		}
		self.components.lock().unwrap().get_mut(&index).unwrap().remove(&component_name);

		// remove the component from the entity locations
		self.entities.lock().unwrap().par_iter_mut().find_any(|e| *e.0 == entity).unwrap().1.location.retain(|c| c.1 != component);
		assert!(self.entities.lock().unwrap().par_iter().find_any(|e| *e.0 == entity).unwrap().1.location.par_iter().find_any(|c| c.1 == component_name).is_none());
		Ok(())
	}

	// return component for givien Uid
	// returns a vec of components for given type
	pub async fn get_components_of_type(&self, t: T) -> Result<BTreeMap<Uid, Component<T>>, String> {
		let index = T::index(&t);
		Ok(self.components.lock().unwrap().get(&index).unwrap().clone())
	}

	// replace a vec of components with a given vec<T>
	pub async fn set_components(&mut self, components: BTreeMap<Uid, Component<T>>) -> Result<BTreeMap<Uid, Component<T>>, String> {
		let index = components.iter().next().unwrap().1.data.index();
		self.components.lock().unwrap().insert(index, components.clone());
		Ok(components)
	}

	// set a component for a given component Uid
	pub async fn set_component(&mut self, component: Uid, data: T) -> Result<Uid, String> {
		let components = self.components.lock().unwrap().clone();
		let mut index = 0;
		for (i, c) in components.iter() {
			if c.par_iter().find_any(|c| *c.0 == component).is_some() {
				index = *i;
				break;
			}
		}
		let mut comp = self.components.lock().unwrap().get_mut(&index).unwrap().get_mut(&component).unwrap().clone();
		comp.data = data;
		self.components.lock().unwrap().get_mut(&index).unwrap().insert(component.clone(), comp);
		Ok(component)
	}

	// removes an entity from the world
	// removes the entity from the entities vec
	// removes the components from the components vec
	// removes 1 from the entities_count
	pub async fn remove_entity(&mut self, entity: Uid) -> Result<(), String> {
		let ent: Entity = if let Some(e) = self.entities.lock().unwrap().par_iter().find_any(|e| *e.0 == entity) {
			e.1.clone()
		} else {
			return Err(format!("entity: {} not found", entity));
		};
		for (index, component) in ent.location {
			self.components.lock().unwrap().get_mut(&index).unwrap().remove(&component);
		}
		self.entities.lock().unwrap().retain(|i, _| *i != entity);
		*self.entities_count.lock().unwrap() -= 1;
		Ok(())
	}

	pub async fn get_entity_components(&self, entity: Uid) -> Result<Vec<(Uid, Component<T>)>, String> {
		let ent: Entity = self.entities.lock().unwrap().par_iter().find_any(|e| *e.0 == entity).unwrap().1.clone();
		let mut components = Vec::new();
		for (index, component) in ent.location {
			let comp = self.components.lock().unwrap().get(&index).unwrap().get(&component).unwrap().clone();
			components.push((component, comp));
		}
		Ok(components)
	}
}
