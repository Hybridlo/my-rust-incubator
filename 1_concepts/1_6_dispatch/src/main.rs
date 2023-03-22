use std::{borrow::Cow, cell::RefCell, collections::HashMap, marker::PhantomData};

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

struct RepositoryUserNotFound;

trait UserRepository<K> {
    fn set(&self, key: K, val: User);
    fn get(&self, key: &K) -> Option<User>;
    // getting a user that doesn't exist just returns None,
    // but removing/updating a user that doesn't exist is an error
    fn remove(&self, key: &K) -> Result<(), RepositoryUserNotFound>;
    fn update(&self, key: K, val: &User) -> Result<(), RepositoryUserNotFound>;
}

struct UserRepositoryStatic<T, K>
where
    T: Storage<K, User>,
{
    storage: RefCell<T>,
    _storage_key: PhantomData<K>,
}

impl<T, K> UserRepositoryStatic<T, K>
where
    T: Storage<K, User>,
{
    pub fn new(storage: T) -> Self {
        Self {
            storage: RefCell::new(storage),
            _storage_key: PhantomData,
        }
    }
}

impl<T, K> UserRepository<K> for UserRepositoryStatic<T, K>
where
    T: Storage<K, User>,
{
    fn set(&self, key: K, val: User) {
        let mut storage = self.storage.borrow_mut();
        storage.set(key, val);
    }

    fn get(&self, key: &K) -> Option<User> {
        let storage = self.storage.borrow();
        let user_raw = storage.get(key)?;
        // maybe it comes in raw binary, or json

        Some(User {
            id: user_raw.id,
            email: user_raw.email.clone(),
            activated: user_raw.activated,
        })
    }

    fn remove(&self, key: &K) -> Result<(), RepositoryUserNotFound> {
        let mut storage = self.storage.borrow_mut();
        storage.remove(key).ok_or(RepositoryUserNotFound)?;

        Ok(())
    }

    fn update(&self, key: K, val: &User) -> Result<(), RepositoryUserNotFound> {
        let mut storage = self.storage.borrow_mut();
        // check user exists
        storage.get(&key).ok_or(RepositoryUserNotFound)?;

        // turn back into raw
        let user_raw = User {
            id: val.id,
            email: val.email.clone(),
            activated: val.activated,
        };

        storage.set(key, user_raw);

        Ok(())
    }
}

struct UserRepositoryDynamic<K> {
    storage: RefCell<Box<dyn Storage<K, User>>>,
}

impl<K> UserRepositoryDynamic<K> {
    pub fn new(storage: Box<dyn Storage<K, User>>) -> Self {
        Self {
            storage: RefCell::new(storage),
        }
    }
}

impl<K> UserRepository<K> for UserRepositoryDynamic<K> {
    fn set(&self, key: K, val: User) {
        let mut storage = self.storage.borrow_mut();
        storage.set(key, val);
    }

    fn get(&self, key: &K) -> Option<User> {
        let storage = self.storage.borrow();
        let user_raw = storage.get(key)?;
        // maybe it comes in raw binary, or json

        Some(User {
            id: user_raw.id,
            email: user_raw.email.clone(),
            activated: user_raw.activated,
        })
    }

    fn remove(&self, key: &K) -> Result<(), RepositoryUserNotFound> {
        let mut storage = self.storage.borrow_mut();
        storage.remove(key).ok_or(RepositoryUserNotFound)?;

        Ok(())
    }

    fn update(&self, key: K, val: &User) -> Result<(), RepositoryUserNotFound> {
        let mut storage = self.storage.borrow_mut();
        // check user exists
        storage.get(&key).ok_or(RepositoryUserNotFound)?;

        // turn back into raw
        let user_raw = User {
            id: val.id,
            email: val.email.clone(),
            activated: val.activated,
        };

        storage.set(key, user_raw);

        Ok(())
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

    let repository = UserRepositoryStatic::new(SomeStorage::default());
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
    println!(
        "remove success: {}",
        repository.remove(&"user2".to_string()).is_ok()
    );
    println!("{:#?}", repository.storage);
    println!(
        "remove fail: {}",
        repository.remove(&"user5".to_string()).is_err()
    );

    println!(
        "update success: {}",
        repository
            .update(
                "user4".to_string(),
                &User {
                    id: 6,
                    email: "email2@example.com".into(),
                    activated: false
                }
            )
            .is_ok()
    );
    println!("{:#?}", repository.storage);
    println!(
        "update fail: {}",
        repository
            .update(
                "user5".to_string(),
                &User {
                    id: 6,
                    email: "email2@example.com".into(),
                    activated: false
                }
            )
            .is_err()
    );
}

fn check_dynamic() {
    println!("\n\nStart dynamic check\n");

    let repository = UserRepositoryDynamic::new(Box::new(SomeStorage::default()));

    repository.set(
        "user4".to_string(),
        User {
            id: 4,
            email: "email4@example.com".into(),
            activated: false,
        },
    );

    let item = repository.get(&"user3".to_string());
    println!("{item:?}");
    assert!(repository.remove(&"user2".to_string()).is_ok());
    assert!(repository.remove(&"user5".to_string()).is_err());
    assert!(repository
        .update(
            "user4".to_string(),
            &User {
                id: 6,
                email: "email2@example.com".into(),
                activated: false
            }
        )
        .is_ok());
    assert!(repository
        .update(
            "user5".to_string(),
            &User {
                id: 6,
                email: "email2@example.com".into(),
                activated: false
            }
        )
        .is_err());
}

fn main() {
    check_static();
    check_dynamic();
}
