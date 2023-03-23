use std::{borrow::Cow, collections::HashMap, marker::PhantomData};

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) email: Cow<'static, str>,
    pub(crate) activated: bool,
}

impl User {
    pub fn new(id: u64, email: &'static str, activated: bool) -> Self {
        Self {
            id,
            email: email.into(),
            activated,
        }
    }
}

pub trait UserRepository<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct UserRepositoryStatic<T, K, V>
where
    T: Storage<K, V>,
{
    storage: T,
    _storage_key: PhantomData<K>,
    _storage_val: PhantomData<V>,
}

impl<T, K, V> UserRepositoryStatic<T, K, V>
where
    T: Storage<K, V>,
{
    pub fn new(storage: T) -> Self {
        Self {
            storage,
            _storage_key: PhantomData,
            _storage_val: PhantomData,
        }
    }
}

impl<T, K, V> UserRepository<K, V> for UserRepositoryStatic<T, K, V>
where
    T: Storage<K, V>,
{
    fn set(&mut self, key: K, val: V) {
        self.storage.set(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key)
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

    let mut repository = UserRepositoryStatic::new(SomeStorage::default());
    println!("{:#?}", repository.storage);

    repository.set(
        "user4".to_string(),
        User {
            id: 4,
            email: "email4@example.com".into(),
            activated: false,
        },
    );
    println!("{:#?}", repository.storage);

    let item = repository.get(&"user3".to_string());
    println!("{item:?}");
    repository.remove(&"user2".to_string());
    println!("{:#?}", repository.storage);
}
