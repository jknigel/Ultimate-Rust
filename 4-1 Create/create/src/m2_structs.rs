#[derive(Debug)]
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}

impl User {
    fn increment_signincount(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email:&str) {
        self.email = String::from(new_email);
    }
}

//Demo of traditional method
fn change_username(user:&mut User, new_username:&str) {
    user.username = String::from(new_username);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let mut user_1:User = User {
            username: String::from("username1"),
            email: String::from("username1@example.com"),
            active: true,
            sign_in_count: 1
        };

        change_username(&mut user_1, "anotherusername1");

        dbg!(user_1);

        let mut user_2:User = User {
            username: String::from("username2"),
            email: String::from("username2@example.com"),
            sign_in_count: 7,
            active: true
        };

        user_2.increment_signincount();
        user_2.change_email("anotherusername2@example.com");
        
        dbg!(user_2);
    }
}