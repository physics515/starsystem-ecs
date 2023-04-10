//! ## What is this?
//! StarSystem is a fully serializable Entity Component System(ECS) with an
//! extra layer of abstraction. It is designed to be used for business
//! applications but can be used in games or for any application that needs
//! the flexibility of an ECS.
//! 
//! ## What are the benefits of using StarSystem?
//! - **Fully Serializable** - StarSystem is fully serializable. This means
//! that you can save the state of your application and load it back in
//! later.
//! - **Fast** - StarSystem is fast. It uses a lot of parallelism to make
//! sure that your application is as fast as possible.
//! 
//! ## What makes it different from other ECSs?
//! - **Additional Layer of Use** - When a starsystem is created, you must
//! pass an enum that contains all of the types of components that you want
//! to use. StarSystem will then create a new World for each type provided.
//! Beings live within the starsystem and consist of a collection of Entities.
//! Entities live on the Worlds and consist of a collection of Properties 
//! (Components in other ECSs). Properties must be of a type that is provided
//! in the enum. This means that a Being can hold any number of Properties of
//! any type that is provided in the enum.
//! 
//! ## How do I use it?
//! ### Defining the types of components
//! First, you must define the types of components that you want to use. This
//! is done by creating an enum that contains all of the types of components
//! that you want to use. The enum must implement the `EnumIndex` trait.
//! 
//! ```rust
//! use starsystem::{EnumIndex, StarSystem, AscendedBeing};
//! use strum_macros::EnumIter;
//! use futures::executor::block_on;
//!
//! #[derive(EnumIter, Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
//! pub enum Edification {
//!     String(String),
//!     Number(usize),
//!     Boolean(bool),
//!     #[default]
//!     None,
//! }
//! 
//! impl EnumIndex for Edification {
//!     fn index(&self) -> usize {
//!         match self {
//!             Edification::String(_) => 0,
//!             Edification::Number(_) => 1,
//!             Edification::Boolean(_) => 2,
//!             Edification::None => 3,
//!         }
//!     }
//! }
//! ```
//! 
//! ### Creating a new starsystem and loading it with data
//! Next, you must create a new starsystem. This is done by calling the
//! `new` method on the `StarSystem` struct. You must pass the enum that
//! contains all of the types of components that you want to use.
//! 
//! ```rust
//! ...
//! fn main() {
//!     block_on(async_main());
//! }
//! 
//! async fn async_main() {
//!     // create a new starsystem
//!     let mut starsystem = StarSystem::<Edification>::new().await;
//! 
//!     // create a new being named "config"
//!     let configuration = starsystem
//!         .conceive_being("config".to_string())
//!         .await
//!         .unwrap();
//! 
//!     // attach an entity "headers" to the being "config"
//!     let headers = starsystem
//!         .constitute_being(configuration.clone(), "headers".to_string())
//!         .await
//!         .unwrap();
//! 
//!     // add a property "Content-Type" to the entity "headers"
//!     starsystem
//!         .add_property(
//!             configuration.clone(),
//!             headers.clone(),
//!             Edification::String("application/json".to_string()),
//!             "Content-Type".to_string(),
//!         )
//!         .await
//!         .unwrap();
//! 
//!     // ascend the configuration to recieve it in a more human readable format
//!     let ascended_configuration = starsystem
//!         .ascend_being(configuration.clone())
//!         .await
//!         .unwrap();
//! 
//!     println!(
//!         "{}",
//!         serde_json::to_string_pretty(&ascended_configuration).unwrap()
//!     );
//! }
//! ...
//! ```
//! 
//! The output of the above code will be:
//! ```json
//! [
//!   {
//!     "name": "config",
//!     "id": "XbwpaoljoLIVcvEorzTMXSLTrClXCYRhobtoApxdnOCrPuSadUGZLstWaughFrVNCYkIErKnGUoLTvUNlPpNCutHvpXQGKvzsLCLOqooWhruxdnypKpjcWyQUYMAwuzH",
//!     "entities": [
//!       {
//!         "name": "headers",
//!         "id": "vNKPtfCwUpREgWSEBXnGDCfCxTXmIAVHPyYCOJGkyBSuHHonpUhOaPaZYzdTUGLnjxJQOHDSbaIJTSKXONoPmCkHYhCPgRuInnmRwOsCrxRqNKkVkcvNCejMRziuryHl",
//!         "components": [
//!           {
//!             "name": "Content-Type",
//!             "id": "ZyBMvhnNppLvZGCxtykAikHNsjhgzLMwdnXWpZSVKVybJyfdtKiObVdkngUSssHqkaXfdnFLQwhWjSZATWxVYJRBgimLGJtwrmgVtMzHdIQiMfMePtWRdlgltEqtsPLX",
//!             "data": {
//!               "String": "application/json"
//!             }
//!           }
//!         ]
//!       }
//!     ]
//!   }
//! ]
//! ```
//! 
//! ### Saving and loading the starsystem
//! You can save the state of your starsystem to a file by calling the
//! ascend_being method on the starsystem for each being. This will
//! return a vector of AscendedBeings. You can then save this vector
//! to a file using any serialization method that you want.
//! 
//! ```rust
//! ...
//! // save the configuration to a file
//! // consolidate all ascended beings
//!     let mut beings: Vec<AscendedBeing<Edification>> = vec![];
//!     for val in ascended_configuration {
//!         beings.push(val);
//!     }
//! 
//!     // write to file
//!     let mut file = std::fs::File::create("./config.json").unwrap();
//!     serde_json::to_writer_pretty(&mut file, &beings).unwrap();
//! ...
//! ```
//! 
//! You can load the state of your starsystem from a file by calling the
//! 'develop_being' method on the starsystem for each being. This will
//! recreae the entities and properties that were saved to the file in
//! the new starsystem.
//! 
//! ```rust
//! ...
//! // load the configuration from a file
//!     let mut file = std::fs::File::open("./config.json").unwrap();
//!     let beings: Vec<AscendedBeing<Edification>> = serde_json::from_reader(&mut file).unwrap();
//! 
//!     // recreate a new starsystem
//!     let mut starsystem2 = StarSystem::<Edification>::new().await;
//!     // recreate the beings
//!     let configuration2 = starsystem2
//!         .conceive_being("config".to_string())
//!         .await
//!         .unwrap();
//! 
//!     // devalop the being by recreating the saved entities and properties in starsystem2
//!     starsystem2
//!         .develop_being(configuration2.clone(), beings)
//!         .await
//!         .unwrap();
//! 
//!     // ascend the configuration2
//!     let ascended_configuration = starsystem
//!         .ascend_being(configuration.clone())
//!         .await
//!         .unwrap();
//! 
//!     println!(
//!         "{}",
//!         serde_json::to_string_pretty(&ascended_configuration).unwrap()
//!     );
//! ...
//! ```
//! 
//! The output of the above code will be:
//! ```json
//! [
//!   {
//!     "name": "config",
//!     "id": "XbwpaoljoLIVcvEorzTMXSLTrClXCYRhobtoApxdnOCrPuSadUGZLstWaughFrVNCYkIErKnGUoLTvUNlPpNCutHvpXQGKvzsLCLOqooWhruxdnypKpjcWyQUYMAwuzH",
//!     "entities": [
//!       {
//!         "name": "headers",
//!         "id": "vNKPtfCwUpREgWSEBXnGDCfCxTXmIAVHPyYCOJGkyBSuHHonpUhOaPaZYzdTUGLnjxJQOHDSbaIJTSKXONoPmCkHYhCPgRuInnmRwOsCrxRqNKkVkcvNCejMRziuryHl",
//!         "components": [
//!           {
//!             "name": "Content-Type",
//!             "id": "ZyBMvhnNppLvZGCxtykAikHNsjhgzLMwdnXWpZSVKVybJyfdtKiObVdkngUSssHqkaXfdnFLQwhWjSZATWxVYJRBgimLGJtwrmgVtMzHdIQiMfMePtWRdlgltEqtsPLX",
//!             "data": {
//!               "String": "application/json"
//!             }
//!           }
//!         ]
//!       }
//!     ]
//!   }
//! ]
//! ```
//! 

pub use starsystem::*;
pub use uid::*;
pub use world::*;

mod starsystem;
mod uid;
mod world;

