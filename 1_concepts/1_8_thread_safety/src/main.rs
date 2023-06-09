use std::{sync::Mutex, thread};

mod thread_structs;

use thread_structs::{NotSyncNorSend, OnlySend, OnlySync, SyncAndSend};

fn main() {
    let mutex = Mutex::new(1);

    if let Ok(mutex_guard) = mutex.lock() {
        let only_sync = OnlySync { value: mutex_guard };

        // we can borrow OnlySync immutably, since it's Sync
        thread::scope(|s| {
            s.spawn(|| {
                println!("{}", *only_sync.value);
            });
        });

        // OnlySync can't be sent to another thread since it's not Send (compile time error)
        /* thread::scope(|s| {
            s.spawn(move || {
                *only_sync.value = 2;
            });
        }); */
    }

    let only_send = OnlySend::default();

    // we can't borrow OnlySend from another thread (compile time error)
    /* thread::scope(|s| {
        s.spawn(|| {
            println!("{}", *(only_send.value.borrow()));
        });
    }); */

    // but we can move it to another thread
    thread::scope(|s| {
        s.spawn(move || {
            *(only_send.value.borrow_mut()) = 2;
        });
    });

    let mut sync_and_send = SyncAndSend::default();

    // we can borrow SyncAndSend
    thread::scope(|s| {
        s.spawn(|| {
            println!("{}", sync_and_send.value);
        });
    });

    // and also can move it to another thread
    thread::scope(|s| {
        s.spawn(move || {
            sync_and_send.value = 2;
        });
    });

    let mut not_sync_nor_send = NotSyncNorSend::default();

    // we can't borrow NotSyncNorSend (compile time error)
    /* thread::scope(|s| {
        s.spawn(|| {
            println!("{}", *not_sync_nor_send.value);
        });
    }); */

    // nor can we move it to another thread (compile time error)
    /* thread::scope(|s| {
        s.spawn(move || {
            Rc::get_mut(&mut not_sync_nor_send.value);
        });
    }); */
}
