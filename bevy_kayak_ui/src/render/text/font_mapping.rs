use bevy::{prelude::Handle, utils::HashMap};

use super::font::KayakFont;

pub struct FontMapping {
    count: u16,
    font_ids: HashMap<Handle<KayakFont>, u16>,
    font_handles: HashMap<u16, Handle<KayakFont>>,
}

impl Default for FontMapping {
    fn default() -> Self {
        Self {
            count: 0,
            font_ids: HashMap::default(),
            font_handles: HashMap::default(),
        }
    }
}

impl FontMapping {
    pub(crate) fn add(&mut self, handle: Handle<KayakFont>) -> u16 {
        if !self.font_ids.contains_key(&handle) {
            let id = self.count;
            self.font_ids.insert(handle.clone(), id);
            self.font_handles.insert(id, handle);
            self.count += 1;

            id
        } else {
            *self.font_ids.get(&handle).unwrap()
        }
    }

    pub(crate) fn get_handle(&self, id: u16) -> Option<Handle<KayakFont>> {
        self.font_handles
            .get(&id)
            .and_then(|item| Some(item.clone()))
    }
}
