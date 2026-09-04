use bevy_ecs::resource::Resource;

use crate::interner::Id;

#[derive(Resource)]
pub struct Registry<T> {
    entries: Vec<Option<T>>
}

impl<T> Registry<T> {
    /// Increase size of vec storing data if necessary
    fn ensure_capacity(&mut self, id: Id<T>) {
        let needed = id.index as usize + 1;
        if self.entries.len() < needed {
            self.entries.resize_with(needed,  || None);
        }
    }

    /// Attempts to register definition by this ID, errors if ID already exists
    pub fn register(&mut self, id: Id<T>, def: T) -> Result<(), String> {
        self.ensure_capacity(id);
        let slot = &mut self.entries[id.index as usize];
        if slot.is_some() {
            return Err(format!("Duplicate ID registered with {id:?}"));
        }
        *slot = Some(def);
        Ok(())
    }

    /// Registers definition by ID, overriding existing data if any
    pub fn register_or_override(&mut self, id: Id<T>, def: T) {
        self.ensure_capacity(id);
        let slot = &mut self.entries[id.index as usize];
        if slot.is_some() {
            //warn!("Overriding existing definition: {}", def.id_str());
        }
        *slot = Some(def);
    }

    /// Return definition data corresponding to this ID, or None
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.entries.get(id.index as usize)?.as_ref()
    }
}