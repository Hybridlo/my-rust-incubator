mod post {
    pub mod types {
        #[derive(Clone, Debug, PartialEq)]
        pub struct Id(u64);
        impl From<u64> for Id {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }
    
        #[derive(Clone, Debug, PartialEq)]
        pub struct Title(String);
        impl From<String> for Title {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    
        #[derive(Clone, Debug, PartialEq)]
        pub struct Body(String);
        impl From<String> for Body {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    }

    pub mod states {
        pub struct New;
        pub struct Unmoderated { pub id: super::types::Id }
        pub struct Published { pub id: super::types::Id }
        pub struct Deleted;
    }
}
mod user {
    pub mod types {
        #[derive(Clone, Debug, PartialEq)]
        pub struct Id(u64);
        impl From<u64> for Id {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

    }
}

#[derive(Clone)]
struct Post<State> {
    user_id: user::types::Id,
    title: post::types::Title,
    body: post::types::Body,
    state: State
}

impl Post<post::states::New> {
    pub fn new(user_id: user::types::Id, title: post::types::Title, body: post::types::Body) -> Self {
        Self {
            user_id,
            title,
            body,
            state: post::states::New,
        }
    }

    pub fn publish(self, new_id: post::types::Id) -> Post<post::states::Unmoderated> {
        Post {
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: post::states::Unmoderated { id: new_id },
        }
    }
}

impl Post<post::states::Unmoderated> {
    pub fn allow(self) -> Post<post::states::Published> {
        Post {
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: post::states::Published { id: self.state.id },
        }
    }

    pub fn deny(self) -> Post<post::states::Deleted> {
        Post {
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: post::states::Deleted,
        }
    }
}

impl Post<post::states::Published> {
    pub fn delete(self) -> Post<post::states::Deleted> {
        Post {
            user_id: self.user_id,
            title: self.title,
            body: self.body,
            state: post::states::Deleted,
        }
    }
}

fn main() {
    let my_new_post = Post::new(
        10.into(),
        "My new cool post".to_string().into(),
        "This cool post is probably about Rust, it's correctness, safety and performance".to_string().into()
    );

    // post is not sent/published yet, can't delete it (compile time error)
    // my_new_post.delete();

    let my_new_post = my_new_post.publish(1.into());
    let my_new_post = my_new_post.allow();
    let my_new_post = my_new_post.delete();

    // deleted post is gone, can't do anything with it anymore (compile time error)
    // my_new_post.allow();
    // my_new_post.delete();

    let my_another_post = Post::new(
        10.into(),
        "My second cool post".to_string().into(),
        "Another post about coolness of Rust".to_string().into()
    );

    let my_another_post = my_another_post.publish(2.into());
    let my_another_post = my_another_post.deny();

    // it was denied already, can't delete it (compile time error)
    // my_another_post.delete();
}
