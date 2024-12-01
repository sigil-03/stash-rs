use uuid::Uuid;

pub struct Item {
    name: String,
}

impl Item {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

pub trait Stash {
    fn add(&mut self, item: Item) -> Uuid;
    fn remove(&self, uuid: Uuid);
    fn inspect(&self, uuid: Uuid);
}
