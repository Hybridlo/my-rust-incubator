use std::{cell::RefCell, rc::Rc, sync::MutexGuard};

pub struct OnlySync<'a> {
    pub value: MutexGuard<'a, i32>,
}

// RefCell is only Send because you can't have multiple shared references on it
// because it allows mutation of values without synchronization
#[derive(Default)]
pub struct OnlySend {
    pub value: RefCell<i32>,
}

// most standard types are Sync and Send, since they don't break borrowing rules
#[derive(Default)]
pub struct SyncAndSend {
    pub value: i32,
}

// Rc by itself is not sync nor send, because cloning mutates the counter non atomically
#[derive(Default)]
pub struct NotSyncNorSend {
    pub value: Rc<i32>,
}
