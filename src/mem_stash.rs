use crate::stash::{Item, Stash};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Entry {
    id: Uuid,
    item: Item,
}

impl Entry {
    pub fn new(item: Item) -> Self {
        Self {
            id: Uuid::new_v4(),
            item,
        }
    }

    /// Returns the id of the entry
    pub fn id(&self) -> Uuid {
        self.id.clone()
    }
}

/// A [`stash::Stash`] which is stored in memory using a hashmap
pub struct MemStash {
    storage: HashMap<Uuid, Entry>,
}

impl MemStash {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl Stash for MemStash {
    fn add(&mut self, item: Item) -> Uuid {
        let entry = Entry::new(item);
        let id = entry.id();
        self.storage.insert(entry.id(), entry);
        id
    }
    fn remove(&self, uuid: Uuid) {
        todo!("remove item from stash");
    }
    fn inspect(&self, uuid: Uuid) {
        todo!("inspect item in stash");
    }
}
