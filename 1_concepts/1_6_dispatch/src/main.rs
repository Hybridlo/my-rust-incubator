use std::{borrow::Cow, collections::HashMap};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct UserRepositoryStatic {}

impl UserRepositoryStatic {
    pub fn set<K, V>(storage: &mut impl Storage<K, V>, key: K, val: V) {
        storage.set(key, val);
    }

    pub fn get<'a, K, V>(storage: &'a impl Storage<K, V>, key: &K) -> Option<&'a V> {
        storage.get(key)
    }

    pub fn remove<K, V>(storage: &mut impl Storage<K, V>, key: &K) -> Option<V> {
        storage.remove(key)
    }
}

struct UserRepositoryDynamic {}

impl UserRepositoryDynamic {
    pub fn set<K, V>(storage: &mut dyn Storage<K, V>, key: K, val: V) {
        storage.set(key, val);
    }

    pub fn get<'a, K, V>(storage: &'a dyn Storage<K, V>, key: &K) -> Option<&'a V> {
        storage.get(key)
    }

    pub fn remove<K, V>(storage: &mut dyn Storage<K, V>, key: &K) -> Option<V> {
        storage.remove(key)
    }
}

#[derive(Debug)]
struct SomeStorage {
    storage: HashMap<String, User>,
}

impl Storage<String, User> for SomeStorage {
    fn set(&mut self, key: String, val: User) {
        self.storage.insert(key, val);
    }

    fn get(&self, key: &String) -> Option<&User> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &String) -> Option<User> {
        self.storage.remove(key)
    }
}

impl Default for SomeStorage {
    fn default() -> Self {
        Self {
            storage: [
                (
                    "user1".to_string(),
                    User {
                        id: 1,
                        email: "email1@example.com".into(),
                        activated: true,
                    },
                ),
                (
                    "user2".to_string(),
                    User {
                        id: 2,
                        email: "email2@example.com".into(),
                        activated: true,
                    },
                ),
                (
                    "user3".to_string(),
                    User {
                        id: 3,
                        email: "email3@example.com".into(),
                        activated: true,
                    },
                ),
            ]
            .into(),
        }
    }
}

fn check_static() {
    println!("\n\nStart static check\n");

    let mut storage = SomeStorage::default();
    println!("{storage:#?}");

    UserRepositoryStatic::set(
        &mut storage,
        "user4".to_string(),
        User {
            id: 4,
            email: "email4@example.com".into(),
            activated: false,
        },
    );
    println!("{storage:#?}");

    let item = UserRepositoryStatic::get(&storage, &"user3".to_string());
    println!("{item:?}");
    UserRepositoryStatic::remove(&mut storage, &"user2".to_string());
    println!("{storage:#?}");
}

fn check_dynamic() {
    println!("\n\nStart dynamic check\n");

    let mut storage = SomeStorage::default();
    println!("{storage:#?}");

    UserRepositoryDynamic::set(
        &mut storage,
        "user4".to_string(),
        User {
            id: 4,
            email: "email4@example.com".into(),
            activated: false,
        },
    );
    println!("{storage:#?}");

    let item = UserRepositoryDynamic::get(&storage, &"user3".to_string());
    println!("{item:?}");
    UserRepositoryDynamic::remove(&mut storage, &"user2".to_string());
    println!("{storage:#?}");
}

fn main() {
    check_static();
    check_dynamic();
}
