use im::{HashSet, HashMap};

#[derive(Clone)]
struct User {
    id: u64,
    name: String
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

trait UsersRepository {
    type UserCollection;

    fn user_by_id(&self, id: u64) -> Option<User>;
    fn users_by_ids(&self, ids: &[u64]) -> Self::UserCollection;
    fn ids_by_name_substring<S: AsRef<str>>(&self, name_sub: S) -> Vec<u64>;
}

#[derive(Default, Clone)]
struct UsersRepositotyImpl {
    users: HashMap<u64, User>,
    name_to_id: HashMap<String, u64>
}

impl UsersRepositotyImpl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(&mut self, user: User) {
        self.name_to_id.insert(user.name.clone(), user.id);
        self.users.insert(user.id, user);
    }
}

impl UsersRepository for UsersRepositotyImpl {
    type UserCollection = HashMap<u64, User>;

    fn user_by_id(&self, id: u64) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn users_by_ids(&self, ids: &[u64]) -> Self::UserCollection {
        let mut sorted_ids = ids.to_owned();
        sorted_ids.sort();

        let mut res = self.users.clone();
        res.retain(|k, _| ids.binary_search(k).is_ok());

        res
    }

    fn ids_by_name_substring<S: AsRef<str>>(&self, name_sub: S) -> Vec<u64> {
        let mut res = self.name_to_id.clone();
        res.retain(|k, _| k.contains(name_sub.as_ref()));

        res.into_iter().map(|(_, v)| v).collect()
    }   

    
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn user_by_id_finds_the_user() {
        let mut repository = UsersRepositotyImpl::new();
        repository.add_user(User { id: 1, name: "user1".to_string() });
        repository.add_user(User { id: 2, name: "user2".to_string() });
        repository.add_user(User { id: 3, name: "user3".to_string() });

        assert!(matches!(repository.user_by_id(1), Some(User { id, name }) if id == 1 && name == "user1"));
        assert!(matches!(repository.user_by_id(2), Some(User { id, name }) if id == 2 && name == "user2"));
        assert!(matches!(repository.user_by_id(3), Some(User { id, name }) if id == 3 && name == "user3"));
    }

    #[test]
    fn user_by_id_returns_none_if_no_id() {
        let mut repository = UsersRepositotyImpl::new();
        repository.add_user(User { id: 1, name: "user1".to_string() });

        assert!(matches!(repository.user_by_id(2), None));
    }

    #[test]
    fn users_by_ids_gets_existing_users() {
        let mut repository = UsersRepositotyImpl::new();
        repository.add_user(User { id: 1, name: "user1".to_string() });
        repository.add_user(User { id: 2, name: "user2".to_string() });
        repository.add_user(User { id: 3, name: "user3".to_string() });

        let found = repository.users_by_ids(&[1, 3, 5]);

        assert!(found.contains_key(&1));
        assert!(found.contains_key(&3));
        assert!(!found.contains_key(&5));
    }

    #[test]
    fn ids_by_name_substring_finds_right_ids() {
        let mut repository = UsersRepositotyImpl::new();
        repository.add_user(User { id: 1, name: "user1".to_string() });
        repository.add_user(User { id: 2, name: "user2".to_string() });
        repository.add_user(User { id: 3, name: "user3".to_string() });

        let found = repository.ids_by_name_substring("user");
        assert!(found.contains(&1));
        assert!(found.contains(&2));
        assert!(found.contains(&3));

        let found = repository.ids_by_name_substring("r2");
        assert!(!found.contains(&1));
        assert!(found.contains(&2));
        assert!(!found.contains(&3));
    }

    #[test]
    fn modifying_collection_clones_doesnt_affet_each_other() {
        let mut repository = UsersRepositotyImpl::new();
        repository.add_user(User { id: 1, name: "user1".to_string() });
        repository.add_user(User { id: 2, name: "user2".to_string() });
        repository.add_user(User { id: 3, name: "user3".to_string() });

        let mut repository2 = repository.clone();
        repository2.add_user(User { id: 4, name: "user4".to_string() });

        // repositoies have same users
        assert!(matches!(repository.user_by_id(3), Some(_)));
        assert!(matches!(repository2.user_by_id(3), Some(_)));

        // modifying repository2 didn't affect repository
        assert!(matches!(repository.user_by_id(4), None));
        assert!(repository.ids_by_name_substring("user4").is_empty());
    }
}