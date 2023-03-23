mod repository;

use repository::{User, UserRepository};

trait Command {}
struct CreateUser<'a> {
    username: &'a str,
}
impl<'a> Command for CreateUser<'a> {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

impl<'a> CommandHandler<CreateUser<'a>> for User {
    type Context = dyn UserRepository<String, User>;
    type Result = Result<(), ()>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        ctx.set(cmd.username.to_string(), self.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeUserRepository {
        called_set_string: Option<String>,
        called_set_user: Option<User>,
    }

    impl UserRepository<String, User> for FakeUserRepository {
        fn set(&mut self, key: String, val: User) {
            self.called_set_string = Some(key);
            self.called_set_user = Some(val);
        }

        fn get(&self, _key: &String) -> Option<&User> {
            unreachable!("Isn't tested")
        }

        fn remove(&mut self, _key: &String) -> Option<User> {
            unreachable!("Isn't tested")
        }
    }

    #[test]
    fn test_command_handler_create_user_impl() {
        let id = 0;
        let email = "";
        let activated = true;

        let username = "user";

        let mut repository = FakeUserRepository::default();

        let user = User::new(id, email, activated);

        let res = user.handle_command(&CreateUser { username }, &mut repository);
        assert!(res.is_ok());

        assert!(
            matches!(repository.called_set_string, Some(set_username) if set_username == username)
        );
        assert!(matches!(
            repository.called_set_user,

            Some(User {
                id: set_user_id,
                email: set_user_email,
                activated: set_user_activated
            }) if set_user_id == id && set_user_email == email && set_user_activated == activated
        ))
    }
}

fn main() {}
