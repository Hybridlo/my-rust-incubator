use std::{rc::Rc, cell::RefCell};

pub struct OnlySync {
    value: Rc<i32>,
}

impl OnlySync {
    pub fn new(val: i32) -> Self {
        Self {
            value: Rc::new(val),
        }
    }
}

impl OnlySync {
    pub fn get_value(&self) -> i32 {
        *self.value
    }

    pub fn change_value(&mut self, new_val: i32) {
        let mut_value = Rc::get_mut(&mut self.value).expect("There can only be one Rc in OnlySync");
        *mut_value = new_val;
    }
}

// safety: You can't clone an Rc from this, and change_value needs an exclusive reference
unsafe impl Sync for OnlySync {}

// RefCell is only Send because you can't have multiple shared references on it
// because it allows mutation of values without synchronization
#[derive(Default)]
pub struct OnlySend {
    pub value: RefCell<i32>
}

// most standard types are Sync and Send, since they don't break borrowing rules
#[derive(Default)]
pub struct SyncAndSend {
    pub value: i32
}

// Rc by itself is not sync nor send, because cloning mutates the counter non atomically
#[derive(Default)]
pub struct NotSyncNorSend {
    pub value: Rc<i32>
}