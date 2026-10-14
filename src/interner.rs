use std::{collections::HashMap, marker::PhantomData};
use bevy_ecs::resource::Resource;

use crate::Id;
/// Responsible for mapping the string ID of a stat to a u32 provided at runtime
/// String is only needed for registration & debug usage, more efficient to store large amounts of IDs as u32 instead of String
/// String ID is denoted as namespace:id (eg, base:sword, mod:gun)
#[derive(Resource)]
pub struct Interner<T> {
    to_id: HashMap<String, Id<T>>,
    to_string: Vec<String>,
}

impl<T> Default for Interner<T> {
    fn default() -> Self {
        let id_map: HashMap<String, Id<T>> = HashMap::new();
        let string_map: Vec<String> = vec![];
        Self { 
            to_id: id_map,
            to_string: string_map
        }
    }
}

impl <T> Interner<T> {
    /// Get ID from string if it exists, otherwise register an ID from string
    pub fn intern(&mut self, s: &str) -> Id<T> {
        if let Some(&id) = self.to_id.get(s) {
            return id;
        }
        let id = Id { index: self.to_string.len() as u32, _marker: PhantomData };
        self.to_string.push(s.to_string());
        self.to_id.insert(s.to_string(), id);
        id
    }

    /// Get ID from string if it exists
    pub fn get_existing(&self, s: &str) -> Option<Id<T>> {
        self.to_id.get(s).copied()
    }

    /// Get string from ID if it exists
    pub fn resolve(&self, id: Id<T>) -> &str {
        &self.to_string[id.index as usize]
    }
}