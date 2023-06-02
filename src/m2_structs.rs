#[cfg(test)]
mod test {
  #[derive(Debug)]
  #[allow(dead_code)]
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
  }
  
  impl User {
    fn increment_signin_count(&mut self) {
      self.sign_in_count += 1;
    }
  
    fn change_email(&mut self, new_email: &str) {
      self.email = String::from(new_email);
    }
  }
  
  fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
  }

  #[test]
  fn tests_structs() {
    let mut user_1: User = User {
      username: String::from("someusername1"),
      email: String::from("someone@example.com"),
      active: true,
      sign_in_count: 1
    };
    
    change_username(&mut user_1, "somenewusername");

    dbg!(user_1);

    let mut user_2: User = User {
      username: String::from("someusername2"),
      email: String::from("someone2@example2.com"),
      active: false,
      sign_in_count: 7  
    };

    user_2.increment_signin_count();

    user_2.change_email("anotheremail@email.com");

    dbg!(user_2);
  }
}
