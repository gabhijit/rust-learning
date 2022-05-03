use std::cell::RefCell;
use std::collections::HashMap;

pub struct ShapeSystemRegistryEntry {
    instance_count: usize,
}

pub struct ShapeSystemRegistryEntryRef<'t> {
    instance_count: &'t mut usize,
}

pub struct ShapeSystemRegistry {
    shape_system_map: HashMap<usize, ShapeSystemRegistryEntry>,
}

impl ShapeSystemRegistry {
    fn get_mut(&mut self, id: usize) -> Option<ShapeSystemRegistryEntryRef> {
        self.shape_system_map.get_mut(&id).and_then(|t| {
            let instance_count = &mut t.instance_count;
            Some(ShapeSystemRegistryEntryRef { instance_count })
        })
    }

    fn register(&mut self, id: usize) -> ShapeSystemRegistryEntryRef {
        let entry = ShapeSystemRegistryEntry { instance_count: 0 };
        self.shape_system_map.insert(id, entry);
        self.get_mut(id).unwrap()
    }

    fn get_or_register_mut(&mut self, id: usize) -> ShapeSystemRegistryEntryRef {
        let shared_entry = self.shape_system_map.get(&id);
        match shared_entry {
            None => self.register(id),
            Some(_) => self.get_mut(id).unwrap(),
        }
    }
}

fn main() {
    let sr = ShapeSystemRegistryEntry {
        instance_count: 0_usize,
    };
}
